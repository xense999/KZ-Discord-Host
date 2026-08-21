pub mod app_state;
pub mod autostart;
pub mod bot_toml;
pub mod commands;
pub mod config;
pub mod logs;
pub mod process;
pub mod supervisor;
pub mod tray;

use std::sync::Arc;

use tauri::{AppHandle, Emitter, Manager, RunEvent, WindowEvent};
use tauri_plugin_autostart::MacosLauncher;

use app_state::AppState;
use supervisor::{EventSink, LogEvent, StateEvent, Supervisor};

pub const EVENT_BOT_STATE: &str = "bot-state";
pub const EVENT_BOT_LOG: &str = "bot-log";

struct TauriSink(AppHandle);

impl EventSink for TauriSink {
    fn on_state(&self, event: StateEvent) {
        let _ = self.0.emit(EVENT_BOT_STATE, event);
    }
    fn on_log(&self, event: LogEvent) {
        let _ = self.0.emit(EVENT_BOT_LOG, event);
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| tray::show_main(app)))
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec![autostart::MINIMIZED_FLAG]),
        ))
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let (cfg, notice) = match config::load(&config::config_path()) {
                Ok(c) => (c, None),
                Err(e) => (config::Config::default(), Some(e.to_string())),
            };
            let sink = Arc::new(TauriSink(app.handle().clone()));
            let supervisor = Supervisor::new(sink, config::logs_dir(), &cfg.bots)?;
            for bot in cfg.bots.iter().filter(|b| b.autostart) {
                let _ = supervisor.start(&bot.id);
            }
            app.manage(AppState { supervisor, startup_notice: notice });
            tray::setup(app.handle())?;
            if !autostart::launched_minimized() {
                tray::show_main(app.handle());
            }
            Ok(())
        })
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                let _ = window.hide();
            }
        })
        .invoke_handler(tauri::generate_handler![
            commands::list_bots,
            commands::list_status,
            commands::startup_notice,
            commands::upsert_bot,
            commands::remove_bot,
            commands::import_bot_folder,
            commands::start_bot,
            commands::stop_bot,
            commands::restart_bot,
            commands::get_log_tail,
            commands::get_host_autostart,
            commands::set_host_autostart,
            commands::open_logs_dir,
            commands::config_path,
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    app.run(|app, event| {
        if let RunEvent::Exit = event {
            if let Some(state) = app.try_state::<AppState>() {
                tauri::async_runtime::block_on(state.supervisor.shutdown_all());
            }
        }
    });
}
