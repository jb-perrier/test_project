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
