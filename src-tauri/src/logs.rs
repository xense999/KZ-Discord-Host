//! Per-bot log: in-memory ring (for the UI) + size-rotated file on disk.

use std::collections::VecDeque;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

use serde::Serialize;

pub const RING_CAPACITY: usize = 500;
pub const MAX_FILE_BYTES: u64 = 5 * 1024 * 1024;
pub const KEEP_ROTATED: u32 = 3;

#[derive(Serialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Stream {
    Stdout,
    Stderr,
    /// Messages from the host itself (started, exited, restarting...).
    System,
}

impl Stream {
    fn tag(self) -> &'static str {
        match self {
            Stream::Stdout => "OUT",
            Stream::Stderr => "ERR",
            Stream::System => "SYS",
        }
    }
}

#[derive(Serialize, Clone, Debug, PartialEq, Eq)]
pub struct LogLine {
    pub ts: String,
    pub stream: Stream,
    pub line: String,
}

pub struct BotLogger {
    path: PathBuf,
    ring: VecDeque<LogLine>,
    ring_capacity: usize,
    max_file_bytes: u64,
    file: Option<File>,
}

impl BotLogger {
    pub fn new(logs_dir: &Path, bot_id: &str) -> Self {
        Self::with_limits(logs_dir, bot_id, RING_CAPACITY, MAX_FILE_BYTES)
    }

    pub fn with_limits(logs_dir: &Path, bot_id: &str, ring_capacity: usize, max_file_bytes: u64) -> Self {
        Self {
            path: logs_dir.join(format!("{bot_id}.log")),
            ring: VecDeque::with_capacity(ring_capacity),
            ring_capacity,
            max_file_bytes,
            file: None,
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Record one line: push to the ring, append to the file (rotating first
    /// when the file is over the limit). File errors are swallowed so a full
    /// disk never takes the supervisor down.
    pub fn append(&mut self, stream: Stream, line: impl Into<String>) -> LogLine {
        let entry = LogLine {
            ts: chrono::Local::now().format("%Y-%m-%dT%H:%M:%S%.3f").to_string(),
            stream,
            line: line.into(),
        };
        if self.ring.len() == self.ring_capacity {
            self.ring.pop_front();
        }
        self.ring.push_back(entry.clone());
        let _ = self.write_file(&entry);
        entry
    }

    pub fn tail(&self) -> Vec<LogLine> {
        self.ring.iter().cloned().collect()
    }

    fn write_file(&mut self, entry: &LogLine) -> std::io::Result<()> {
        if self.file.is_none() {
            if let Some(parent) = self.path.parent() {
                fs::create_dir_all(parent)?;
            }
            self.file = Some(OpenOptions::new().create(true).append(true).open(&self.path)?);
        }
        let over_limit = self
            .file
            .as_ref()
            .and_then(|f| f.metadata().ok())
            .is_some_and(|m| m.len() >= self.max_file_bytes);
        if over_limit {
            self.file = None;
            self.rotate()?;
            self.file = Some(OpenOptions::new().create(true).append(true).open(&self.path)?);
        }
        let file = self.file.as_mut().expect("file opened above");
        writeln!(file, "{} [{}] {}", entry.ts, entry.stream.tag(), entry.line)
    }

    fn rotated(&self, n: u32) -> PathBuf {
        self.path.with_extension(format!("{n}.log"))
    }

    /// `<id>.log` -> `<id>.1.log` -> ... -> `<id>.{KEEP}.log` (oldest dropped).
    fn rotate(&self) -> std::io::Result<()> {
        let oldest = self.rotated(KEEP_ROTATED);
        if oldest.exists() {
            fs::remove_file(&oldest)?;
        }
        for n in (1..KEEP_ROTATED).rev() {
            let from = self.rotated(n);
            if from.exists() {
                fs::rename(&from, self.rotated(n + 1))?;
            }
        }
        if self.path.exists() {
            fs::rename(&self.path, self.rotated(1))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ring_keeps_only_last_n_lines() {
        let dir = tempfile::tempdir().unwrap();
        let mut log = BotLogger::with_limits(dir.path(), "bot", 3, MAX_FILE_BYTES);
        for i in 0..5 {
            log.append(Stream::Stdout, format!("line {i}"));
        }
        let tail: Vec<String> = log.tail().into_iter().map(|l| l.line).collect();
        assert_eq!(tail, vec!["line 2", "line 3", "line 4"]);
    }

    #[test]
    fn file_gets_every_line_with_stream_tag() {
        let dir = tempfile::tempdir().unwrap();
        let mut log = BotLogger::new(dir.path(), "bot");
        log.append(Stream::Stdout, "hello");
        log.append(Stream::Stderr, "oops");
        log.append(Stream::System, "started");
        let text = fs::read_to_string(log.path()).unwrap();
        let lines: Vec<&str> = text.lines().collect();
        assert_eq!(lines.len(), 3);
        assert!(lines[0].ends_with("[OUT] hello"), "{}", lines[0]);
        assert!(lines[1].ends_with("[ERR] oops"));
        assert!(lines[2].ends_with("[SYS] started"));
    }

    #[test]
    fn rotates_when_over_limit_and_keeps_three() {
        let dir = tempfile::tempdir().unwrap();
        let mut log = BotLogger::with_limits(dir.path(), "bot", 10, 50);
        for i in 0..40 {
            log.append(Stream::Stdout, format!("padding line number {i}"));
        }
        assert!(dir.path().join("bot.log").exists());
        assert!(dir.path().join("bot.1.log").exists());
        assert!(dir.path().join("bot.2.log").exists());
        assert!(dir.path().join("bot.3.log").exists());
        assert!(!dir.path().join("bot.4.log").exists());
        let newest = fs::read_to_string(dir.path().join("bot.log")).unwrap();
        assert!(newest.contains("padding line number 39"));
    }
}
