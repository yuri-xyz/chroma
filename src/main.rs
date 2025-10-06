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

  let loaded_config = load_config_with_overrides(&cli_args)?;

  #[cfg(feature = "audio")]
  {
    run_application(loaded_config, cli_args.audio_device)
  }

  #[cfg(not(feature = "audio"))]
  {
    run_application(loaded_config)
  }
}

/// Load configuration from file if specified, then apply CLI overrides
fn load_config_with_overrides(cli_args: &CliArgs) -> Result<Option<ShaderParams>> {
  let mut params = if let Some(ref path) = cli_args.config {
    match ShaderParams::load_from_file(path) {
      Ok(config) => {
        println!("✓ Loaded configuration from: {}", path);
        config
      }
      Err(error) => {
        eprintln!("✗ Failed to load config from {}: {}", path, error);
        eprintln!("  Falling back to default configuration.\n");
        ShaderParams::default()
      }
    }
  } else {
    ShaderParams::default()
  };

  // Apply CLI overrides (these take precedence over config file)
  apply_cli_overrides(&mut params, cli_args);

  // Print CLI overrides if any were applied
  let has_overrides = cli_args.frequency.is_some()
    || cli_args.amplitude.is_some()
    || cli_args.speed.is_some()
    || cli_args.brightness.is_some()
    || cli_args.pattern.is_some()
    || cli_args.color_mode.is_some();

  if has_overrides {
    println!("✓ Applied command-line overrides\n");
  }

  Ok(Some(params))
}

/// Apply CLI argument overrides to params (CLI args take precedence over config)
fn apply_cli_overrides(params: &mut ShaderParams, cli: &CliArgs) {
  // Visual parameters
  if let Some(v) = cli.frequency {
    params.frequency = v;
  }
  if let Some(v) = cli.amplitude {
    params.amplitude = v;
  }
  if let Some(v) = cli.speed {
    params.speed = v;
  }
  if let Some(v) = cli.scale {
    params.scale = v;
  }
  if let Some(v) = cli.brightness {
    params.brightness = v;
  }
  if let Some(v) = cli.contrast {
    params.contrast = v;
  }
  if let Some(v) = cli.saturation {
    params.saturation = v;
  }
  if let Some(v) = cli.hue {
    params.hue = v;
  }

  // Pattern type
  if let Some(ref pattern_str) = cli.pattern {
    params.pattern_type = parse_pattern_type(pattern_str);
  }

  // Color mode
  if let Some(ref mode_str) = cli.color_mode {
    params.color_mode = parse_color_mode(mode_str);
  }

  // Palette
  if let Some(ref palette_str) = cli.palette {
    params.palette = parse_palette_type(palette_str);
  }

  // Audio parameters
  #[cfg(feature = "audio")]
  {
    if let Some(v) = cli.audio_enabled {
      params.audio_enabled = v;
    }
    if let Some(v) = cli.bass_influence {
      params.bass_influence = v;
    }
    if let Some(v) = cli.mid_influence {
      params.mid_influence = v;
    }
    if let Some(v) = cli.treble_influence {
      params.treble_influence = v;
    }
    if let Some(v) = cli.beat_distortion {
      params.beat_distortion_strength = v;
    }
    if let Some(v) = cli.beat_zoom {
      params.beat_zoom_strength = v;
    }
  }

  // Distortion
  if let Some(v) = cli.noise_strength {
    params.noise_strength = v;
  }
  if let Some(v) = cli.distort_amplitude {
    params.distort_amplitude = v;
  }

  // Effects
  if let Some(v) = cli.vignette {
    params.vignette = v;
  }

  // Apply clamping after overrides
  params.clamp_all();
}

fn parse_pattern_type(s: &str) -> chroma::params::PatternType {
  use chroma::params::PatternType;

  match s.to_lowercase().as_str() {
    "plasma" => PatternType::Plasma,
    "waves" => PatternType::Waves,
    "ripples" => PatternType::Ripples,
    "vortex" => PatternType::Vortex,
    "noise" => PatternType::Noise,
    "geometric" | "geo" => PatternType::Geometric,
    "voronoi" => PatternType::Voronoi,
    "truchet" => PatternType::Truchet,
    "hexagonal" | "hexagon" | "hex" => PatternType::Hexagonal,
    "interference" | "interf" => PatternType::Interference,
    "fractal" => PatternType::Fractal,
    "glitch" => PatternType::Glitch,
    "spiral" => PatternType::Spiral,
    "rings" => PatternType::Rings,
    "grid" => PatternType::Grid,
    "diamonds" | "diamond" => PatternType::Diamonds,
    "sphere" => PatternType::Sphere,
    "octgrams" | "octgram" => PatternType::Octgrams,
    "warped" | "warpedfbm" => PatternType::WarpedFbm,
    _ => PatternType::Plasma,
  }
}

fn parse_color_mode(s: &str) -> chroma::params::ColorMode {
  use chroma::params::ColorMode;

  match s.to_lowercase().as_str() {
    "rainbow" => ColorMode::Rainbow,
    "monochrome" | "mono" => ColorMode::Monochrome,
    "duotone" => ColorMode::Duotone,
    "warm" => ColorMode::Warm,
    "cool" => ColorMode::Cool,
    "neon" => ColorMode::Neon,
    "pastel" => ColorMode::Pastel,
    "cyberpunk" | "cyber" => ColorMode::Cyberpunk,
    "warped" => ColorMode::Warped,
    "chromatic" | "chrome" => ColorMode::Chromatic,
    _ => ColorMode::Rainbow,
  }
}

fn parse_palette_type(s: &str) -> chroma::params::PaletteType {
  use chroma::params::PaletteType;

  match s.to_lowercase().as_str() {
    "standard" | "std" => PaletteType::Standard,
    "blocks" | "block" => PaletteType::Blocks,
    "circles" | "circle" => PaletteType::Circles,
    "smooth" => PaletteType::Smooth,
    "braille" => PaletteType::Braille,
    "geometric" | "geo" => PaletteType::Geometric,
    "mixed" => PaletteType::Mixed,
    "dots" => PaletteType::Dots,
    "shades" | "shade" => PaletteType::Shades,
    "lines" => PaletteType::Lines,
    "triangles" | "tri" => PaletteType::Triangles,
    "arrows" | "arrow" => PaletteType::Arrows,
    "powerline" | "power" => PaletteType::Powerline,
    "boxdraw" | "box" => PaletteType::BoxDraw,
    "extended" | "extend" => PaletteType::Extended,
    "simple" => PaletteType::Simple,
    _ => PaletteType::Simple,
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
