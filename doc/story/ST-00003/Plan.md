# Plan

## Summary
- Rework the Snake screen from a bordered title plus fixed sidebar into a compact board-first layout.
- Keep the existing `ratatui` and `crossterm` approach and avoid gameplay or control changes.
- Treat "increase game size" as increasing board emphasis and terminal footprint, not changing the logical playfield, unless clarified.

## Findings
- `snake/src/render.rs` is the main source of UI overhead: `HEADER_HEIGHT = 3`, `SIDEBAR_MIN_WIDTH = 32`, a separate title block, and a board/sidebar split.
- The same render module already owns stats, phase text, controls, board drawing, and resize-warning behavior, so the layout change is localized.
- `snake/src/main.rs` fixes the board at `20x15`; changing that would change the playfield and is not required by the draft goals.
- `snake/src/input.rs` and `snake/src/game.rs` already match the required controls and unchanged gameplay behavior.
- `snake/README.md` and `doc/spec/overview.md` still describe the current side-panel layout and need to be updated.

## Affected Files
- `snake/src/render.rs`
- `snake/README.md`
- `doc/spec/overview.md`
- Possible only if scope changes: `snake/src/main.rs`

## Implementation Approach
- Remove the separate title block and fixed sidebar, and replace them with a compact HUD around the board.
- Keep score, tick speed, and phase/status visible at all times in shorter text.
- Make controls more compact and phase-aware so ready and game-over guidance stays available without a large permanent help panel.
- Recalculate minimum width and height for the new layout and keep the resize-warning path instead of clipping.
- Preserve current snake and food distinction. If the compact layout still makes the board feel too small, widen the rendered cell presentation in `render.rs` rather than changing gameplay logic.

## Risks
- The story title is ambiguous about visual size versus logical board size; the plan assumes visual/layout size only.
- Over-compressing help text could make ready or game-over guidance too easy to miss.
- Resize thresholds and condensed text lengths can introduce off-by-one clipping if not recalculated carefully.

## Validation
- Run `cargo test --manifest-path snake/Cargo.toml`.
- Run `cargo run --manifest-path snake/Cargo.toml` and verify:
  - the board is the visual focus,
  - score and tick speed stay visible,
  - phase-specific status and controls remain available,
  - snake and food remain distinct.
- Resize the terminal above and below the new minimum to confirm normal rendering versus the resize warning.
- Review `snake/README.md` and `doc/spec/overview.md` for updated layout descriptions.
