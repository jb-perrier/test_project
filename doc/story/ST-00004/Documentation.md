# Spec Update

## Target Spec File
doc/spec/gameplay.md

## Change Summary
Adds a dedicated Snake gameplay specification that defines the current runtime rules for the terminal game, including the single AI enemy snake, shared food behavior, fatal collision handling, and the rule that only player food increases score and accelerates tick speed.

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
- The game keeps the existing three phases:
  - `Ready`
  - `Running`
  - `GameOver`

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
- Score starts at 0 for each new run.

## Phases

### Ready

- The board is visible before movement begins.
- The player snake, enemy snake, and food are all shown in their starting positions.
- No movement occurs until the player starts the run.

### Running

- The player snake and the enemy snake both advance once per gameplay tick.
- Player input may change the player snake direction subject to normal turn rules.
- Food, growth, collisions, score updates, and tick-speed changes are resolved during this phase.

### GameOver

- The board remains visible in its ended state.
- No further movement occurs until the run is restarted.

## Controls and Turn Rules

- The controls remain:
  - `Enter` or `Space` starts from `Ready`
  - Arrow keys or `W`, `A`, `S`, `D` steer the player snake
  - `R`, `Enter`, or `Space` restarts after `GameOver`
  - `Q` or `Esc` quits
- The player uses four-direction movement: up, down, left, and right.
- During `Running`, the player may queue one direction change per tick.
- Direct reversal into the opposite direction is not allowed while the snake length is greater than 1.
- Repeating the current direction does not change movement.

## Player Snake Rules

- The player snake follows standard Snake movement with wraparound.
- On a normal move, the head advances and the tail vacates its last cell.
- When the player eats food:
  - the player snake grows by one segment,
  - score increases by 1,
  - a new shared food item spawns on an unoccupied cell.

## Enemy Snake Rules

- The game currently includes one AI-controlled enemy snake.
- The enemy moves automatically once per gameplay tick during `Running`.
- The enemy uses the same movement model as the player:
  - four directions,
  - wraparound at board edges,
  - growth on food,
  - occupied-cell collision checks.
- The enemy chooses movement with a simple deterministic heuristic:
  - it prefers directions that move it toward the current food using wrapped board distance,
  - if a preferred move would cause an immediate fatal collision, it falls back to another safe direction,
  - if multiple candidates are available, selection follows a stable priority order rather than random pathfinding.
- The enemy does not affect controls or introduce separate player input.

## Shared Food Rules

- Only one food item is present at a time.
- Food never spawns on a cell occupied by either snake.
- The food is shared by the player and the enemy.
- If the enemy eats the food:
  - the enemy grows by one segment,
  - score does not change,
  - tick speed does not change,
  - a new food item spawns on an unoccupied cell.

## Collision Rules

Any fatal collision ends the run by transitioning to `GameOver`.

Fatal cases include:

- player self-collision,
- enemy self-collision,
- player head entering a cell occupied by the enemy in a fatal way,
- enemy head entering a cell occupied by the player in a fatal way,
- both heads moving into the same cell on the same tick.

Collision handling is resolved from the same tick snapshot so that player and enemy movement is evaluated consistently.

## Score and Tick Speed

- Score belongs only to the player.
- Score increases only when the player eats food.
- Enemy food does not add to score.
- Tick speed progression remains tied to player score only.
- As player score rises, tick duration decreases until it reaches the existing minimum floor.
- Enemy growth alone does not accelerate the game.

## Food Spawning and Occupancy Constraints

- Food placement must consider occupancy from both snakes.
- A valid food spawn cell is any board cell not occupied by the player or enemy.
- Gameplay assumes the board can place two snakes and one food item without overlap.

## Current Gameplay Boundaries

- Current gameplay supports one AI enemy snake.
- No enemy-specific score, HUD, victory state, pause flow, difficulty setting, or advanced pathfinding is required.
- Enemy-related fatal outcomes reuse the existing `GameOver` phase rather than introducing a separate win screen.
