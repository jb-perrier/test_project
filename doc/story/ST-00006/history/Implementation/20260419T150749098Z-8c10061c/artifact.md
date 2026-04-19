# Implementation

## Execution Summary
- Updated `snake/src/game.rs` to remove the out-of-scope adjacent-food penalty added in the prior pass.
- Kept the alternate-tick enemy replan cadence and skipped-tick safe fallback behavior intact.
- Restored replan ticks to the existing safe food-seeking preference and replaced the prior anti-orbit regression with a scope-aligned test.

## Files Changed
- `snake/src/game.rs`
  - Removed `ENEMY_MISSED_ADJACENT_FOOD_PENALTY`.
  - Removed `enemy_would_miss_adjacent_food(...)` and its scoring branch from `score_enemy_move(...)`.
  - Added `enemy_replan_tick_still_prefers_the_safe_turn_that_closes_food_distance` to confirm replan ticks still choose the safe move that closes food distance.

## Validation Commands
- `cargo test --manifest-path snake/Cargo.toml` — not run in this step; no command-execution tool was available.
- `cargo run --manifest-path snake/Cargo.toml` — not run in this step; no command-execution tool was available.

## Notes
- Documentation follow-up remains for `snake/README.md` and `doc/spec/gameplay.md`.
- `doc/spec/overview.md` may also need a later update if the broader Snake summary mentions enemy behavior.
- No documentation files were edited in this implementation step.
