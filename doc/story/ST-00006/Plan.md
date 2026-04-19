# Plan
## Summary
- Soften the Snake enemy AI only in `snake/src/game.rs` by reducing how often it performs a fresh food-seeking replan.
- Keep the enemy moving every running tick, but make full steering recalculation happen on the first running tick and then every other tick.
- Preserve current wraparound, food, growth, collision, score, tick-speed, input, and rendering behavior; no UI or config changes are expected.

## Findings
- `snake/src/game.rs` appears to own tick progression, enemy steering, growth, collisions, and scoring, so the rebalance can stay localized there.
- The existing enemy logic already looks deterministic and weighted toward safe food pursuit, which makes cadence control a smaller change than retuning many AI weights or adding randomness.
- `snake/README.md` and `doc/spec/gameplay.md` currently describe the more aggressive enemy behavior; they should be updated in a later Documentation step. `doc/spec/overview.md` only needs follow-up if the broad summary should mention the softer AI.

## Affected Files
- `snake/src/game.rs`

## Implementation Approach
- Add a small enemy replan cadence field to `Game` and reset it in new-game, start, and restart paths so the first actual running movement tick is always a replan tick.
- Split enemy direction selection into two paths:
  - replan ticks: reuse the current safe food-seeking ranking;
  - skipped ticks: keep the carried direction if it is safe, otherwise pick another safe direction if one exists, and otherwise preserve the existing unavoidable-crash outcome.
- Leave the rest of `step()` intact so both snakes still move every gameplay tick and enemy food still only affects enemy growth and food respawn, not player score or tick speed.
- Add or adjust focused unit tests in `snake/src/game.rs`, using direct state setup where needed to avoid RNG-dependent assertions.

## Risks
- Off-by-one handling around `Ready` to `Running` or `restart` could make the first running tick skip replanning.
- If skipped ticks still reuse the full food-seeking chooser, the difficulty drop may be too small; if safety checks are rewritten too broadly, collision behavior could change unintentionally.
- Documentation will remain stale until the later Documentation step updates the Snake behavior description.

## Validation
- Run `cargo test --manifest-path snake/Cargo.toml`.
- Manually run `cargo run --manifest-path snake/Cargo.toml` and verify:
  - the enemy still advances every running tick;
  - it replans on the first running tick and then every other tick;
  - it keeps its current direction on intervening ticks when safe;
  - it still takes a safe fallback when its carried direction becomes unsafe;
  - enemy food consumption still grows the enemy without increasing player score or speeding up the game.
- Later Documentation work should update `snake/README.md` and `doc/spec/gameplay.md`, with `doc/spec/overview.md` only if the higher-level summary should mention the new enemy cadence.
