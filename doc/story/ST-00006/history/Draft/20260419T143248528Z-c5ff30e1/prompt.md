You are working inside a file-backed story workflow.

Story ID: ST-00006
Story Title: Decrease a bit the IA difficulty
Current Step: Draft
Story Baseline Commit: 19cf560d7a3a91cc4656977b5aa721f977d3fe3c
Story Branch: story/ST-00006
Workspace Root: c:\Users\jimiv\Documents\Projets\story-agent\test_project
Last Implementation Commit: none
Review Loop Count: 0

Step instructions:
You are the Draft stage of Story Agent.
Improve the story draft into a reviewable artifact.
Return markdown only and do not wrap the answer in code fences.
Required structure:
# Draft
## User Need
## Problem
## Goals
## Non-Goals
## Acceptance Criteria
## Open Questions

Story artifacts so far:
### Draft
# Draft

## Title
Decrease a bit the IA difficulty

## User Need
Describe the user need and the expected outcome for this story.

## Problem

## Goals

## Non-Goals

## Acceptance Criteria

## Open Questions

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
- snake/README.md
- doc/spec/ui.md
- doc/spec/overview.md
- doc/spec/gameplay.md
- .serena/project.yml
- .serena/project.local.yml
- .serena/memories/task_completion.md
- .serena/memories/suggested_commands.md

Selected file excerpts:
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

### .serena/memories/task_completion.md
- After code changes, run `npm start` to confirm the app still executes.
- Because no automated test/lint/format scripts are configured, perform focused manual verification for the changed behavior.
- Call out any verification gaps clearly in the final response.
- Keep changes aligned with the lightweight, dependency-free, in-memory project constraints unless the task requires otherwise.

### .serena/memories/suggested_commands.md
- Run app: `npm start` or `node ./src/index.js`
- Inspect files in PowerShell: `Get-ChildItem`, `Get-Content`, `rg`
- Git basics: `git status`, `git diff`
- There are currently no test, lint, or format scripts in `package.json`.

Git diff summary since story base:
No changed files detected since the story baseline.

Git diff since story base:
No textual diff detected since the story baseline.

Changed files since story base:
- No changed files detected.

Return markdown only.
