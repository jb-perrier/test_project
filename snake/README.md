# Snake

A small standalone Rust terminal Snake game for Story `ST-00001`.

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

- The board wraps at every edge.
- Eating food increases score and snake length.
- Tick speed increases with score until it reaches a minimum floor.
- The crate is intentionally isolated under `snake/` and does not modify the existing JS/TS workspace.
