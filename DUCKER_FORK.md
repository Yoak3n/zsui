# Ducker thin fork

This repository is an independent thin fork of
[`qiu7824/zsui`](https://github.com/qiu7824/zsui), based on the crates.io
`zsui 0.2.0-preview.7` source.

It exists because the upstream `native_window(...).stateful_view(...)` host does
not expose native window visibility, position, size or close-to-tray control.
Ducker needs those operations for its tray, silent-launch, close-to-tray and
window-state features.

## Branches and tags

- base: `v0.2.0-preview.7` (imported source commit)
- patch branch: `ducker/native-window-control`
- release tag: `v0.2.0-preview.7-ducker.1`

## Patch scope

See [PATCHES.md](PATCHES.md). The patch is intentionally limited to:

- `src/control.rs` — public control API and state (new file);
- `src/lib.rs` — exports;
- `src/window.rs` / `src/native.rs` — initial position;
- `src/linux_direct.rs` — apply control requests, record visible/rect state,
  close-to-tray behavior.

The rest of the crate is upstream source.

## Upstream sync

- Recommended path: fork upstream on GitHub (keeps history), then apply
  `patches/*.patch` with `git am`. See [DUCKER_FORK_SYNC.md](DUCKER_FORK_SYNC.md).
- This local staging repo imports the crates.io source as a squashed base
  commit because the build host could not clone upstream over git.
- After syncing, run `cargo test control::tests`, `cargo check` for the target
  platforms, tag `vX.Y.Z-ducker.N`, and update Ducker's dependency ref.

## License

MPL-2.0, inherited from upstream. `LICENSE` and `THIRD_PARTY_NOTICES.md` are
kept in this repository.
