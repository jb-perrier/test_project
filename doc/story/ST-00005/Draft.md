# Draft

## User Need
- As a player, the enemy snake should feel like a believable opponent instead of a perfect solver.
- Runs should feel less repetitive while the enemy still competes for food and creates pressure.

## Problem
- The current enemy logic heavily favors the closest safe path to food.
- This makes the opponent look overly optimal and predictable, especially in open space.
- Similar situations resolve the same way too often, so the enemy feels scripted rather than human.

## Goals
- Make enemy choices look more human by using simple heuristics instead of shortest-path-only behavior.
- Keep a clear but softer bias toward food.
- Allow believable, safe, slightly non-optimal moves such as continuing straight or taking a small detour.
- Preserve safety, readability, and the lightweight implementation style of the Snake crate.

## Non-Goals
- Making the enemy random, trivial, or obviously self-sabotaging.
- Changing player controls, phases, board size, wraparound, scoring, growth, food spawning, collision rules, or tick-speed progression.
- Adding new UI, difficulty settings, or heavy AI/pathfinding dependencies.
- Reworking unrelated Snake systems.

## Acceptance Criteria
- Given the enemy has more than one safe move during `Running`, when it selects a direction, then the choice is not based only on the shortest distance to food.
- Given continuing straight is safe and turning toward food is only a small improvement, when the enemy picks its next move, then it may continue straight instead of always taking the optimal turn immediately.
- Given a safe move toward food exists and no stronger safety concern applies, when the enemy evaluates its options, then it still generally prefers moves that close distance to food over moves that clearly drift away.
- Given at least one safe move exists, when the enemy chooses its next direction, then it does not choose an immediately fatal move.
- Given the story is implemented, when the game is played, then existing controls, phases, wraparound, collision behavior, food behavior, scoring, growth, and tick-speed progression remain unchanged.
- Automated tests cover representative enemy decision states, and any variability used by the logic is reproducible in tests.

## Open Questions
- Default assumption: the enemy should become only slightly weaker, not easy. Confirm if this balance is acceptable.
- Default assumption: variation should come from heuristic bias first, not pure randomness. Confirm if deterministic behavior is preferred.
- Proposed first human-like traits: safer forward momentum and softer food pursuit. Confirm if occasional hesitation should also be modeled.
