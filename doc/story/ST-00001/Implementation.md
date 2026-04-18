# Implementation

## Execution Summary
- Inspected the existing `snake/` crate files.
- Kept the standalone Rust Snake implementation isolated under `snake/`.
- Applied one code fix in `snake/src/main.rs`:
  - hardened terminal setup cleanup if entering the alternate screen fails after raw mode is enabled.
- Left gameplay, input mapping, rendering, and tests unchanged.

## Files Changed
- `snake/src/main.rs`
  - Updated `TerminalGuard::enter` to roll back terminal state on setup failure by restoring cursor/screen state and disabling raw mode before returning the error.

## Validation Commands
- `cargo fmt --manifest-path snake/Cargo.toml`
- `cargo test --manifest-path snake/Cargo.toml`
- `cargo run --manifest-path snake/Cargo.toml`

## Notes
- No files outside `snake/` were modified.
- I did not run commands because no command execution tool was available here.
- Existing unrelated baseline drift in the root JS/TS files was not touched.
