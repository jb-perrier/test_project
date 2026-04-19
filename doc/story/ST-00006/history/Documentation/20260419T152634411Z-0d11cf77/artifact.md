# Spec Update

## Target Spec File
doc/spec/gameplay.md

## Change Summary
- Records the Snake enemy as a slightly softened competitor: it still moves every running tick and chases the shared food, but full steering replans now happen on the first running tick and then every other tick.
- Specifies skipped-tick behavior: keep the carried direction when safe, switch to a safe fallback when needed, and otherwise leave wraparound, scoring, growth, collisions, controls, and tick-speed progression unchanged.

## Canonical Spec
# Snake Gameplay Specification

## Scope

This document defines the gameplay rules for the terminal Snake sample under `snake/`. Presentation details such as layout, colors, and resize messaging belong in `doc/spec/ui.md`.

## Game Summary

- The Snake sample is a real-time terminal game played on a fixed logical board.
- Each run includes:
  - one player-controlled snake,
  - one AI-controlled enemy snake,
  - one shared food item.
- The game uses three phases:
  - `Ready`
  - `Running`
  - `GameOver`
- The game tracks player score only. The enemy has no separate score.

## Board Model

- The logical board size is 20 columns by 15 rows.
- Board edges wrap on all sides.
- Moving past one edge places the snake on the opposite edge on the same axis.
- A board cell can be empty or occupied by:
  - the player head,
  - the player body,
  - the enemy head,
  - the enemy body,
  - food.
- Food spawns only on unoccupied cells.

## Starting State

- A new run initializes the player snake, the enemy snake, and the food on distinct cells.
- The player and enemy both begin as length-3 snakes.
- The game starts in the `Ready` phase.
- Player score starts at 0 for each new run.
- The running tick duration starts at 180 ms.
- Restarting after `GameOver` creates a fresh run and resumes `Running` immediately.

## Phases

### `Ready`

- The board is visible before movement begins.
- The player snake, enemy snake, and food are all shown in their starting positions.
- No movement occurs until the player starts the run.

### `Running`

- The player snake and the enemy snake both advance once per gameplay tick.
- Player steering, enemy steering, movement, food resolution, growth, collisions, score updates, and tick-speed changes are resolved during this phase.

### `GameOver`

- The board remains visible in its ended state.
- No further movement occurs until the run is restarted.

## Controls and Player Turn Rules

- The controls are:
  - `Enter` or `Space` starts from `Ready`.
  - Arrow keys or `W`, `A`, `S`, `D` steer the player snake.
  - `R`, `Enter`, or `Space` restarts after `GameOver`.
  - `Q` or `Esc` quits.
- The player uses four-direction movement: up, down, left, and right.
- During `Running`, the player may queue a pending direction change for the next tick.
- Direct reversal into the opposite direction is not allowed.

## Movement and Tick Resolution

- Each `Running` tick resolves the player direction, chooses the enemy direction for that tick, and then moves both snakes one cell.
- Movement, food consumption, growth, collision resolution, score updates, and tick-speed updates are part of the same gameplay tick.
- If a fatal collision occurs, the phase changes to `GameOver` and movement stops until restart.

## Food, Growth, and Score

- The game uses one shared food item.
- A snake that eats the food grows by one segment.
- When the player eats the food:
  - player score increases by 1,
  - a new food item spawns on an empty cell.
- When the enemy eats the food:
  - the enemy grows,
  - a new food item spawns on an empty cell,
  - player score does not change,
  - tick speed does not change.
- Food never spawns on either snake.

## Enemy Movement Rules

- The enemy follows the same wraparound movement, food, growth, and collision rules as the player.
- The enemy moves on every `Running` tick.
- The enemy uses a lightweight heuristic rather than full pathfinding.
- The first `Running` tick after starting from `Ready` or restarting from `GameOver` is always a fresh replan tick.
- On a replan tick, the enemy evaluates safe non-reversing directions and prefers choices that close wrapped distance to the shared food so it remains a meaningful opponent.
- When several safe choices are similarly strong, the enemy may choose among near-best options.
- After the first running tick, the enemy performs another fresh replan every other `Running` tick.
- On the intervening ticks, the enemy keeps its current direction if that move remains safe.
- If the carried direction becomes unsafe and another safe direction exists, the enemy immediately switches to a safe fallback direction.
- If no safe direction exists, the enemy continues its current direction and the unavoidable collision resolves under the normal collision rules.
- The difficulty reduction is intentionally modest: the enemy remains active and able to contest the shared food.

## Collision Rules

- Any fatal collision by either snake ends the run.
- Fatal collisions include:
  - self-collision,
  - head-to-head contact between the player and the enemy,
  - a snake head entering a cell occupied by the other snake during the tick's movement resolution.
- The ended board remains visible in `GameOver` until restart.

## Tick-Speed Progression

- The running tick duration starts at 180 ms.
- Each point of player score reduces the tick duration by 8 ms.
- Tick duration has a minimum floor of 70 ms.
- Only player scoring changes tick speed.

## Gameplay Constraints

- The fixed board size, wraparound, controls, shared-food model, growth rules, collision rules, and player-only score ownership are core gameplay behavior.
- The softer enemy difficulty is a cadence adjustment, not a broader rebalance of controls, board geometry, scoring, or speed rules.
- The gameplay does not add player-selectable difficulty settings or a heavier pathfinding system.
