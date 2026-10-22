# Ducker native window control patch

Base: `dev` (0.3.0-alpha.1).

This patch adds the small native window control surface Ducker needs for its
tray, silent-launch, close-to-tray and window-state features:

- `src/control.rs` — new public control API (`show/hide/toggle/quit`,
  `set_main_window_size/position`, `request_close`, `set_close_to_tray`,
  `last_window_rect`, `last_window_visible`) plus control-state tests.
- `src/lib.rs` — exports `control`.
- `src/window.rs` — `WindowSpec::position`.
- `src/native.rs` — `NativeWindowBuilder::position` and typed-builder forwarding.
- `src/linux_direct.rs` — event-loop control requests, window rect/visibility
  recording, and close-to-tray handling.

The rest of the crate is upstream `0.3.0-alpha.1`.
