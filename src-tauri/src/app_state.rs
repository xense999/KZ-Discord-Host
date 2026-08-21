//! Process-wide state handed to Tauri (`app.manage`).

use crate::config::{self, Config};
use crate::supervisor::Supervisor;

pub struct AppState {
    pub supervisor: Supervisor,
    /// Set when the config file could not be read at startup; the UI shows it
    /// as a banner. The broken file is left untouched until the user edits.
    pub startup_notice: Option<String>,
}

impl AppState {
    /// Write the supervisor's current bot list to `config.json`.
    pub fn persist(&self) -> Result<(), String> {
        let cfg = Config { bots: self.supervisor.specs(), ..Config::default() };
        config::save(&config::config_path(), &cfg).map_err(|e| e.to_string())
    }
}
