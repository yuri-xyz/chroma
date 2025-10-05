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
}
