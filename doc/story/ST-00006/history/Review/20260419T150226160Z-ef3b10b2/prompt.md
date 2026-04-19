You are working inside a file-backed story workflow.

Story ID: ST-00006
Story Title: Decrease a bit the IA difficulty
Current Step: Review
Story Baseline Commit: 19cf560d7a3a91cc4656977b5aa721f977d3fe3c
Story Branch: story/ST-00006
Workspace Root: c:\Users\jimiv\Documents\Projets\story-agent\test_project
Last Implementation Commit: 875955c5ab7ac0b8f8b795315b57460ce36fdbe2
Review Loop Count: 0

Step instructions:
You are the Review stage of Story Agent.
Operate as an evidence-driven reviewer over the pinned review session supplied in the prompt.
Review only the implementation work that is already due at this stage and only for the exact base and head commits captured in the pinned review session.
Authoritative evidence order: the pinned review manifest, the exact git diff for that session, the current contents of changed files at the pinned head commit, the base contents of changed files when needed, then the draft, plan, and implementation artifact.
Use the read-only review tools whenever the inline diff is partial, when a finding depends on file context outside the inline diff, or when you need to inspect deleted or renamed files.
Keep tool use tight: inspect only the changed files needed to support the verdict, and once the evidence is sufficient, stop calling tools and return the artifact immediately.
Only raise blocking or material issues for files that appear in the pinned review session. Ignore unrelated workspace drift and untouched files.
Do not require documentation or specification updates in Review unless a missing document makes the implemented code incorrect or impossible to assess. Documentation belongs to the next step.
Treat validation conservatively. A command listed in the implementation artifact is not proof it ran. If execution evidence is unclear, say so explicitly instead of assuming validation happened.
If the available evidence is insufficient to finish the review safely, return Verdict: manual-review-required or Verdict: blocked instead of guessing.
Never omit the required evidence lines. If a field is unknown or empty, use the allowed placeholder values instead: Reviewed Head Commit: unknown, Changed Files Reviewed: none, and - none under Blind Spots when coverage is complete.
Return markdown only and do not wrap the answer in code fences.
The first non-heading line must be exactly one of:
Verdict: approved
Verdict: changes-required
Verdict: manual-review-required
Verdict: blocked
Required structure after the verdict:
# Review
## Scope
Include a line in exactly this form: Coverage: complete|partial.
Include a line in exactly this form: Reviewed Head Commit: <commit or unknown>.
Include a line in exactly this form: Changed Files Reviewed: <comma-separated paths or none>.
## Blind Spots
List any missing evidence, unreviewed files, or reasons that manual review is still required. Write exactly "- none" when coverage is complete.
## Findings
Include a line in exactly this form: Validation Status: verified|declared-only|not-provided|mixed.
Only include issues grounded in the pinned review evidence or in missing or unclear validation evidence.
## Evidence Summary
Summarize which files, diffs, or logs you inspected and whether the pinned inline diff was complete or partial.
## Follow-up

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
- Updated `snake/src/game.rs` to keep the alternate-tick enemy slowdown while reducing the food-orbiting behavior.
- Added a scoring penalty for replan turns that leave food adjacent and likely missed on the next carried tick, so the enemy is less likely to circle around food.
- Kept the skipped-tick safe fallback behavior and added a focused regression test for the anti-orbit case.

## Files Changed
- `snake/src/game.rs`
  - Added `ENEMY_MISSED_ADJACENT_FOOD_PENALTY`.
  - Added `enemy_would_miss_adjacent_food(...)` and applied it in `score_enemy_move(...)`.
  - Added `enemy_avoids_turning_beside_food_when_a_skipped_tick_would_pass_it` coverage.

## Validation Commands
- Not run in this step.
- `cargo test --manifest-path snake/Cargo.toml`
- `cargo run --manifest-path snake/Cargo.toml`

## Notes
- Documentation was not edited in this step because implementation scope stayed limited to `snake/src/game.rs`.
- Follow-up remains for `snake/README.md` and `doc/spec/gameplay.md`.
- `doc/spec/overview.md` may also need a later update if the broader Snake summary should mention the softer enemy cadence.

### Review
# Review

Run the Review step to generate this artifact.

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

Pinned review session manifest:
Base commit: 19cf560d7a3a91cc4656977b5aa721f977d3fe3c
Head commit: 875955c5ab7ac0b8f8b795315b57460ce36fdbe2
Changed file count: 1
Inline diff coverage: complete
Changed files:
- snake/src/game.rs [modified]

Diff summary:
snake/src/game.rs | 205 +++++++++++++++++++++++++++++++++++++++++++++++++++---
 1 file changed, 195 insertions(+), 10 deletions(-)

Pinned inline diff for this review session:
diff --git a/snake/src/game.rs b/snake/src/game.rs
index d7d774a..effe83f 100644
--- a/snake/src/game.rs
+++ b/snake/src/game.rs
@@ -18,2 +18,3 @@ const ENEMY_FOLLOW_UP_WEIGHT: i32 = 1;
 const ENEMY_DEAD_END_PENALTY: i32 = 3;
+const ENEMY_MISSED_ADJACENT_FOOD_PENALTY: i32 = 4;
 const ENEMY_NEAR_BEST_BAND: i32 = 1;
@@ -133,2 +134,3 @@ pub struct Game {
   phase: Phase,
+  enemy_replans_next_tick: bool,
 }
@@ -148,2 +150,3 @@ impl Game {
       phase: Phase::Ready,
+      enemy_replans_next_tick: true,
     };
@@ -162,2 +165,3 @@ impl Game {
     self.food = self.spawn_food(rng);
+    self.reset_enemy_replan_cadence();
   }
@@ -167,2 +171,3 @@ impl Game {
       self.phase = Phase::Running;
+      self.reset_enemy_replan_cadence();
     }
@@ -173,2 +178,3 @@ impl Game {
     self.phase = Phase::Running;
+    self.reset_enemy_replan_cadence();
   }
@@ -194,2 +200,3 @@ impl Game {
     let enemy_direction = self.choose_enemy_direction(player_direction, rng);
+    self.advance_enemy_replan_cadence();
     let player_next_head = self.advance(self.player.head(), player_direction);
@@ -268,2 +275,10 @@ impl Game {
 
+  fn reset_enemy_replan_cadence(&mut self) {
+    self.enemy_replans_next_tick = true;
+  }
+
+  fn advance_enemy_replan_cadence(&mut self) {
+    self.enemy_replans_next_tick = !self.enemy_replans_next_tick;
+  }
+
   fn resolve_player_direction(&mut self) -> Direction {
@@ -289,5 +304,15 @@ impl Game {
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
@@ -295,6 +320,2 @@ impl Game {
 
-      if self.move_is_fatal(player_next_head, enemy_next_head, player_grows, enemy_grows) {
-        continue;
-      }
-
       safe_moves.push(EnemyMove {
@@ -311,6 +332,2 @@ impl Game {
 
-    if safe_moves.is_empty() {
-      return self.enemy.direction;
-    }
-
     let best_score = safe_moves
@@ -320,2 +337,3 @@ impl Game {
       .expect("safe_moves is not empty");
+
     let near_best: Vec<Direction> = safe_moves
@@ -334,2 +352,34 @@ impl Game {
 
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
@@ -361,2 +411,12 @@ impl Game {
 
+    if self.enemy_would_miss_adjacent_food(
+      player_next_head,
+      player_grows,
+      enemy_next_head,
+      direction,
+      enemy_grows,
+    ) {
+      score -= ENEMY_MISSED_ADJACENT_FOOD_PENALTY;
+    }
+
     let follow_up_options = self.enemy_follow_up_options(
@@ -404,2 +464,33 @@ impl Game {
 
+  fn enemy_would_miss_adjacent_food(
+    &self,
+    player_next_head: Point,
+    player_grows: bool,
+    enemy_next_head: Point,
+    direction: Direction,
+    enemy_grows: bool,
+  ) -> bool {
+    if enemy_grows {
+      return false;
+    }
+
+    let Some(food) = self.food else {
+      return false;
+    };
+
+    if self.wrapped_distance_score(enemy_next_head, food) != 1 {
+      return false;
+    }
+
+    let player_after_move =
+      Self::snake_segments_after_move(&self.player, player_next_head, player_grows);
+    let enemy_after_move =
+      Self::snake_segments_after_move(&self.enemy, enemy_next_head, enemy_grows);
+    let straight_head = self.advance(enemy_next_head, direction);
+
+    !Self::segments_collision(&player_after_move, straight_head, false)
+      && !Self::segments_collision(&enemy_after_move, straight_head, false)
+      && straight_head != food
+  }
+
   fn enemy_follow_up_options(
@@ -763,2 +854,3 @@ mod tests {
     game.enemy.pending_direction = Some(Direction::Down);
+    game.enemy_replans_next_tick = false;
     game.reset(&mut rng);
@@ -775,2 +867,3 @@ mod tests {
     assert_eq!(game.enemy.pending_direction, None);
+    assert!(game.enemy_replans_next_tick);
     assert_eq!(
@@ -783,2 +876,15 @@ mod tests {
 
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
@@ -872,2 +978,66 @@ mod tests {
 
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
@@ -931,2 +1101,16 @@ mod tests {
 
+  #[test]
+  fn enemy_avoids_turning_beside_food_when_a_skipped_tick_would_pass_it() {
+    let mut rng = seeded_rng();
+    let mut game = Game::new(7, 7, &mut rng);
+
+    game.phase = Phase::Running;
+    game.player = snake(Direction::Right, [point(6, 6), point(5, 6), point(4, 6)]);
+    game.enemy = snake(Direction::Right, [point(3, 3), point(2, 3), point(1, 3)]);
+    game.food = Some(point(2, 2));
+    game.enemy_replans_next_tick = true;
+
+    assert_eq!(enemy_choice(&game, Direction::Right, 7), Direction::Right);
+  }
+
   #[test]
@@ -1065,2 +1249,3 @@ mod tests {
     game.food = Some(point(2, 0));
+    game.enemy_replans_next_tick = false;

Return markdown only.
