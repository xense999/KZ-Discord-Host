//! `bot.toml` import: turn a bot folder into a `BotSpec` (secret values left empty).

use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::config::{BotSpec, EnvVar};

pub const MANIFEST_FILE_NAME: &str = "bot.toml";

#[derive(Debug, thiserror::Error)]
pub enum ImportError {
    #[error("資料夾裡沒有 {MANIFEST_FILE_NAME}：{0}")]
    Missing(PathBuf),
    #[error("讀取 {MANIFEST_FILE_NAME} 失敗：{0}")]
    Io(#[from] std::io::Error),
    #[error("{MANIFEST_FILE_NAME} 格式錯誤：{0}")]
    Parse(#[from] toml::de::Error),
    #[error("{MANIFEST_FILE_NAME} 的 name 不可為空")]
    EmptyName,
    #[error("找不到執行檔：{0}")]
    ExeNotFound(PathBuf),
}

#[derive(Deserialize)]
struct Manifest {
    name: String,
    exe: PathBuf,
    #[serde(default)]
    args: Vec<String>,
    cwd: Option<PathBuf>,
    #[serde(default = "default_true")]
    autostart: bool,
    #[serde(default)]
    env: Vec<ManifestEnv>,
}

#[derive(Deserialize)]
struct ManifestEnv {
    name: String,
    #[serde(default)]
    secret: bool,
    description: Option<String>,
    value: Option<String>,
}

fn default_true() -> bool {
    true
}

fn resolve(base: &Path, p: &Path) -> PathBuf {
    if p.is_absolute() { p.to_path_buf() } else { base.join(p) }
}

/// Parse `<dir>/bot.toml`. Relative paths resolve against `dir`; `cwd` defaults to `dir`.
pub fn import(dir: &Path) -> Result<BotSpec, ImportError> {
    let manifest_path = dir.join(MANIFEST_FILE_NAME);
    if !manifest_path.is_file() {
        return Err(ImportError::Missing(manifest_path));
    }
    let text = std::fs::read_to_string(&manifest_path)?;
    let m: Manifest = toml::from_str(&text)?;
    if m.name.trim().is_empty() {
        return Err(ImportError::EmptyName);
    }
    let exe = resolve(dir, &m.exe);
    if !exe.is_file() {
        return Err(ImportError::ExeNotFound(exe));
    }
    let cwd = Some(m.cwd.map(|c| resolve(dir, &c)).unwrap_or_else(|| dir.to_path_buf()));
    let env = m
        .env
        .into_iter()
        .map(|e| EnvVar {
            name: e.name,
            value: if e.secret { String::new() } else { e.value.unwrap_or_default() },
            secret: e.secret,
            description: e.description,
        })
        .collect();
    Ok(BotSpec {
        id: BotSpec::new_id(),
        name: m.name.trim().to_string(),
        exe,
        args: m.args,
        cwd,
        env,
        autostart: m.autostart,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn folder_with(toml_text: &str, exe_name: Option<&str>) -> tempfile::TempDir {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join(MANIFEST_FILE_NAME), toml_text).unwrap();
        if let Some(exe) = exe_name {
            fs::write(dir.path().join(exe), b"").unwrap();
        }
        dir
    }

    #[test]
    fn full_manifest_imports_with_secret_value_blank() {
        let dir = folder_with(
            r#"
name = "Xense Bot"
description = "demo"
exe = "xense-bot.exe"
args = ["--a", "b"]
cwd = "work"
autostart = false

[[env]]
name = "DISCORD_TOKEN"
secret = true
description = "token"
value = "should-be-ignored"

[[env]]
name = "MODE"
value = "prod"
"#,
            Some("xense-bot.exe"),
        );
        let spec = import(dir.path()).unwrap();
        assert_eq!(spec.name, "Xense Bot");
        assert_eq!(spec.exe, dir.path().join("xense-bot.exe"));
        assert_eq!(spec.args, vec!["--a", "b"]);
        assert_eq!(spec.cwd, Some(dir.path().join("work")));
        assert!(!spec.autostart);
        assert_eq!(spec.env.len(), 2);
        assert_eq!(spec.env[0].name, "DISCORD_TOKEN");
        assert!(spec.env[0].secret);
        assert_eq!(spec.env[0].value, "", "secret values are never taken from the manifest");
        assert_eq!(spec.env[0].description.as_deref(), Some("token"));
        assert_eq!(spec.env[1].value, "prod");
        assert!(!spec.env[1].secret);
        assert!(!spec.id.is_empty());
    }

    #[test]
    fn minimal_manifest_uses_defaults() {
        let dir = folder_with("name = \"m\"\nexe = \"m.exe\"\n", Some("m.exe"));
        let spec = import(dir.path()).unwrap();
        assert!(spec.autostart);
        assert!(spec.args.is_empty());
        assert!(spec.env.is_empty());
        assert_eq!(spec.cwd, Some(dir.path().to_path_buf()), "cwd defaults to the folder");
    }

    #[test]
    fn absolute_exe_path_is_kept() {
        let exe_dir = tempfile::tempdir().unwrap();
        let exe = exe_dir.path().join("abs.exe");
        fs::write(&exe, b"").unwrap();
        let text = format!("name = \"m\"\nexe = '{}'\n", exe.display());
        let dir = folder_with(&text, None);
        assert_eq!(import(dir.path()).unwrap().exe, exe);
    }

    #[test]
    fn errors_are_specific() {
        let empty = tempfile::tempdir().unwrap();
        assert!(matches!(import(empty.path()), Err(ImportError::Missing(_))));

        let bad = folder_with("name = \"x\"\n", Some("x.exe"));
        assert!(matches!(import(bad.path()), Err(ImportError::Parse(_))), "exe is required");

        let blank = folder_with("name = \"  \"\nexe = \"x.exe\"\n", Some("x.exe"));
        assert!(matches!(import(blank.path()), Err(ImportError::EmptyName)));

        let missing_exe = folder_with("name = \"x\"\nexe = \"nope.exe\"\n", None);
        assert!(matches!(import(missing_exe.path()), Err(ImportError::ExeNotFound(_))));
    }
}
