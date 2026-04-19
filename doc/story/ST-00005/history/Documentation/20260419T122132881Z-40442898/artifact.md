# Spec Update

## Target Spec File
doc/spec/gameplay.md

## Change Summary
- Update the Snake gameplay spec to describe the enemy as a safety-first, heuristic opponent instead of a shortest-path-only food solver.
- Record that the enemy still contests food, but may keep moving straight or take small safe detours when a turn offers only a minor improvement.
- Clarify that limited, seedable variation is allowed among near-best safe moves while all other gameplay rules remain unchanged.

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

## Starting State

- A new run initializes the player snake, the enemy snake, and the food on distinct cells.
- The player and enemy both begin as length-3 snakes.
- The game starts in the `Ready` phase.
- Player score starts at 0 for each new run.
- The running tick duration starts at the base game speed.

## Phases

### Ready

- The board is visible before movement begins.
- The player snake, enemy snake, and food are all shown in their starting positions.
- No movement occurs until the player starts the run.

### Running

- The player snake and the enemy snake both advance once per gameplay tick.
- Player input may change the player snake direction subject to the normal turn rules.
- Enemy movement, food, growth, collisions, score updates, and tick-speed changes are resolved during this phase.

### GameOver

- The board remains visible in its ended state.
- No further movement occurs until the run is restarted.

## Controls and Player Turn Rules

- The controls are:
  - `Enter` or `Space` starts from `Ready`.
  - Arrow keys or `W`, `A`, `S`, `D` steer the player snake.
  - `R`, `Enter`, or `Space` restarts after `GameOver`.
  - `Q` or `Esc` quits.
- The player uses four-direction movement: up, down, left, and right.
- During `Running`, the player may queue one direction change per tick.
- Direct reversal into the opposite direction is not allowed while the snake length is greater than 1.
- Repeating the current direction or entering an illegal opposite turn leaves movement unchanged.

## Tick Resolution

- On each running tick, the game resolves the player's next legal direction first.
- The enemy then chooses its direction for the same tick using the current board state and the projected player move.
- The next head position for each snake is computed using wraparound movement.
- A snake grows on a tick when its next head lands on the shared food cell.
- When a snake does not grow, its tail vacates as part of the same move.
- Movement is resolved simultaneously for collision purposes, so collision checks account for both next-head positions and whether each tail vacates on that tick.

## Food, Growth, and Scoring

- There is exactly one shared food item on the board during play.
- Food never spawns on a cell occupied by either snake.
- When the player eats food:
  - the player grows by one segment,
  - player score increases by 1,
  - a new food item spawns on an unoccupied cell.
- When the enemy eats food:
  - the enemy grows by one segment,
  - a new food item spawns on an unoccupied cell,
  - player score does not change,
  - tick speed does not change.
- The game does not track a separate enemy score.

## Enemy Snake Behavior

- The enemy is intended to feel like a believable opponent that competes for shared food and board space without acting like a perfect solver.
- The enemy follows the same wraparound movement and collision rules as the player.
- The enemy only considers legal directions for its current body state:
  - straight ahead,
  - a left turn,
  - a right turn,
  - reverse only when the snake length is 1.
- When at least one immediately safe move exists, the enemy does not choose an immediately fatal move.
- Among safe moves, the enemy uses lightweight heuristics instead of a shortest-path-only food strategy.
- The enemy still has a clear food bias:
  - moves that reduce wrapped distance to food are generally preferred over moves that clearly drift away,
  - safe immediate food captures receive extra priority.
- The enemy also uses human-like momentum and safety heuristics:
  - continuing straight is mildly favored,
  - turning carries a small cost,
  - moves that preserve more safe follow-up space are preferred,
  - obvious dead ends are penalized.
- Because of these heuristics, the enemy may continue straight or take a small safe detour when an immediate turn toward food offers only a minor improvement.
- When several safe moves fall within a narrow near-best band, the enemy may choose any of them to add limited variation between runs.
- This variation must remain seedable so automated tests can reproduce representative decision states.
- If every legal move is immediately fatal, the enemy keeps its current direction and normal collision resolution determines the outcome.

## Collision and End Conditions

- Any fatal collision involving either snake ends the run and enters `GameOver`.
- Fatal outcomes include:
  - player self-collision,
  - enemy self-collision,
  - head-to-head contact,
  - the player head entering a snake segment that remains occupied after movement resolution,
  - the enemy head entering a snake segment that remains occupied after movement resolution.
- Collision checks respect growth and tail movement:
  - entering a tail cell that vacates on the same tick is allowed,
  - entering a segment that does not vacate on that tick is fatal.
- The game does not continue after an enemy-only death. A fatal collision by either snake ends the shared run.

## Speed Progression

- Running speed starts at 180 ms per tick.
- Each point of player score reduces tick duration by 8 ms.
- Tick duration does not go below 70 ms.
- Enemy food pickups do not affect tick duration.

## Constraints

- The enemy should feel competitive and readable, not perfectly optimal or purely random.
- Human-like variation should come from simple, lightweight heuristics and limited tie-breaking variation, not heavyweight pathfinding or difficulty systems.
- Existing controls, board size, wraparound, food spawning, scoring, growth, collision rules, phase behavior, and tick-speed progression remain part of the stable gameplay contract unless explicitly changed elsewhere.
