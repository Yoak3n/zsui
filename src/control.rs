//! Application-level control of the native main window.
//!
//! ZSUI 0.2's `native_window` path owns the platform event loop, so tray icons,
//! global shortcuts and single-instance handoff need a way to ask the loop to
//! show, hide, resize or quit the window. This module is deliberately small:
//! the active platform host installs a sender, application-level integrations
//! call the `*_main_window` helpers, and the host applies the request on its
//! event-loop thread.
//!
//! The Linux/Wayland host installs a `winit` event-loop proxy. Other backends
//! may leave the sender unset; callers can fall back to platform APIs or treat
//! the operation as unsupported.

use std::sync::atomic::{AtomicBool, AtomicI32, AtomicU32, Ordering};
use std::sync::{Mutex, OnceLock};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NativeWindowControl {
    Show,
    Hide,
    Toggle,
    Quit,
    SetSize { width: u32, height: u32 },
    SetPosition { x: i32, y: i32 },
    /// Simulate the platform title-bar close request.
    RequestClose,
}

type ControlSender = Box<dyn Fn(NativeWindowControl) -> bool + Send + Sync + 'static>;

static SENDER: OnceLock<Mutex<Option<ControlSender>>> = OnceLock::new();
static CLOSE_TO_TRAY: AtomicBool = AtomicBool::new(false);
static VISIBLE: AtomicBool = AtomicBool::new(true);
static HAS_RECT: AtomicBool = AtomicBool::new(false);
static LAST_X: AtomicI32 = AtomicI32::new(0);
static LAST_Y: AtomicI32 = AtomicI32::new(0);
static LAST_WIDTH: AtomicU32 = AtomicU32::new(0);
static LAST_HEIGHT: AtomicU32 = AtomicU32::new(0);

fn sender_slot() -> &'static Mutex<Option<ControlSender>> {
    SENDER.get_or_init(|| Mutex::new(None))
}

/// Installs the event-loop callback used by the public control helpers.
/// Installing a second sender replaces the first one.
pub fn install_control_sender(
    sender: impl Fn(NativeWindowControl) -> bool + Send + Sync + 'static,
) {
    if let Ok(mut slot) = sender_slot().lock() {
        *slot = Some(Box::new(sender));
    }
}

/// Returns true when the request was accepted by the active host.
pub fn request(control: NativeWindowControl) -> bool {
    sender_slot()
        .lock()
        .ok()
        .and_then(|slot| slot.as_ref().map(|sender| sender(control)))
        .unwrap_or(false)
}

pub fn show_main_window() -> bool {
    request(NativeWindowControl::Show)
}

pub fn hide_main_window() -> bool {
    request(NativeWindowControl::Hide)
}

pub fn toggle_main_window() -> bool {
    request(NativeWindowControl::Toggle)
}

pub fn quit() -> bool {
    request(NativeWindowControl::Quit)
}

pub fn set_main_window_size(width: u32, height: u32) -> bool {
    request(NativeWindowControl::SetSize { width, height })
}

pub fn set_main_window_position(x: i32, y: i32) -> bool {
    request(NativeWindowControl::SetPosition { x, y })
}

pub fn request_close() -> bool {
    request(NativeWindowControl::RequestClose)
}

/// When enabled, the native close button hides the window instead of exiting.
/// Tray integrations enable this so the process keeps running in the tray.
pub fn set_close_to_tray(enabled: bool) {
    CLOSE_TO_TRAY.store(enabled, Ordering::Relaxed);
}

pub fn close_to_tray() -> bool {
    CLOSE_TO_TRAY.load(Ordering::Relaxed)
}

/// Called by the platform host after showing/hiding the main window.
pub fn record_window_visible(visible: bool) {
    VISIBLE.store(visible, Ordering::Relaxed);
}

pub fn last_window_visible() -> bool {
    VISIBLE.load(Ordering::Relaxed)
}

/// Called by the platform host whenever the main window moves or resizes.
pub fn record_window_rect(x: i32, y: i32, width: u32, height: u32) {
    if width == 0 || height == 0 {
        return;
    }
    LAST_X.store(x, Ordering::Relaxed);
    LAST_Y.store(y, Ordering::Relaxed);
    LAST_WIDTH.store(width, Ordering::Relaxed);
    LAST_HEIGHT.store(height, Ordering::Relaxed);
    HAS_RECT.store(true, Ordering::Relaxed);
}

/// Last known outer rect `(x, y, width, height)`.
pub fn last_window_rect() -> Option<(i32, i32, u32, u32)> {
    if !HAS_RECT.load(Ordering::Relaxed) {
        return None;
    }
    let width = LAST_WIDTH.load(Ordering::Relaxed);
    let height = LAST_HEIGHT.load(Ordering::Relaxed);
    if width == 0 || height == 0 {
        return None;
    }
    Some((
        LAST_X.load(Ordering::Relaxed),
        LAST_Y.load(Ordering::Relaxed),
        width,
        height,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn window_rect_round_trip() {
        record_window_rect(10, 20, 800, 600);
        assert_eq!(last_window_rect(), Some((10, 20, 800, 600)));
    }

    #[test]
    fn visibility_round_trip() {
        record_window_visible(false);
        assert!(!last_window_visible());
        record_window_visible(true);
        assert!(last_window_visible());
    }

    #[test]
    fn close_to_tray_round_trip() {
        set_close_to_tray(true);
        assert!(close_to_tray());
        set_close_to_tray(false);
        assert!(!close_to_tray());
    }
}
