//! Host autostart (HKCU Run key via tauri-plugin-autostart, launched with `--minimized`).

use tauri::AppHandle;
use tauri_plugin_autostart::ManagerExt;

pub const MINIMIZED_FLAG: &str = "--minimized";

pub fn is_enabled(app: &AppHandle) -> Result<bool, String> {
    app.autolaunch().is_enabled().map_err(|e| e.to_string())
}

pub fn set_enabled(app: &AppHandle, enabled: bool) -> Result<(), String> {
    let launcher = app.autolaunch();
    let result = if enabled { launcher.enable() } else { launcher.disable() };
    result.map_err(|e| e.to_string())
}

/// True when this process was launched with `--minimized` (autostart at logon).
pub fn launched_minimized() -> bool {
    std::env::args().skip(1).any(|a| a == MINIMIZED_FLAG)
}
