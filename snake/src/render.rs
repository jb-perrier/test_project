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
    Span::raw(game.score().to_string()),
    Span::raw("  "),
    Span::styled("Speed: ", label_style()),
    Span::raw(format!("{}ms", game.tick_duration().as_millis())),
    Span::raw("  "),
    Span::styled("Phase: ", label_style()),
    Span::styled(phase_label(game.phase()), phase_style(game.phase())),
  ])
}

fn board_lines(game: &Game) -> Vec<Line<'static>> {
  let mut lines = Vec::with_capacity(game.height() as usize);

  for y in 0..game.height() {
    let mut spans = Vec::with_capacity(game.width() as usize);

    for x in 0..game.width() {
      let style = match game.cell_at(x, y) {
        Cell::Empty => Style::default().bg(Color::DarkGray),
        Cell::Head => Style::default()
          .bg(Color::LightGreen)
          .add_modifier(Modifier::BOLD),
        Cell::Body => Style::default().bg(Color::Green),
        Cell::Food => Style::default().bg(Color::Red).add_modifier(Modifier::BOLD),
      };

      spans.push(Span::styled("  ", style));
    }

    lines.push(Line::from(spans));
  }

  lines
}

fn footer_text(phase: Phase) -> &'static str {
  match phase {
    Phase::Ready => "Enter/Space start | Arrows/WASD move | Q/Esc quit",
    Phase::Running => "Eat food | Arrows/WASD move | Q/Esc quit",
    Phase::GameOver => "R/Enter/Space restart | Q/Esc quit",
  }
}

fn phase_label(phase: Phase) -> &'static str {
  match phase {
    Phase::Ready => "Ready",
    Phase::Running => "Running",
    Phase::GameOver => "Game over",
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

fn title_style() -> Style {
  Style::default()
    .fg(Color::Cyan)
    .add_modifier(Modifier::BOLD)
}

fn label_style() -> Style {
  Style::default()
    .fg(Color::Cyan)
    .add_modifier(Modifier::BOLD)
}

fn board_width(game: &Game) -> u16 {
  game.width() as u16 * CELL_WIDTH + 2
}

fn board_height(game: &Game) -> u16 {
  game.height() as u16 + 2
}

fn min_width(game: &Game) -> u16 {
  board_width(game).max(INFO_MIN_WIDTH)
}

fn min_height(game: &Game) -> u16 {
  HUD_HEIGHT + board_height(game) + FOOTER_HEIGHT
}
