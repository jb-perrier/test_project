# Implementation
## Execution Summary
- Updated `snake/src/game.rs` to fix the failing dead-end test fixture.
- Reworked `enemy_can_keep_moving_straight_when_a_food_turn_leads_into_a_dead_end` so the upward food turn now has $0$ follow-up options under the implemented scorer.
- Added fixture assertions in that test to verify the `Up` branch is safe immediately but becomes a true dead end before asserting the final enemy choice.
- Left gameplay heuristics unchanged outside the test setup and assertions.

## Files Changed
- `snake/src/game.rs`
  - Expanded the player fixture in the dead-end test to block the enemy's post-turn escape cells.
  - Added explicit assertions for `move_is_fatal(...) == false` and `enemy_follow_up_options(...) == 0` for the `Up` branch.
  - Kept the expected chosen direction as `Direction::Right` once the dead-end condition is real.

## Validation Commands
- `cargo test --manifest-path snake/Cargo.toml` — not run; no command execution tool was available.
- `cargo run --manifest-path snake/Cargo.toml` — not run; no command execution tool was available.

## Notes
- Validation output could not be attached in this environment because the available workspace tools support file inspection and editing only.
- Documentation follow-up remains out of scope for this step: `snake/README.md` and `doc/spec/*` were not edited.
