Verdict: changes-required

# Review
## Scope
Coverage: complete
Reviewed Head Commit: 875955c5ab7ac0b8f8b795315b57460ce36fdbe2
Changed Files Reviewed: snake/src/game.rs

## Blind Spots
- none

## Findings
Validation Status: declared-only
- `snake/src/game.rs:412-420` adds a new `ENEMY_MISSED_ADJACENT_FOOD_PENALTY`, and `snake/src/game.rs:1103-1113` locks in a case where a replan tick chooses `Right` instead of the safe `Up` move that closes food distance. The approved change was the alternate-tick replan cadence; replan ticks were still meant to keep the existing food-seeking preference. This extra heuristic changes AI policy beyond that scope.

## Evidence Summary
- Reviewed the complete pinned inline diff for `snake/src/game.rs`.
- Inspected the head commit sections covering cadence reset/toggle, `step()`, `choose_enemy_direction()`, `enemy_safe_directions()`, `score_enemy_move()`, `enemy_would_miss_adjacent_food()`, and the added tests.
- Compared the base implementation of `choose_enemy_direction()` and `score_enemy_move()` to confirm the new penalty is the behavior change.
- No test or run logs were provided; the listed Cargo commands were declared only.

## Follow-up
- Remove or retune the adjacent-food penalty so the story stays limited to alternating replans, then run the declared Snake validation commands.
