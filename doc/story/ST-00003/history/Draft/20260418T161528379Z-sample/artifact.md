# Draft

## User Need
- As a Snake player, the screen should spend less space on supporting UI and more space on the board.
- The expected outcome is a cleaner layout where gameplay is the focus without losing essential status or control information.

## Problem
- The current Snake UI uses a fixed board-plus-sidebar layout with separate title, stats, status, and controls sections.
- That layout gives too much terminal space to chrome relative to the playable board.
- The result is a game presentation that feels smaller than necessary and can hit resize limits because of UI overhead.

## Goals
- Reduce the space used by title, help, and other supporting UI in `snake`.
- Increase the terminal space dedicated to the playable board.
- Keep essential information available: score, tick speed, phase/status, and core controls.
- Preserve the lightweight `ratatui` and `crossterm` approach within the isolated `snake/` crate.

## Non-Goals
- Changing movement, wraparound, collisions, scoring, food spawning logic, or speed progression.
- Changing the control scheme.
- Adding a larger UI framework or changing the task list application.
- Implementing fully responsive scaling for every terminal size.

## Acceptance Criteria
- Running `cargo run --manifest-path snake/Cargo.toml` shows a more compact Snake UI than the current baseline.
- The redesigned layout allocates more terminal space to the board than the current fixed board-plus-sidebar layout.
- Score and current tick speed remain visible during active play.
- Phase-specific status and control guidance remain available, even if presented more compactly.
- Snake and food remain visually distinct on the board.
- Existing controls remain unchanged:
  - `Enter` or `Space` starts from ready.
  - Arrow keys or `W`, `A`, `S`, `D` move the snake.
  - `R`, `Enter`, or `Space` restarts after game over.
  - `Q` or `Esc` quits.
- Movement rules, wraparound, collisions, scoring, food spawning logic, and tick-speed progression remain unchanged.
- If the terminal is too small for the redesigned layout, the game shows a resize warning rather than clipping or distorting gameplay.
- Relevant Snake documentation and spec text are updated to match the new layout.

## Open Questions
- Should "increase game size" mean only a larger on-screen board footprint, or also larger logical board dimensions?
- Is there a target terminal size the redesigned layout should support?
- Should full controls/help remain visible during active play, or can they be reduced outside ready and game-over states?
