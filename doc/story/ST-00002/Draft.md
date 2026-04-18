# Draft

## User Need
As a player running the terminal Snake game, the UI should feel structured and purpose-built for a terminal application so the board, score, status, and instructions are easier to read and maintain as the game evolves.

## Problem
The current Snake UI is assembled as a plain string and printed with `crossterm`. This works, but layout and presentation are manual, tightly coupled to string formatting, and harder to extend with clearer regions, styling, or responsive terminal rendering. The story asks to adopt `ratatui` for the UI layer without changing the core gameplay.

## Goals
- Replace the current string-based rendering path with a `ratatui`-based terminal UI in the `snake` crate.
- Preserve the existing game loop, controls, gameplay rules, and state transitions.
- Present the main game elements in a clearer layout:
  - title
  - score and speed
  - board
  - contextual status/help message
- Keep the implementation small and explicit, consistent with the existing code style.
- Keep the game runnable from the terminal with the current `cargo run --manifest-path snake/Cargo.toml` workflow.

## Non-Goals
- Changing snake movement, collision rules, scoring, wrap behavior, or tick-speed logic.
- Adding menus, settings screens, persistence, or new gameplay features.
- Redesigning input handling beyond what is needed to work with the `ratatui` render loop.
- Introducing a large UI architecture or framework-style abstraction layer.

## Acceptance Criteria
- The `snake` crate uses `ratatui` to render the terminal UI.
- The previous string-frame rendering approach is removed or no longer the primary render path.
- Running the game still opens an interactive terminal UI and supports the documented controls:
  - `Enter` or `Space` to start from ready
  - arrow keys or `W`, `A`, `S`, `D` to move
  - `R`, `Enter`, or `Space` to restart after game over
  - `Q` or `Esc` to quit
- The UI displays, at minimum:
  - game title
  - current score
  - current speed or tick rate
  - the game board with visible snake and food
  - a phase-specific message for ready, running, and game-over states
- Gameplay behavior remains unchanged from the current implementation.
- The game still builds and tests successfully with:
  - `cargo run --manifest-path snake/Cargo.toml`
  - `cargo test --manifest-path snake/Cargo.toml`
- README usage notes are updated if the UI dependency or terminal behavior changes in a user-visible way.

## Open Questions
- Should the first `ratatui` version keep a minimal monochrome layout, or should it introduce colors/borders/styling immediately?
- Should the board size remain fixed as it is now, or should `ratatui` layout adapt when the terminal is too small?
- Should the implementation continue to use `crossterm` directly for event handling while `ratatui` is used only for rendering?
