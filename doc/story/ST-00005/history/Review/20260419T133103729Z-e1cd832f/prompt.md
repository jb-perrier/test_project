You are working inside a file-backed story workflow.

Story ID: ST-00005
Story Title: Make the IA looks more human
Current Step: Review
Story Baseline Commit: 23c566accbbfee92c374763b3407f53e59392af3
Story Branch: story/ST-00005
Workspace Root: c:\Users\jimiv\Documents\Projets\story-agent\test_project
Last Implementation Commit: ca4a3638162d1cc21d81410e71d37f74a32f50fa
Review Loop Count: 3

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
- Updated `snake/src/game.rs` to fix the failing dead-end test fixture.
- Reworked `enemy_can_keep_moving_straight_when_a_food_turn_leads_into_a_dead_end` so the upward food turn now has $0$ follow-up options under the implemented scorer.
- Added fixture assertions in that test to verify the `Up` branch is safe immediately but becomes a true dead end before asserting the final enemy choice.
- Left gameplay heuristics unchanged outside the test setup and assertions.

## Files Changed
- `snake/src/game.rs`
  - Expanded the player fixture in the dead-end test to block the enemy's post-turn escape cells.
  - Added explicit assertions for `move_is_fatal(...) == false` and `enemy_follow_up_options(...) == 0` for the `Up` branch.
  - Kept the expected chosen direction as `Direction::Right` once the dead-end condition is real.

## Validation Commands
- `cargo test --manifest-path snake/Cargo.toml` — not run; no command execution tool was available.
- `cargo run --manifest-path snake/Cargo.toml` — not run; no command execution tool was available.

## Notes
- Validation output could not be attached in this environment because the available workspace tools support file inspection and editing only.
- Documentation follow-up remains out of scope for this step: `snake/README.md` and `doc/spec/*` were not edited.

### Review
Verdict: changes-required

# Review
## Scope
Coverage: complete
Reviewed Head Commit: bacb8b28e0215303c1ecd416be77e2274f495ffb
Changed Files Reviewed: snake/src/game.rs

## Blind Spots
- none

## Findings
Validation Status: declared-only
- `snake/src/game.rs:874-893` does not set up the claimed dead-end case. With the implemented scoring and follow-up counting in `snake/src/game.rs:343-375` and `snake/src/game.rs:405-426`, `Up` is still the best safe move for that fixture: it scores `4` (`+3` food progress, `-1` turn, `+2` follow-ups), while `Right` scores `2` (`-3` food drift, `+3` straight, `+2` follow-ups). Because `snake_segments_after_move`/`segments_collision` treat `(3, 1)` as the player's tail after the simulated move (`snake/src/game.rs:450-471`), the `Up` branch still has two follow-up options. `enemy_choice(&game, Direction::Right, 7)` should therefore return `Up`, so the assertion expecting `Right` is incorrect and will fail.
- The implementation artifact declares `cargo test --manifest-path snake/Cargo.toml` and `cargo run --manifest-path snake/Cargo.toml`, but no execution output was provided.

## Evidence Summary
- Inspected the complete pinned diff for `snake/src/game.rs`.
- Read the head implementation for enemy move selection, scoring, follow-up simulation, collision helpers, and the added tests (`snake/src/game.rs:283-471`, `snake/src/game.rs:689-950`).
- No validation logs were available. The pinned inline diff was complete.

## Follow-up
- Rework `enemy_can_keep_moving_straight_when_a_food_turn_leads_into_a_dead_end` so the food turn actually loses enough follow-up safety, or update the expected direction to match the implemented scorer.
- Run the declared Snake validation commands and attach the output.

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
Repository branch story/ST-00005 is clean.

Pinned review session manifest:
Base commit: 23c566accbbfee92c374763b3407f53e59392af3
Head commit: ca4a3638162d1cc21d81410e71d37f74a32f50fa
Changed file count: 1
Inline diff coverage: complete
Changed files:
- snake/src/game.rs [modified]

Diff summary:
snake/src/game.rs | 365 +++++++++++++++++++++++++++++++++++++++++++-----------
 1 file changed, 294 insertions(+), 71 deletions(-)

Pinned inline diff for this review session:
diff --git a/snake/src/game.rs b/snake/src/game.rs
index fe61552..d7d774a 100644
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
@@ -257,3 +272,4 @@ impl Game {
     if let Some(next_direction) = self.player.pending_direction.take() {
-      if next_direction != self.player.direction && !next_direction.is_opposite(self.player.direction)
+      if next_direction != self.player.direction
+        && !next_direction.is_opposite(self.player.direction)
       {
@@ -266,5 +282,10 @@ impl Game {
 
-  fn choose_enemy_direction(&self, player_direction: Direction) -> Direction {
+  fn choose_enemy_direction(
+    &self,
+    player_direction: Direction,
+    rng: &mut impl Rng,
+  ) -> Direction {
     let player_next_head = self.advance(self.player.head(), player_direction);
     let player_grows = self.food == Some(player_next_head);
+    let mut safe_moves = Vec::with_capacity(4);
 
@@ -274,60 +295,97 @@ impl Game {
 
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
@@ -337,12 +395,8 @@ impl Game {
 
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
+      Some(food) => {
+        self.wrapped_distance_score(from, food) - self.wrapped_distance_score(to, food)
+      }
+      None => 0,
     }
@@ -350,13 +404,24 @@ impl Game {
 
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
+    let player_after_move =
+      Self::snake_segments_after_move(&self.player, player_next_head, player_grows);
+    let enemy_after_move =
+      Self::snake_segments_after_move(&self.enemy, enemy_next_head, enemy_grows);
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
@@ -381,10 +446,13 @@ impl Game {
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
@@ -394,2 +462,13 @@ impl Game {
 
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
@@ -609,2 +688,59 @@ mod tests {
 
+  fn enemy_choice(game: &Game, player_direction: Direction, seed: u64) -> Direction {
+    let mut rng = StdRng::seed_from_u64(seed);
+    game.choose_enemy_direction(player_direction, &mut rng)
+  }
+
+  fn near_best_enemy_choices(game: &Game, player_direction: Direction) -> Vec<Direction> {
+    let player_next_head = game.advance(game.player.head(), player_direction);
+    let player_grows = game.food == Some(player_next_head);
+    let mut safe_moves = Vec::with_capacity(4);
+
+    for direction in game.enemy_move_candidates() {
+      let enemy_next_head = game.advance(game.enemy.head(), direction);
+      let enemy_grows = game.food == Some(enemy_next_head);
+
+      if game.move_is_fatal(player_next_head, enemy_next_head, player_grows, enemy_grows) {
+        continue;
+      }
+
+      safe_moves.push(EnemyMove {
+        direction,
+        score: game.score_enemy_move(
+          player_next_head,
+          player_grows,
+          direction,
+          enemy_next_head,
+          enemy_grows,
+        ),
+      });
+    }
+
+    let best_score = safe_moves
+      .iter()
+      .map(|candidate| candidate.score)
+      .max()
+      .expect("tie-break fixture should have safe moves");
+
+    safe_moves
+      .into_iter()
+      .filter(|candidate| candidate.score + ENEMY_NEAR_BEST_BAND >= best_score)
+      .map(|candidate| candidate.direction)
+      .collect()
+  }
+
+  fn tie_break_game() -> Game {
+    let mut rng = seeded_rng();
+    let mut game = Game::new(7, 7, &mut rng);
+
+    game.phase = Phase::Running;
+    game.player = snake(
+      Direction::Right,
+      [point(6, 6), point(5, 6), point(3, 2), point(4, 6)],
+    );
+    game.enemy = snake(Direction::Up, [point(3, 3), point(3, 4), point(2, 4)]);
+    game.food = Some(point(3, 5));
+    game
+  }
+
   fn snakes_occupy(game: &Game, point: Point) -> bool {
@@ -736,2 +872,74 @@ mod tests {
 
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
+        point(3, 4),
+        point(4, 2),
+        point(3, 1),
+        point(2, 2),
+        point(1, 2),
+        point(1, 1),
+      ],
+    );
+    game.enemy = snake(Direction::Right, [point(3, 3), point(2, 3), point(1, 3)]);
+    game.food = Some(point(3, 0));
+
+    let player_next_head = game.advance(game.player.head(), Direction::Right);
+    let player_grows = game.food == Some(player_next_head);
+    let upward_head = game.advance(game.enemy.head(), Direction::Up);
+    let upward_grows = game.food == Some(upward_head);
+
+    assert!(!game.move_is_fatal(
+      player_next_head,
+      upward_head,
+      player_grows,
+      upward_grows,
+    ));
+    assert_eq!(
+      game.enemy_follow_up_options(
+        player_next_head,
+        player_grows,
+        upward_head,
+        Direction::Up,
+        upward_grows,
+      ),
+      0
+    );
+    assert_eq!(enemy_choice(&game, Direction::Right, 7), Direction::Right);
+  }
+
+  #[test]
+  fn enemy_can_keep_moving_straight_when_straight_also_closes_food_distance() {
+    let mut rng = seeded_rng();
+    let mut game = Game::new(7, 7, &mut rng);
+
+    game.phase = Phase::Running;
+    game.player = snake(Direction::Right, [point(6, 6), point(5, 6), point(4, 6)]);
+    game.enemy = snake(Direction::Right, [point(3, 3), point(2, 3), point(1, 3)]);
+    game.food = Some(point(5, 2));
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
@@ -752,2 +960,16 @@ mod tests {
 
+  #[test]
+  fn seeded_rng_makes_enemy_tie_breaking_reproducible() {
+    let game = tie_break_game();
+    let near_best = near_best_enemy_choices(&game, Direction::Right);
+    let first = enemy_choice(&game, Direction::Right, 11);
+    let second = enemy_choice(&game, Direction::Right, 11);
+    let other_seed_choice = enemy_choice(&game, Direction::Right, 12);
+
+    assert_eq!(near_best, vec![Direction::Left, Direction::Right]);
+    assert_eq!(first, second);
+    assert!(near_best.contains(&first));
+    assert!(near_best.contains(&other_seed_choice));
+  }
+
   #[test]
@@ -785,2 +1007,3 @@ mod tests {
 
+    game.queue_turn(Direction::Left);
     game.step(&mut rng);

Return markdown only.
