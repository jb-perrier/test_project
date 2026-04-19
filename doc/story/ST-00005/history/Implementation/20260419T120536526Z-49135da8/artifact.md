# Implementation
## Execution Summary
- Replaced the enemy move picker in `snake/src/game.rs` with scored safe-move evaluation instead of closest-food-only ordering.
- Added lightweight heuristics for food progress, straight-line bias, turn cost, follow-up safety, dead-end avoidance, and seeded near-best tie-breaking.
- Added helper logic for legal enemy directions, projected post-move segments, and reusable collision checks.
- Expanded unit tests in `snake/src/game.rs` with representative enemy-decision and seeded-RNG scenarios.

## Files Changed
- `snake/src/game.rs`
  - Implemented the more human-like enemy decision logic.
  - Added internal scoring/helpers for safe candidate evaluation.
  - Added enemy behavior tests for straight-vs-turn, food preference, and reproducible variability.

## Validation Commands
- Not run in this step:
  - `cargo test --manifest-path snake/Cargo.toml`
  - `cargo run --manifest-path snake/Cargo.toml`

## Notes
- Documentation follow-up remains for `snake/README.md` and `doc/spec/*`, which still describe the older food-first enemy behavior.
- Validation should recheck any legacy head-to-head expectations, since the enemy now avoids an immediately fatal move when a safe alternative exists.
