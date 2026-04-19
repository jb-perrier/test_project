# Plan

## Summary
- The draft calls for a Rust terminal Snake MVP, but the current workspace is a lightweight JS/TS sample with no Rust scaffold.
- The safest approach is to add the game as an isolated `snake/` crate so the story can be implemented without disturbing existing code or unrelated baseline drift.

## Findings
- Draft: the MVP is clear—arrow keys or WASD, score, faster pacing as score rises, wrap-around borders, self-collision game over, and restart after loss.
- Source: current files under `src/` implement task/report behavior only; there is no `Cargo.toml`, Rust source, or terminal game loop.
- Source: the root `src/` directory is already occupied, so Rust should not be introduced there.
- Documentation: `README.md`, `doc/spec/overview.md`, and `.serena` notes describe a small dependency-free JS project, not a game.
- Repo state: unrelated README/package/TypeScript drift already exists since the baseline, so scope should stay isolated.

## Affected Files
- `snake/Cargo.toml` — standalone Rust crate manifest with a minimal terminal-focused dependency set.
- `snake/src/main.rs` — entry point, tick loop, game-state transitions, and terminal cleanup.
- `snake/src/game.rs` — pure rules for movement, wrap-around, food spawning, growth, scoring, speed scaling, collision, and reset.
- `snake/src/input.rs` — key polling and mapping for arrows, WASD, start/restart, and quit.
- `snake/src/render.rs` — ASCII board, score display, instructions, and game-over screen.
- `snake/README.md` — local build/run instructions and control mapping.
- No changes planned to existing JS/TS task-report files or root docs.

## Implementation Approach
1. Create a standalone binary crate in `snake/` targeting stable Rust and keep the UI text-only.
2. Represent state with small explicit types for coordinates, direction, snake body, food, score, and phases such as `Ready`, `Running`, and `GameOver`.
3. Keep gameplay logic separate from terminal I/O so it can be unit-tested: ignore immediate reverse turns, wrap at edges, grow on food, respawn food only on empty cells, end on self-collision, and reduce tick duration as score increases with a minimum floor.
4. Implement a terminal loop that polls input between ticks, redraws the frame each update, and always restores raw mode, cursor visibility, and screen state on exit.
5. Support start/restart and quit actions with clear on-screen instructions and visible score.
6. Add focused tests around the pure rules rather than the interactive terminal layer.

## Risks
- The story is outside the current documented product scope, so accidental edits to root files would add review noise.
- Raw terminal handling can leave the shell in a bad state if cleanup is incomplete.
- Interactive timing and input behavior are harder to validate automatically than pure state transitions.
- Rust adds a second toolchain to a repo that otherwise assumes Node.js and lightweight JS.

## Validation
- Run `cargo test --manifest-path snake/Cargo.toml`.
- Run `cargo run --manifest-path snake/Cargo.toml`.
- Manually verify start, movement with arrows and WASD, score growth on food, speed increase with score, wrap-around behavior, clear game-over on self-collision, restart, and clean terminal restoration on quit.
- Run the existing `npm start` path as a regression check if no root files were touched.
