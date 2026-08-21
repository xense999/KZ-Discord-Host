//! System tray icon, its menu, and main-window show/hide lifecycle.

use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Manager};

pub const MAIN_WINDOW: &str = "main";
const MENU_SHOW: &str = "show";
const MENU_QUIT: &str = "quit";

pub fn show_main(app: &AppHandle) {
    if let Some(w) = app.get_webview_window(MAIN_WINDOW) {
        let _ = w.show();
        let _ = w.unminimize();
        let _ = w.set_focus();
    }
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
