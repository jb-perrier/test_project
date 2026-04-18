# Snake

A small standalone Rust terminal Snake game for Story `ST-00002`.

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

- The UI now uses `ratatui` for a structured terminal layout and keeps `crossterm` for input handling.
- If the terminal is too small for the board and side panel, the game shows a resize message.
- The board wraps at every edge.
- Eating food increases score and snake length.
- Tick speed increases with score until it reaches a minimum floor.
- The crate is intentionally isolated under `snake/` and does not modify the existing JS/TS workspace.
