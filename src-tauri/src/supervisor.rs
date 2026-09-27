//! The single authority over every bot process: start / stop / restart,
//! crash detection, exponential backoff, log fan-out.

use std::path::PathBuf;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use serde::Serialize;
use tokio::io::{AsyncBufReadExt, AsyncRead, BufReader};
use tokio::sync::oneshot;

use crate::config::BotSpec;
use crate::logs::{BotLogger, LogLine, Stream};
use crate::process;

pub const BACKOFF_BASE: Duration = Duration::from_secs(3);
pub const BACKOFF_MAX: Duration = Duration::from_secs(300);
/// A run at least this long resets the backoff attempt counter.
pub const STABLE_AFTER: Duration = Duration::from_secs(60);
const STOP_TIMEOUT: Duration = Duration::from_secs(10);

/// `min(3s * 2^attempt, 300s)`.
pub fn backoff_delay(attempt: u32) -> Duration {
    let factor = 1u32 << attempt.min(16);
    BACKOFF_BASE.saturating_mul(factor).min(BACKOFF_MAX)
}

#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum BotState {
    Stopped,
    Starting,
    Running { pid: u32, since_ms: i64 },
    Backoff { until_ms: i64, attempt: u32 },
    Stopping,
}

#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
pub struct StateEvent {
    pub id: String,
    pub state: BotState,
    pub restarts: u32,
}

#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
pub struct LogEvent {
    pub id: String,
    #[serde(flatten)]
    pub line: LogLine,
}

/// Where state/log events go (the Tauri window in production, a channel in tests).
pub trait EventSink: Send + Sync + 'static {
    fn on_state(&self, event: StateEvent);
    fn on_log(&self, event: LogEvent);
}

struct Shared {
    id: String,
    sink: Arc<dyn EventSink>,
    logger: Mutex<BotLogger>,
    state: Mutex<BotState>,
    restarts: AtomicU32,
}

impl Shared {
    fn set_state(&self, state: BotState) {
        *self.state.lock().unwrap() = state.clone();
        self.sink.on_state(StateEvent {
            id: self.id.clone(),
            state,
            restarts: self.restarts.load(Ordering::Relaxed),
        });
    }

    fn log(&self, stream: Stream, text: impl Into<String>) {
        let line = self.logger.lock().unwrap().append(stream, text);
        self.sink.on_log(LogEvent { id: self.id.clone(), line });
    }

    fn status(&self) -> StateEvent {
        StateEvent {
            id: self.id.clone(),
            state: self.state.lock().unwrap().clone(),
            restarts: self.restarts.load(Ordering::Relaxed),
        }
    }
}

/// A live supervise task. Both fields are taken by the first `stop` so the
/// slot stays occupied (start is rejected) until the task has really ended.
struct Run {
    stop_tx: Option<oneshot::Sender<()>>,
    done: Option<tauri::async_runtime::JoinHandle<()>>,
}

struct Slot {
    spec: BotSpec,
    shared: Arc<Shared>,
    run: Option<Run>,
}

pub struct Supervisor {
    sink: Arc<dyn EventSink>,
    logs_dir: PathBuf,
    slots: Mutex<Vec<Slot>>,
}

impl Supervisor {
    pub fn new(sink: Arc<dyn EventSink>, logs_dir: PathBuf, bots: &[BotSpec]) -> Self {
        let sup = Self { sink, logs_dir, slots: Mutex::new(Vec::new()) };
        for spec in bots {
            sup.upsert(spec.clone());
        }
        sup
    }

    /// Add or replace a bot definition. A running bot keeps running with its
    /// old definition until the next start.
    pub fn upsert(&self, spec: BotSpec) {
        let mut slots = self.slots.lock().unwrap();
        match slots.iter_mut().find(|s| s.shared.id == spec.id) {
            Some(slot) => slot.spec = spec,
            None => {
                let shared = Arc::new(Shared {
                    id: spec.id.clone(),
                    sink: self.sink.clone(),
                    logger: Mutex::new(BotLogger::new(&self.logs_dir, &spec.id)),
                    state: Mutex::new(BotState::Stopped),
                    restarts: AtomicU32::new(0),
                });
                slots.push(Slot { spec, shared, run: None });
            }
        }
    }

    /// Take the slot out first so no `start` can reach it, then stop its run.
    pub async fn remove(&self, id: &str) -> Result<(), String> {
        let slot = {
            let mut slots = self.slots.lock().unwrap();
            let i = slots.iter().position(|s| s.shared.id == id).ok_or_else(|| not_found(id))?;
            slots.remove(i)
        };
        if let Some(mut run) = slot.run {
            if let (Some(stop_tx), Some(done)) = (run.stop_tx.take(), run.done.take()) {
                finish_run(&slot.shared, stop_tx, done).await;
            }
        }
        Ok(())
    }

    pub fn specs(&self) -> Vec<BotSpec> {
        self.slots.lock().unwrap().iter().map(|s| s.spec.clone()).collect()
    }

    pub fn statuses(&self) -> Vec<StateEvent> {
        self.slots.lock().unwrap().iter().map(|s| s.shared.status()).collect()
    }

    pub fn log_tail(&self, id: &str) -> Result<Vec<LogLine>, String> {
        let shared = self.shared_of(id)?;
        let tail = shared.logger.lock().unwrap().tail();
        Ok(tail)
    }

    fn shared_of(&self, id: &str) -> Result<Arc<Shared>, String> {
        self.slots
            .lock()
            .unwrap()
            .iter()
            .find(|s| s.shared.id == id)
            .map(|s| s.shared.clone())
            .ok_or_else(|| not_found(id))
    }

    pub fn start(&self, id: &str) -> Result<(), String> {
        let mut slots = self.slots.lock().unwrap();
        let slot = slots
            .iter_mut()
            .find(|s| s.shared.id == id)
            .ok_or_else(|| not_found(id))?;
        if slot.run.is_some() {
            return Err(format!("「{}」已在執行中", slot.spec.name));
        }
        slot.shared.restarts.store(0, Ordering::Relaxed);
        let (stop_tx, stop_rx) = oneshot::channel();
        let done = tauri::async_runtime::spawn(supervise(slot.shared.clone(), slot.spec.clone(), stop_rx));
        slot.run = Some(Run { stop_tx: Some(stop_tx), done: Some(done) });
        Ok(())
    }

    /// Stop and wait until the process is gone. Stopping a stopped bot is a no-op.
    pub async fn stop(&self, id: &str) -> Result<(), String> {
        let shared = self.shared_of(id)?;
        self.stop_shared(&shared).await;
        Ok(())
    }

    pub async fn restart(&self, id: &str) -> Result<(), String> {
        self.stop(id).await?;
        self.start(id)
    }

    pub async fn shutdown_all(&self) {
        let all: Vec<Arc<Shared>> = self.slots.lock().unwrap().iter().map(|s| s.shared.clone()).collect();
        for shared in all {
            self.stop_shared(&shared).await;
        }
    }

    /// Take the run's handles (leaving the slot occupied) and stop it. Only
    /// then free the slot so a new start cannot overlap the dying process.
    async fn stop_shared(&self, shared: &Arc<Shared>) {
        let taken = {
            let mut slots = self.slots.lock().unwrap();
            let Some(slot) = slots.iter_mut().find(|s| s.shared.id == shared.id) else { return };
            let Some(run) = slot.run.as_mut() else { return };
            (run.stop_tx.take(), run.done.take())
        };
        let (Some(stop_tx), Some(done)) = taken else {
            // Another stop is already in flight; it owns the cleanup.
            return;
        };
        finish_run(shared, stop_tx, done).await;
        if let Some(slot) = self.slots.lock().unwrap().iter_mut().find(|s| s.shared.id == shared.id) {
            slot.run = None;
        }
    }
}

/// Signal stop and wait; on timeout abort the task (dropping its Job kills
/// the process tree).
async fn finish_run(
    shared: &Shared,
    stop_tx: oneshot::Sender<()>,
    mut done: tauri::async_runtime::JoinHandle<()>,
) {
    shared.set_state(BotState::Stopping);
    let _ = stop_tx.send(());
    if tokio::time::timeout(STOP_TIMEOUT, &mut done).await.is_err() {
        done.abort();
        shared.log(Stream::System, "停止逾時，強制終止");
    }
    if *shared.state.lock().unwrap() != BotState::Stopped {
        shared.set_state(BotState::Stopped);
    }
}

fn not_found(id: &str) -> String {
    format!("找不到 bot：{id}")
}

fn now_ms() -> i64 {
    chrono::Utc::now().timestamp_millis()
}

fn spawn_reader<R>(shared: Arc<Shared>, reader: Option<R>, stream: Stream) -> tauri::async_runtime::JoinHandle<()>
where
    R: AsyncRead + Unpin + Send + 'static,
{
    tauri::async_runtime::spawn(async move {
        let Some(reader) = reader else { return };
        // Raw bytes, not `lines()`: one non-UTF-8 line must not end the
        // reader, or the undrained pipe fills up and the bot blocks on write.
        let mut reader = BufReader::new(reader);
        let mut buf = Vec::new();
        loop {
            buf.clear();
            match reader.read_until(b'\n', &mut buf).await {
                Ok(0) | Err(_) => break,
                Ok(_) => {}
            }
            while matches!(buf.last(), Some(b'\n' | b'\r')) {
                buf.pop();
            }
            shared.log(stream, process::decode_output(&buf));
        }
    })
}

async fn supervise(shared: Arc<Shared>, spec: BotSpec, mut stop_rx: oneshot::Receiver<()>) {
    let mut attempt: u32 = 0;
    loop {
        shared.set_state(BotState::Starting);
        match process::spawn(&spec) {
            Err(e) => shared.log(Stream::System, format!("啟動失敗：{e}")),
            Ok((mut child, job)) => {
                let pid = child.id().unwrap_or(0);
                let started = Instant::now();
                shared.set_state(BotState::Running { pid, since_ms: now_ms() });
                shared.log(Stream::System, format!("已啟動（pid {pid}）"));
                let out = spawn_reader(shared.clone(), child.stdout.take(), Stream::Stdout);
                let err = spawn_reader(shared.clone(), child.stderr.take(), Stream::Stderr);
                let exit = tokio::select! {
                    status = child.wait() => Some(status),
                    _ = &mut stop_rx => None,
                };
                // Kill the whole tree (on a crash: whatever the bot left
                // behind). That also closes pipes a grandchild inherited, so
                // the readers can finish.
                drop(job);
                let _ = child.wait().await;
                let _ = tokio::join!(out, err);
                let Some(status) = exit else {
                    shared.log(Stream::System, "已停止");
                    shared.set_state(BotState::Stopped);
                    return;
                };
                let code = status.ok().and_then(|s| s.code());
                shared.log(
                    Stream::System,
                    match code {
                        Some(c) => format!("程序結束（exit code {c}）"),
                        None => "程序結束".to_string(),
                    },
                );
                if started.elapsed() >= STABLE_AFTER {
                    attempt = 0;
                }
            }
        }

        let delay = backoff_delay(attempt);
        shared.restarts.fetch_add(1, Ordering::Relaxed);
        shared.set_state(BotState::Backoff { until_ms: now_ms() + delay.as_millis() as i64, attempt });
        shared.log(Stream::System, format!("{} 秒後自動重啟（第 {} 次）", delay.as_secs(), attempt + 1));
        attempt = attempt.saturating_add(1);
        tokio::select! {
            _ = tokio::time::sleep(delay) => {}
            _ = &mut stop_rx => {
                shared.log(Stream::System, "已停止");
                shared.set_state(BotState::Stopped);
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;

    #[test]
    fn backoff_table() {
        let cases = [(0, 3), (1, 6), (2, 12), (3, 24), (4, 48), (5, 96), (6, 192), (7, 300), (8, 300), (20, 300)];
        for (attempt, secs) in cases {
            assert_eq!(backoff_delay(attempt), Duration::from_secs(secs), "attempt {attempt}");
        }
    }

    struct ChannelSink(Mutex<mpsc::Sender<StateEvent>>);
    impl EventSink for ChannelSink {
        fn on_state(&self, event: StateEvent) {
            let _ = self.0.lock().unwrap().send(event);
        }
        fn on_log(&self, _event: LogEvent) {}
    }

    fn cmd_spec(id: &str, args: &[&str]) -> BotSpec {
        BotSpec {
            id: id.into(),
            name: id.into(),
            exe: PathBuf::from("cmd.exe"),
            args: args.iter().map(|s| s.to_string()).collect(),
            cwd: Some(std::env::temp_dir()),
            ..Default::default()
        }
    }

    fn wait_for(rx: &mpsc::Receiver<StateEvent>, pred: impl Fn(&BotState) -> bool) -> StateEvent {
        let deadline = Instant::now() + Duration::from_secs(10);
        loop {
            let left = deadline.saturating_duration_since(Instant::now());
            let ev = rx.recv_timeout(left).expect("expected state event before timeout");
            if pred(&ev.state) {
                return ev;
            }
        }
    }

    #[tokio::test]
    async fn crash_goes_to_backoff_and_stop_ends_in_stopped() {
        let (tx, rx) = mpsc::channel();
        let dir = tempfile::tempdir().unwrap();
        let sup = Supervisor::new(
            Arc::new(ChannelSink(Mutex::new(tx))),
            dir.path().to_path_buf(),
            &[cmd_spec("crash", &["/c", "exit 1"])],
        );

        sup.start("crash").unwrap();
        wait_for(&rx, |s| matches!(s, BotState::Running { .. }));
        let backoff = wait_for(&rx, |s| matches!(s, BotState::Backoff { .. }));
        assert!(matches!(backoff.state, BotState::Backoff { attempt: 0, .. }), "{:?}", backoff.state);
        assert_eq!(backoff.restarts, 1);
        assert!(sup.start("crash").is_err(), "double start must be rejected");

        sup.stop("crash").await.unwrap();
        wait_for(&rx, |s| *s == BotState::Stopping);
        wait_for(&rx, |s| *s == BotState::Stopped);
        let tail = sup.log_tail("crash").unwrap();
        assert!(tail.iter().any(|l| l.line.contains("exit code 1")), "{tail:?}");
        assert!(tail.iter().any(|l| l.line == "已停止"));
        sup.start("crash").unwrap();
        sup.shutdown_all().await;
        assert!(sup.statuses().iter().all(|s| s.state == BotState::Stopped));
    }

    #[tokio::test]
    async fn stop_kills_a_long_running_child_and_restart_works() {
        let (tx, rx) = mpsc::channel();
        let dir = tempfile::tempdir().unwrap();
        let sup = Supervisor::new(
            Arc::new(ChannelSink(Mutex::new(tx))),
            dir.path().to_path_buf(),
            &[cmd_spec("long", &["/c", "ping -n 60 127.0.0.1 >nul"])],
        );
        sup.start("long").unwrap();
        wait_for(&rx, |s| matches!(s, BotState::Running { .. }));
        let t = Instant::now();
        sup.stop("long").await.unwrap();
        assert!(t.elapsed() < Duration::from_secs(5));
        wait_for(&rx, |s| *s == BotState::Stopped);
        assert!(sup.stop("long").await.is_ok(), "stopping a stopped bot is a no-op");
        sup.start("long").unwrap();
        wait_for(&rx, |s| matches!(s, BotState::Running { .. }));
        sup.restart("long").await.unwrap();
        wait_for(&rx, |s| matches!(s, BotState::Running { .. }));
        sup.shutdown_all().await;
        assert!(sup.statuses().iter().all(|s| s.state == BotState::Stopped));
    }

    #[tokio::test]
    async fn stop_also_kills_what_the_bot_spawned() {
        // cmd starts ping as its own child, and ping holds the stdout pipe.
        let (tx, rx) = mpsc::channel();
        let dir = tempfile::tempdir().unwrap();
        let sup = Supervisor::new(
            Arc::new(ChannelSink(Mutex::new(tx))),
            dir.path().to_path_buf(),
            &[cmd_spec("tree", &["/c", "ping -n 60 127.0.0.1"])],
        );
        sup.start("tree").unwrap();
        wait_for(&rx, |s| matches!(s, BotState::Running { .. }));
        tokio::time::sleep(Duration::from_millis(500)).await;
        let t = Instant::now();
        sup.stop("tree").await.unwrap();
        assert!(t.elapsed() < Duration::from_secs(5), "stop took {:?}", t.elapsed());
        let tail = sup.log_tail("tree").unwrap();
        assert!(!tail.iter().any(|l| l.line.contains("逾時")), "{tail:?}");
    }

    #[tokio::test]
    async fn non_utf8_output_does_not_stop_the_log() {
        let (tx, rx) = mpsc::channel();
        let dir = tempfile::tempdir().unwrap();
        let text = dir.path().join("big5.txt");
        std::fs::write(&text, b"before\r\n\xB4\xFA\xB8\xD5\r\nafter\r\n").unwrap();
        let mut spec = cmd_spec("enc", &["/c", "type big5.txt"]);
        spec.cwd = Some(dir.path().to_path_buf());
        let sup = Supervisor::new(Arc::new(ChannelSink(Mutex::new(tx))), dir.path().to_path_buf(), &[spec]);
        sup.start("enc").unwrap();
        wait_for(&rx, |s| matches!(s, BotState::Backoff { .. }));
        let lines: Vec<String> = sup.log_tail("enc").unwrap().into_iter().map(|l| l.line).collect();
        assert!(lines.contains(&"before".to_string()), "{lines:?}");
        assert!(lines.contains(&"after".to_string()), "{lines:?}");
        sup.shutdown_all().await;
    }

    #[tokio::test]
    async fn remove_stops_the_bot_and_frees_the_id() {
        let (tx, rx) = mpsc::channel();
        let dir = tempfile::tempdir().unwrap();
        let sup = Supervisor::new(
            Arc::new(ChannelSink(Mutex::new(tx))),
            dir.path().to_path_buf(),
            &[cmd_spec("gone", &["/c", "ping -n 60 127.0.0.1 >nul"])],
        );
        sup.start("gone").unwrap();
        wait_for(&rx, |s| matches!(s, BotState::Running { .. }));
        sup.remove("gone").await.unwrap();
        wait_for(&rx, |s| *s == BotState::Stopped);
        assert!(sup.start("gone").is_err(), "a removed bot cannot be started");
        assert!(sup.statuses().is_empty());
    }
}
