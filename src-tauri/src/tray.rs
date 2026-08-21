//! System tray icon, its menu, and window show/hide lifecycle.

use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Emitter, Manager};

use crate::app_state::AppState;

pub const MAIN_WINDOW: &str = "main";
const MENU_SHOW: &str = "show";
const MENU_QUIT: &str = "quit";
/// Sent to a child window so it re-routes to whatever was just requested.
pub const EVENT_ROUTE: &str = "route";

/// Labels of the windows declared in tauri.conf.json besides `main`.
pub const CHILD_WINDOWS: [&str; 2] = ["form", "settings"];

pub fn is_child(label: &str) -> bool {
    CHILD_WINDOWS.contains(&label)
}

pub fn show_main(app: &AppHandle) {
    show(app, MAIN_WINDOW);
}

fn show(app: &AppHandle, label: &str) {
    let Some(w) = app.get_webview_window(label) else { return };
    let _ = w.show();
    let _ = w.unminimize();
    let _ = w.set_focus();
}

/// Park `query` for the window to pick up, tell it to re-route, and show it.
pub fn open_child(app: &AppHandle, label: &str, query: &str) {
    if let Some(state) = app.try_state::<AppState>() {
        state.routes.lock().unwrap().insert(label.to_string(), query.to_string());
    }
    if let Some(w) = app.get_webview_window(label) {
        let _ = w.emit(EVENT_ROUTE, query.to_string());
    }
    show(app, label);
}

/// Exit the app; bots are stopped by the `RunEvent::Exit` handler in lib.rs
/// (and the Job Object is the backstop for any other exit path).
pub fn quit(app: &AppHandle) {
    app.exit(0);
}

pub fn setup(app: &AppHandle) -> tauri::Result<()> {
    let show = MenuItem::with_id(app, MENU_SHOW, "顯示", true, None::<&str>)?;
    let quit_item = MenuItem::with_id(app, MENU_QUIT, "結束", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&show, &quit_item])?;
    let icon = app
        .default_window_icon()
        .cloned()
        .expect("bundle icon configured in tauri.conf.json");

    TrayIconBuilder::with_id("main-tray")
        .icon(icon)
        .tooltip("KZ Bot Host")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id.as_ref() {
            MENU_SHOW => show_main(app),
            MENU_QUIT => quit(app),
            _ => {}
        })
        .on_tray_icon_event(|tray, event| match event {
            TrayIconEvent::Click { button: MouseButton::Left, button_state: MouseButtonState::Up, .. }
            | TrayIconEvent::DoubleClick { button: MouseButton::Left, .. } => show_main(tray.app_handle()),
            _ => {}
        })
        .build(app)?;
    Ok(())
}
