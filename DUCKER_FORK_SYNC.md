# Syncing with upstream

The cleanest way to maintain this fork is to keep upstream history:

1. On GitHub, fork `qiu7824/zsui`.
2. Clone your fork and check out the upstream tag you want to base on.
3. Apply the patch series from `patches/` in order:

   ```bash
   git am patches/*.patch
   ```

4. Resolve conflicts, run tests, and push:

   ```bash
   cargo test control::tests
   git push origin ducker/native-window-control --tags
   ```

`patches/` is generated from the local staging repo with
`git format-patch <base>..HEAD`. The local repo imports the crates.io source as a
single base commit because the build host could not clone upstream over git; the
GitHub fork should use the real upstream history instead.
