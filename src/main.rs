// A GPU-accelerated shader visualization tool that renders beautiful
// patterns and effects directly in your terminal using ASCII art.

use std::{
  path::Path,
  sync::{Arc, Mutex},
};

use anyhow::{Context, Result};
use clap::Parser;

mod app;
mod cli;
mod list_commands;
mod terminal;

use app::{App, AppOptions, ConfigLoader, PresetCycler};
use chroma::{
  constants::DEFAULT_FPS,
  params::ShaderParams,
  presets::{PresetCycle, PresetOrder},
};
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

  let preset = preset_selection(&cli_args)?;
  let layers = Arc::new(ParamLayers::new(base_params(&cli_args, preset), &cli_args));
  let loaded_config = layers.load(cli_args.config.as_deref().map(Path::new))?;
  let config_loader = config_loader(Arc::clone(&layers));
  let preset_cycler = preset_cycler(layers, &cli_args, preset);

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
    loaded_config: Some(loaded_config),
    config_loader,
    preset_cycler,
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

/// `--preset` resolved to a concrete built-in preset.
#[derive(Debug, Clone, Copy)]
struct PresetSelection {
  index: u32,
  /// How `--preset-interval` moves on from `index`.
  order: PresetOrder,
}

fn preset_selection(cli_args: &CliArgs) -> Result<Option<PresetSelection>> {
  let Some(ref preset_value) = cli_args.preset else {
    return Ok(None);
  };

  if preset_value.eq_ignore_ascii_case("random") {
    return Ok(Some(PresetSelection {
      index: chroma::presets::random_preset_index(),
      order: PresetOrder::Random,
    }));
  }

  let index = preset_value.parse::<u32>().map_err(|_| {
    anyhow::anyhow!(
      "Invalid preset: '{}'. Use a number (0-{}) or 'random'",
      preset_value,
      chroma::presets::preset_count() - 1
    )
  })?;

  Ok(Some(PresetSelection {
    index,
    order: PresetOrder::Sequential,
  }))
}

/// Build the layers below the config file.
/// Priority order (lowest to highest): randomized -> preset -> config file -> CLI args
fn base_params(cli_args: &CliArgs, preset: Option<PresetSelection>) -> ShaderParams {
  // Apply built-in preset if specified (overrides random)
  if let Some(preset) = preset {
    return chroma::presets::get_preset(preset.index);
  }

  // Start with audio-reactive defaults
  let mut params = ShaderParams::with_audio_reactive_defaults();

  // Apply randomization if requested (lowest priority)
  if cli_args.random {
    params.randomize();
  }

  params
}

/// The layers that produce the final params. The base is shared because preset
/// cycling swaps it at runtime, and config reloads that happen afterwards must
/// layer over the preset now showing rather than the one chroma started with.
struct ParamLayers {
  base: Mutex<ShaderParams>,
  cli_args: CliArgs,
}

impl ParamLayers {
  fn new(base: ShaderParams, cli_args: &CliArgs) -> Self {
    Self {
      base: Mutex::new(base),
      cli_args: cli_args.clone(),
    }
  }

  fn load(&self, config_path: Option<&Path>) -> Result<ShaderParams> {
    let base = self
      .base
      .lock()
      .map_err(|_| anyhow::anyhow!("Base params lock was poisoned"))?
      .clone();

    layered_params(&base, config_path, &self.cli_args)
  }

  fn set_base(&self, base: ShaderParams) -> Result<()> {
    *self
      .base
      .lock()
      .map_err(|_| anyhow::anyhow!("Base params lock was poisoned"))? = base;

    Ok(())
  }
}

/// Apply the config file (if any) over `base`, then the CLI overrides.
fn layered_params(
  base: &ShaderParams,
  config_path: Option<&Path>,
  cli_args: &CliArgs,
) -> Result<ShaderParams> {
  let mut params = match config_path {
    Some(path) => ShaderParams::load_from_file_over(path, base)
      .context(format!("Failed to load config file: {}", path.display()))?,
    None => base.clone(),
  };

  apply_cli_overrides(&mut params, cli_args)?;

  Ok(params)
}

/// Reloads re-run the same layering so edits to the config file keep the
/// preset underneath and the CLI overrides on top.
fn config_loader(layers: Arc<ParamLayers>) -> ConfigLoader {
  Arc::new(move |path: &Path| layers.load(Some(path)))
}

/// `--preset-interval` swaps the preset layer and re-runs the same layering, so
/// the config file and CLI overrides stay on top of every preset in the cycle.
fn preset_cycler(
  layers: Arc<ParamLayers>,
  cli_args: &CliArgs,
  preset: Option<PresetSelection>,
) -> Option<PresetCycler> {
  let interval_seconds = cli_args.preset_interval?;
  let preset = preset?;
  let config_path = cli_args.config.clone();

  Some(PresetCycler {
    cycle: PresetCycle::new(interval_seconds as f32, preset.order, preset.index),
    loader: Box::new(move |index: u32| {
      layers.set_base(chroma::presets::get_preset(index))?;
      layers.load(config_path.as_deref().map(Path::new))
    }),
  })
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
      .map_err(|error: String| anyhow::anyhow!("{error}. Run --list-patterns to see options"))?;
  }

  // Color mode
  if let Some(ref mode_str) = cli.color_mode {
    params.color_mode = mode_str
      .parse()
      .map_err(|error: String| anyhow::anyhow!("{error}. Run --list-color-modes to see options"))?;
  }

  // Palette
  if let Some(ref palette_str) = cli.palette {
    params.palette = palette_str
      .parse()
      .map_err(|error: String| anyhow::anyhow!("{error}. Run --list-palettes to see options"))?;
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

#[cfg(test)]
mod tests {
  use std::{
    fs,
    time::{SystemTime, UNIX_EPOCH},
  };

  use chroma::params::PatternType;

  use super::*;

  fn parse_cli(args: &[&str]) -> CliArgs {
    CliArgs::try_parse_from(std::iter::once("chroma").chain(args.iter().copied())).unwrap()
  }

  fn write_temp_config(name: &str, content: &str) -> std::path::PathBuf {
    let timestamp = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .unwrap()
      .as_nanos();
    let path = std::env::temp_dir().join(format!("chroma-main-{name}-{timestamp}.toml"));

    fs::write(&path, content).unwrap();
    path
  }

  #[test]
  fn test_partial_config_keeps_preset_values() {
    let path = write_temp_config("preset", "brightness = 0.9\n");
    let cli = parse_cli(&["--preset", "3", "-c", path.to_str().unwrap()]);
    let preset = chroma::presets::get_preset(3);

    let base = base_params(&cli, preset_selection(&cli).unwrap());
    let params = layered_params(&base, Some(path.as_path()), &cli).unwrap();

    assert_eq!(params.brightness, 0.9);
    assert_eq!(params.pattern_type, preset.pattern_type);
    assert_eq!(params.color_mode, preset.color_mode);

    let _ = fs::remove_file(path);
  }

  #[test]
  fn test_config_loader_reapplies_cli_overrides_on_reload() {
    let path = write_temp_config("reload", "bass_influence = 0.1\npattern_type = \"waves\"\n");
    let cli = parse_cli(&["--bass-influence", "0.8", "-c", path.to_str().unwrap()]);
    let loader = config_loader(Arc::new(ParamLayers::new(base_params(&cli, None), &cli)));

    let params = loader(&path).unwrap();

    assert_eq!(params.bass_influence, 0.8);
    assert_eq!(params.pattern_type, PatternType::Waves);

    let _ = fs::remove_file(path);
  }

  #[test]
  fn test_preset_cycle_keeps_config_and_cli_layers_on_top() {
    let path = write_temp_config("cycle", "brightness = 0.9\n");
    let cli = parse_cli(&[
      "--preset",
      "3",
      "--preset-interval",
      "30",
      "--bass-influence",
      "0.8",
      "-c",
      path.to_str().unwrap(),
    ]);
    let preset = preset_selection(&cli).unwrap();
    let layers = Arc::new(ParamLayers::new(base_params(&cli, preset), &cli));
    let loader = config_loader(Arc::clone(&layers));
    let cycler = preset_cycler(layers, &cli, preset).expect("expected a preset cycler");
    let next_preset = chroma::presets::get_preset(4);

    let cycled = (cycler.loader)(4).unwrap();
    let reloaded = loader(&path).unwrap();

    // A config reload after the switch layers over the new preset, not the startup one.
    for params in [cycled, reloaded] {
      assert_eq!(params.pattern_type, next_preset.pattern_type);
      assert_eq!(params.color_mode, next_preset.color_mode);
      assert_eq!(params.brightness, 0.9);
      assert_eq!(params.bass_influence, 0.8);
    }

    let _ = fs::remove_file(path);
  }

  #[test]
  fn test_preset_cycler_needs_preset_interval() {
    let cli = parse_cli(&["--preset", "random"]);
    let preset = preset_selection(&cli).unwrap();
    let layers = Arc::new(ParamLayers::new(base_params(&cli, preset), &cli));

    assert!(preset_cycler(layers, &cli, preset).is_none());
  }

  #[test]
  fn test_invalid_preset_value_is_rejected() {
    let cli = parse_cli(&["--preset", "sparkly"]);

    assert!(preset_selection(&cli).is_err());
  }

  #[test]
  fn test_unknown_enum_names_are_rejected() {
    for args in [
      ["--pattern", "plasmaa"],
      ["--color-mode", "rainbw"],
      ["--palette", "dotz"],
    ] {
      let cli = parse_cli(&args);
      let mut params = ShaderParams::default();

      assert!(apply_cli_overrides(&mut params, &cli).is_err());
    }
  }
}
