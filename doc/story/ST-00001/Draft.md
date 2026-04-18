# Draft

## Title
Snake game

## User Need
Casual terminal players want a simple, responsive Snake game they can launch locally and play with a keyboard.

## Problem
There is no defined MVP yet for a Rust terminal Snake game. Without a clear draft, implementation could drift on core rules such as controls, scoring, pacing, and border behavior.

## Goals
- Ship a playable single-player Snake game in a text terminal using the stable Rust toolchain.
- Support real-time movement through arrow keys or WASD with keyboard-only input.
- Let players start a run, track their score, and restart after a game-over state.
- Increase game speed as the score grows to keep the session engaging.
- Use wrap-around borders so crossing an edge moves the snake to the opposite side.
- Keep the first version lightweight and focused on the core game loop.

## Non-Goals
- Fancy graphics beyond a text-based UI.
- Audio.

## Acceptance Criteria
- A player can start a new game and restart after reaching game over.
- The snake responds to arrow keys or WASD during active play.
- Eating food increases the score.
- Game speed increases as the score grows.
- Crossing a border wraps the snake to the opposite side.
- Self-collision ends the run and shows a clear game-over state.

## Open Questions
None at this stage.
