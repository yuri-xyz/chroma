use std::str::FromStr;

use chroma::render::StreamFormat;
use clap::Parser;

/// Dimensions for stream mode (width x height in terminal cells)
#[derive(Debug, Clone, Copy)]
pub struct StreamDimensions {
  pub width: u16,
  pub height: u16,
}

impl FromStr for StreamDimensions {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let parts: Vec<&str> = s.split('x').collect();

    if parts.len() != 2 {
      return Err(format!(
        "Invalid format '{}'. Expected format: WIDTHxHEIGHT (e.g., 20x12)",
        s
      ));
    }

    let width = parts[0]
      .parse::<u16>()
      .map_err(|_| format!("Invalid width '{}'. Must be a positive integer.", parts[0]))?;

    let height = parts[1]
      .parse::<u16>()
      .map_err(|_| format!("Invalid height '{}'. Must be a positive integer.", parts[1]))?;

    if width == 0 || height == 0 {
      return Err("Width and height must be greater than 0.".to_string());
    }

    if width > 1000 || height > 1000 {
      return Err(format!(
        "Dimensions {}x{} are too large. Maximum is 1000x1000.",
        width, height
      ));
    }

    Ok(StreamDimensions { width, height })
  }
}

/// Command-line arguments
#[derive(Parser, Debug)]
#[command(name = "chroma")]
#[command(
  about = "GPU-accelerated terminal shader visualizer with audio reactivity",
  long_about = "A beautiful real-time shader visualizer that renders directly in your terminal \
using ASCII art. Features audio reactivity, multiple pattern types, customizable color modes, \
and extensive configuration options. Press 'Q' to quit, 'R' to randomize, 'S' to save config."
)]
pub struct CliArgs {
  /// Load configuration from a saved TOML file (created with 'S' key during runtime)
  #[arg(short, long, value_name = "FILE")]
  pub config: Option<String>,

  /// Audio device name for capture. Use --list-audio-devices to see available devices
  #[arg(short = 'a', long, value_name = "DEVICE")]
  pub audio_device: Option<String>,

  /// List all available audio input devices and exit
  #[arg(long)]
  pub list_audio_devices: bool,

  /// List all available pattern types and exit
  #[arg(long)]
  pub list_patterns: bool,

  /// List all available color modes and exit
  #[arg(long)]
  pub list_color_modes: bool,

  /// List all available ASCII palettes and exit
  #[arg(long)]
  pub list_palettes: bool,

  /// Disable status bar (shader fills entire terminal)
  #[arg(long)]
  pub no_status: bool,

  /// Stream mode: output frames at fixed dimensions (e.g., 20x12) for embedding in other TUI apps.
  /// Disables terminal setup, status bar, and interactive features. Outputs full frames to stdout.
  #[arg(long, value_name = "WIDTHxHEIGHT")]
  pub stream: Option<StreamDimensions>,

  /// Stream output format. ansi preserves colored terminal text; cells emits tab-separated cell records.
  #[arg(long, value_name = "FORMAT", default_value = "ansi")]
  pub stream_format: StreamFormat,

  /// Start with randomized parameters (lowest priority, overridden by config and args)
  #[arg(short = 'r', long)]
  pub random: bool,

  /// Use a built-in preset by number (0-24) or "random". Wraps around if exceeds max.
  /// Presets are embedded in the binary, no external files needed.
  #[arg(long, value_name = "NUM|random")]
  pub preset: Option<String>,

  // Visual parameters
  /// Pattern wave density/detail level. Higher = more detail. Range: 3.0-18.0
  #[arg(short = 'f', long, value_name = "FLOAT")]
  pub frequency: Option<f32>,

  /// Wave height/intensity. Higher = more extreme variations. Range: 0.0-2.0
  #[arg(short = 'A', long, value_name = "FLOAT")]
  pub amplitude: Option<f32>,

  /// Animation speed. 0 = frozen, 1 = fast. Range: 0.0-1.0
  #[arg(short = 's', long, value_name = "FLOAT")]
  pub speed: Option<f32>,

  /// Zoom level. Lower = zoomed in, higher = zoomed out. Range: 0.1-5.0
  #[arg(short = 'S', long, value_name = "FLOAT")]
  pub scale: Option<f32>,

  /// Overall brightness multiplier. Range: 0.0-2.0
  #[arg(short = 'b', long, value_name = "FLOAT")]
  pub brightness: Option<f32>,

  /// Contrast adjustment. Lower = softer, higher = sharper. Range: 0.2-2.0
  #[arg(short = 'C', long, value_name = "FLOAT")]
  pub contrast: Option<f32>,

  /// Color saturation. 0 = grayscale, 2 = very vibrant. Range: 0.0-2.0
  #[arg(short = 't', long, value_name = "FLOAT")]
  pub saturation: Option<f32>,

  /// Hue rotation in degrees. Shifts all colors around the color wheel. Range: 0.0-360.0
  #[arg(short = 'H', long, value_name = "DEGREES")]
  pub hue: Option<f32>,

  /// Pattern type: plasma, waves, ripples, vortex, noise, geometric, voronoi, truchet, hexagonal, interference, fractal, glitch, spiral, rings, grid, diamonds, sphere, octgrams, warped, kaleidoscope, tunnel, metaballs, world, fluid, pyramid, infinity
  #[arg(short = 'p', long, value_name = "PATTERN")]
  pub pattern: Option<String>,

  /// Color scheme: rainbow, monochrome, duotone, warm, cool, neon, pastel, cyberpunk, warped, chromatic
  #[arg(short = 'm', long, value_name = "MODE")]
  pub color_mode: Option<String>,

  /// ASCII character set: standard, blocks, circles, smooth, braille, geometric, mixed, dots, shades, lines, triangles, arrows, powerline, boxdraw, extended, simple
  #[arg(short = 'P', long, value_name = "PALETTE")]
  pub palette: Option<String>,

  // Audio parameters
  /// How much bass frequencies affect amplitude. Range: 0.0-1.0
  #[arg(short = 'B', long, value_name = "FLOAT")]
  pub bass_influence: Option<f32>,

  /// How much mid frequencies affect pattern frequency. Range: 0.0-1.0
  #[arg(short = 'M', long, value_name = "FLOAT")]
  pub mid_influence: Option<f32>,

  /// How much treble frequencies affect animation speed. Range: 0.0-1.0
  #[arg(short = 'T', long, value_name = "FLOAT")]
  pub treble_influence: Option<f32>,

  /// Beat detection sensitivity. Higher = more sensitive to subtle beats. Range: 0.1-3.0, Default: 1.0
  #[arg(long, value_name = "FLOAT")]
  pub beat_sensitivity: Option<f32>,

  /// Beat-triggered distortion effect strength. Range: 0.0-2.0
  #[arg(short = 'D', long, value_name = "FLOAT")]
  pub beat_distortion: Option<f32>,

  /// Beat-triggered zoom pulse effect strength. Range: 0.0-2.0
  #[arg(short = 'z', long, value_name = "FLOAT")]
  pub beat_zoom: Option<f32>,

  // Distortion parameters
  /// Subtle noise overlay strength. Adds texture/grain. Range: 0.0-0.5
  #[arg(short = 'n', long, value_name = "FLOAT")]
  pub noise_strength: Option<f32>,

  /// Spatial distortion/warping amount. Range: 0.0-2.0
  #[arg(short = 'x', long, value_name = "FLOAT")]
  pub distort_amplitude: Option<f32>,

  // Effects
  /// Edge darkening effect strength. 0 = off. Range: 0.0-1.0
  #[arg(short = 'v', long, value_name = "FLOAT")]
  pub vignette: Option<f32>,

  /// Terminal background color in hex format (e.g. FF0000, #00FF00, ABC, #123456). Sets the background color for the terminal cells/window
  #[arg(long, value_name = "HEX")]
  pub background_color: Option<String>,

  /// Load a custom WGSL shader file (overrides --pattern and config pattern settings)
  #[arg(long, value_name = "FILE")]
  pub custom_shader: Option<String>,

  /// Target frames per second for rendering. Must be > 0. Default: 60
  #[arg(long, value_name = "FPS", default_value = "60")]
  pub fps: u32,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_stream_dimensions_parses_valid_input() {
    let dimensions = "80x24".parse::<StreamDimensions>().unwrap();

    assert_eq!(dimensions.width, 80);
    assert_eq!(dimensions.height, 24);
  }

  #[test]
  fn test_stream_dimensions_rejects_missing_separator() {
    let error = "8024".parse::<StreamDimensions>().unwrap_err();

    assert!(error.contains("Expected format"));
  }

  #[test]
  fn test_stream_dimensions_rejects_zero_dimension() {
    let error = "80x0".parse::<StreamDimensions>().unwrap_err();

    assert!(error.contains("greater than 0"));
  }

  #[test]
  fn test_stream_dimensions_rejects_oversized_dimension() {
    let error = "1001x24".parse::<StreamDimensions>().unwrap_err();

    assert!(error.contains("too large"));
  }

  #[test]
  fn test_cli_args_parse_stream_and_flags() {
    let args = CliArgs::try_parse_from([
      "chroma",
      "--stream",
      "64x32",
      "--no-status",
      "--fps",
      "30",
      "--stream-format",
      "cells",
      "--pattern",
      "waves",
      "--palette",
      "dots",
    ])
    .unwrap();

    let stream = args.stream.expect("expected parsed stream dimensions");

    assert_eq!(stream.width, 64);
    assert_eq!(stream.height, 32);
    assert_eq!(args.stream_format, StreamFormat::Cells);
    assert!(args.no_status);
    assert_eq!(args.fps, 30);
    assert_eq!(args.pattern.as_deref(), Some("waves"));
    assert_eq!(args.palette.as_deref(), Some("dots"));
  }

  #[test]
  fn test_cli_args_use_default_fps_when_not_provided() {
    let args = CliArgs::try_parse_from(["chroma"]).unwrap();

    assert_eq!(args.fps, 60);
  }

  #[test]
  fn test_cli_args_use_ansi_stream_format_by_default() {
    let args = CliArgs::try_parse_from(["chroma", "--stream", "64x32"]).unwrap();

    assert_eq!(args.stream_format, StreamFormat::Ansi);
  }

  #[test]
  fn test_cli_args_reject_invalid_stream_value() {
    let error = CliArgs::try_parse_from(["chroma", "--stream", "wide"]).unwrap_err();
    let error_text = error.to_string();

    assert!(error_text.contains("Expected format"));
  }

  #[test]
  fn test_cli_args_reject_invalid_stream_format() {
    let error = CliArgs::try_parse_from(["chroma", "--stream-format", "json"]).unwrap_err();
    let error_text = error.to_string();

    assert!(error_text.contains("Expected one of"));
  }
}
