//! Host autostart (HKCU Run key via tauri-plugin-autostart, launched with `--minimized`).

use tauri::AppHandle;
use tauri_plugin_autostart::ManagerExt;

pub const MINIMIZED_FLAG: &str = "--minimized";
/// Run-key value name. Fixed and ASCII so a later display-name change does
/// not orphan the entry (the plugin would otherwise use `productName`).
pub const RUN_KEY_NAME: &str = "KZ Discord Host";
/// Run-key value written by the app before the rename.
const LEGACY_RUN_KEY_NAME: &str = "KZ Bot Host";

pub fn is_enabled(app: &AppHandle) -> Result<bool, String> {
    app.autolaunch().is_enabled().map_err(|e| e.to_string())
}

pub fn set_enabled(app: &AppHandle, enabled: bool) -> Result<(), String> {
    let launcher = app.autolaunch();
    let result = if enabled { launcher.enable() } else { launcher.disable() };
    result.map_err(|e| e.to_string())
}

/// Carry "start with Windows" over from the old name, then remove the old
/// entry: left in place it would start the old install as well, and both
/// would launch the same bots.
pub fn migrate_legacy(app: &AppHandle) {
    let legacy = auto_launch::AutoLaunch::new(LEGACY_RUN_KEY_NAME, "", &[] as &[&str]);
    if !legacy.is_enabled().unwrap_or(false) {
        return;
    }
    if legacy.disable().is_ok() {
        let _ = set_enabled(app, true);
    }
}

/// True when this process was launched with `--minimized` (autostart at logon).
pub fn launched_minimized() -> bool {
    std::env::args().skip(1).any(|a| a == MINIMIZED_FLAG)
}
