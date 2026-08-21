//! Rounded window corners via an OS-level window region, so the windows can
//! stay opaque (Windows 10 has no native rounding for undecorated windows).

use tauri::{Window, WindowEvent};
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Gdi::CreateRoundRectRgn;
use windows::Win32::Graphics::Gdi::SetWindowRgn;

pub const CORNER_RADIUS: i32 = 12;

/// Clip `window` to a rounded rectangle. Must be re-applied after a resize.
pub fn apply_rounded(window: &Window) {
    let (Ok(handle), Ok(size)) = (window.hwnd(), window.outer_size()) else {
        return;
    };
    unsafe {
        // SetWindowRgn takes ownership of the region: no DeleteObject here.
        let region = CreateRoundRectRgn(
            0,
            0,
            size.width as i32 + 1,
            size.height as i32 + 1,
            CORNER_RADIUS,
            CORNER_RADIUS,
        );
        SetWindowRgn(HWND(handle.0 as _), Some(region), true);
    }
}

/// Keep the rounded shape after the window changes size.
pub fn on_window_event(window: &Window, event: &WindowEvent) {
    if matches!(event, WindowEvent::Resized(_) | WindowEvent::ScaleFactorChanged { .. }) {
        apply_rounded(window);
    }
}
