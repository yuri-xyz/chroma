// A GPU-accelerated shader visualization tool that renders beautiful
// patterns and effects directly in your terminal using ASCII art.

use anyhow::Result;
use clap::Parser;
use crossterm::{cursor, execute, terminal};
use std::io::stdout;

mod app;
mod cli;
mod constants;
mod utils;

use app::App;
use chroma::params::ShaderParams;
use cli::CliArgs;

fn main() -> Result<()> {
  let cli_args = CliArgs::parse();

  // Handle --list-audio-devices flag
  #[cfg(feature = "audio")]
  if cli_args.list_audio_devices {
    use chroma::audio::AudioCapture;
    return AudioCapture::list_devices();
  }

  let loaded_config = load_config(cli_args.config)?;

  #[cfg(feature = "audio")]
  {
    run_application(loaded_config, cli_args.audio_device)
  }

  #[cfg(not(feature = "audio"))]
  {
    run_application(loaded_config)
  }
}

/// Load configuration from file if specified
fn load_config(config_path: Option<String>) -> Result<Option<ShaderParams>> {
  if let Some(path) = config_path {
    match ShaderParams::load_from_file(&path) {
      Ok(config) => {
        println!("✓ Loaded configuration from: {}", path);

        Ok(Some(config))
      }
      Err(error) => {
        eprintln!("✗ Failed to load config from {}: {}", path, error);
        eprintln!("  Falling back to default configuration.\n");

        Ok(None)
      }
    }
  } else {
    Ok(None)
  }
}

/// Initialize terminal, run app, and cleanup
fn run_application(
  loaded_config: Option<ShaderParams>,
  #[cfg(feature = "audio")] audio_device: Option<String>,
) -> Result<()> {
  setup_terminal()?;

  let result = pollster::block_on(async {
    #[cfg(feature = "audio")]
    let mut app = App::new(loaded_config, audio_device).await?;

    #[cfg(not(feature = "audio"))]
    let mut app = App::new(loaded_config).await?;

    app.run()
  });

  cleanup_terminal()?;

  result
}

/// Setup terminal for rendering
fn setup_terminal() -> Result<()> {
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
fn cleanup_terminal() -> Result<()> {
  execute!(stdout(), cursor::Show, terminal::LeaveAlternateScreen)?;
  terminal::disable_raw_mode()?;

  Ok(())
}
