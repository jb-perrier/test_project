You are working inside a file-backed story workflow.

Story ID: ST-00006
Story Title: Decrease a bit the IA difficulty
Current Step: Implementation
Story Baseline Commit: 19cf560d7a3a91cc4656977b5aa721f977d3fe3c
Story Branch: story/ST-00006
Workspace Root: c:\Users\jimiv\Documents\Projets\story-agent\test_project
Last Implementation Commit: 394162269c99356437021e232194ec87d58758c5
Review Loop Count: 0

Step instructions:
You are the Implementation stage of Story Agent.
Use the available workspace tools to inspect and edit project files for real before you write the final artifact.
Keep tool use tight: discover relevant files once, read only the files you will actually change, then write the final contents instead of repeatedly exploring the workspace.
Do not just describe intended changes. Apply them in the workspace first, then summarize the actual changes you made.
The root repository working tree is the source of truth for this step. If generated files should be ignored, update .gitignore as part of the implementation.
Treat the Plan artifact's "## Affected Files" section as implementation-facing scope only. If older artifacts still mention doc/spec/ or doc/story/ there, treat those entries as out-of-scope documentation follow-up and do not edit them in this step.
If the plan, draft, or current docs mention updates under doc/spec/ or doc/story/, do not edit those files in this step. Canonical spec updates belong to Documentation, and story artifacts are managed by the workflow runtime.
When documentation or spec follow-up is needed, keep implementing the code changes only and mention the follow-up in the Implementation artifact Notes section instead of writing doc/spec/ directly.
You may create new files or overwrite existing project files, but do not modify doc/story/, doc/spec/, .story-agent/, .git/, node_modules/, out/, dist/, or coverage/.
Once the workspace changes are applied, stop calling tools and return the final artifact immediately.
Return markdown only and do not wrap the answer in code fences.
Required structure:
# Implementation
## Execution Summary
## Files Changed
## Validation Commands
## Notes

Story artifacts so far:
### Draft
# Draft

## User Need
- As a player of the Snake sample, the enemy AI should feel slightly less punishing so the game is fairer and easier to learn without losing the tension of a competing snake.
- The desired outcome is a small difficulty reduction, not a major rebalance.

## Problem
- The current enemy replans aggressively every running tick and follows food very efficiently, which can make runs feel harsher than intended.
- The current draft does not define what “decrease a bit” means, so implementation could weaken the enemy too much, too little, or change unrelated gameplay.

## Goals
- Reduce enemy difficulty slightly.
- Keep the enemy visible, active, and able to compete for shared food.
- Preserve existing board size, wraparound, phases, controls, scoring ownership, growth rules, collision rules, and lightweight implementation style.
- Keep the behavior simple to explain, implement, and verify.

## Non-Goals
- Adding difficulty settings or a settings UI.
- Removing the enemy or making it random/useless.
- Changing player controls, UI layout, board dimensions, scoring rules, or tick-speed rules.
- Replacing the simple AI with a larger pathfinding system.

## Acceptance Criteria
- During `Running`, the enemy still advances once per gameplay tick and remains subject to the current wraparound, food, growth, and collision rules.
- The enemy AI is softened by reduced steering responsiveness: it makes a fresh food-seeking steering decision on the first running tick and then every other running tick after that.
- On intervening ticks, the enemy keeps its current direction if that move is safe.
- If its carried direction becomes unsafe and another safe direction exists, the enemy immediately selects a safe fallback instead of knowingly crashing.
- When it does recalculate, the enemy still prefers a safe direction that moves it toward food, so it remains a meaningful opponent.
- Player score and tick-speed progression remain player-only. Enemy food consumption still grows the enemy and respawns food without increasing player score or speed.
- Relevant documentation, including `snake/README.md` and `doc/spec/gameplay.md`, is updated to describe the softened enemy behavior.

## Open Questions
- Is alternate-tick replanning the right amount of difficulty reduction, or should it be adjusted after a short playtest?
- Should the softened behavior remain fully deterministic, or is a small amount of randomness acceptable if more tuning is needed?
- Does `doc/spec/overview.md` also need a brief update if it summarizes current enemy behavior?

### Plan
# Plan
## Summary
- Soften the Snake enemy AI only in `snake/src/game.rs` by reducing how often it performs a fresh food-seeking replan.
- Keep the enemy moving every running tick, but make full steering recalculation happen on the first running tick and then every other tick.
- Preserve current wraparound, food, growth, collision, score, tick-speed, input, and rendering behavior; no UI or config changes are expected.

## Findings
- `snake/src/game.rs` appears to own tick progression, enemy steering, growth, collisions, and scoring, so the rebalance can stay localized there.
- The existing enemy logic already looks deterministic and weighted toward safe food pursuit, which makes cadence control a smaller change than retuning many AI weights or adding randomness.
- `snake/README.md` and `doc/spec/gameplay.md` currently describe the more aggressive enemy behavior; they should be updated in a later Documentation step. `doc/spec/overview.md` only needs follow-up if the broad summary should mention the softer AI.

## Affected Files
- `snake/src/game.rs`

## Implementation Approach
- Add a small enemy replan cadence field to `Game` and reset it in new-game, start, and restart paths so the first actual running movement tick is always a replan tick.
- Split enemy direction selection into two paths:
  - replan ticks: reuse the current safe food-seeking ranking;
  - skipped ticks: keep the carried direction if it is safe, otherwise pick another safe direction if one exists, and otherwise preserve the existing unavoidable-crash outcome.
- Leave the rest of `step()` intact so both snakes still move every gameplay tick and enemy food still only affects enemy growth and food respawn, not player score or tick speed.
- Add or adjust focused unit tests in `snake/src/game.rs`, using direct state setup where needed to avoid RNG-dependent assertions.

## Risks
- Off-by-one handling around `Ready` to `Running` or `restart` could make the first running tick skip replanning.
- If skipped ticks still reuse the full food-seeking chooser, the difficulty drop may be too small; if safety checks are rewritten too broadly, collision behavior could change unintentionally.
- Documentation will remain stale until the later Documentation step updates the Snake behavior description.

## Validation
- Run `cargo test --manifest-path snake/Cargo.toml`.
- Manually run `cargo run --manifest-path snake/Cargo.toml` and verify:
  - the enemy still advances every running tick;
  - it replans on the first running tick and then every other tick;
  - it keeps its current direction on intervening ticks when safe;
  - it still takes a safe fallback when its carried direction becomes unsafe;
  - enemy food consumption still grows the enemy without increasing player score or speeding up the game.
- Later Documentation work should update `snake/README.md` and `doc/spec/gameplay.md`, with `doc/spec/overview.md` only if the higher-level summary should mention the new enemy cadence.

### Implementation
# Implementation

## Execution Summary
- Softened the Snake enemy AI in `snake/src/game.rs` by adding an alternate-tick replan cadence.
- The enemy now performs a fresh food-seeking choice on the first running tick and then every other tick after that.
- On skipped ticks, the enemy keeps its current direction when safe; if that direction is unsafe, it now takes a safe fallback when available.
- Added focused unit tests for cadence reset, alternate-tick behavior, skipped-tick fallback, and unavoidable-crash handling.

## Files Changed
- `snake/src/game.rs`
  - Added `enemy_replans_next_tick` state plus reset/toggle helpers.
  - Refactored enemy direction selection into replan and carry-direction paths.
  - Added coverage for the reduced-difficulty behavior in unit tests.

## Validation Commands
- Not run in this step.
- `cargo test --manifest-path snake/Cargo.toml`
- `cargo run --manifest-path snake/Cargo.toml`

## Notes
- Documentation follow-up remains for `snake/README.md` and `doc/spec/gameplay.md`.
- `doc/spec/overview.md` may also need a follow-up if the broader Snake summary should mention the softer enemy cadence.

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
- The game uses three phases:
  - `Ready`
  - `Running`
  - `GameOver`
- The game tracks player score only. The enemy has no separate score.

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
- Player score starts at 0 for each new run.
- The running tick duration starts at the base game speed.

## Phases

### Ready

- The board is visible before movement begins.
- The player snake, enemy snake, and food are all shown in their starting positions.
- No movement occurs until the player starts the run.

### Running

- The player snake and the enemy snake both advance once per gameplay tick.
- Player input may change the player snake direction subject to the normal turn rules.
- Enemy movement, food, growth, collisions, score updates, and tick-speed changes are resolved during this phase.

### GameOver

- The board remains visible in its ended state.
- No further movement occurs until

[content truncated]

Repository validation:
Repository branch story/ST-00006 is clean.

Project workspace snapshot:
Project root: c:\Users\jimiv\Documents\Projets\story-agent\test_project

Selected file index:
- snake/src/game.rs
- snake/README.md
- doc/spec/ui.md
- doc/spec/overview.md
- doc/spec/gameplay.md
- snake/Cargo.toml
- .serena/project.yml
- .serena/project.local.yml
- snake/src/main.rs
- snake/src/input.rs
- snake/src/render.rs
- .serena/memories/style_conventions.md
- .serena/memories/project_overview.md
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
- The game uses three phases:
  - `Ready`
  - `Running`
  - `GameOver`
- The game tracks player score only. The enemy has no separate score.

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
- Player score starts at 0 for each new run.
- The running tick duration starts at the base game speed.

## Phases

### Ready

- The board is visible before movement begins.
- The player snake, enemy snake, and food are all shown in their starting positions.
- No movement occurs until the player starts the run.

### Running

- The player snake and the enemy snake both advance once per gameplay tick.
- Player input may change the player snake direction subject to the normal turn rules.
- Enemy movement, food, growth, collisions, score updates, and tick-speed changes are resolved during this phase.

### GameOver

- The board remains visible in its ended state.
- No further movement occurs until the run is restarted.

## Controls and Player Turn Rules

- The controls are:
  - `Enter` or `Space` starts from `Ready`.
  - Arrow keys or `W`, `A`, `S`, `D` steer the player snake.
  - `R`, `Enter`, or `Space` restarts after `GameOver`.
  - `Q` or `Esc` quits.
- The player uses four-direction movement: up, down, left, and right.
- During `Running`, the player may queue one direction change per 

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

### .serena/memories/style_conventions.md
- Use plain JavaScript ESM imports/exports.
- Match the existing lightweight style: small explicit functions, minimal abstraction.
- Preserve the dependency-free and in-memory design unless a task explicitly changes that constraint.
- Existing code uses semicolons and 2-space indentation.
- Task objects use explicit fields like `id`, `title`, `status`, `priority`, and optional `dueDate`.

### .serena/memories/project_overview.md
- Purpose: Story Agent test workspace for validating the extension flow with a tiny codebase and generated task docs.
- Tech stack: Node.js, plain JavaScript ESM (`type: module`), no external dependencies.
- Core behavior: manages an in-memory task list and produces a text status report.
- Key files: `src/index.js` seeds sample tasks and prints a report; `src/reporting.js` groups tasks by status; `src/taskStore.js` manages in-memory task data; `doc/spec/overview.md` stores the spec snapshot; `doc/story/` stores story artifacts.
- Spec constraints: keep it lightweight and dependency free, preserve the simple in-memory design, prefer small explicit functions over framework-style abstractions.

### .serena/memories/suggested_commands.md
- Run app: `npm start` or `node ./src/index.js`
- Inspect files in PowerShell: `Get-ChildItem`, `Get-Content`, `rg`
- Git basics: `git status`, `git diff`
- There are currently no test, lint, or format scripts in `package.json`.

Git diff summary since story base:
snake/src/game.rs | 149 ++++++++++++++++++++++++++++++++++++++++++++++++++----
 1 file changed, 139 insertions(+), 10 deletions(-)

Git diff since story base:
diff --git a/snake/src/game.rs b/snake/src/game.rs
index d7d774a..9eb53b7 100644
--- a/snake/src/game.rs
+++ b/snake/src/game.rs
@@ -133,2 +133,3 @@ pub struct Game {
   phase: Phase,
+  enemy_replans_next_tick: bool,
 }
@@ -148,2 +149,3 @@ impl Game {
       phase: Phase::Ready,
+      enemy_replans_next_tick: true,
     };
@@ -162,2 +164,3 @@ impl Game {
     self.food = self.spawn_food(rng);
+    self.reset_enemy_replan_cadence();
   }
@@ -167,2 +170,3 @@ impl Game {
       self.phase = Phase::Running;
+      self.reset_enemy_replan_cadence();
     }
@@ -173,2 +177,3 @@ impl Game {
     self.phase = Phase::Running;
+    self.reset_enemy_replan_cadence();
   }
@@ -194,2 +199,3 @@ impl Game {
     let enemy_direction = self.choose_enemy_direction(player_direction, rng);
+    self.advance_enemy_replan_cadence();
     let player_next_head = self.advance(self.player.head(), player_direction);
@@ -268,2 +274,10 @@ impl Game {
 
+  fn reset_enemy_replan_cadence(&mut self) {
+    self.enemy_replans_next_tick = true;
+  }
+
+  fn advance_enemy_replan_cadence(&mut self) {
+    self.enemy_replans_next_tick = !self.enemy_replans_next_tick;
+  }
+
   fn resolve_player_direction(&mut self) -> Direction {
@@ -289,5 +303,15 @@ impl Game {
     let player_grows = self.food == Some(player_next_head);
-    let mut safe_moves = Vec::with_capacity(4);
+    let safe_directions = self.enemy_safe_directions(player_next_head, player_grows);
 
-    for direction in self.enemy_move_candidates() {
+    if safe_directions.is_empty() {
+      return self.enemy.direction;
+    }
+
+    if !self.enemy_replans_next_tick {
+      return self.choose_carried_enemy_direction_or_safe_fallback(&safe_directions);
+    }
+
+    let mut safe_moves = Vec::with_capacity(safe_directions.len());
+
+    for direction in safe_directions {
       let enemy_next_head = self.advance(self.enemy.head(), direction);
@@ -295,6 +319,2 @@ impl Game {
 
-      if self.move_is_fatal(player_next_head, enemy_next_head, player_grows, enemy_grows) {
-        continue;
-      }
-
       safe_moves.push(EnemyMove {
@@ -311,6 +331,2 @@ impl Game {
 
-    if safe_moves.is_empty() {
-      return self.enemy.direction;
-    }
-
     let best_score = safe_moves
@@ -320,2 +336,3 @@ impl Game {
       .expect("safe_moves is not empty");
+
     let near_best: Vec<Direction> = safe_moves
@@ -334,2 +351,34 @@ impl Game {
 
+  fn choose_carried_enemy_direction_or_safe_fallback(
+    &self,
+    safe_directions: &[Direction],
+  ) -> Direction {
+    if safe_directions.contains(&self.enemy.direction) {
+      self.enemy.direction
+    } else {
+      safe_directions[0]
+    }
+  }
+
+  fn enemy_safe_directions(
+    &self,
+    player_next_head: Point,
+    player_grows: bool,
+  ) -> Vec<Direction> {
+    let mut safe_directions = Vec::with_capacity(4);
+
+    for direction in self.enemy_move_candidates() {
+      let enemy_next_head = self.advance(self.enemy.head(), direction);
+      let enemy_grows = self.food == Some(enemy_next_head);
+
+      if self.move_is_fatal(player_next_head, enemy_next_head, player_grows, enemy_grows) {
+        continue;
+      }
+
+      safe_directions.push(direction);
+    }
+
+    safe_directions
+  }
+
   fn score_enemy_move(
@@ -763,2 +812,3 @@ mod tests {
     game.enemy.pending_direction = Some(Direction::Down);
+    game.enemy_replans_next_tick = false;
     game.reset(&mut rng);
@@ -775,2 +825,3 @@ mod tests {
     assert_eq!(game.enemy.pending_direction, None);
+    assert!(game.enemy_replans_next_tick);
     assert_eq!(
@@ -783,2 +834,15 @@ mod tests {
 
+  #[test]
+  fn restart_resets_enemy_replan_cadence_for_the_next_tick() {
+    let mut rng = seeded_rng();
+    let mut game = Game::new(10, 8, &mut rng);
+
+    game.phase = Phase::GameOver;
+    game.enemy_replans_next_tick = false;
+    game.restart(&mut rng);
+
+    assert_eq!(game.phase, Phase::Running);
+    assert!(game.enemy_replans_next_tick);
+  }
+
   #[test]
@@ -872,2 +936,66 @@ mod tests {
 
+  #[test]
+  fn enemy_replans_on_first_running_tick_then_every_other_tick() {
+    let mut rng = seeded_rng();
+    let mut game = Game::new(7, 7, &mut rng);
+
+    game.player = snake(Direction::Right, [point(6, 6), point(5, 6), point(4, 6)]);
+    game.enemy = snake(Direction::Right, [point(3, 3), point(2, 3), point(1, 3)]);
+    game.food = Some(point(1, 2));
+    game.enemy_replans_next_tick = false;
+
+    game.start();
+
+    assert_eq!(game.phase, Phase::Running);
+    assert!(game.enemy_replans_next_tick);
+
+    game.step(&mut rng);
+
+    assert_eq!(game.enemy.direction, Direction::Up);
+    assert_eq!(game.enemy.head(), point(3, 2));
+    assert!(!game.enemy_replans_next_tick);
+
+    game.step(&mut rng);
+
+    assert_eq!(game.enemy.direction, Direction::Up);
+    assert_eq!(game.enemy.head(), point(3, 1));
+    assert!(game.enemy_replans_next_tick);
+
+    game.step(&mut rng);
+
+    assert_eq!(game.enemy.direction, Direction::Left);
+    assert_eq!(game.enemy.head(), point(2, 1));
+    assert_eq!(game.phase, Phase::Running);
+    assert!(!game.enemy_replans_next_tick);
+  }
+
+  #[test]
+  fn enemy_uses_safe_fallback_on_skipped_ticks_when_straight_becomes_unsafe() {
+    let mut rng = seeded_rng();
+    let mut game = Game::new(6, 6, &mut rng);
+
+    game.phase = Phase::Running;
+    game.player = snake(Direction::Right, [point(5, 5), point(4, 5), point(3, 5)]);
+    game.enemy = snake(
+      Direction::Up,
+      [
+        point(2, 2),
+        point(2, 1),
+        point(1, 1),
+        point(1, 2),
+        point(1, 3),
+        point(2, 3),
+      ],
+    );
+    game.food = Some(point(0, 0));
+    game.enemy_replans_next_tick = false;
+
+    game.step(&mut rng);
+
+    assert_eq!(game.enemy.direction, Direction::Right);
+    assert_eq!(game.enemy.head(), point(3, 2));
+    assert_eq!(game.phase, Phase::Running);
+    assert!(game.enemy_replans_next_tick);
+  }
+
   #[test]
@@ -1065,2 +1193,3 @@ mod tests {
     game.food = Some(point(2, 0));
+    game.enemy_replans_next_tick = false;

Changed files since story base:
- snake/src/game.rs

User steering for this run:
the IA turn around food endless

Return markdown only.
