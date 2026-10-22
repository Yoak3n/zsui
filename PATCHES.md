# ZSUI local patch

Source: crates.io `zsui 0.2.0-preview.7` (MPL-2.0).

This vendored copy exists only because Ducker needs window visibility/close
control and window-rect reporting from the native host. The patch is intentionally
small and confined to the following files:

- `src/control.rs` (new): `NativeWindowControl` enum and helpers such as
  `show_main_window`, `hide_main_window`, `toggle_main_window`, `quit`,
  `set_main_window_size`, `set_main_window_position`, `request_close`,
  `set_close_to_tray`, `last_window_rect`, `last_window_visible`.
- `src/lib.rs`: export `control` and its helpers.
- `src/window.rs`: add `WindowSpec::position` and the `position` field.
- `src/native.rs`: add `NativeWindowBuilder::position` and the typed-builder
  forwarding method.
- `src/linux_direct.rs`:
  - create the event loop with `NativeWindowControl` user events;
  - install an event-loop sender from `control`;
  - apply show/hide/toggle/quit/resize/position/request-close requests;
  - record the window rect on creation, resize and move;
  - honor `close_to_tray()` for the native close button;
  - record visibility for `last_window_visible()`.

Everything else is upstream `0.2.0-preview.7`. If upstream publishes equivalent
APIs, this directory can be replaced by a crates.io dependency again.
