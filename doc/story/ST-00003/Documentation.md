# Spec Update
## Target Spec File
doc/spec/ui.md
## Change Summary
- The Snake section now specifies a compact board-first terminal layout instead of a larger title-plus-sidebar presentation.
- The spec now requires condensed always-visible game information, compact phase-specific guidance, and a larger on-screen playfield footprint through wider rendered board cells.
- Snake gameplay rules, controls, crate isolation, and resize-warning behavior remain unchanged.

## Canonical Spec
# Test Project Specification

## Overview

- The project contains a lightweight in-memory task-reporting sample and an isolated Rust terminal Snake sample under `snake/`.

## Task List Application

### Current behavior

- The application stores tasks in memory and produces a text report.
- Each task has an id, title, status, priority, and an optional due date.
- The report groups tasks by status.
- Closed tasks remain included in the report.
- There is no archive concept.

### Constraints

- Keep the main application lightweight and dependency free.
- Preserve the simple in-memory design.
- Prefer small, explicit functions over framework-style abstractions.

## Snake Sample

### Current behavior

- Running `cargo run --manifest-path snake/Cargo.toml` launches the interactive terminal Snake sample.
- The Snake sample uses `ratatui` for rendering and `crossterm` for input and terminal lifecycle management.
- The game uses a fixed 20-column by 15-row logical board and has ready, running, and game-over phases.
- The board wraps at every edge.
- Eating food increases score and snake length.
- Colliding with the snake's own body ends the run.
- Tick speed increases as score rises until it reaches a minimum floor.
- The UI uses a compact board-first terminal layout that prioritizes the playfield over supporting UI.
- A compact HUD above the board keeps score, current tick speed, and phase visible at all times.
- The board remains the visual focus inside a bordered playfield and uses wider rendered cells so it occupies more on-screen space without changing logical board dimensions.
- The snake head, snake body, food, and board cells remain visually distinct.
- A compact footer below the board keeps phase-specific status and control guidance visible for ready, running, and game-over states.
- The controls are:
  - `Enter` or `Space` starts from ready.
  - Arrow keys or `W`, `A`, `S`, `D` move the snake.
  - `R`, `Enter`, or `Space` restarts after game over.
  - `Q` or `Esc` quits.
- If the terminal is too small for the compact layout, the game shows a resize warning instead of clipping, distorting, or scaling gameplay.

### Constraints

- Snake gameplay behavior remains unchanged, including movement rules, wraparound, collisions, scoring, food spawning, and tick-speed progression.
- `ratatui` is the primary rendering path for the `snake` crate.
- `crossterm` remains responsible for raw mode, alternate-screen handling, cursor visibility, terminal lifecycle, and keyboard event polling.
- The Snake implementation should remain small and explicit and should not introduce a larger UI framework or abstraction layer.
- Rust dependencies are isolated to the `snake/` crate and do not change the lightweight, dependency-free constraint for the main task-reporting sample.
