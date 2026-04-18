# Spec Update

## Context
- The repository includes an isolated Rust `snake/` crate alongside the main in-memory task-list project.
- Story `ST-00002` replaces the Snake game's manual string-frame rendering with a structured terminal UI built with `ratatui`.
- The change is limited to presentation and terminal rendering. Existing game state, timing, and input behavior remain in the current game loop and input handling.

## Behavior
- Running `cargo run --manifest-path snake/Cargo.toml` still launches the interactive terminal Snake game.
- The Snake UI is now rendered through `ratatui` and is organized into distinct regions for:
  - game title
  - board/playfield
  - score and current tick speed
  - phase/status message
  - controls/help
- The board continues to show the snake and food distinctly.
- Ready, running, and game-over phases each present a phase-specific status message.
- Existing controls are preserved:
  - `Enter` or `Space` starts from ready
  - arrow keys or `W`, `A`, `S`, `D` move the snake
  - `R`, `Enter`, or `Space` restarts after game over
  - `Q` or `Esc` quits
- If the terminal is too small for the fixed board-plus-sidebar layout, the game shows a resize warning instead of scaling gameplay.

## Constraints
- Snake gameplay behavior remains unchanged, including movement rules, wraparound, collisions, scoring, food spawning, and tick-speed progression.
- `ratatui` is the primary rendering path for the `snake` crate. `crossterm` remains responsible for raw mode, alternate-screen handling, cursor visibility, and keyboard event polling.
- The Snake UI implementation should stay small and explicit and should not introduce a larger UI framework or abstraction layer.
- The added Rust UI dependency is scoped to the isolated `snake/` crate and does not change the lightweight, dependency-free constraint for the main JS task-list application.

## Validation Notes
- Expected checks for this story remain:
  - `cargo test --manifest-path snake/Cargo.toml`
  - `cargo run --manifest-path snake/Cargo.toml`
- Manual verification should confirm:
  - preserved controls
  - ready/running/game-over messaging
  - score and speed updates
  - board, snake, and food rendering
  - resize-warning behavior
  - terminal cleanup on exit
- Review approval was based on code inspection. Execution evidence was not provided, so build and runtime validation remain pending.
