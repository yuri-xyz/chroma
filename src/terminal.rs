use std::io::{stdout, Write};

use anyhow::Result;
use crossterm::{cursor, execute, terminal};

fn write_setup_sequence<W: Write>(writer: &mut W) -> Result<()> {
  execute!(
    writer,
    terminal::EnterAlternateScreen,
    cursor::Hide,
    terminal::Clear(terminal::ClearType::All)
  )?;

  Ok(())
}

fn write_cleanup_sequence<W: Write>(writer: &mut W) -> Result<()> {
  execute!(writer, cursor::Show, terminal::LeaveAlternateScreen)?;

  Ok(())
}

/// Setup terminal for rendering
pub fn setup() -> Result<()> {
  terminal::enable_raw_mode()?;

  let mut out = stdout();
  write_setup_sequence(&mut out)?;

  Ok(())
}

/// Restore terminal to normal state
pub fn cleanup() -> Result<()> {
  let mut out = stdout();
  write_cleanup_sequence(&mut out)?;
  terminal::disable_raw_mode()?;

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_setup_sequence_writes_alternate_screen_hide_and_clear() {
    let mut output = Vec::new();

    write_setup_sequence(&mut output).unwrap();

    let text = String::from_utf8(output).unwrap();

    assert_eq!(text, "\u{1b}[?1049h\u{1b}[?25l\u{1b}[2J");
  }

  #[test]
  fn test_cleanup_sequence_writes_show_and_leave_alternate_screen() {
    let mut output = Vec::new();

    write_cleanup_sequence(&mut output).unwrap();

    let text = String::from_utf8(output).unwrap();

    assert_eq!(text, "\u{1b}[?25h\u{1b}[?1049l");
  }
}
