use std::collections::VecDeque;
use std::time::Duration;

use rand::Rng;

const BASE_TICK_MS: u64 = 180;
const SPEED_STEP_MS: u64 = 8;
const MIN_TICK_MS: u64 = 70;

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
  fn is_opposite(self, other: Direction) -> bool {
    matches!(
      (self, other),
      (Direction::Up, Direction::Down)
        | (Direction::Down, Direction::Up)
        | (Direction::Left, Direction::Right)
        | (Direction::Right, Direction::Left)
    )
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
  Food,
}

pub struct Game {
  width: i32,
  height: i32,
  snake: VecDeque<Point>,
  direction: Direction,
  pending_direction: Option<Direction>,
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
      snake: VecDeque::new(),
      direction: Direction::Right,
      pending_direction: None,
      food: None,
      score: 0,
      phase: Phase::Ready,
    };

    game.reset(rng);
    game
  }

  pub fn reset(&mut self, rng: &mut impl Rng) {
    self.snake.clear();

    let center = Point {
      x: self.width / 2,
      y: self.height / 2,
    };

    self.snake.push_back(center);
    self.snake.push_back(self.wrap_point(Point {
      x: center.x - 1,
      y: center.y,
    }));
    self.snake.push_back(self.wrap_point(Point {
      x: center.x - 2,
      y: center.y,
    }));

    self.direction = Direction::Right;
    self.pending_direction = None;
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
    if self.phase != Phase::Running || self.pending_direction.is_some() {
      return;
    }

    if direction.is_opposite(self.direction) || direction == self.direction {
      return;
    }

    self.pending_direction = Some(direction);
  }

  pub fn step(&mut self, rng: &mut impl Rng) {
    if self.phase != Phase::Running {
      return;
    }

    if let Some(next_direction) = self.pending_direction.take() {
      if !next_direction.is_opposite(self.direction) {
        self.direction = next_direction;
      }
    }

    let next_head = self.advance(self.head(), self.direction);
    let growing = self.food == Some(next_head);
    let collision_len = if growing {
      self.snake.len()
    } else {
      self.snake.len().saturating_sub(1)
    };

    if self
      .snake
      .iter()
      .take(collision_len)
      .any(|segment| *segment == next_head)
    {
      self.phase = Phase::GameOver;
      return;
    }

    self.snake.push_front(next_head);

    if growing {
      self.score += 1;
      self.food = self.spawn_food(rng);
    } else {
      self.snake.pop_back();
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

    if self.snake.front().copied() == Some(point) {
      return Cell::Head;
    }

    if self.snake.iter().skip(1).any(|segment| *segment == point) {
      return Cell::Body;
    }

    if self.food == Some(point) {
      return Cell::Food;
    }

    Cell::Empty
  }

  fn head(&self) -> Point {
    *self.snake.front().expect("snake always has a head")
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
        if !self.snake.iter().any(|segment| *segment == point) {
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
}

#[cfg(test)]
mod tests {
  use super::*;
  use rand::rngs::StdRng;
  use rand::SeedableRng;

  fn seeded_rng() -> StdRng {
    StdRng::seed_from_u64(7)
  }

  #[test]
  fn reset_restores_centered_ready_state() {
    let mut rng = seeded_rng();
    let mut game = Game::new(10, 8, &mut rng);

    game.phase = Phase::GameOver;
    game.score = 4;
    game.pending_direction = Some(Direction::Up);
    game.reset(&mut rng);

    assert_eq!(game.phase, Phase::Ready);
    assert_eq!(game.score, 0);
    assert_eq!(game.direction, Direction::Right);
    assert_eq!(game.pending_direction, None);
    assert_eq!(game.snake[0], Point { x: 5, y: 4 });
    assert_eq!(game.snake[1], Point { x: 4, y: 4 });
    assert_eq!(game.snake[2], Point { x: 3, y: 4 });
    assert!(game.food.is_some());
    assert!(!game.snake.iter().any(|segment| Some(*segment) == game.food));
  }

  #[test]
  fn wrap_around_moves_to_opposite_edge() {
    let mut rng = seeded_rng();
    let mut game = Game::new(6, 5, &mut rng);

    game.phase = Phase::Running;
    game.direction = Direction::Left;
    game.pending_direction = None;
    game.snake = VecDeque::from([
      Point { x: 0, y: 2 },
      Point { x: 1, y: 2 },
      Point { x: 2, y: 2 },
    ]);
    game.food = Some(Point { x: 4, y: 4 });

    game.step(&mut rng);

    assert_eq!(game.snake.front().copied(), Some(Point { x: 5, y: 2 }));
    assert_eq!(game.phase, Phase::Running);
  }

  #[test]
  fn immediate_reverse_turn_is_ignored() {
    let mut rng = seeded_rng();
    let mut game = Game::new(10, 8, &mut rng);

    game.start();
    let previous_head = game.head();
    game.queue_turn(Direction::Left);
    game.step(&mut rng);

    assert_eq!(game.direction, Direction::Right);
    assert_eq!(game.head(), Point {
      x: previous_head.x + 1,
      y: previous_head.y,
    });
  }

  #[test]
  fn eating_food_increases_score_and_length() {
    let mut rng = seeded_rng();
    let mut game = Game::new(10, 8, &mut rng);

    game.phase = Phase::Running;
    game.food = Some(Point { x: 6, y: 4 });
    let previous_len = game.snake.len();

    game.step(&mut rng);

    assert_eq!(game.score, 1);
    assert_eq!(game.snake.len(), previous_len + 1);
    assert_ne!(game.food, Some(Point { x: 6, y: 4 }));
    assert!(!game.snake.iter().any(|segment| Some(*segment) == game.food));
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
    game.direction = Direction::Up;
    game.snake = VecDeque::from([
      Point { x: 2, y: 2 },
      Point { x: 2, y: 1 },
      Point { x: 1, y: 1 },
      Point { x: 1, y: 2 },
      Point { x: 1, y: 3 },
      Point { x: 2, y: 3 },
      Point { x: 3, y: 3 },
      Point { x: 3, y: 2 },
      Point { x: 3, y: 1 },
    ]);
    game.food = Some(Point { x: 5, y: 5 });

    game.step(&mut rng);

    assert_eq!(game.phase, Phase::GameOver);
  }

  #[test]
  fn food_spawns_only_on_empty_cells() {
    let mut rng = seeded_rng();
    let mut game = Game::new(3, 3, &mut rng);

    game.snake = VecDeque::from([
      Point { x: 0, y: 0 },
      Point { x: 1, y: 0 },
      Point { x: 2, y: 0 },
      Point { x: 0, y: 1 },
      Point { x: 1, y: 1 },
      Point { x: 2, y: 1 },
      Point { x: 0, y: 2 },
      Point { x: 1, y: 2 },
    ]);

    let food = game.spawn_food(&mut rng);

    assert_eq!(food, Some(Point { x: 2, y: 2 }));
  }

  #[test]
  fn moving_into_the_old_tail_cell_is_allowed() {
    let mut rng = seeded_rng();
    let mut game = Game::new(5, 5, &mut rng);

    game.phase = Phase::Running;
    game.direction = Direction::Up;
    game.snake = VecDeque::from([
      Point { x: 2, y: 2 },
      Point { x: 2, y: 1 },
      Point { x: 1, y: 1 },
      Point { x: 1, y: 2 },
    ]);
    game.food = Some(Point { x: 4, y: 4 });

    game.queue_turn(Direction::Left);
    game.step(&mut rng);

    assert_eq!(game.phase, Phase::Running);
    assert_eq!(game.head(), Point { x: 1, y: 2 });
  }
}
