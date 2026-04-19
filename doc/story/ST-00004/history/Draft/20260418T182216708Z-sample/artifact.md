# Draft

## User Need
- As a player of the terminal Snake sample, the user wants one AI-controlled enemy snake on the board.
- The enemy should make each run feel less static by competing for space and food.
- The result should add challenge without changing the current controls or lightweight terminal presentation.

## Problem
- The current Snake sample is a solo loop with only the player snake and food.
- Challenge comes only from self-collision and rising tick speed.
- The game state and rendering do not yet model rival snakes, so there is no current path to introduce one enemy now and support more later.

## Goals
- Add one AI-controlled enemy snake to the isolated Rust `snake/` crate.
- Keep the existing ready, running, and game-over phases and preserve the current controls.
- Render the enemy distinctly from the player snake, food, and empty board cells.
- Give the enemy simple autonomous movement that is understandable, testable, and small in scope.
- Make the enemy follow the same board rules where applicable: wraparound, occupied-cell checks, food interaction, and collision handling.
- Keep player score and tick-speed progression understandable and consistent with current behavior.
- Avoid design choices that would make future support for multiple AI snakes unnecessarily hard.
- Update Snake-facing documentation and spec text for the new behavior.

## Non-Goals
- Supporting more than one AI enemy in this story.
- Adding new controls, menus, difficulty settings, pause/resume, or configuration.
- Implementing advanced pathfinding or highly optimized AI.
- Adding a dedicated versus HUD, enemy score display, or new victory system.
- Redesigning the terminal layout beyond what is needed to show the enemy clearly.
- Changing the task-reporting sample or adding cross-project dependencies.

## Acceptance Criteria
- `cargo run --manifest-path snake/Cargo.toml` still launches the Snake sample, and the existing controls continue to work unchanged.
- `cargo test --manifest-path snake/Cargo.toml` passes, including tests for enemy-specific behavior.
- A new run initializes the player snake, one enemy snake, and food on distinct board cells.
- The enemy snake is rendered in ready, running, and game-over states and is visually distinct from the player snake; enemy head and body are distinct if player head and body are distinct.
- During `Running`, the enemy advances automatically once per gameplay tick using the same four-direction movement model and wraparound rules as the player.
- The enemy uses a simple, testable heuristic that prefers moving toward food and falls back to another safe direction when its first choice would cause an immediate collision.
- Food never spawns on a cell occupied by either snake.
- If the enemy eats the shared food, it grows and a new food item spawns on an unoccupied cell.
- Player score increases only when the player eats food, and tick-speed progression remains based on player score as it is today.
- Any fatal enemy-related collision, including player-to-enemy overlap, head-to-head contact, or enemy self-collision, transitions to the existing game-over phase.
- The compact board-first UI and resize-warning behavior continue to work with the added enemy rendering.
- `snake/README.md` and relevant spec documentation describe the enemy snake, its food interaction, and its collision behavior.

## Open Questions
- None blocking for this story.
- Should a future story expose enemy-specific HUD data such as length or score?
- How far should this story generalize the internal model for multiple AI snakes: clear extension points only, or a collection-based model now?
- Should a future story add a distinct "player wins" outcome when the enemy crashes, instead of reusing game-over?
