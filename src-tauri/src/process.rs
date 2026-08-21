//! Child process spawning (no console window) and the kill-on-close Job Object
//! that guarantees every bot dies with the host.

use std::io;
use std::process::Stdio;

use tokio::process::{Child, Command};
use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::System::JobObjects::{
    AssignProcessToJobObject, CreateJobObjectW, JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE,
    JOBOBJECT_EXTENDED_LIMIT_INFORMATION, JobObjectExtendedLimitInformation,
    SetInformationJobObject,
};
use windows::Win32::System::Threading::CREATE_NO_WINDOW;
use windows::core::PCWSTR;

use crate::config::BotSpec;

/// Windows Job Object with `KILL_ON_JOB_CLOSE`: when the last handle closes
/// (host exits or is killed) every assigned process is terminated.
pub struct Job(HANDLE);

// HANDLE is a raw pointer newtype; a job handle is safe to share across threads.
unsafe impl Send for Job {}
unsafe impl Sync for Job {}

impl Job {
    pub fn new() -> io::Result<Self> {
        unsafe {
            let handle = CreateJobObjectW(None, PCWSTR::null())
                .map_err(|e| io::Error::other(e.to_string()))?;
            let mut info = JOBOBJECT_EXTENDED_LIMIT_INFORMATION::default();
            info.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;
            SetInformationJobObject(
                handle,
                JobObjectExtendedLimitInformation,
                &info as *const _ as *const std::ffi::c_void,
                std::mem::size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>() as u32,
            )
            .map_err(|e| io::Error::other(e.to_string()))?;
            Ok(Self(handle))
        }
    }

    pub fn assign(&self, child: &Child) -> io::Result<()> {
        let raw = child
            .raw_handle()
            .ok_or_else(|| io::Error::other("child process has no handle"))?;
        unsafe { AssignProcessToJobObject(self.0, HANDLE(raw as *mut _)) }
            .map_err(|e| io::Error::other(e.to_string()))
    }
}

impl Drop for Job {
    fn drop(&mut self) {
        unsafe {
            let _ = CloseHandle(self.0);
        }
    }
}

/// Spawn the bot described by `spec`: hidden window, stdout/stderr piped,
/// inherited environment overlaid with `spec.env`, assigned to `job`.
pub fn spawn(spec: &BotSpec, job: &Job) -> io::Result<Child> {
    let mut cmd = Command::new(&spec.exe);
    cmd.args(&spec.args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .creation_flags(CREATE_NO_WINDOW.0)
        .kill_on_drop(true);
    if let Some(cwd) = spec.effective_cwd() {
        cmd.current_dir(cwd);
    }
    for var in &spec.env {
        if !var.name.is_empty() {
            cmd.env(&var.name, &var.value);
        }
    }
    let child = cmd.spawn()?;
    job.assign(&child)?;
    Ok(child)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::time::Duration;
    use tokio::io::{AsyncBufReadExt, BufReader};

    fn cmd_spec(args: &[&str]) -> BotSpec {
        BotSpec {
            id: "t".into(),
            name: "t".into(),
            exe: PathBuf::from("cmd.exe"),
            args: args.iter().map(|s| s.to_string()).collect(),
            cwd: Some(std::env::temp_dir()),
            ..Default::default()
        }
    }

    #[tokio::test]
    async fn spawn_captures_stdout_and_env() {
        let job = Job::new().unwrap();
        let mut spec = cmd_spec(&["/c", "echo hello %KZ_TEST_VAR%"]);
        spec.env.push(crate::config::EnvVar {
            name: "KZ_TEST_VAR".into(),
            value: "world".into(),
            ..Default::default()
        });
        let mut child = spawn(&spec, &job).unwrap();
        let stdout = child.stdout.take().unwrap();
        let mut lines = BufReader::new(stdout).lines();
        let first = lines.next_line().await.unwrap().unwrap();
        assert_eq!(first.trim(), "hello world");
        let status = child.wait().await.unwrap();
        assert!(status.success());
    }

    #[tokio::test]
    async fn kill_terminates_child() {
        let job = Job::new().unwrap();
        let mut child = spawn(&cmd_spec(&["/c", "ping -n 30 127.0.0.1 >nul"]), &job).unwrap();
        child.kill().await.unwrap();
        let status = tokio::time::timeout(Duration::from_secs(5), child.wait())
            .await
            .expect("child must exit after kill")
            .unwrap();
        assert!(!status.success());
    }

    #[tokio::test]
    async fn dropping_job_kills_assigned_children() {
        let job = Job::new().unwrap();
        let mut child = spawn(&cmd_spec(&["/c", "ping -n 30 127.0.0.1 >nul"]), &job).unwrap();
        let t = std::time::Instant::now();
        drop(job);
        // Exit code after a job kill is unspecified; what matters is that a
        // 30-second ping ends almost immediately.
        tokio::time::timeout(Duration::from_secs(5), child.wait())
            .await
            .expect("child must die when the job closes")
            .unwrap();
        assert!(t.elapsed() < Duration::from_secs(5));
    }
}
