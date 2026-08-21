//! Tauri IPC surface. Thin: validates nothing beyond types, delegates to
//! supervisor / config / bot_toml / autostart and maps errors to strings.

use std::path::PathBuf;

use tauri::{AppHandle, State};

use crate::app_state::AppState;
use crate::config::{self, BotSpec};
use crate::logs::LogLine;
use crate::supervisor::StateEvent;
use crate::{autostart, bot_toml};

#[tauri::command]
pub fn list_bots(state: State<'_, AppState>) -> Vec<BotSpec> {
    state.supervisor.specs()
}

#[tauri::command]
pub fn list_status(state: State<'_, AppState>) -> Vec<StateEvent> {
    state.supervisor.statuses()
}

#[tauri::command]
pub fn startup_notice(state: State<'_, AppState>) -> Option<String> {
    state.startup_notice.clone()
}

#[tauri::command]
pub fn upsert_bot(state: State<'_, AppState>, mut spec: BotSpec) -> Result<BotSpec, String> {
    if spec.name.trim().is_empty() {
        return Err("名稱不可為空".into());
    }
    if spec.exe.as_os_str().is_empty() {
        return Err("執行檔路徑不可為空".into());
    }
    if spec.id.is_empty() {
        spec.id = BotSpec::new_id();
    }
    state.supervisor.upsert(spec.clone());
    state.persist()?;
    Ok(spec)
}

#[tauri::command]
pub async fn remove_bot(state: State<'_, AppState>, id: String) -> Result<(), String> {
    state.supervisor.remove(&id).await?;
    state.persist()
}

#[tauri::command]
pub fn import_bot_folder(dir: String) -> Result<BotSpec, String> {
    bot_toml::import(&PathBuf::from(dir)).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn start_bot(state: State<'_, AppState>, id: String) -> Result<(), String> {
    state.supervisor.start(&id)
}

#[tauri::command]
pub async fn stop_bot(state: State<'_, AppState>, id: String) -> Result<(), String> {
    state.supervisor.stop(&id).await
}

#[tauri::command]
pub async fn restart_bot(state: State<'_, AppState>, id: String) -> Result<(), String> {
    state.supervisor.restart(&id).await
}

#[tauri::command]
pub fn get_log_tail(state: State<'_, AppState>, id: String) -> Result<Vec<LogLine>, String> {
    state.supervisor.log_tail(&id)
}

#[tauri::command]
pub fn get_host_autostart(app: AppHandle) -> Result<bool, String> {
    autostart::is_enabled(&app)
}

#[tauri::command]
pub fn set_host_autostart(app: AppHandle, enabled: bool) -> Result<(), String> {
    autostart::set_enabled(&app, enabled)
}

#[tauri::command]
pub fn open_logs_dir(app: AppHandle) -> Result<(), String> {
    use tauri_plugin_opener::OpenerExt;
    let dir = config::logs_dir();
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    app.opener()
        .open_path(dir.to_string_lossy().to_string(), None::<&str>)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn config_path() -> String {
    config::config_path().to_string_lossy().to_string()
}

