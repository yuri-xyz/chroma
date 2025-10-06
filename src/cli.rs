use clap::Parser;

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
  #[cfg(feature = "audio")]
  #[arg(short = 'a', long, value_name = "DEVICE")]
  pub audio_device: Option<String>,

  /// List all available audio input devices and exit
  #[cfg(feature = "audio")]
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

  /// Start with randomized parameters (lowest priority, overridden by config and args)
  #[arg(short = 'r', long)]
  pub random: bool,

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

  /// Pattern type: plasma, waves, ripples, vortex, noise, geometric, voronoi, truchet, hexagonal, interference, fractal, glitch, spiral, rings, grid, diamonds, sphere, octgrams, warped
  #[arg(short = 'p', long, value_name = "PATTERN")]
  pub pattern: Option<String>,

  /// Color scheme: rainbow, monochrome, duotone, warm, cool, neon, pastel, cyberpunk, warped, chromatic
  #[arg(short = 'm', long, value_name = "MODE")]
  pub color_mode: Option<String>,

  /// ASCII character set: standard, blocks, circles, smooth, braille, geometric, mixed, dots, shades, lines, triangles, arrows, powerline, boxdraw, extended, simple
  #[arg(short = 'P', long, value_name = "PALETTE")]
  pub palette: Option<String>,

  // Audio parameters
  #[cfg(feature = "audio")]
  /// Enable or disable audio reactivity. Defaults to true when built with audio feature
  #[arg(short = 'e', long, value_name = "BOOL")]
  pub audio_enabled: Option<bool>,

  #[cfg(feature = "audio")]
  /// How much bass frequencies affect amplitude. Range: 0.0-1.0
  #[arg(short = 'B', long, value_name = "FLOAT")]
  pub bass_influence: Option<f32>,

  #[cfg(feature = "audio")]
  /// How much mid frequencies affect pattern frequency. Range: 0.0-1.0
  #[arg(short = 'M', long, value_name = "FLOAT")]
  pub mid_influence: Option<f32>,

  #[cfg(feature = "audio")]
  /// How much treble frequencies affect animation speed. Range: 0.0-1.0
  #[arg(short = 'T', long, value_name = "FLOAT")]
  pub treble_influence: Option<f32>,

  #[cfg(feature = "audio")]
  /// Beat-triggered distortion effect strength. Range: 0.0-2.0
  #[arg(short = 'D', long, value_name = "FLOAT")]
  pub beat_distortion: Option<f32>,

  #[cfg(feature = "audio")]
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
}
