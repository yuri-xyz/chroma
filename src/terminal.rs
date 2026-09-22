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
  install_panic_cleanup();
  terminal::enable_raw_mode()?;

  let mut out = stdout();
  write_setup_sequence(&mut out)?;

  Ok(())
}

/// Restore the terminal before the panic message is printed, so a panic does
/// not leave the shell in raw mode on the alternate screen. Only a panic on the
/// thread that set the terminal up ends the program; one on a background thread
/// (audio capture) leaves rendering running, so the terminal stays as it is.
fn install_panic_cleanup() {
  let previous_hook = std::panic::take_hook();
  let render_thread = std::thread::current().id();

  std::panic::set_hook(Box::new(move |info| {
    if std::thread::current().id() == render_thread {
      let _ = cleanup();
    }
    previous_hook(info);
  }));
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
