# Plan

## Summary
Replace the manual string-and-`crossterm` render path in `snake/` with a small `ratatui` UI layer. Keep gameplay, timing, and key handling unchanged. Scope should stay inside the isolated `snake` crate.

## Findings
- The draft limits the change to presentation. Movement, scoring, wrapping, phases, and controls must remain unchanged.
- `snake/src/render.rs` currently builds the full screen as a `String`, including title, score, speed, board, controls, and phase message.
- `snake/src/main.rs` already separates ticking, input polling, and rendering, so the main loop can stay mostly intact.
- `snake/src/input.rs` already matches the required controls through `crossterm` events and can remain the input path.
- `snake/src/game.rs` already exposes the data needed for UI rendering: board size, cell lookup, score, phase, and tick duration.
- `snake/Cargo.toml` does not yet include `ratatui`.
- `snake/README.md` already documents the run/test flow and controls; only user-visible UI behavior notes should need updates.
- The root spec prefers lightweight changes, and `snake/` is already isolated from the main JS workspace, so the new dependency should remain contained there.

## Affected Files
- `snake/Cargo.toml` — add `ratatui`.
- `snake/src/main.rs` — switch from manual `Print(render::frame(...))` to a `ratatui` terminal draw call and adjust terminal setup/cleanup.
- `snake/src/render.rs` — replace `String` frame assembly with ratatui layout/widget rendering.
- `snake/README.md` — update notes if terminal-size behavior or UI expectations change.
- `snake/src/input.rs` — likely unchanged, unless a small integration cleanup is needed.
- `snake/src/game.rs` — expected unchanged.

## Implementation Approach
- Add `ratatui` with the crossterm backend in the `snake` crate.
- Keep `crossterm` for raw mode, alternate screen, and event polling.
- In `main.rs`, create a `ratatui` terminal and replace the current render helper with `terminal.draw(...)`.
- Retire `render::frame(game) -> String` as the primary renderer.
- Rework `render.rs` into small draw helpers that render:
  - a header with title, score, and speed
  - a bordered board area
  - a footer/status area with phase-specific text and controls
- Keep the board representation explicit by building board rows from `game.cell_at(...)` and rendering them inside ratatui widgets, rather than introducing a custom UI framework layer.
- Keep the board size fixed. If terminal space is too small, prefer a simple warning view instead of scaling gameplay.

## Risks
- Terminal state cleanup may regress if cursor visibility, raw mode, or alternate screen restoration is mishandled.
- Small terminals may clip the UI if no minimum-size handling is added.
- The board rewrite could accidentally change symbol placement or visible state if the current mapping is not preserved.
- `ratatui` and `crossterm` versions must be chosen compatibly.

## Validation
- Run `cargo test --manifest-path snake/Cargo.toml`.
- Run `cargo run --manifest-path snake/Cargo.toml`.
- Manually verify:
  - ready, running, and game-over messages
  - `Enter`/`Space` start behavior
  - arrow/WASD movement
  - `R`/`Enter`/`Space` restart after game over
  - `Q`/`Esc` quit
  - score and speed display updates
  - board, snake, and food rendering
- If a small-terminal fallback is added, verify that case manually as well.
