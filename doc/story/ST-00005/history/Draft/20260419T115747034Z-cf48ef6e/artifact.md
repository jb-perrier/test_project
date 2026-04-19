# Draft

## User Need
- As a player, the enemy snake should feel less perfect and less repetitive.
- The opponent should look closer to a human player so runs feel more varied and less scripted.

## Problem
- The current enemy AI strongly favors the closest safe path to food.
- This makes the opponent look overly optimal and predictable.
- Similar board states tend to produce the same behavior, which reduces gameplay variety.

## Goals
- Make enemy decisions look more human-like, with believable imperfections.
- Add controlled variation so similar states can lead to different reasonable moves.
- Keep the enemy competitive and readable instead of random.
- Preserve the existing Snake rules and lightweight implementation style.

## Non-Goals
- Turning the enemy into a purely random or trivial opponent.
- Changing controls, phases, board size, scoring, growth, food spawning, wraparound, or tick-speed progression.
- Adding new UI flows, difficulty settings, or heavy AI/pathfinding dependencies.

## Acceptance Criteria
- Given the enemy has multiple safe moves during `Running`, when it chooses a direction, then the logic is not based only on the shortest path to food.
- Given no immediate danger, when the enemy evaluates moves, then it still shows a general bias toward pursuing food instead of wandering aimlessly.
- Given a safe detour exists, when the enemy moves, then it may choose a plausible non-optimal move so behavior is less deterministic across similar situations.
- Given at least one safe move exists, when the enemy chooses its next move, then it does not intentionally pick an immediately fatal move.
- Given the story is implemented, when the game is played, then existing controls, phases, scoring, growth, food behavior, collision rules, wraparound, and tick-speed behavior remain unchanged.
- Automated tests cover the updated enemy decision behavior, using reproducible randomness if needed.

## Open Questions
- How much weaker should the enemy become, if at all?
- Should the variation come from weighted randomness, heuristic bias, or both?
- Which human-like traits matter most: occasional hesitation, continuing straight, or softer food pursuit?
