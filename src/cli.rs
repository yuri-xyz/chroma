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
}
