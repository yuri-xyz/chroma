use std::fmt::Write as _;

use crossterm::style::Color;
use unicode_width::UnicodeWidthChar;

use crate::{constants::MIN_BRIGHTNESS_THRESHOLD, utils::color::calculate_brightness};

mod stream;

pub use stream::StreamFormat;

pub type RgbColor = (u8, u8, u8);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RenderedCell {
  pub character: char,
  pub foreground: Option<RgbColor>,
  pub background: Option<RgbColor>,
  pub display_width: usize,
}

impl RenderedCell {
  pub fn new(character: char, foreground: Option<RgbColor>, background: Option<RgbColor>) -> Self {
    Self {
      character,
      foreground,
      background,
      display_width: character.width().unwrap_or(1),
    }
  }

  pub fn blank() -> Self {
    Self {
      character: ' ',
      foreground: None,
      background: None,
      display_width: 1,
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderedFrame {
  width: usize,
  height: usize,
  rows: Vec<Vec<RenderedCell>>,
  status_bar: Option<Vec<RenderedCell>>,
  terminal_background: Option<RgbColor>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct StyleState {
  pub(super) foreground: Option<RgbColor>,
  pub(super) background: Option<RgbColor>,
}

impl RenderedFrame {
  pub fn from_ascii_frame(
    ascii_frame: &[Vec<(char, Color)>],
    width: usize,
    height: usize,
    status_bar: Option<Vec<RenderedCell>>,
    terminal_background: Option<RgbColor>,
  ) -> Self {
    let mut rows = ascii_frame
      .iter()
      .take(height)
      .map(|row| build_content_row(row, width))
      .collect::<Vec<_>>();

    while rows.len() < height {
      rows.push(blank_row(width));
    }

    Self {
      width,
      height,
      rows,
      status_bar,
      terminal_background,
    }
  }

  pub fn width(&self) -> usize {
    self.width
  }

  pub fn height(&self) -> usize {
    self.height
  }

  pub fn rows(&self) -> &[Vec<RenderedCell>] {
    &self.rows
  }

  pub fn cell(&self, x: usize, y: usize) -> Option<&RenderedCell> {
    if x >= self.width || y >= self.height {
      return None;
    }

    cell_at_display_column(self.rows.get(y)?, x)
  }

  pub fn status_cell(&self, x: usize) -> Option<&RenderedCell> {
    cell_at_display_column(self.status_bar.as_deref()?, x)
  }

  pub fn to_terminal_string(&self) -> String {
    let mut buffer =
      String::with_capacity(self.height * self.width * 25 + self.status_bar_width() * 25);
    let mut style_state = StyleState {
      foreground: None,
      background: self.terminal_background,
    };

    buffer.push_str("\x1b[?25l\x1b[H\x1b[0m");

    if let Some((r, g, b)) = self.terminal_background {
      let _ = write!(buffer, "\x1b[48;2;{};{};{}m", r, g, b);
    } else {
      buffer.push_str("\x1b[49m");
    }

    for (row_idx, row) in self.rows.iter().enumerate() {
      push_cells(
        &mut buffer,
        row,
        &mut style_state,
        StyleState {
          foreground: None,
          background: self.terminal_background,
        },
      );

      if row_idx < self.rows.len() - 1 || self.status_bar.is_some() {
        reset_to_base_style(
          &mut buffer,
          &mut style_state,
          StyleState {
            foreground: None,
            background: self.terminal_background,
          },
        );

        if self.terminal_background.is_some() {
          buffer.push_str("\r\n");
        } else {
          buffer.push_str("\x1b[0m\r\n");
          style_state = StyleState {
            foreground: None,
            background: None,
          };
        }
      }
    }

    if let Some(status_bar) = &self.status_bar {
      buffer.push_str("\x1b[0m\x1b[49m");
      style_state = StyleState {
        foreground: None,
        background: None,
      };
      push_cells(
        &mut buffer,
        status_bar,
        &mut style_state,
        StyleState {
          foreground: None,
          background: None,
        },
      );
      reset_to_base_style(
        &mut buffer,
        &mut style_state,
        StyleState {
          foreground: None,
          background: None,
        },
      );
    }

    buffer
  }

  pub fn to_stream_string(&self, format: StreamFormat, frame_index: u64) -> String {
    stream::to_framed_stream_string(self, format, frame_index)
  }

  fn status_bar_width(&self) -> usize {
    self
      .status_bar
      .as_deref()
      .map(display_width)
      .unwrap_or_default()
  }
}

fn build_content_row(row: &[(char, Color)], expected_cols: usize) -> Vec<RenderedCell> {
  let mut cells = Vec::with_capacity(expected_cols);
  let mut current_col = 0;
  let mut col_idx = 0;

  while col_idx < row.len() && current_col < expected_cols {
    let (character, color) = &row[col_idx];
    let cell = content_cell(*character, *color);

    if current_col + cell.display_width > expected_cols {
      break;
    }

    current_col += cell.display_width;
    cells.push(cell);
    col_idx += 1;
  }

  while current_col < expected_cols {
    cells.push(RenderedCell::blank());
    current_col += 1;
  }

  cells
}

fn blank_row(width: usize) -> Vec<RenderedCell> {
  std::iter::repeat_with(RenderedCell::blank)
    .take(width)
    .collect()
}

fn content_cell(character: char, color: Color) -> RenderedCell {
  if character == ' ' {
    return RenderedCell::blank();
  }

  match color {
    Color::Rgb { r, g, b } if calculate_brightness(r, g, b) >= MIN_BRIGHTNESS_THRESHOLD => {
      RenderedCell::new(character, Some((r, g, b)), None)
    }
    Color::Rgb { .. } => RenderedCell::blank(),
    _ => RenderedCell::new(character, None, None),
  }
}

fn cell_at_display_column(row: &[RenderedCell], x: usize) -> Option<&RenderedCell> {
  let mut current_col = 0;

  for cell in row {
    let next_col = current_col + cell.display_width;

    if x < next_col {
      return Some(cell);
    }

    current_col = next_col;
  }

  None
}

fn display_width(row: &[RenderedCell]) -> usize {
  row.iter().map(|cell| cell.display_width).sum()
}

pub(super) fn push_cells(
  buffer: &mut String,
  cells: &[RenderedCell],
  style_state: &mut StyleState,
  base_style: StyleState,
) {
  for cell in cells {
    write_style_transition(
      buffer,
      style_state,
      cell.foreground.or(base_style.foreground),
      cell.background.or(base_style.background),
    );

    buffer.push(cell.character);
  }
}

fn write_style_transition(
  buffer: &mut String,
  style_state: &mut StyleState,
  foreground: Option<RgbColor>,
  background: Option<RgbColor>,
) {
  if style_state.background != background {
    match background {
      Some((r, g, b)) => {
        let _ = write!(buffer, "\x1b[48;2;{};{};{}m", r, g, b);
      }
      None => buffer.push_str("\x1b[49m"),
    }
    style_state.background = background;
  }

  if style_state.foreground != foreground {
    match foreground {
      Some((r, g, b)) => {
        let _ = write!(buffer, "\x1b[38;2;{};{};{}m", r, g, b);
      }
      None => buffer.push_str("\x1b[39m"),
    }
    style_state.foreground = foreground;
  }
}

pub(super) fn reset_to_base_style(
  buffer: &mut String,
  style_state: &mut StyleState,
  base_style: StyleState,
) {
  write_style_transition(
    buffer,
    style_state,
    base_style.foreground,
    base_style.background,
  );
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_dark_rgb_pixels_are_blank_in_rendered_frame() {
    let ascii_frame = vec![vec![
      ('@', Color::Rgb { r: 255, g: 0, b: 0 }),
      (
        '#',
        Color::Rgb {
          r: 10,
          g: 10,
          b: 10,
        },
      ),
    ]];

    let frame = RenderedFrame::from_ascii_frame(&ascii_frame, 2, 1, None, None);

    assert_eq!(frame.cell(0, 0).map(|cell| cell.character), Some('@'));
    assert_eq!(frame.cell(1, 0).map(|cell| cell.character), Some(' '));
  }

  #[test]
  fn test_frame_pads_missing_columns_and_rows() {
    let ascii_frame = vec![vec![('A', Color::White)]];

    let frame = RenderedFrame::from_ascii_frame(&ascii_frame, 3, 2, None, None);

    assert_eq!(frame.width(), 3);
    assert_eq!(frame.height(), 2);
    assert_eq!(frame.cell(0, 0).map(|cell| cell.character), Some('A'));
    assert_eq!(frame.cell(1, 0).map(|cell| cell.character), Some(' '));
    assert_eq!(frame.cell(2, 0).map(|cell| cell.character), Some(' '));
    assert_eq!(frame.cell(0, 1).map(|cell| cell.character), Some(' '));
    assert_eq!(frame.cell(2, 1).map(|cell| cell.character), Some(' '));
  }

  #[test]
  fn test_wide_character_can_be_addressed_from_each_covered_column() {
    let ascii_frame = vec![vec![('界', Color::White), ('Z', Color::White)]];

    let frame = RenderedFrame::from_ascii_frame(&ascii_frame, 3, 1, None, None);

    assert_eq!(frame.cell(0, 0).map(|cell| cell.character), Some('界'));
    assert_eq!(frame.cell(1, 0).map(|cell| cell.character), Some('界'));
    assert_eq!(frame.cell(2, 0).map(|cell| cell.character), Some('Z'));
  }

  #[test]
  fn test_wide_character_is_dropped_when_it_would_overflow_remaining_columns() {
    let ascii_frame = vec![vec![('A', Color::White), ('界', Color::White)]];

    let frame = RenderedFrame::from_ascii_frame(&ascii_frame, 2, 1, None, None);

    assert_eq!(frame.cell(0, 0).map(|cell| cell.character), Some('A'));
    assert_eq!(frame.cell(1, 0).map(|cell| cell.character), Some(' '));
  }

  #[test]
  fn test_terminal_string_reuses_consecutive_style_sequences() {
    let ascii_frame = vec![vec![
      (
        'A',
        Color::Rgb {
          r: 240,
          g: 220,
          b: 200,
        },
      ),
      (
        'B',
        Color::Rgb {
          r: 240,
          g: 220,
          b: 200,
        },
      ),
    ]];

    let frame = RenderedFrame::from_ascii_frame(&ascii_frame, 2, 1, None, None);

    assert_eq!(
      frame.to_terminal_string(),
      "\x1b[?25l\x1b[H\x1b[0m\x1b[49m\x1b[38;2;240;220;200mAB"
    );
  }
}
