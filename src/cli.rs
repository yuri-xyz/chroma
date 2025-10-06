use clap::Parser;

/// Command-line arguments
#[derive(Parser, Debug)]
#[command(name = "chroma")]
#[command(
  about = "Terminal-based shader visualizer with optional audio reactivity",
  long_about = None
)]
pub struct CliArgs {
  /// Load configuration from a saved config file
  #[arg(short, long, value_name = "FILE")]
  pub config: Option<String>,

  /// Specify audio device name for capture (use --list-audio-devices to see available devices)
  #[cfg(feature = "audio")]
  #[arg(short, long, value_name = "DEVICE")]
  pub audio_device: Option<String>,

  /// List available audio devices and exit
  #[cfg(feature = "audio")]
  #[arg(long)]
  pub list_audio_devices: bool,

  // Visual parameters
  /// Set pattern frequency (3.0-18.0)
  #[arg(long)]
  pub frequency: Option<f32>,

  /// Set amplitude (0.0-2.0)
  #[arg(long)]
  pub amplitude: Option<f32>,

  /// Set animation speed (0.0-1.0)
  #[arg(long)]
  pub speed: Option<f32>,

  /// Set scale factor (0.1-5.0)
  #[arg(long)]
  pub scale: Option<f32>,

  /// Set brightness (0.0-2.0)
  #[arg(long)]
  pub brightness: Option<f32>,

  /// Set contrast (0.2-2.0)
  #[arg(long)]
  pub contrast: Option<f32>,

  /// Set saturation (0.0-2.0)
  #[arg(long)]
  pub saturation: Option<f32>,

  /// Set hue rotation (0.0-360.0)
  #[arg(long)]
  pub hue: Option<f32>,

  /// Set pattern type (plasma, waves, ripples, vortex, etc.)
  #[arg(long, value_name = "PATTERN")]
  pub pattern: Option<String>,

  /// Set color mode (rainbow, monochrome, duotone, warm, cool, neon, etc.)
  #[arg(long, value_name = "MODE")]
  pub color_mode: Option<String>,

  /// Set ASCII palette (standard, blocks, circles, smooth, braille, etc.)
  #[arg(long, value_name = "PALETTE")]
  pub palette: Option<String>,

  // Audio parameters
  #[cfg(feature = "audio")]
  /// Enable audio reactivity
  #[arg(long)]
  pub audio_enabled: Option<bool>,

  #[cfg(feature = "audio")]
  /// Set bass influence (0.0-1.0)
  #[arg(long)]
  pub bass_influence: Option<f32>,

  #[cfg(feature = "audio")]
  /// Set mid frequency influence (0.0-1.0)
  #[arg(long)]
  pub mid_influence: Option<f32>,

  #[cfg(feature = "audio")]
  /// Set treble influence (0.0-1.0)
  #[arg(long)]
  pub treble_influence: Option<f32>,

  #[cfg(feature = "audio")]
  /// Set beat distortion strength (0.0-2.0)
  #[arg(long)]
  pub beat_distortion: Option<f32>,

  #[cfg(feature = "audio")]
  /// Set beat zoom strength (0.0-2.0)
  #[arg(long)]
  pub beat_zoom: Option<f32>,

  // Distortion parameters
  /// Set noise strength (0.0-0.5)
  #[arg(long)]
  pub noise_strength: Option<f32>,

  /// Set distortion amplitude (0.0-2.0)
  #[arg(long)]
  pub distort_amplitude: Option<f32>,

  // Effects
  /// Set vignette strength (0.0-1.0)
  #[arg(long)]
  pub vignette: Option<f32>,
}
