# Implementation

## Execution Summary
- Updated `snake/src/game.rs` to keep the alternate-tick enemy slowdown while reducing the food-orbiting behavior.
- Added a scoring penalty for replan turns that leave food adjacent and likely missed on the next carried tick, so the enemy is less likely to circle around food.
- Kept the skipped-tick safe fallback behavior and added a focused regression test for the anti-orbit case.

## Files Changed
- `snake/src/game.rs`
  - Added `ENEMY_MISSED_ADJACENT_FOOD_PENALTY`.
  - Added `enemy_would_miss_adjacent_food(...)` and applied it in `score_enemy_move(...)`.
  - Added `enemy_avoids_turning_beside_food_when_a_skipped_tick_would_pass_it` coverage.

## Validation Commands
- Not run in this step.
- `cargo test --manifest-path snake/Cargo.toml`
- `cargo run --manifest-path snake/Cargo.toml`

## Notes
- Documentation was not edited in this step because implementation scope stayed limited to `snake/src/game.rs`.
- Follow-up remains for `snake/README.md` and `doc/spec/gameplay.md`.
- `doc/spec/overview.md` may also need a later update if the broader Snake summary should mention the softer enemy cadence.
