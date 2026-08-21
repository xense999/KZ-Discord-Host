//! Persistent configuration: `%APPDATA%\KZ Bot Host\config.json` (plain text).

use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

pub const APP_DIR_NAME: &str = "KZ Bot Host";
pub const CONFIG_FILE_NAME: &str = "config.json";
pub const SCHEMA_VERSION: u32 = 1;

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("讀取設定檔失敗：{0}")]
    Io(#[from] std::io::Error),
    #[error("設定檔格式錯誤（已保留原檔不覆寫）：{0}")]
    Parse(#[from] serde_json::Error),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(default)]
pub struct EnvVar {
    pub name: String,
    pub value: String,
    pub secret: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(default)]
pub struct BotSpec {
    pub id: String,
    pub name: String,
    pub exe: PathBuf,
    pub args: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cwd: Option<PathBuf>,
    pub env: Vec<EnvVar>,
    pub autostart: bool,
}

impl Default for BotSpec {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            exe: PathBuf::new(),
            args: Vec::new(),
            cwd: None,
            env: Vec::new(),
            autostart: true,
        }
    }
}

impl BotSpec {
    pub fn new_id() -> String {
        uuid::Uuid::new_v4().to_string()
    }

    /// Trim, reject empty name/exe, assign an id when missing.
    pub fn normalize(&mut self) -> Result<(), String> {
        self.name = self.name.trim().to_string();
        if self.name.is_empty() {
            return Err("名稱不可為空".into());
        }
        if self.exe.as_os_str().is_empty() {
            return Err("執行檔路徑不可為空".into());
        }
        if self.id.is_empty() {
            self.id = Self::new_id();
        }
        Ok(())
    }

    /// Working directory used when spawning: explicit `cwd`, else the exe folder.
    pub fn effective_cwd(&self) -> Option<PathBuf> {
        self.cwd
            .clone()
            .or_else(|| self.exe.parent().map(Path::to_path_buf))
            .filter(|p| !p.as_os_str().is_empty())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(default)]
pub struct Config {
    pub schema_version: u32,
    pub bots: Vec<BotSpec>,
}

impl Default for Config {
    fn default() -> Self {
        Self { schema_version: SCHEMA_VERSION, bots: Vec::new() }
    }
}

pub fn app_dir() -> PathBuf {
    let base = std::env::var_os("APPDATA")
        .map(PathBuf::from)
        .unwrap_or_else(std::env::temp_dir);
    base.join(APP_DIR_NAME)
}

pub fn config_path() -> PathBuf {
    app_dir().join(CONFIG_FILE_NAME)
}

pub fn logs_dir() -> PathBuf {
    app_dir().join("logs")
}

/// `logs_dir()`, created if missing.
pub fn ensure_logs_dir() -> std::io::Result<PathBuf> {
    let dir = logs_dir();
    fs::create_dir_all(&dir)?;
    Ok(dir)
}

/// Missing file -> default config. Unreadable or malformed file -> Err, file untouched.
pub fn load(path: &Path) -> Result<Config, ConfigError> {
    match fs::read(path) {
        Ok(bytes) => Ok(serde_json::from_slice(&bytes)?),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Config::default()),
        Err(e) => Err(e.into()),
    }
}

/// Atomic save: write `<path>.tmp` then rename over `path`.
pub fn save(path: &Path, config: &Config) -> Result<(), ConfigError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let tmp = path.with_extension("json.tmp");
    fs::write(&tmp, serde_json::to_vec_pretty(config)?)?;
    fs::rename(&tmp, path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Config {
        Config {
            schema_version: SCHEMA_VERSION,
            bots: vec![BotSpec {
                id: "abc".into(),
                name: "Xense".into(),
                exe: PathBuf::from(r"C:\bots\xense\xense-bot.exe"),
                args: vec!["--verbose".into()],
                cwd: None,
                env: vec![EnvVar {
                    name: "DISCORD_TOKEN".into(),
                    value: "t0k".into(),
                    secret: true,
                    description: None,
                }],
                autostart: true,
            }],
        }
    }

    #[test]
    fn save_then_load_round_trips() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("nested").join("config.json");
        save(&path, &sample()).unwrap();
        assert_eq!(load(&path).unwrap(), sample());
        assert!(!path.with_extension("json.tmp").exists(), "tmp file must be renamed away");
    }

    #[test]
    fn save_overwrites_existing_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("config.json");
        save(&path, &Config::default()).unwrap();
        save(&path, &sample()).unwrap();
        assert_eq!(load(&path).unwrap(), sample());
    }

    #[test]
    fn missing_file_is_default() {
        let dir = tempfile::tempdir().unwrap();
        let cfg = load(&dir.path().join("none.json")).unwrap();
        assert_eq!(cfg, Config::default());
        assert_eq!(cfg.schema_version, SCHEMA_VERSION);
    }

    #[test]
    fn malformed_file_is_error_and_untouched() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("config.json");
        fs::write(&path, b"{ not json").unwrap();
        assert!(matches!(load(&path), Err(ConfigError::Parse(_))));
        assert_eq!(fs::read(&path).unwrap(), b"{ not json");
    }

    #[test]
    fn missing_fields_get_defaults() {
        let json = r#"{"bots":[{"name":"x","exe":"x.exe"}]}"#;
        let cfg: Config = serde_json::from_str(json).unwrap();
        let bot = &cfg.bots[0];
        assert!(bot.autostart, "autostart defaults to true");
        assert!(bot.args.is_empty());
        assert!(bot.env.is_empty());
        assert_eq!(cfg.schema_version, SCHEMA_VERSION);
    }

    #[test]
    fn normalize_trims_validates_and_assigns_id() {
        let mut bot = BotSpec { name: "  x  ".into(), exe: PathBuf::from("x.exe"), ..Default::default() };
        bot.normalize().unwrap();
        assert_eq!(bot.name, "x");
        assert!(!bot.id.is_empty());
        let mut blank = BotSpec { name: "  ".into(), exe: PathBuf::from("x.exe"), ..Default::default() };
        assert!(blank.normalize().is_err());
        let mut no_exe = BotSpec { name: "x".into(), ..Default::default() };
        assert!(no_exe.normalize().is_err());
    }

    #[test]
    fn effective_cwd_falls_back_to_exe_folder() {
        let mut bot = BotSpec { exe: PathBuf::from(r"C:\bots\x\bot.exe"), ..Default::default() };
        assert_eq!(bot.effective_cwd(), Some(PathBuf::from(r"C:\bots\x")));
        bot.cwd = Some(PathBuf::from(r"D:\work"));
        assert_eq!(bot.effective_cwd(), Some(PathBuf::from(r"D:\work")));
    }
}
