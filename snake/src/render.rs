use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

use crate::game::{Cell, Game, Phase};

const HEADER_HEIGHT: u16 = 3;
const SIDEBAR_MIN_WIDTH: u16 = 32;

pub fn draw(frame: &mut Frame<'_>, game: &Game) {
  let area = frame.size();

  if area.width < min_width(game) || area.height < min_height(game) {
    draw_resize_warning(frame, game, area);
    return;
  }

  let sections = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
      Constraint::Length(HEADER_HEIGHT),
      Constraint::Length(board_height(game)),
      Constraint::Min(0),
    ])
    .split(area);

  draw_title(frame, sections[0]);
  draw_content(frame, sections[1], game);
}

fn draw_title(frame: &mut Frame<'_>, area: Rect) {
  let title = Paragraph::new("Snake")
    .alignment(Alignment::Center)
    .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
    .block(Block::default().borders(Borders::ALL).title("Game"));

  frame.render_widget(title, area);
}

fn draw_content(frame: &mut Frame<'_>, area: Rect, game: &Game) {
  let sections = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([
      Constraint::Length(board_width(game)),
      Constraint::Min(SIDEBAR_MIN_WIDTH),
    ])
    .split(area);

  draw_board(frame, sections[0], game);
  draw_sidebar(frame, sections[1], game);
}

fn draw_board(frame: &mut Frame<'_>, area: Rect, game: &Game) {
  let board = Paragraph::new(board_lines(game)).block(
    Block::default()
      .borders(Borders::ALL)
      .title("Board"),
  );

  frame.render_widget(board, area);
}

fn draw_sidebar(frame: &mut Frame<'_>, area: Rect, game: &Game) {
  let sections = Layout::default()
    .direction(Direction::Vertical)
    .constraints([Constraint::Length(4), Constraint::Length(4), Constraint::Min(0)])
    .split(area);

  draw_stats(frame, sections[0], game);
  draw_status(frame, sections[1], game);
  draw_controls(frame, sections[2]);
}

fn draw_stats(frame: &mut Frame<'_>, area: Rect, game: &Game) {
  let stats = Paragraph::new(vec![
    Line::from(vec![
      Span::styled("Score: ", label_style()),
      Span::raw(game.score().to_string()),
    ]),
    Line::from(vec![
      Span::styled("Speed: ", label_style()),
      Span::raw(format!("{} ms/tick", game.tick_duration().as_millis())),
    ]),
  ])
  .block(Block::default().borders(Borders::ALL).title("Stats"));

  frame.render_widget(stats, area);
}

fn draw_status(frame: &mut Frame<'_>, area: Rect, game: &Game) {
  let status = Paragraph::new(vec![
    Line::from(vec![
      Span::styled("Phase: ", label_style()),
      Span::styled(phase_label(game.phase()), phase_style(game.phase())),
    ]),
    Line::from(phase_message(game.phase())),
  ])
  .block(Block::default().borders(Borders::ALL).title("Status"));

  frame.render_widget(status, area);
}

fn draw_controls(frame: &mut Frame<'_>, area: Rect) {
  let controls = Paragraph::new(vec![
    Line::from("Arrows or WASD: move"),
    Line::from("Enter/Space: start"),
    Line::from("R, Enter/Space: restart"),
    Line::from("Q or Esc: quit"),
  ])
  .block(Block::default().borders(Borders::ALL).title("Controls"));

  frame.render_widget(controls, area);
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

fn board_lines(game: &Game) -> Vec<Line<'static>> {
  let mut lines = Vec::with_capacity(game.height() as usize);

  for y in 0..game.height() {
    let mut spans = Vec::with_capacity(game.width() as usize);

    for x in 0..game.width() {
      let (symbol, style) = match game.cell_at(x, y) {
        Cell::Empty => (" ", Style::default()),
        Cell::Head => (
          "@",
          Style::default()
            .fg(Color::Green)
            .add_modifier(Modifier::BOLD),
        ),
        Cell::Body => ("o", Style::default().fg(Color::Green)),
        Cell::Food => (
          "*",
          Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        ),
      };

      spans.push(Span::styled(symbol, style));
    }

    lines.push(Line::from(spans));
  }

  lines
}

fn phase_label(phase: Phase) -> &'static str {
  match phase {
    Phase::Ready => "Ready",
    Phase::Running => "Running",
    Phase::GameOver => "Game over",
  }
}

fn phase_message(phase: Phase) -> &'static str {
  match phase {
    Phase::Ready => "Enter or Space starts.",
    Phase::Running => "Eat food and stay alive.",
    Phase::GameOver => "R, Enter, or Space restarts.",
  }
}

fn phase_style(phase: Phase) -> Style {
  match phase {
    Phase::Ready => Style::default()
      .fg(Color::Yellow)
      .add_modifier(Modifier::BOLD),
    Phase::Running => Style::default().fg(Color::Green),
    Phase::GameOver => Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
  }
}

fn label_style() -> Style {
  Style::default()
    .fg(Color::Cyan)
    .add_modifier(Modifier::BOLD)
}

fn board_width(game: &Game) -> u16 {
  game.width() as u16 + 2
}

fn board_height(game: &Game) -> u16 {
  game.height() as u16 + 2
}

fn min_width(game: &Game) -> u16 {
  board_width(game) + SIDEBAR_MIN_WIDTH
}

fn min_height(game: &Game) -> u16 {
  HEADER_HEIGHT + board_height(game)
}
