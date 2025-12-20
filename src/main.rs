// A GPU-accelerated shader visualization tool that renders beautiful
// patterns and effects directly in your terminal using ASCII art.

use anyhow::{Context, Result};
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
use constants::DEFAULT_FPS;

fn main() -> Result<()> {
  let cli_args = CliArgs::parse();

  // Handle --list-audio-devices flag
  #[cfg(feature = "audio")]
  if cli_args.list_audio_devices {
    use chroma::audio::AudioCapture;
    return AudioCapture::list_devices();
  }

  // Handle --list-patterns flag
  if cli_args.list_patterns {
    return list_patterns();
  }

  // Handle --list-color-modes flag
  if cli_args.list_color_modes {
    return list_color_modes();
  }

  // Handle --list-palettes flag
  if cli_args.list_palettes {
    return list_palettes();
  }

  let loaded_config = load_config_with_overrides(&cli_args)?;
  let show_status_bar = !cli_args.no_status;
  let stream_dimensions = cli_args.stream;
  let config_path = cli_args.config.clone();

  // Load custom shader if provided
  let custom_shader = if let Some(ref shader_path) = cli_args.custom_shader {
    Some(load_custom_shader(shader_path)?)
  } else {
    None
  };

  // Validate FPS is positive
  let target_fps = if cli_args.fps == 0 {
    eprintln!(
      "Warning: FPS cannot be 0. Using default of {} FPS.",
      DEFAULT_FPS
    );
    DEFAULT_FPS
  } else {
    cli_args.fps
  };

  #[cfg(feature = "audio")]
  {
    run_application(
      loaded_config,
      show_status_bar,
      stream_dimensions,
      config_path,
      cli_args.audio_device,
      custom_shader,
      target_fps,
    )
  }

  #[cfg(not(feature = "audio"))]
  {
    run_application(
      loaded_config,
      show_status_bar,
      stream_dimensions,
      config_path,
      custom_shader,
      target_fps,
    )
  }
}

/// Load configuration from file if specified, then apply CLI overrides
/// Priority order (lowest to highest): randomized -> preset -> config file -> CLI args
fn load_config_with_overrides(cli_args: &CliArgs) -> Result<Option<ShaderParams>> {
  // Step 1: Start with defaults (or audio defaults)
  #[cfg(feature = "audio")]
  let mut params = ShaderParams::with_audio_reactive_defaults();

  #[cfg(not(feature = "audio"))]
  let mut params = ShaderParams::default();

  // Step 2: Apply randomization if requested (lowest priority)
  if cli_args.random {
    params.randomize();
  }

  // Step 3: Apply built-in preset if specified (low priority, overrides random)
  if let Some(ref preset_value) = cli_args.preset {
    params = if preset_value.eq_ignore_ascii_case("random") {
      chroma::presets::get_random_preset()
    } else {
      let index = preset_value.parse::<u32>().map_err(|_| {
        anyhow::anyhow!(
          "Invalid preset: '{}'. Use a number (0-{}) or 'random'",
          preset_value,
          chroma::presets::preset_count() - 1
        )
      })?;
      chroma::presets::get_preset(index)
    };
  }

  // Step 4: Apply config file overrides (medium priority)
  if let Some(ref path) = cli_args.config {
    params = ShaderParams::load_from_file(path)
      .context(format!("Failed to load config file: {}", path))?;
  }

  // Step 5: Apply CLI overrides (highest priority)
  apply_cli_overrides(&mut params, cli_args)?;

  Ok(Some(params))
}

/// Apply CLI argument overrides to params (CLI args take precedence over config)
fn apply_cli_overrides(params: &mut ShaderParams, cli: &CliArgs) -> Result<()> {
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
    if let Some(v) = cli.beat_sensitivity {
      params.beat_sensitivity = v;
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

  // Background color (parse hex) - for terminal cell background
  if let Some(ref hex_color) = cli.background_color {
    let (r, g, b) = chroma::utils::color::parse_hex_color(hex_color)
      .map_err(|e| anyhow::anyhow!("Invalid background color '{}': {}", hex_color, e))?;

    params.terminal_bg_r = r;
    params.terminal_bg_g = g;
    params.terminal_bg_b = b;
  }

  // Apply clamping after overrides
  params.clamp_all();

  Ok(())
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

/// Load and validate custom shader file
fn load_custom_shader(shader_path: &str) -> Result<String> {
  use std::fs;
  use std::path::Path;

  let path = Path::new(shader_path);

  if !path.exists() {
    anyhow::bail!(
      "Custom shader file not found: '{}'\nPlease provide a valid path to a WGSL shader file.",
      shader_path
    );
  }

  if !path.is_file() {
    anyhow::bail!(
      "Custom shader path is not a file: '{}'\nPlease provide a path to a WGSL file.",
      shader_path
    );
  }

  let shader_source = fs::read_to_string(path).context(format!(
    "Failed to read custom shader file: {}",
    shader_path
  ))?;

  if shader_source.trim().is_empty() {
    anyhow::bail!(
      "Custom shader file is empty: '{}'\nPlease provide a valid WGSL shader file.",
      shader_path
    );
  }

  Ok(shader_source)
}

/// Initialize terminal, run app, and cleanup
#[cfg(feature = "audio")]
fn run_application(
  loaded_config: Option<ShaderParams>,
  show_status_bar: bool,
  stream_dimensions: Option<cli::StreamDimensions>,
  config_path: Option<String>,
  audio_device: Option<String>,
  custom_shader: Option<String>,
  target_fps: u32,
) -> Result<()> {
  // Skip terminal setup in stream mode
  if stream_dimensions.is_none() {
    setup_terminal()?;
  }

  let result = pollster::block_on(async {
    let mut app = App::new(
      loaded_config,
      show_status_bar,
      stream_dimensions,
      config_path,
      audio_device,
      custom_shader,
      target_fps,
    )
    .await?;
    app.run()
  });

  // Skip terminal cleanup in stream mode
  if stream_dimensions.is_none() {
    cleanup_terminal()?;
  }

  result
}

/// Initialize terminal, run app, and cleanup
#[cfg(not(feature = "audio"))]
fn run_application(
  loaded_config: Option<ShaderParams>,
  show_status_bar: bool,
  stream_dimensions: Option<cli::StreamDimensions>,
  config_path: Option<String>,
  custom_shader: Option<String>,
  target_fps: u32,
) -> Result<()> {
  // Skip terminal setup in stream mode
  if stream_dimensions.is_none() {
    setup_terminal()?;
  }

  let result = pollster::block_on(async {
    let mut app = App::new(
      loaded_config,
      show_status_bar,
      stream_dimensions,
      config_path,
      custom_shader,
      target_fps,
    )
    .await?;
    app.run()
  });

  // Skip terminal cleanup in stream mode
  if stream_dimensions.is_none() {
    cleanup_terminal()?;
  }

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

/// List all available pattern types
fn list_patterns() -> Result<()> {
  use chroma::params::PatternType;

  println!("Available Pattern Types:");
  println!();

  for pattern in PatternType::all() {
    println!(
      "  {:<15} (display: {})",
      pattern.full_name(),
      pattern.name()
    );
  }

  println!();
  println!("Use with: --pattern <PATTERN>");
  println!("In-app: Press 'T' to cycle through patterns");

  Ok(())
}

/// List all available color modes
fn list_color_modes() -> Result<()> {
  use chroma::params::ColorMode;

  println!("Available Color Modes:");
  println!();

  for mode in ColorMode::all() {
    println!("  {:<15} (display: {})", mode.full_name(), mode.name());
  }

  println!();
  println!("Use with: --color-mode <MODE>");
  println!("In-app: Press 'C' to cycle through color modes");

  Ok(())
}

/// List all available palette types
fn list_palettes() -> Result<()> {
  use chroma::params::PaletteType;

  println!("Available ASCII Palettes:");
  println!();

  for palette in PaletteType::all() {
    println!(
      "  {:<15} (display: {})",
      palette.full_name(),
      palette.name()
    );
  }

  println!();
  println!("Use with: --palette <PALETTE>");
  println!("In-app: Press 'P' to cycle through palettes");

  Ok(())
}
