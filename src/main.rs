// A GPU-accelerated shader visualization tool that renders beautiful
// patterns and effects directly in your terminal using ASCII art.

use anyhow::{Context, Result};
use clap::Parser;

mod app;
mod cli;
mod list_commands;
mod terminal;

use app::{App, AppOptions};
use chroma::{constants::DEFAULT_FPS, params::ShaderParams};
use cli::CliArgs;

fn main() -> Result<()> {
  let cli_args = CliArgs::parse();

  if cli_args.list_audio_devices {
    use chroma::audio::AudioCapture;
    return AudioCapture::list_devices();
  }

  // Handle --list-patterns flag
  if cli_args.list_patterns {
    return list_commands::list_patterns();
  }

  // Handle --list-color-modes flag
  if cli_args.list_color_modes {
    return list_commands::list_color_modes();
  }

  // Handle --list-palettes flag
  if cli_args.list_palettes {
    return list_commands::list_palettes();
  }

  let loaded_config = load_config_with_overrides(&cli_args)?;

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

  let app_options = AppOptions {
    loaded_config,
    show_status_bar: !cli_args.no_status,
    stream_dimensions: cli_args.stream,
    stream_format: cli_args.stream_format,
    config_path: cli_args.config.clone(),
    audio_device: cli_args.audio_device,
    custom_shader,
    target_fps,
  };

  run_application(app_options)
}

/// Load configuration from file if specified, then apply CLI overrides
/// Priority order (lowest to highest): randomized -> preset -> config file -> CLI args
fn load_config_with_overrides(cli_args: &CliArgs) -> Result<Option<ShaderParams>> {
  // Step 1: Start with audio-reactive defaults
  let mut params = ShaderParams::with_audio_reactive_defaults();

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
    params.pattern_type = pattern_str
      .parse()
      .unwrap_or(chroma::params::PatternType::Plasma);
  }

  // Color mode
  if let Some(ref mode_str) = cli.color_mode {
    params.color_mode = mode_str
      .parse()
      .unwrap_or(chroma::params::ColorMode::Rainbow);
  }

  // Palette
  if let Some(ref palette_str) = cli.palette {
    params.palette = palette_str
      .parse()
      .unwrap_or(chroma::params::PaletteType::Simple);
  }

  // Audio parameters
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
  if let Some(v) = cli.gravity {
    params.gravity = v;
  }
  if let Some(v) = cli.mouse_fight {
    params.mouse_fight = v;
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

/// Load and validate custom shader file
fn load_custom_shader(shader_path: &str) -> Result<String> {
  use std::{fs, path::Path};

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
fn run_application(app_options: AppOptions) -> Result<()> {
  let stream_mode = app_options.stream_dimensions.is_some();

  // Skip terminal setup in stream mode
  if !stream_mode {
    terminal::setup()?;
  }

  let result = pollster::block_on(async {
    let mut app = App::new(app_options).await?;
    app.run()
  });

  // Skip terminal cleanup in stream mode
  if !stream_mode {
    terminal::cleanup()?;
  }

  result
}
