# Snake

A small standalone Rust terminal Snake game for Story `ST-00003`.

## Run

- `cargo run --manifest-path snake/Cargo.toml`

## Test

- `cargo test --manifest-path snake/Cargo.toml`

## Controls

- `Enter` or `Space`: start from the ready screen
- Arrow keys or `W`, `A`, `S`, `D`: move the snake
- `R`, `Enter`, or `Space`: restart after game over
- `Q` or `Esc`: quit

## Notes

- The UI uses `ratatui` for a compact board-first terminal layout and keeps `crossterm` for input handling.
- A single-line HUD keeps score, tick speed, and phase visible while giving more terminal space to the board.
- The board uses larger cells, and phase-specific control guidance stays visible below the playfield.
- If the terminal is too small for the compact layout, the game shows a resize message.
- The board wraps at every edge.
- Eating food increases score and snake length.
- Tick speed increases with score until it reaches a minimum floor.
- The crate is intentionally isolated under `snake/` and does not modify the existing JS/TS workspace.
