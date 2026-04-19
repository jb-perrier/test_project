# Implementation
## Execution Summary
- Applied the enemy decision update in `snake/src/game.rs` using heuristic scoring instead of closest-food-only selection.
- Preserved safety-first filtering, wraparound behavior, collision rules, scoring, growth, and seeded near-best tie-breaking.
- Added an explicit unit test for the “keep moving straight when it is still a reasonable food-closing move” case.

## Files Changed
- `snake/src/game.rs`
  - Scored safe enemy moves with food-distance progress, straight-line bias, turn cost, food-capture bonus, and follow-up safety checks.
  - Reused projected-segment collision helpers for candidate and follow-up evaluation.
  - Expanded seeded tests for straight-vs-turn behavior, food preference, safe fallback behavior, and reproducible tie-breaking.

## Validation Commands
- Not run in this step:
  - `cargo test --manifest-path snake/Cargo.toml`
  - `cargo run --manifest-path snake/Cargo.toml`

## Notes
- Documentation follow-up remains for `snake/README.md` and `doc/spec/*`, which still describe the older enemy behavior.
- No documentation files were edited in this step, per scope restrictions.
