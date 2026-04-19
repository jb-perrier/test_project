# Implementation

## Execution Summary
- Softened the Snake enemy AI in `snake/src/game.rs` by adding an alternate-tick replan cadence.
- The enemy now performs a fresh food-seeking choice on the first running tick and then every other tick after that.
- On skipped ticks, the enemy keeps its current direction when safe; if that direction is unsafe, it now takes a safe fallback when available.
- Added focused unit tests for cadence reset, alternate-tick behavior, skipped-tick fallback, and unavoidable-crash handling.

## Files Changed
- `snake/src/game.rs`
  - Added `enemy_replans_next_tick` state plus reset/toggle helpers.
  - Refactored enemy direction selection into replan and carry-direction paths.
  - Added coverage for the reduced-difficulty behavior in unit tests.

## Validation Commands
- Not run in this step.
- `cargo test --manifest-path snake/Cargo.toml`
- `cargo run --manifest-path snake/Cargo.toml`

## Notes
- Documentation follow-up remains for `snake/README.md` and `doc/spec/gameplay.md`.
- `doc/spec/overview.md` may also need a follow-up if the broader Snake summary should mention the softer enemy cadence.
