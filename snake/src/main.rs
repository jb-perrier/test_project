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

    if let Err(error) = execute!(stdout, EnterAlternateScreen, cursor::Hide) {
      let _ = execute!(stdout, cursor::Show, LeaveAlternateScreen);
      let _ = terminal::disable_raw_mode();
      return Err(error);
    }

    Ok(Self)
  }
}

impl Drop for TerminalGuard {
  fn drop(&mut self) {
    let mut stdout = stdout();
    let _ = execute!(stdout, cursor::Show, LeaveAlternateScreen);
    let _ = terminal::disable_raw_mode();
  }
}
