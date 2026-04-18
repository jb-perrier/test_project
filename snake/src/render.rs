use crate::game::{Cell, Game, Phase};

pub fn frame(game: &Game) -> String {
  let mut lines = Vec::new();
  let border = format!("+{}+", "-".repeat(game.width() as usize));

  lines.push("Snake".to_string());
  lines.push(format!(
    "Score: {}  Speed: {} ms/tick",
    game.score(),
    game.tick_duration().as_millis()
  ));
  lines.push("Controls: Arrows/WASD move | Enter/Space start | R restart | Q/Esc quit".to_string());
  lines.push(String::new());
  lines.push(border.clone());

  for y in 0..game.height() {
    let mut row = String::with_capacity(game.width() as usize + 2);
    row.push('|');

    for x in 0..game.width() {
      let symbol = match game.cell_at(x, y) {
        Cell::Empty => ' ',
        Cell::Head => '@',
        Cell::Body => 'o',
        Cell::Food => '*',
      };
      row.push(symbol);
    }

    row.push('|');
    lines.push(row);
  }

  lines.push(border);
  lines.push(String::new());

  match game.phase() {
    Phase::Ready => lines.push("Press Enter or Space to start.".to_string()),
    Phase::Running => lines.push("Stay alive and keep eating food.".to_string()),
    Phase::GameOver => lines.push("Game over. Press R, Enter, or Space to restart.".to_string()),
  }

  lines.join("\n")
}
