use anyhow::Result;
use crossterm::{cursor, execute, terminal};
use std::io::stdout;

/// Setup terminal for rendering
pub fn setup() -> Result<()> {
  terminal::enable_raw_mode()?;

  execute!(
    stdout(),
    terminal::EnterAlternateScreen,
    cursor::Hide,
    terminal::Clear(terminal::ClearType::All)
  )?;

  Ok(())
}

/// Restore terminal to normal state
pub fn cleanup() -> Result<()> {
  execute!(stdout(), cursor::Show, terminal::LeaveAlternateScreen)?;
  terminal::disable_raw_mode()?;

  Ok(())
}
