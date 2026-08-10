use std::io::{stdout, Write};

use anyhow::Result;
use crossterm::{cursor, event, execute, terminal};

fn write_setup_sequence<W: Write>(writer: &mut W) -> Result<()> {
  execute!(
    writer,
    terminal::EnterAlternateScreen,
    cursor::Hide,
    terminal::Clear(terminal::ClearType::All),
    event::EnableMouseCapture
  )?;

  Ok(())
}

fn write_cleanup_sequence<W: Write>(writer: &mut W) -> Result<()> {
  execute!(
    writer,
    event::DisableMouseCapture,
    cursor::Show,
    terminal::LeaveAlternateScreen
  )?;

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
  fn test_setup_sequence_writes_alternate_screen_hide_clear_and_mouse() {
    let mut output = Vec::new();

    write_setup_sequence(&mut output).unwrap();

    let text = String::from_utf8(output).unwrap();

    assert!(text.contains("\u{1b}[?1049h"));
    assert!(text.contains("\u{1b}[?25l"));
    assert!(text.contains("\u{1b}[2J"));
    assert!(
      text.contains("\u{1b}[?1000h")
        || text.contains("\u{1b}[?1002h")
        || text.contains("\u{1b}[?1003h")
        || text.contains("\u{1b}[?1006h")
    );
  }

  #[test]
  fn test_cleanup_sequence_disables_mouse_shows_cursor_and_leaves_alt_screen() {
    let mut output = Vec::new();

    write_cleanup_sequence(&mut output).unwrap();

    let text = String::from_utf8(output).unwrap();

    assert!(text.contains("\u{1b}[?25h"));
    assert!(text.contains("\u{1b}[?1049l"));
    assert!(
      text.contains("\u{1b}[?1000l")
        || text.contains("\u{1b}[?1002l")
        || text.contains("\u{1b}[?1003l")
        || text.contains("\u{1b}[?1006l")
    );
  }
}
