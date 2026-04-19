You are working inside a file-backed story workflow.

Story ID: ST-00005
Story Title: Make the IA looks more human
Current Step: Documentation
Story Baseline Commit: 23c566accbbfee92c374763b3407f53e59392af3
Story Branch: story/ST-00005
Workspace Root: c:\Users\jimiv\Documents\Projets\story-agent\test_project
Last Implementation Commit: 297212e6a5c9bed076e1825899937776ee169dfc
Review Loop Count: 0

Step instructions:
You are the Documentation stage of Story Agent.
Generate a durable project-spec update based on the accepted story.
Documentation.md in the story folder is a trace artifact. The canonical project spec lives under doc/spec/.
Default to doc/spec/overview.md for small projects.
When the project specification grows, target a topical file under doc/spec/ such as doc/spec/gameplay.md, doc/spec/ui.md, or doc/spec/architecture.md.
Never use story IDs or changelog-style filenames under doc/spec/.
Choose the target spec file based on the actual documentation content and changed files, not by always falling back to overview.md.
The Canonical Spec section must contain the complete replacement content for the target spec file, written as the current project specification rather than a per-story changelog.
Return markdown only and do not wrap the answer in code fences.
Required structure:
# Spec Update
## Target Spec File
One markdown path under doc/spec/. Use doc/spec/overview.md by default.
## Change Summary
Summarize the spec impact in project terms.
## Canonical Spec
Write the full markdown document that should be saved to the target spec file.

Documentation target guidance:
Recommended target spec file: doc/spec/overview.md
Recommendation confidence: low
Recommendation reason: No strong topical signals were detected, so the canonical project summary should stay in overview.md.
Canonical spec file strategy:
- Use doc/spec/overview.md for broad or cross-cutting project specification updates.
- Use doc/spec/ui.md for presentation or rendering changes.
- Use doc/spec/gameplay.md for user-facing rules or runtime behavior.
- Use doc/spec/architecture.md for internal structure, workflows, or system design.
- Reuse an existing topical file when it already matches the change.

Existing canonical spec files:
- doc/spec/gameplay.md
- doc/spec/overview.md
- doc/spec/ui.md

Story artifacts so far:
### Draft
# Draft

## User Need
- As a player, the enemy snake should feel like a believable opponent instead of a perfect solver.
- Runs should feel less repetitive while the enemy still competes for food and creates pressure.

## Problem
- The current enemy logic heavily favors the closest safe path to food.
- This makes the opponent look overly optimal and predictable, especially in open space.
- Similar situations resolve the same way too often, so the enemy feels scripted rather than human.

## Goals
- Make enemy choices look more human by using simple heuristics instead of shortest-path-only behavior.
- Keep a clear but softer bias toward food.
- Allow believable, safe, slightly non-optimal moves such as continuing straight or taking a small detour.
- Preserve safety, readability, and the lightweight implementation style of the Snake crate.

## Non-Goals
- Making the enemy random, trivial, or obviously self-sabotaging.
- Changing player controls, phases, board size, wraparound, scoring, growth, food spawning, collision rules, or tick-speed progression.
- Adding new UI, difficulty settings, or heavy AI/pathfinding dependencies.
- Reworking unrelated Snake systems.

## Acceptance Criteria
- Given the enemy has more than one safe move during `Running`, when it selects a direction, then the choice is not based only on the shortest distance to food.
- Given continuing straight is safe and turning toward food is only a small improvement, when the enemy picks its next move, then it may continue straight instead of always taking the optimal turn immediately.
- Given a safe move toward food exists and no stronger safety concern applies, when the enemy evaluates its options, then it still generally prefers moves that close distance to food over moves that clearly drift away.
- Given at least one safe move exists, when the enemy chooses its next direction, then it does not choose an immediately fatal move.
- Given the story is implemented, when the game is played, then existing controls, phases, wraparound, collision behavior, food behavior, scoring, growth, and tick-speed progression remain unchanged.
- Automated tests cover representative enemy decision states, and any variability used by the logic is reproducible in tests.

## Open Questions
- Default assumption: the enemy should become only slightly weaker, not easy. Confirm if this balance is acceptable.
- Default assumption: variation should come from heuristic bias first, not pure randomness. Confirm if deterministic behavior is preferred.
- Proposed first human-like traits: safer forward momentum and softer food pursuit. Confirm if occasional hesitation should also be modeled.

### Plan
# Plan
## Summary
- Replace the current closest-safe-food enemy choice in `snake/src/game.rs` with a lightweight heuristic that still favors food but can safely keep moving straight or take a small detour.
- Keep all existing gameplay, input, and rendering behavior unchanged.
- Add seeded unit tests for representative enemy-decision states.

## Findings
- Enemy movement, collision resolution, and food handling are centralized in `snake/src/game.rs`, so the story can stay isolated to gameplay code.
- `Game::step` already receives `&mut impl Rng`, and `rand` is already present, so reproducible tie-breaking can be added without changing runtime wiring or dependencies.
- Current README/spec text still describes a simpler food-first enemy. A later Documentation step should update `snake/README.md` and the Snake gameplay spec after implementation.

## Affected Files
- `snake/src/game.rs`

## Implementation Approach
- Replace the current safe-move sort with explicit candidate-evaluation helpers in `snake/src/game.rs`:
  - enumerate legal enemy directions,
  - reject immediately fatal moves using the existing collision rules,
  - score the remaining moves,
  - choose from the best or near-best band.
- Base the score on small, readable heuristics instead of shortest-path-only behavior:
  - wrapped food-distance improvement,
  - bonus for continuing straight,
  - mild turn cost so a tiny food gain does not always force a turn,
  - local openness or follow-up safety bonus to avoid cramped moves when a safer option exists.
- Keep food-seeking as the main bias when the advantage is clear, but allow straight or slight-detour moves when scores are close.
- Use the existing RNG only for low-amplitude tie-breaking or near-best selection so runs vary while tests stay seedable.
- Add or extend `#[cfg(test)]` coverage in `snake/src/game.rs`, using local state-building helpers rather than exposing new public setup APIs.

## Risks
- Heuristic weights may leave the enemy too optimal or make it too weak; constants will need light tuning.
- Safety and openness helpers must match actual next-tick collision behavior, including wraparound and head/body rules, or the AI may rate a fatal move as safe.
- Documentation will be temporarily stale until the later Documentation step lands.

## Validation
- Run `cargo test --manifest-path snake/Cargo.toml`.
- Add tests that cover:
  - safe straight-vs-turn choice with only a small food improvement,
  - clear food-closing preference,
  - fatal-move avoidance when safe alternatives exist,
  - deterministic results with a seeded RNG.
- Run `cargo run --manifest-path snake/Cargo.toml` for a short manual play check that controls, phases, wraparound, collisions, scoring, growth, food spawning, and tick-speed progression remain unchanged while the enemy feels less repetitive.

### Implementation
# Implementation
## Execution Summary
- Replaced the enemy move picker in `snake/src/game.rs` with scored safe-move evaluation instead of closest-food-only ordering.
- Added lightweight heuristics for food progress, straight-line bias, turn cost, follow-up safety, dead-end avoidance, and seeded near-best tie-breaking.
- Added helper logic for legal enemy directions, projected post-move segments, and reusable collision checks.
- Expanded unit tests in `snake/src/game.rs` with representative enemy-decision and seeded-RNG scenarios.

## Files Changed
- `snake/src/game.rs`
  - Implemented the more human-like enemy decision logic.
  - Added internal scoring/helpers for safe candidate evaluation.
  - Added enemy behavior tests for straight-vs-turn, food preference, and reproducible variability.

## Validation Commands
- Not run in this step:
  - `cargo test --manifest-path snake/Cargo.toml`
  - `cargo run --manifest-path snake/Cargo.toml`

## Notes
- Documentation follow-up remains for `snake/README.md` and `doc/spec/*`, which still describe the older food-first enemy behavior.
- Validation should recheck any legacy head-to-head expectations, since the enemy now avoids an immediately fatal move when a safe alternative exists.

### Review
Verdict: approved

# Review
## Scope
Coverage: complete
Reviewed Head Commit: 297212e6a5c9bed076e1825899937776ee169dfc
Changed Files Reviewed: snake/src/game.rs

## Blind Spots
- none

## Findings
Validation Status: declared-only
- No blocking or material defects were identified in the reviewed diff.
- Validation is still unverified: the implementation artifact says `cargo test --manifest-path snake/Cargo.toml` and `cargo run --manifest-path snake/Cargo.toml` were not run, and no execution logs were provided.

## Evidence Summary
- Inspected the complete pinned inline diff for `snake/src/game.rs`.
- Read head context covering the new enemy scoring, safe-move selection, collision helpers, and added tests.
- Read the corresponding base section for the replaced enemy-selection logic.
- The pinned inline diff was complete.

## Follow-up
- Run the declared Snake validation commands to replace declared-only validation with execution evidence.

### Documentation
# Spec Update

Run the Documentation step to generate this artifact.

## Target Spec File
doc/spec/overview.md

## Change Summary

## Canonical Spec

Project specification snapshot:
### doc/spec/ui.md
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
- A compact footer below the board keeps phase-specific status and control guidance visible for ready, running, and game-

[content truncated]

### doc/spec/overview.md
# Test Project Specification

## Overview

- The test project contains a lightweight in-memory task-reporting sample and an isolated Rust terminal Snake sample under `snake/`.
- `doc/spec/overview.md` is the canonical project spec for broad or cross-cutting updates. As the spec grows, topical files such as `doc/spec/ui.md`, `doc/spec/gameplay.md`, or `doc/spec/architecture.md` can hold focused specifications.

## Task List Application

### Current behavior

- Tasks have an id, title, status, priority, and optional due date.
- The report groups tasks by status.
- Closed tasks are still included in the report.
- There is no archive concept.

### Constraints

- Keep the project lightweight and dependency free.
- Preserve the simple in-memory design.
- Prefer small, explicit functions over framework-style abstractions.

## Snake Sample

### Current behavior

- Running `cargo run --manifest-path snake/Cargo.toml` launches the interactive terminal Snake sample.
- The Snake UI uses `ratatui` and presents a structured layout with a title area, board, score and tick-speed stats, a phase-specific status message, and controls/help text.
- The board continues to render the snake and food distinctly.
- The existing controls are preserved:
	- `Enter` or `Space` starts from ready.
	- Arrow keys or `W`, `A`, `S`, `D` move the snake.
	- `R`, `Enter`, or `Space` restarts after game over.
	- `Q` or `Esc` quits.
- If the terminal is too small for the fixed board-plus-sidebar layout, the game shows a resize warning instead of scaling gameplay.

### Constraints

- Snake gameplay behavior remains unchanged, including movement rules, wraparound, collisions, scoring, food spawning, and tick-speed progression.
- `ratatui` is the primary rendering path for t

[content truncated]

### doc/spec/gameplay.md
# Snake Gameplay Specification

## Scope

This document defines the gameplay rules for the terminal Snake sample under `snake/`. Presentation details such as layout, colors, and resize messaging belong in `doc/spec/ui.md`.

## Game Summary

- The Snake sample is a real-time terminal game played on a fixed logical board.
- Each run includes:
  - one player-controlled snake,
  - one AI-controlled enemy snake,
  - one shared food item.
- The game keeps the existing three phases:
  - `Ready`
  - `Running`
  - `GameOver`

## Board Model

- The logical board size is 20 columns by 15 rows.
- Board edges wrap on all sides.
- Moving past one edge places the snake on the opposite edge on the same axis.
- A board cell can be empty or occupied by:
  - the player head,
  - the player body,
  - the enemy head,
  - the enemy body,
  - food.

## Starting State

- A new run initializes the player snake, the enemy snake, and the food on distinct cells.
- The player and enemy both begin as length-3 snakes.
- The game starts in the `Ready` phase.
- Score starts at 0 for each new run.

## Phases

### Ready

- The board is visible before movement begins.
- The player snake, enemy snake, and food are all shown in their starting positions.
- No movement occurs until the player starts the run.

### Running

- The player snake and the enemy snake both advance once per gameplay tick.
- Player input may change the player snake direction subject to normal turn rules.
- Food, growth, collisions, score updates, and tick-speed changes are resolved during this phase.

### GameOver

- The board remains visible in its ended state.
- No further movement occurs until the run is restarted.

## Controls and Turn Rules

- The controls remain:
  - `Enter` or `Space` starts from `Ready`
  - Arrow keys or `W`, `

[content truncated]

Repository validation:
Repository branch story/ST-00005 is clean.

Project workspace snapshot:
Project root: c:\Users\jimiv\Documents\Projets\story-agent\test_project

Selected file index:
- snake/src/game.rs
- snake/README.md
- doc/spec/gameplay.md
- doc/spec/overview.md
- doc/spec/ui.md
- snake/Cargo.toml
- .serena/project.yml
- .serena/project.local.yml
- snake/src/render.rs
- snake/src/main.rs
- snake/src/input.rs
- .serena/memories/project_overview.md
- .serena/memories/style_conventions.md
- .serena/memories/suggested_commands.md

Selected file excerpts:
### snake/src/game.rs
use std::cmp::Reverse;
use std::collections::VecDeque;
use std::time::Duration;

use rand::Rng;

const BASE_TICK_MS: u64 = 180;
const SPEED_STEP_MS: u64 = 8;
const MIN_TICK_MS: u64 = 70;
const START_LENGTH: usize = 3;
const ENEMY_FOOD_DISTANCE_WEIGHT: i32 = 2;
const ENEMY_FOOD_PROGRESS_BONUS: i32 = 1;
const ENEMY_FOOD_DRIFT_PENALTY: i32 = 1;
const ENEMY_STRAIGHT_BONUS: i32 = 3;
const ENEMY_TURN_PENALTY: i32 = 1;
const ENEMY_FOOD_CAPTURE_BONUS: i32 = 4;
const ENEMY_FOLLOW_UP_WEIGHT: i32 = 1;
const ENEMY_DEAD_END_PENALTY: i32 = 3;
const ENEMY_NEAR_BEST_BAND: i32 = 1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Point {
  pub x: i32,
  pub y: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
  Up,
  Down,
  Left,
  Right,
}

impl Direction {
  fn opposite(self) -> Direction {
    match self {
      Direction::Up => Direction::Down,
      Direction::Down => Direction::Up,
      Direction::Left => Direction::Right,
      Direction::Right => Direction::Left,
    }
  }

  fn left(self) -> Direction {
    match self {
      Direction::Up => Direction::Left,
      Direction::Down => Direction::Right,
      Direction::Left => Direction::Down,
      Direction::Right => Direction::Up,
    }
  }

  fn right(self) -> Direction {
    match self {
      Direction::Up => Direction::Right,
      Direction::Down => Direction::Left,
      Direction::Left => Direction::Up,
      Direction::Right => Direction::Down,
    }
  }

  fn is_opposite(self, other: Direction) -> bool {
    self.opposite() == other
  }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
  Ready,
  Running,
  GameOver,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
  Empty,
  Head,
  Body,
  EnemyHead,
  EnemyBody,
  Food,
}

#[derive(Clone, Debug)]
struct Snake {
  segments: VecDeque<Point>,
  direction: Direction,
  pending_direction: Option<Direction>,
}

impl Default for Snake {
  fn default() -> Self {
    Self {
      segments: VecDeque::new(),
      direction: Direction::Right,
      pending_direction: None,
    }
  }
}

impl Snake {
  fn new(segments: VecDeque<Point>, direction: Direction) -> Self {
    Self {
      segments,
      direct

[content truncated]

### snake/README.md
# Snake

A small standalone Rust terminal Snake game for Story `ST-00004`.

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
- A new run places the player snake, one AI enemy snake, and shared food on distinct cells.
- The player and enemy remain visible in ready, running, and game-over states, with distinct head/body colors for each snake.
- The enemy moves every running tick, follows the same wraparound movement rules as the player, prefers moving toward food, and falls back to another safe direction when needed.
- Food never spawns on either snake.
- Player-only food increases score and speeds up the tick rate until the existing minimum floor.
- If the enemy eats the shared food, it grows and a new food item spawns without changing player score or speed.
- Fatal collisions involving either snake, including self-collision, head-to-head contact, and player-to-enemy overlap, end the run through the existing game-over phase.
- If the terminal is too small for the compact layout, the game shows a resize message.
- The crate is intentionally isolated under `snake/` and does not modify the existing JS/TS workspace.

### doc/spec/gameplay.md
# Snake Gameplay Specification

## Scope

This document defines the gameplay rules for the terminal Snake sample under `snake/`. Presentation details such as layout, colors, and resize messaging belong in `doc/spec/ui.md`.

## Game Summary

- The Snake sample is a real-time terminal game played on a fixed logical board.
- Each run includes:
  - one player-controlled snake,
  - one AI-controlled enemy snake,
  - one shared food item.
- The game keeps the existing three phases:
  - `Ready`
  - `Running`
  - `GameOver`

## Board Model

- The logical board size is 20 columns by 15 rows.
- Board edges wrap on all sides.
- Moving past one edge places the snake on the opposite edge on the same axis.
- A board cell can be empty or occupied by:
  - the player head,
  - the player body,
  - the enemy head,
  - the enemy body,
  - food.

## Starting State

- A new run initializes the player snake, the enemy snake, and the food on distinct cells.
- The player and enemy both begin as length-3 snakes.
- The game starts in the `Ready` phase.
- Score starts at 0 for each new run.

## Phases

### Ready

- The board is visible before movement begins.
- The player snake, enemy snake, and food are all shown in their starting positions.
- No movement occurs until the player starts the run.

### Running

- The player snake and the enemy snake both advance once per gameplay tick.
- Player input may change the player snake direction subject to normal turn rules.
- Food, growth, collisions, score updates, and tick-speed changes are resolved during this phase.

### GameOver

- The board remains visible in its ended state.
- No further movement occurs until the run is restarted.

## Controls and Turn Rules

- The controls remain:
  - `Enter` or `Space` starts from `Ready`
  - Arrow keys or `W`, `A`, `S`, `D` steer the player snake
  - `R`, `Enter`, or `Space` restarts after `GameOver`
  - `Q` or `Esc` quits
- The player uses four-direction movement: up, down, left, and right.
- During `Running`, the player may queue one direction change per tick.
- Direct reversal into the opposite direction is not allowed while the snake length is greater than 1.
- Repeating the current direction does no

[content truncated]

### doc/spec/overview.md
# Test Project Specification

## Overview

- The test project contains a lightweight in-memory task-reporting sample and an isolated Rust terminal Snake sample under `snake/`.
- `doc/spec/overview.md` is the canonical project spec for broad or cross-cutting updates. As the spec grows, topical files such as `doc/spec/ui.md`, `doc/spec/gameplay.md`, or `doc/spec/architecture.md` can hold focused specifications.

## Task List Application

### Current behavior

- Tasks have an id, title, status, priority, and optional due date.
- The report groups tasks by status.
- Closed tasks are still included in the report.
- There is no archive concept.

### Constraints

- Keep the project lightweight and dependency free.
- Preserve the simple in-memory design.
- Prefer small, explicit functions over framework-style abstractions.

## Snake Sample

### Current behavior

- Running `cargo run --manifest-path snake/Cargo.toml` launches the interactive terminal Snake sample.
- The Snake UI uses `ratatui` and presents a structured layout with a title area, board, score and tick-speed stats, a phase-specific status message, and controls/help text.
- The board continues to render the snake and food distinctly.
- The existing controls are preserved:
	- `Enter` or `Space` starts from ready.
	- Arrow keys or `W`, `A`, `S`, `D` move the snake.
	- `R`, `Enter`, or `Space` restarts after game over.
	- `Q` or `Esc` quits.
- If the terminal is too small for the fixed board-plus-sidebar layout, the game shows a resize warning instead of scaling gameplay.

### Constraints

- Snake gameplay behavior remains unchanged, including movement rules, wraparound, collisions, scoring, food spawning, and tick-speed progression.
- `ratatui` is the primary rendering path for the `snake` crate. `crossterm` remains responsible for raw mode, alternate-screen handling, cursor visibility, and keyboard event polling.
- The Snake UI should stay small and explicit and should not introduce a larger UI framework or abstraction layer.
- The added Rust UI dependency is scoped to the isolated `snake/` crate and does not affect the lightweight, dependency-free constraint for the m

[content truncated]

### doc/spec/ui.md
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

- Snake gameplay behavior remains 

[content truncated]

### snake/Cargo.toml
[package]
name = "snake"
version = "0.1.0"
edition = "2021"

[dependencies]
crossterm = "0.27"
rand = "0.8"
ratatui = "0.26"

### .serena/project.yml
# the name by which the project can be referenced within Serena
project_name: "test_project"


# list of languages for which language servers are started; choose from:
#   al                  bash                clojure             cpp                 csharp
#   csharp_omnisharp    dart                elixir              elm                 erlang
#   fortran             fsharp              go                  groovy              haskell
#   haxe                java                julia               kotlin              lua
#   markdown
#   matlab              nix                 pascal              perl                php
#   php_phpactor        powershell          python              python_jedi         r
#   rego                ruby                ruby_solargraph     rust                scala
#   swift               terraform           toml                typescript          typescript_vts
#   vue                 yaml                zig
#   (This list may be outdated. For the current list, see values of Language enum here:
#   https://github.com/oraios/serena/blob/main/src/solidlsp/ls_config.py
#   For some languages, there are alternative language servers, e.g. csharp_omnisharp, ruby_solargraph.)
# Note:
#   - For C, use cpp
#   - For JavaScript, use typescript
#   - For Free Pascal/Lazarus, use pascal
# Special requirements:
#   Some languages require additional setup/installations.
#   See here for details: https://oraios.github.io/serena/01-about/020_programming-languages.html#language-servers
# When using multiple languages, the first language server that supports a given file will be used for that file.
# The first language is the default language and the respective language server will be used as a fallback.
# Note that when using the JetBrains backend, language servers are not used and this list is correspondingly ignored.
languages:
- typescript

# the encoding used by text files in the project
# For a list of possible encodings, see https://docs.python.org/3.11/library/codecs.html#standard-encodings
encoding: "utf-8"

# line ending convention to use when writing source files.
# Possible values: unset (use global s

[content truncated]

### .serena/project.local.yml
# This file allows you to locally override settings in project.yml for development purposes.
#
# Use the same keys as in project.yml here. Any setting you specify will override the corresponding
# setting in project.yml, allowing you to customise the configuration for your local development environment
# without affecting the project configuration in project.yml (which is intended to be versioned).

### snake/src/render.rs
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

use crate::game::{Cell, Game, Phase};

const HUD_HEIGHT: u16 = 1;
const FOOTER_HEIGHT: u16 = 1;
const CELL_WIDTH: u16 = 2;
const INFO_MIN_WIDTH: u16 = 49;

pub fn draw(frame: &mut Frame<'_>, game: &Game) {
  let area = frame.size();

  if area.width < min_width(game) || area.height < min_height(game) {
    draw_resize_warning(frame, game, area);
    return;
  }

  let sections = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
      Constraint::Length(HUD_HEIGHT),
      Constraint::Length(board_height(game)),
      Constraint::Length(FOOTER_HEIGHT),
      Constraint::Min(0),
    ])
    .split(area);

  draw_hud(frame, sections[0], game);
  draw_board(frame, sections[1], game);
  draw_footer(frame, sections[2], game);
}

fn draw_hud(frame: &mut Frame<'_>, area: Rect, game: &Game) {
  let hud = Paragraph::new(hud_line(game)).alignment(Alignment::Center);

  frame.render_widget(hud, area);
}

fn draw_board(frame: &mut Frame<'_>, area: Rect, game: &Game) {
  let board = Paragraph::new(board_lines(game))
    .alignment(Alignment::Center)
    .block(Block::default().borders(Borders::ALL));

  frame.render_widget(board, area);
}

fn draw_footer(frame: &mut Frame<'_>, area: Rect, game: &Game) {
  let footer = Paragraph::new(footer_text(game.phase())).alignment(Alignment::Center);

  frame.render_widget(footer, area);
}

fn draw_resize_warning(frame: &mut Frame<'_>, game: &Game, area: Rect) {
  let warning = Paragraph::new(vec![
    Line::from("Terminal too small."),
    Line::from(format!("Need {}x{} or larger.", min_width(game), min_height(game))),
    Line::from("Q or Esc quits."),
  ])
  .alignment(Alignment::Center)
  .block(Block::default().borders(Borders::ALL).title("Snake"));

  frame.render_widget(warning, area);
}

fn hud_line(game: &Game) -> Line<'static> {
  Line::from(vec![
    Span::styled("Snake", title_style()),
    Span::raw("  "),
    Span::styled("Score: ", label_style()),
    Span::raw(game.score().to_string(

[content truncated]

### snake/src/main.rs
mod game;
mod input;
mod render;

use std::io::{self, stdout, Stdout};
use std::time::{Duration, Instant};

use crossterm::cursor;
use crossterm::execute;
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use rand::thread_rng;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use crate::game::{Game, Phase};
use crate::input::Action;

const BOARD_WIDTH: i32 = 20;
const BOARD_HEIGHT: i32 = 15;
const IDLE_POLL_MS: u64 = 250;

type SnakeTerminal = Terminal<CrosstermBackend<Stdout>>;

fn main() -> io::Result<()> {
  let mut rng = thread_rng();
  let mut game = Game::new(BOARD_WIDTH, BOARD_HEIGHT, &mut rng);
  let _terminal_guard = TerminalGuard::enter()?;
  let backend = CrosstermBackend::new(stdout());
  let mut terminal = Terminal::new(backend)?;
  terminal.clear()?;
  let mut last_tick = Instant::now();

  render_frame(&mut terminal, &game)?;

  loop {
    let timeout = match game.phase() {
      Phase::Running => game
        .tick_duration()
        .checked_sub(last_tick.elapsed())
        .unwrap_or(Duration::from_millis(0)),
      Phase::Ready | Phase::GameOver => Duration::from_millis(IDLE_POLL_MS),
    };

    if let Some(action) = input::poll_action(game.phase(), timeout)? {
      match action {
        Action::Quit => break,
        Action::Turn(direction) => game.queue_turn(direction),
        Action::Start => {
          game.start();
          last_tick = Instant::now();
        }
        Action::Restart => {
          game.restart(&mut rng);
          last_tick = Instant::now();
        }
      }

      render_frame(&mut terminal, &game)?;
    }

    if game.phase() == Phase::Running && last_tick.elapsed() >= game.tick_duration() {
      game.step(&mut rng);
      last_tick = Instant::now();
      render_frame(&mut terminal, &game)?;
    }
  }

  Ok(())
}

fn render_frame(terminal: &mut SnakeTerminal, game: &Game) -> io::Result<()> {
  terminal.draw(|frame| render::draw(frame, game))?;
  Ok(())
}

struct TerminalGuard;

impl TerminalGuard {
  fn enter() -> io::Result<Self> {
    terminal::enable_raw_mode()?;
    let mut stdout = stdout();

    if let Err(error) = execute!(stdout, EnterAlternateScreen, cursor::H

[content truncated]

### snake/src/input.rs
use std::io;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use crate::game::{Direction, Phase};

pub enum Action {
  Start,
  Restart,
  Quit,
  Turn(Direction),
}

pub fn poll_action(phase: Phase, timeout: Duration) -> io::Result<Option<Action>> {
  if !event::poll(timeout)? {
    return Ok(None);
  }

  match event::read()? {
    Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
      Ok(map_key_event(phase, key_event))
    }
    _ => Ok(None),
  }
}

fn map_key_event(phase: Phase, key_event: KeyEvent) -> Option<Action> {
  match key_event.code {
    KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('W') => Some(Action::Turn(Direction::Up)),
    KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('S') => {
      Some(Action::Turn(Direction::Down))
    }
    KeyCode::Left | KeyCode::Char('a') | KeyCode::Char('A') => {
      Some(Action::Turn(Direction::Left))
    }
    KeyCode::Right | KeyCode::Char('d') | KeyCode::Char('D') => {
      Some(Action::Turn(Direction::Right))
    }
    KeyCode::Enter | KeyCode::Char(' ') => match phase {
      Phase::Ready => Some(Action::Start),
      Phase::GameOver => Some(Action::Restart),
      Phase::Running => None,
    },
    KeyCode::Char('r') | KeyCode::Char('R') => {
      if phase == Phase::GameOver {
        Some(Action::Restart)
      } else {
        None
      }
    }
    KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => Some(Action::Quit),
    _ => None,
  }
}

### .serena/memories/project_overview.md
- Purpose: Story Agent test workspace for validating the extension flow with a tiny codebase and generated task docs.
- Tech stack: Node.js, plain JavaScript ESM (`type: module`), no external dependencies.
- Core behavior: manages an in-memory task list and produces a text status report.
- Key files: `src/index.js` seeds sample tasks and prints a report; `src/reporting.js` groups tasks by status; `src/taskStore.js` manages in-memory task data; `doc/spec/overview.md` stores the spec snapshot; `doc/story/` stores story artifacts.
- Spec constraints: keep it lightweight and dependency free, preserve the simple in-memory design, prefer small explicit functions over framework-style abstractions.

### .serena/memories/style_conventions.md
- Use plain JavaScript ESM imports/exports.
- Match the existing lightweight style: small explicit functions, minimal abstraction.
- Preserve the dependency-free and in-memory design unless a task explicitly changes that constraint.
- Existing code uses semicolons and 2-space indentation.
- Task objects use explicit fields like `id`, `title`, `status`, `priority`, and optional `dueDate`.

### .serena/memories/suggested_commands.md
- Run app: `npm start` or `node ./src/index.js`
- Inspect files in PowerShell: `Get-ChildItem`, `Get-Content`, `rg`
- Git basics: `git status`, `git diff`
- There are currently no test, lint, or format scripts in `package.json`.

Git diff summary since story base:
snake/src/game.rs | 287 +++++++++++++++++++++++++++++++++++++++++-------------
 1 file changed, 217 insertions(+), 70 deletions(-)

Git diff since story base:
diff --git a/snake/src/game.rs b/snake/src/game.rs
index fe61552..b1ce041 100644
--- a/snake/src/game.rs
+++ b/snake/src/game.rs
@@ -10,2 +10,11 @@ const MIN_TICK_MS: u64 = 70;
 const START_LENGTH: usize = 3;
+const ENEMY_FOOD_DISTANCE_WEIGHT: i32 = 2;
+const ENEMY_FOOD_PROGRESS_BONUS: i32 = 1;
+const ENEMY_FOOD_DRIFT_PENALTY: i32 = 1;
+const ENEMY_STRAIGHT_BONUS: i32 = 3;
+const ENEMY_TURN_PENALTY: i32 = 1;
+const ENEMY_FOOD_CAPTURE_BONUS: i32 = 4;
+const ENEMY_FOLLOW_UP_WEIGHT: i32 = 1;
+const ENEMY_DEAD_END_PENALTY: i32 = 3;
+const ENEMY_NEAR_BEST_BAND: i32 = 1;
 
@@ -110,2 +119,8 @@ impl Snake {
 
+#[derive(Clone, Copy)]
+struct EnemyMove {
+  direction: Direction,
+  score: i32,
+}
+
 pub struct Game {
@@ -178,3 +193,3 @@ impl Game {
     let player_direction = self.resolve_player_direction();
-    let enemy_direction = self.choose_enemy_direction(player_direction);
+    let enemy_direction = self.choose_enemy_direction(player_direction, rng);
     let player_next_head = self.advance(self.player.head(), player_direction);
@@ -266,5 +281,6 @@ impl Game {
 
-  fn choose_enemy_direction(&self, player_direction: Direction) -> Direction {
+  fn choose_enemy_direction(&self, player_direction: Direction, rng: &mut impl Rng) -> Direction {
     let player_next_head = self.advance(self.player.head(), player_direction);
     let player_grows = self.food == Some(player_next_head);
+    let mut safe_moves = Vec::with_capacity(4);
 
@@ -274,60 +290,97 @@ impl Game {
 
-      if !self.move_is_fatal(player_next_head, enemy_next_head, player_grows, enemy_grows) {
-        return direction;
+      if self.move_is_fatal(player_next_head, enemy_next_head, player_grows, enemy_grows) {
+        continue;
       }
+
+      safe_moves.push(EnemyMove {
+        direction,
+        score: self.score_enemy_move(
+          player_next_head,
+          player_grows,
+          direction,
+          enemy_next_head,
+          enemy_grows,
+        ),
+      });
     }
 
-    self.enemy.direction
-  }
+    if safe_moves.is_empty() {
+      return self.enemy.direction;
+    }
 
-  fn enemy_move_candidates(&self) -> Vec<Direction> {
-    let mut candidates = Vec::with_capacity(5);
+    let best_score = safe_moves
+      .iter()
+      .map(|candidate| candidate.score)
+      .max()
+      .expect("safe_moves is not empty");
+    let near_best: Vec<Direction> = safe_moves
+      .iter()
+      .filter(|candidate| candidate.score + ENEMY_NEAR_BEST_BAND >= best_score)
+      .map(|candidate| candidate.direction)
+      .collect();
 
-    for direction in self.preferred_food_directions(self.enemy.head()) {
-      self.push_enemy_candidate(&mut candidates, direction);
+    if near_best.len() == 1 {
+      near_best[0]
+    } else {
+      let index = rng.gen_range(0..near_best.len());
+      near_best[index]
     }
+  }
 
-    self.push_enemy_candidate(&mut candidates, self.enemy.direction);
-    self.push_enemy_candidate(&mut candidates, self.enemy.direction.left());
-    self.push_enemy_candidate(&mut candidates, self.enemy.direction.right());
-
-    if self.enemy.len() == 1 {
-      self.push_enemy_candidate(&mut candidates, self.enemy.direction.opposite());
+  fn score_enemy_move(
+    &self,
+    player_next_head: Point,
+    player_grows: bool,
+    direction: Direction,
+    enemy_next_head: Point,
+    enemy_grows: bool,
+  ) -> i32 {
+    let mut score = 0;
+    let food_delta = self.food_distance_delta(self.enemy.head(), enemy_next_head);
+
+    if food_delta > 0 {
+      score += food_delta * ENEMY_FOOD_DISTANCE_WEIGHT + ENEMY_FOOD_PROGRESS_BONUS;
+    } else if food_delta < 0 {
+      score += food_delta * ENEMY_FOOD_DISTANCE_WEIGHT - ENEMY_FOOD_DRIFT_PENALTY;
     }
 
-    candidates
-  }
+    if direction == self.enemy.direction {
+      score += ENEMY_STRAIGHT_BONUS;
+    } else {
+      score -= ENEMY_TURN_PENALTY;
+    }
 
-  fn push_enemy_candidate(&self, candidates: &mut Vec<Direction>, direction: Direction) {
-    if self.enemy.len() > 1 && direction.is_opposite(self.enemy.direction) {
-      return;
+    if enemy_grows {
+      score += ENEMY_FOOD_CAPTURE_BONUS;
     }
 
-    if !candidates.contains(&direction) {
-      candidates.push(direction);
+    let follow_up_options = self.enemy_follow_up_options(
+      player_next_head,
+      player_grows,
+      enemy_next_head,
+      direction,
+      enemy_grows,
+    );
+    score += follow_up_options as i32 * ENEMY_FOLLOW_UP_WEIGHT;
+
+    if follow_up_options == 0 {
+      score -= ENEMY_DEAD_END_PENALTY;
     }
+
+    score
   }
 
-  fn preferred_food_directions(&self, from: Point) -> Vec<Direction> {
-    let mut directions = Vec::with_capacity(2);
-    let food = match self.food {
-      Some(food) => food,
-      None => return directions,
-    };
-    let horizontal = self.horizontal_food_direction(from, food);
-    let vertical = self.vertical_food_direction(from, food);
-
-    match (horizontal, vertical) {
-      (Some((horizontal_direction, horizontal_gap)), Some((vertical_direction, vertical_gap))) => {
-        if horizontal_gap >= vertical_gap {
-          directions.push(horizontal_direction);
-          directions.push(vertical_direction);
-        } else {
-          directions.push(vertical_direction);
-          directions.push(horizontal_direction);
-        }
-      }
-      (Some((horizontal_direction, _)), None) => directions.push(horizontal_direction),
-      (None, Some((vertical_direction, _))) => directions.push(vertical_direction),
-      (None, None) => {}
+  fn enemy_move_candidates(&self) -> Vec<Direction> {
+    Self::legal_directions(self.enemy.direction, self.enemy.len())
+  }
+
+  fn legal_directions(direction: Direction, len: usize) -> Vec<Direction> {
+    let mut directions = Vec::with_capacity(4);
+
+    directions.push(direction);
+    directions.push(direction.left());
+    directions.push(direction.right());
+
+    if len == 1 {
+      directions.push(direction.opposite());
     }
@@ -337,12 +390,6 @@ impl Game {
 
-  fn horizontal_food_direction(&self, from: Point, food: Point) -> Option<(Direction, i32)> {
-    let right_distance = (food.x - from.x).rem_euclid(self.width);
-    let left_distance = (from.x - food.x).rem_euclid(self.width);
-
-    if right_distance == 0 {
-      None
-    } else if right_distance <= left_distance {
-      Some((Direction::Right, right_distance))
-    } else {
-      Some((Direction::Left, left_distance))
+  fn food_distance_delta(&self, from: Point, to: Point) -> i32 {
+    match self.food {
+      Some(food) => self.wrapped_distance_score(from, food) - self.wrapped_distance_score(to, food),
+      None => 0,
     }
@@ -350,13 +397,22 @@ impl Game {
 
-  fn vertical_food_direction(&self, from: Point, food: Point) -> Option<(Direction, i32)> {
-    let down_distance = (food.y - from.y).rem_euclid(self.height);
-    let up_distance = (from.y - food.y).rem_euclid(self.height);
-
-    if down_distance == 0 {
-      None
-    } else if down_distance <= up_distance {
-      Some((Direction::Down, down_distance))
-    } else {
-      Some((Direction::Up, up_distance))
-    }
+  fn enemy_follow_up_options(
+    &self,
+    player_next_head: Point,
+    player_grows: bool,
+    enemy_next_head: Point,
+    direction: Direction,
+    enemy_grows: bool,
+  ) -> usize {
+    let player_after_move = Self::snake_segments_after_move(&self.player, player_next_head, player_grows);
+    let enemy_after_move = Self::snake_segments_after_move(&self.enemy, enemy_next_head, enemy_grows);
+
+    Self::legal_directions(direction, enemy_after_move.len())
+      .into_iter()
+      .filter(|follow_direction| {
+        let follow_head = self.advance(enemy_next_head, *follow_direction);
+
+        !Self::segments_collision(&player_after_move, follow_head, false)
+          && !Self::segments_collision(&enemy_after_move, follow_head, false)
+      })
+      .count()
   }
@@ -381,10 +437,13 @@ impl Game {
   fn snake_collision(snake: &Snake, next_head: Point, grows: bool) -> bool {
+    Self::segments_collision(&snake.segments, next_head, grows)
+  }
+
+  fn segments_collision(segments: &VecDeque<Point>, next_head: Point, grows: bool) -> bool {
     let collision_len = if grows {
-      snake.len()
+      segments.len()
     } else {
-      snake.len().saturating_sub(1)
+      segments.len().saturating_sub(1)
     };
 
-    snake
-      .segments
+    segments
       .iter()
@@ -394,2 +453,13 @@ impl Game {
 
+  fn snake_segments_after_move(snake: &Snake, next_head: Point, grows: bool) -> VecDeque<Point> {
+    let mut segments = snake.segments.clone();
+    segments.push_front(next_head);
+
+    if !grows {
+      segments.pop_back();
+    }
+
+    segments
+  }
+
   fn apply_move_to_snake(snake: &mut Snake, direction: Direction, next_head: Point, grows: bool) {
@@ -609,2 +679,21 @@ mod tests {
 
+  fn enemy_choice(game: &Game, player_direction: Direction, seed: u64) -> Direction {
+    let mut rng = StdRng::seed_from_u64(seed);
+    game.choose_enemy_direction(player_direction, &mut rng)
+  }
+
+  fn tie_break_game() -> Game {
+    let mut rng = seeded_rng();
+    let mut game = Game::new(7, 7, &mut rng);
+
+    game.phase = Phase::Running;
+    game.player = snake(
+      Direction::Right,
+      [point(6, 6), point(5, 6), point(4, 6), point(3, 2)],
+    );
+    game.enemy = snake(Direction::Up, [point(3, 3), point(3, 4), point(2, 4)]);
+    game.food = Some(point(3, 5));
+    game
+  }
+
   fn snakes_occupy(game: &Game, point: Point) -> bool {
@@ -736,2 +825,38 @@ mod tests {
 
+  #[test]
+  fn enemy_can_keep_moving_straight_when_a_food_turn_leads_into_a_dead_end() {
+    let mut rng = seeded_rng();
+    let mut game = Game::new(7, 7, &mut rng);
+
+    game.phase = Phase::Running;
+    game.player = snake(
+      Direction::Right,
+      [
+        point(6, 6),
+        point(5, 6),
+        point(4, 6),
+        point(4, 2),
+        point(3, 1),
+        point(2, 2),
+      ],
+    );
+    game.enemy = snake(Direction::Right, [point(3, 3), point(2, 3), point(1, 3)]);
+    game.food = Some(point(3, 0));
+
+    assert_eq!(enemy_choice(&game, Direction::Right, 7), Direction::Right);
+  }
+
+  #[test]
+  fn enemy_prefers_open_move_that_closes_food_distance() {
+    let mut rng = seeded_rng();
+    let mut game = Game::new(7, 7, &mut rng);
+
+    game.phase = Phase::Running;
+    game.player = snake(Direction::Right, [point(6, 6), point(5, 6), point(4, 6)]);
+    game.enemy = snake(Direction::Up, [point(3, 3), point(3, 4), point(3, 5)]);
+    game.food = Some(point(1, 3));
+
+    assert_eq!(enemy_choice(&game, Direction::Right, 7), Direction::Left);
+  }
+
   #[test]
@@ -752,2 +877,24 @@ mod tests {
 
+  #[test]
+  fn seeded_rng_makes_enemy_tie_breaking_reproducible() {
+    let game = tie_break_game();
+    let first = enemy_choice(&game, Direction::Right, 11);
+    let second = enemy_choice(&game, Direction::Right, 11);
+    let mut saw_left = false;
+    let mut saw_right = false;
+
+    for seed in 0..32 {
+      match enemy_choice(&game, Direction::Right, seed) {
+        Direction::Left => saw_left = true,
+        Direction::Right => saw_right = true,
+        direction => panic!("unexpected direction {:?}", direction),
+      }
+    }
+
+    assert_eq!(first, second);
+    assert!(matches!(first, Direction::Left | Direction::Right));
+    assert!(saw_left);
+    assert!(saw_right);
+  }
+
   #[test]

Changed files since story base:
- snake/src/game.rs

Return markdown only.
