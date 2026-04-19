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
      direction,
      pending_direction: None,
    }
  }

  fn head(&self) -> Point {
    *self.segments.front().expect("snake always has a head")
  }

  fn len(&self) -> usize {
    self.segments.len()
  }
}

#[derive(Clone, Copy)]
struct EnemyMove {
  direction: Direction,
  score: i32,
}

pub struct Game {
  width: i32,
  height: i32,
  player: Snake,
  enemy: Snake,
  food: Option<Point>,
  score: u32,
  phase: Phase,
}

impl Game {
  pub fn new(width: i32, height: i32, rng: &mut impl Rng) -> Self {
    assert!(width > 0, "width must be positive");
    assert!(height > 0, "height must be positive");

    let mut game = Self {
      width,
      height,
      player: Snake::default(),
      enemy: Snake::default(),
      food: None,
      score: 0,
      phase: Phase::Ready,
    };

    game.reset(rng);
    game
  }

  pub fn reset(&mut self, rng: &mut impl Rng) {
    let (player, enemy) = self.starting_snakes();

    self.player = player;
    self.enemy = enemy;
    self.score = 0;
    self.phase = Phase::Ready;
    self.food = self.spawn_food(rng);
  }

  pub fn start(&mut self) {
    if self.phase == Phase::Ready {
      self.phase = Phase::Running;
    }
  }

  pub fn restart(&mut self, rng: &mut impl Rng) {
    self.reset(rng);
    self.phase = Phase::Running;
  }

  pub fn queue_turn(&mut self, direction: Direction) {
    if self.phase != Phase::Running || self.player.pending_direction.is_some() {
      return;
    }

    if direction == self.player.direction || direction.is_opposite(self.player.direction) {
      return;
    }

    self.player.pending_direction = Some(direction);
  }

  pub fn step(&mut self, rng: &mut impl Rng) {
    if self.phase != Phase::Running {
      return;
    }

    let player_direction = self.resolve_player_direction();
    let enemy_direction = self.choose_enemy_direction(player_direction, rng);
    let player_next_head = self.advance(self.player.head(), player_direction);
    let enemy_next_head = self.advance(self.enemy.head(), enemy_direction);
    let player_grows = self.food == Some(player_next_head);
    let enemy_grows = self.food == Some(enemy_next_head);

    if self.move_is_fatal(player_next_head, enemy_next_head, player_grows, enemy_grows) {
      self.phase = Phase::GameOver;
      return;
    }

    Self::apply_move_to_snake(
      &mut self.player,
      player_direction,
      player_next_head,
      player_grows,
    );
    Self::apply_move_to_snake(&mut self.enemy, enemy_direction, enemy_next_head, enemy_grows);

    if player_grows {
      self.score += 1;
    }

    if player_grows || enemy_grows {
      self.food = self.spawn_food(rng);
    }
  }

  pub fn width(&self) -> i32 {
    self.width
  }

  pub fn height(&self) -> i32 {
    self.height
  }

  pub fn score(&self) -> u32 {
    self.score
  }

  pub fn phase(&self) -> Phase {
    self.phase
  }

  pub fn tick_duration(&self) -> Duration {
    let reduction = SPEED_STEP_MS.saturating_mul(self.score as u64);
    Duration::from_millis(BASE_TICK_MS.saturating_sub(reduction).max(MIN_TICK_MS))
  }

  pub fn cell_at(&self, x: i32, y: i32) -> Cell {
    let point = Point { x, y };

    if self.player.head() == point {
      return Cell::Head;
    }

    if self.player.segments.iter().skip(1).any(|segment| *segment == point) {
      return Cell::Body;
    }

    if self.enemy.head() == point {
      return Cell::EnemyHead;
    }

    if self.enemy.segments.iter().skip(1).any(|segment| *segment == point) {
      return Cell::EnemyBody;
    }

    if self.food == Some(point) {
      return Cell::Food;
    }

    Cell::Empty
  }

  fn resolve_player_direction(&mut self) -> Direction {
    let mut direction = self.player.direction;

    if let Some(next_direction) = self.player.pending_direction.take() {
      if next_direction != self.player.direction
        && !next_direction.is_opposite(self.player.direction)
      {
        direction = next_direction;
      }
    }

    direction
  }

  fn choose_enemy_direction(
    &self,
    player_direction: Direction,
    rng: &mut impl Rng,
  ) -> Direction {
    let player_next_head = self.advance(self.player.head(), player_direction);
    let player_grows = self.food == Some(player_next_head);
    let mut safe_moves = Vec::with_capacity(4);

    for direction in self.enemy_move_candidates() {
      let enemy_next_head = self.advance(self.enemy.head(), direction);
      let enemy_grows = self.food == Some(enemy_next_head);

      if self.move_is_fatal(player_next_head, enemy_next_head, player_grows, enemy_grows) {
        continue;
      }

      safe_moves.push(EnemyMove {
        direction,
        score: self.score_enemy_move(
          player_next_head,
          player_grows,
          direction,
          enemy_next_head,
          enemy_grows,
        ),
      });
    }

    if safe_moves.is_empty() {
      return self.enemy.direction;
    }

    let best_score = safe_moves
      .iter()
      .map(|candidate| candidate.score)
      .max()
      .expect("safe_moves is not empty");
    let near_best: Vec<Direction> = safe_moves
      .iter()
      .filter(|candidate| candidate.score + ENEMY_NEAR_BEST_BAND >= best_score)
      .map(|candidate| candidate.direction)
      .collect();

    if near_best.len() == 1 {
      near_best[0]
    } else {
      let index = rng.gen_range(0..near_best.len());
      near_best[index]
    }
  }

  fn score_enemy_move(
    &self,
    player_next_head: Point,
    player_grows: bool,
    direction: Direction,
    enemy_next_head: Point,
    enemy_grows: bool,
  ) -> i32 {
    let mut score = 0;
    let food_delta = self.food_distance_delta(self.enemy.head(), enemy_next_head);

    if food_delta > 0 {
      score += food_delta * ENEMY_FOOD_DISTANCE_WEIGHT + ENEMY_FOOD_PROGRESS_BONUS;
    } else if food_delta < 0 {
      score += food_delta * ENEMY_FOOD_DISTANCE_WEIGHT - ENEMY_FOOD_DRIFT_PENALTY;
    }

    if direction == self.enemy.direction {
      score += ENEMY_STRAIGHT_BONUS;
    } else {
      score -= ENEMY_TURN_PENALTY;
    }

    if enemy_grows {
      score += ENEMY_FOOD_CAPTURE_BONUS;
    }

    let follow_up_options = self.enemy_follow_up_options(
      player_next_head,
      player_grows,
      enemy_next_head,
      direction,
      enemy_grows,
    );
    score += follow_up_options as i32 * ENEMY_FOLLOW_UP_WEIGHT;

    if follow_up_options == 0 {
      score -= ENEMY_DEAD_END_PENALTY;
    }

    score
  }

  fn enemy_move_candidates(&self) -> Vec<Direction> {
    Self::legal_directions(self.enemy.direction, self.enemy.len())
  }

  fn legal_directions(direction: Direction, len: usize) -> Vec<Direction> {
    let mut directions = Vec::with_capacity(4);

    directions.push(direction);
    directions.push(direction.left());
    directions.push(direction.right());

    if len == 1 {
      directions.push(direction.opposite());
    }

    directions
  }

  fn food_distance_delta(&self, from: Point, to: Point) -> i32 {
    match self.food {
      Some(food) => {
        self.wrapped_distance_score(from, food) - self.wrapped_distance_score(to, food)
      }
      None => 0,
    }
  }

  fn enemy_follow_up_options(
    &self,
    player_next_head: Point,
    player_grows: bool,
    enemy_next_head: Point,
    direction: Direction,
    enemy_grows: bool,
  ) -> usize {
    let player_after_move =
      Self::snake_segments_after_move(&self.player, player_next_head, player_grows);
    let enemy_after_move =
      Self::snake_segments_after_move(&self.enemy, enemy_next_head, enemy_grows);

    Self::legal_directions(direction, enemy_after_move.len())
      .into_iter()
      .filter(|follow_direction| {
        let follow_head = self.advance(enemy_next_head, *follow_direction);

        !Self::segments_collision(&player_after_move, follow_head, false)
          && !Self::segments_collision(&enemy_after_move, follow_head, false)
      })
      .count()
  }

  fn move_is_fatal(
    &self,
    player_next_head: Point,
    enemy_next_head: Point,
    player_grows: bool,
    enemy_grows: bool,
  ) -> bool {
    if player_next_head == enemy_next_head {
      return true;
    }

    Self::snake_collision(&self.player, player_next_head, player_grows)
      || Self::snake_collision(&self.enemy, enemy_next_head, enemy_grows)
      || Self::snake_collision(&self.enemy, player_next_head, enemy_grows)
      || Self::snake_collision(&self.player, enemy_next_head, player_grows)
  }

  fn snake_collision(snake: &Snake, next_head: Point, grows: bool) -> bool {
    Self::segments_collision(&snake.segments, next_head, grows)
  }

  fn segments_collision(segments: &VecDeque<Point>, next_head: Point, grows: bool) -> bool {
    let collision_len = if grows {
      segments.len()
    } else {
      segments.len().saturating_sub(1)
    };

    segments
      .iter()
      .take(collision_len)
      .any(|segment| *segment == next_head)
  }

  fn snake_segments_after_move(snake: &Snake, next_head: Point, grows: bool) -> VecDeque<Point> {
    let mut segments = snake.segments.clone();
    segments.push_front(next_head);

    if !grows {
      segments.pop_back();
    }

    segments
  }

  fn apply_move_to_snake(snake: &mut Snake, direction: Direction, next_head: Point, grows: bool) {
    snake.direction = direction;
    snake.pending_direction = None;
    snake.segments.push_front(next_head);

    if !grows {
      snake.segments.pop_back();
    }
  }

  fn advance(&self, point: Point, direction: Direction) -> Point {
    let next = match direction {
      Direction::Up => Point {
        x: point.x,
        y: point.y - 1,
      },
      Direction::Down => Point {
        x: point.x,
        y: point.y + 1,
      },
      Direction::Left => Point {
        x: point.x - 1,
        y: point.y,
      },
      Direction::Right => Point {
        x: point.x + 1,
        y: point.y,
      },
    };

    self.wrap_point(next)
  }

  fn wrap_point(&self, point: Point) -> Point {
    Point {
      x: point.x.rem_euclid(self.width),
      y: point.y.rem_euclid(self.height),
    }
  }

  fn spawn_food(&self, rng: &mut impl Rng) -> Option<Point> {
    let mut empty_cells = Vec::new();

    for y in 0..self.height {
      for x in 0..self.width {
        let point = Point { x, y };
        if !self.is_occupied(point) {
          empty_cells.push(point);
        }
      }
    }

    if empty_cells.is_empty() {
      None
    } else {
      let index = rng.gen_range(0..empty_cells.len());
      Some(empty_cells[index])
    }
  }

  fn is_occupied(&self, point: Point) -> bool {
    self
      .player
      .segments
      .iter()
      .chain(self.enemy.segments.iter())
      .any(|segment| *segment == point)
  }

  fn starting_snakes(&self) -> (Snake, Snake) {
    let player = self
      .find_snake_candidate(
        self.player_candidate_heads(),
        &[Direction::Right, Direction::Down, Direction::Left, Direction::Up],
        &[],
      )
      .expect("board must be able to place the player snake");

    let occupied: Vec<Point> = player.segments.iter().copied().collect();
    let enemy = self
      .find_snake_candidate(
        self.enemy_candidate_heads(player.head()),
        &[Direction::Left, Direction::Up, Direction::Down, Direction::Right],
        &occupied,
      )
      .expect("board must be able to place the enemy snake");

    (player, enemy)
  }

  fn find_snake_candidate(
    &self,
    heads: Vec<Point>,
    directions: &[Direction],
    occupied: &[Point],
  ) -> Option<Snake> {
    for head in heads {
      for direction in directions.iter().copied() {
        let segments = match self.build_snake_segments(head, direction, START_LENGTH) {
          Some(segments) => segments,
          None => continue,
        };

        if segments.iter().any(|segment| occupied.contains(segment)) {
          continue;
        }

        return Some(Snake::new(segments, direction));
      }
    }

    None
  }

  fn build_snake_segments(
    &self,
    head: Point,
    direction: Direction,
    len: usize,
  ) -> Option<VecDeque<Point>> {
    let mut segments = VecDeque::with_capacity(len);
    let mut current = head;

    for index in 0..len {
      if segments.iter().any(|segment| *segment == current) {
        return None;
      }

      segments.push_back(current);

      if index + 1 < len {
        current = self.advance(current, direction.opposite());
      }
    }

    Some(segments)
  }

  fn player_candidate_heads(&self) -> Vec<Point> {
    let center = Point {
      x: self.width / 2,
      y: self.height / 2,
    };
    let mut points = self.all_points();

    points.sort_by_key(|point| (self.center_distance_sq(*point, center), point.y, point.x));
    points
  }

  fn enemy_candidate_heads(&self, player_head: Point) -> Vec<Point> {
    let mut points = self.all_points();

    points.sort_by_key(|point| {
      (
        Reverse(self.wrapped_distance_score(*point, player_head)),
        point.y,
        point.x,
      )
    });
    points
  }

  fn all_points(&self) -> Vec<Point> {
    let mut points = Vec::with_capacity((self.width * self.height) as usize);

    for y in 0..self.height {
      for x in 0..self.width {
        points.push(Point { x, y });
      }
    }

    points
  }

  fn center_distance_sq(&self, point: Point, center: Point) -> i32 {
    let dx = point.x - center.x;
    let dy = point.y - center.y;

    dx * dx + dy * dy
  }

  fn wrapped_distance_score(&self, a: Point, b: Point) -> i32 {
    self.wrapped_axis_distance(a.x, b.x, self.width)
      + self.wrapped_axis_distance(a.y, b.y, self.height)
  }

  fn wrapped_axis_distance(&self, a: i32, b: i32, size: i32) -> i32 {
    let delta = (a - b).abs();

    delta.min(size - delta)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use rand::rngs::StdRng;
  use rand::SeedableRng;

  fn seeded_rng() -> StdRng {
    StdRng::seed_from_u64(7)
  }

  fn point(x: i32, y: i32) -> Point {
    Point { x, y }
  }

  fn snake<const N: usize>(direction: Direction, segments: [Point; N]) -> Snake {
    Snake {
      segments: VecDeque::from(segments),
      direction,
      pending_direction: None,
    }
  }

  fn enemy_choice(game: &Game, player_direction: Direction, seed: u64) -> Direction {
    let mut rng = StdRng::seed_from_u64(seed);
    game.choose_enemy_direction(player_direction, &mut rng)
  }

  fn tie_break_game() -> Game {
    let mut rng = seeded_rng();
    let mut game = Game::new(7, 7, &mut rng);

    game.phase = Phase::Running;
    game.player = snake(
      Direction::Right,
      [point(6, 6), point(5, 6), point(4, 6), point(3, 2)],
    );
    game.enemy = snake(Direction::Up, [point(3, 3), point(3, 4), point(2, 4)]);
    game.food = Some(point(3, 5));
    game
  }

  fn snakes_occupy(game: &Game, point: Point) -> bool {
    game
      .player
      .segments
      .iter()
      .chain(game.enemy.segments.iter())
      .any(|segment| *segment == point)
  }

  #[test]
  fn reset_restores_centered_ready_state_with_enemy_and_food() {
    let mut rng = seeded_rng();
    let mut game = Game::new(10, 8, &mut rng);

    game.phase = Phase::GameOver;
    game.score = 4;
    game.player.pending_direction = Some(Direction::Up);
    game.enemy.pending_direction = Some(Direction::Down);
    game.reset(&mut rng);

    assert_eq!(game.phase, Phase::Ready);
    assert_eq!(game.score, 0);
    assert_eq!(game.player.direction, Direction::Right);
    assert_eq!(game.player.pending_direction, None);
    assert_eq!(
      game.player.segments,
      VecDeque::from([point(5, 4), point(4, 4), point(3, 4)])
    );
    assert_eq!(game.enemy.direction, Direction::Left);
    assert_eq!(game.enemy.pending_direction, None);
    assert_eq!(
      game.enemy.segments,
      VecDeque::from([point(0, 0), point(1, 0), point(2, 0)])
    );
    assert!(game.food.is_some());
    assert!(!snakes_occupy(&game, game.food.unwrap()));
  }

  #[test]
  fn cell_at_distinguishes_player_enemy_food_and_empty_cells() {
    let mut rng = seeded_rng();
    let mut game = Game::new(5, 5, &mut rng);

    game.player = snake(Direction::Right, [point(1, 1), point(0, 1), point(0, 0)]);
    game.enemy = snake(Direction::Down, [point(3, 2), point(3, 1), point(2, 1)]);
    game.food = Some(point(4, 4));

    assert_eq!(game.cell_at(1, 1), Cell::Head);
    assert_eq!(game.cell_at(0, 1), Cell::Body);
    assert_eq!(game.cell_at(3, 2), Cell::EnemyHead);
    assert_eq!(game.cell_at(3, 1), Cell::EnemyBody);
    assert_eq!(game.cell_at(4, 4), Cell::Food);
    assert_eq!(game.cell_at(2, 2), Cell::Empty);
  }

  #[test]
  fn wrap_around_moves_to_opposite_edge() {
    let mut rng = seeded_rng();
    let mut game = Game::new(6, 5, &mut rng);

    game.phase = Phase::Running;
    game.player = snake(Direction::Left, [point(0, 2), point(1, 2), point(2, 2)]);
    game.enemy = snake(Direction::Right, [point(3, 4), point(2, 4), point(1, 4)]);
    game.food = Some(point(5, 4));

    game.step(&mut rng);

    assert_eq!(game.player.head(), point(5, 2));
    assert_eq!(game.phase, Phase::Running);
  }

  #[test]
  fn immediate_reverse_turn_is_ignored() {
    let mut rng = seeded_rng();
    let mut game = Game::new(10, 8, &mut rng);

    game.start();
    game.enemy = snake(Direction::Left, [point(0, 0), point(1, 0), point(2, 0)]);
    game.food = Some(point(9, 7));
    let previous_head = game.player.head();
    game.queue_turn(Direction::Left);
    game.step(&mut rng);

    assert_eq!(game.player.direction, Direction::Right);
    assert_eq!(game.player.head(), point(previous_head.x + 1, previous_head.y));
  }

  #[test]
  fn eating_food_increases_score_and_length() {
    let mut rng = seeded_rng();
    let mut game = Game::new(10, 8, &mut rng);

    game.phase = Phase::Running;
    game.player = snake(Direction::Right, [point(5, 4), point(4, 4), point(3, 4)]);
    game.enemy = snake(Direction::Left, [point(0, 0), point(1, 0), point(2, 0)]);
    game.food = Some(point(6, 4));
    let previous_len = game.player.len();

    game.step(&mut rng);

    assert_eq!(game.score, 1);
    assert_eq!(game.player.len(), previous_len + 1);
    assert_ne!(game.food, Some(point(6, 4)));
    assert!(game.food.map(|food| !snakes_occupy(&game, food)).unwrap_or(true));
  }

  #[test]
  fn enemy_eating_food_grows_without_scoring_and_wraps() {
    let mut rng = seeded_rng();
    let mut game = Game::new(6, 5, &mut rng);

    game.phase = Phase::Running;
    game.player = snake(Direction::Right, [point(2, 2), point(1, 2), point(0, 2)]);
    game.enemy = snake(Direction::Left, [point(0, 0), point(1, 0), point(2, 0)]);
    game.food = Some(point(5, 0));
    let previous_len = game.enemy.len();

    game.step(&mut rng);

    assert_eq!(game.score, 0);
    assert_eq!(game.enemy.head(), point(5, 0));
    assert_eq!(game.enemy.len(), previous_len + 1);
    assert_ne!(game.food, Some(point(5, 0)));
    assert!(game.food.map(|food| !snakes_occupy(&game, food)).unwrap_or(true));
    assert_eq!(game.phase, Phase::Running);
  }

  #[test]
  fn enemy_can_keep_moving_straight_when_a_food_turn_leads_into_a_dead_end() {
    let mut rng = seeded_rng();
    let mut game = Game::new(7, 7, &mut rng);

    game.phase = Phase::Running;
    game.player = snake(
      Direction::Right,
      [
        point(6, 6),
        point(5, 6),
        point(4, 6),
        point(4, 2),
        point(3, 1),
        point(2, 2),
      ],
    );
    game.enemy = snake(Direction::Right, [point(3, 3), point(2, 3), point(1, 3)]);
    game.food = Some(point(3, 0));

    assert_eq!(enemy_choice(&game, Direction::Right, 7), Direction::Right);
  }

  #[test]
  fn enemy_can_keep_moving_straight_when_straight_also_closes_food_distance() {
    let mut rng = seeded_rng();
    let mut game = Game::new(7, 7, &mut rng);

    game.phase = Phase::Running;
    game.player = snake(Direction::Right, [point(6, 6), point(5, 6), point(4, 6)]);
    game.enemy = snake(Direction::Right, [point(3, 3), point(2, 3), point(1, 3)]);
    game.food = Some(point(5, 2));

    assert_eq!(enemy_choice(&game, Direction::Right, 7), Direction::Right);
  }

  #[test]
  fn enemy_prefers_open_move_that_closes_food_distance() {
    let mut rng = seeded_rng();
    let mut game = Game::new(7, 7, &mut rng);

    game.phase = Phase::Running;
    game.player = snake(Direction::Right, [point(6, 6), point(5, 6), point(4, 6)]);
    game.enemy = snake(Direction::Up, [point(3, 3), point(3, 4), point(3, 5)]);
    game.food = Some(point(1, 3));

    assert_eq!(enemy_choice(&game, Direction::Right, 7), Direction::Left);
  }

  #[test]
  fn enemy_prefers_food_but_falls_back_to_a_safe_direction() {
    let mut rng = seeded_rng();
    let mut game = Game::new(7, 7, &mut rng);

    game.phase = Phase::Running;
    game.player = snake(Direction::Right, [point(0, 0), point(6, 0), point(5, 0)]);
    game.enemy = snake(Direction::Right, [point(3, 3), point(3, 2), point(2, 2)]);
    game.food = Some(point(3, 1));

    game.step(&mut rng);

    assert_eq!(game.enemy.head(), point(4, 3));
    assert_eq!(game.phase, Phase::Running);
  }

  #[test]
  fn seeded_rng_makes_enemy_tie_breaking_reproducible() {
    let game = tie_break_game();
    let first = enemy_choice(&game, Direction::Right, 11);
    let second = enemy_choice(&game, Direction::Right, 11);
    let mut saw_left = false;
    let mut saw_right = false;

    for seed in 0..32 {
      match enemy_choice(&game, Direction::Right, seed) {
        Direction::Left => saw_left = true,
        Direction::Right => saw_right = true,
        direction => panic!("unexpected direction {:?}", direction),
      }
    }

    assert_eq!(first, second);
    assert!(matches!(first, Direction::Left | Direction::Right));
    assert!(saw_left);
    assert!(saw_right);
  }

  #[test]
  fn speed_has_a_minimum_floor() {
    let mut rng = seeded_rng();
    let mut game = Game::new(10, 8, &mut rng);

    game.score = 100;

    assert_eq!(game.tick_duration(), Duration::from_millis(70));
  }

  #[test]
  fn self_collision_ends_the_run() {
    let mut rng = seeded_rng();
    let mut game = Game::new(6, 6, &mut rng);

    game.phase = Phase::Running;
    game.player = snake(
      Direction::Up,
      [
        point(2, 2),
        point(2, 1),
        point(1, 1),
        point(1, 2),
        point(1, 3),
        point(2, 3),
        point(3, 3),
        point(3, 2),
        point(3, 1),
      ],
    );
    game.enemy = snake(Direction::Left, [point(5, 5), point(0, 5), point(1, 5)]);
    game.food = Some(point(4, 4));

    game.step(&mut rng);

    assert_eq!(game.phase, Phase::GameOver);
  }

  #[test]
  fn head_to_head_contact_ends_the_run() {
    let mut rng = seeded_rng();
    let mut game = Game::new(7, 5, &mut rng);

    game.phase = Phase::Running;
    game.player = snake(Direction::Right, [point(1, 2), point(0, 2), point(6, 2)]);
    game.enemy = snake(Direction::Left, [point(3, 2), point(4, 2), point(5, 2)]);
    game.food = Some(point(0, 2));

    game.step(&mut rng);

    assert_eq!(game.phase, Phase::GameOver);
  }

  #[test]
  fn player_moving_into_enemy_body_ends_the_run() {
    let mut rng = seeded_rng();
    let mut game = Game::new(7, 7, &mut rng);

    game.phase = Phase::Running;
    game.player = snake(Direction::Right, [point(1, 3), point(0, 3), point(6, 3)]);
    game.enemy = snake(Direction::Up, [point(2, 2), point(2, 3), point(2, 4)]);
    game.food = Some(point(5, 5));

    game.step(&mut rng);

    assert_eq!(game.phase, Phase::GameOver);
  }

  #[test]
  fn enemy_self_collision_ends_the_run_when_no_safe_fallback_exists() {
    let mut rng = seeded_rng();
    let mut game = Game::new(6, 6, &mut rng);

    game.phase = Phase::Running;
    game.player = snake(Direction::Right, [point(0, 5), point(5, 5), point(4, 5)]);
    game.enemy = snake(
      Direction::Up,
      [
        point(2, 2),
        point(2, 1),
        point(1, 1),
        point(1, 2),
        point(1, 3),
        point(2, 3),
        point(3, 3),
        point(3, 2),
        point(3, 1),
      ],
    );
    game.food = Some(point(2, 0));

    game.step(&mut rng);

    assert_eq!(game.phase, Phase::GameOver);
  }

  #[test]
  fn food_spawns_only_on_empty_cells_of_both_snakes() {
    let mut rng = seeded_rng();
    let mut game = Game::new(3, 3, &mut rng);

    game.player = snake(
      Direction::Right,
      [point(0, 0), point(1, 0), point(2, 0), point(0, 1), point(1, 1)],
    );
    game.enemy = snake(Direction::Left, [point(2, 1), point(0, 2), point(1, 2)]);

    let food = game.spawn_food(&mut rng);

    assert_eq!(food, Some(point(2, 2)));
  }

  #[test]
  fn moving_into_the_old_tail_cell_is_allowed() {
    let mut rng = seeded_rng();
    let mut game = Game::new(5, 5, &mut rng);

    game.phase = Phase::Running;
    game.player = snake(
      Direction::Up,
      [point(2, 2), point(2, 1), point(1, 1), point(1, 2)],
    );
    game.enemy = snake(Direction::Left, [point(4, 4), point(3, 4), point(2, 4)]);
    game.food = Some(point(0, 0));

    game.queue_turn(Direction::Left);
    game.step(&mut rng);

    assert_eq!(game.phase, Phase::Running);
    assert_eq!(game.player.head(), point(1, 2));
  }
}
