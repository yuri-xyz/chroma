pub mod analyzer;
pub mod capture;
pub mod device_selector;

pub use analyzer::AudioAnalyzer;
pub use capture::AudioCapture;

#[derive(Debug, Clone, Copy)]
pub struct AudioFeatures {
  pub bass: f32,          // 20-250 Hz
  pub mid: f32,           // 250-2000 Hz
  pub treble: f32,        // 2000-20000 Hz
  pub overall: f32,       // Overall volume
  pub beat_strength: f32, // Beat detection
  pub is_drop: bool,      // Bass drop detected
}

impl Default for AudioFeatures {
  fn default() -> Self {
    Self {
      bass: 0.0,
      mid: 0.0,
      treble: 0.0,
      overall: 0.0,
      beat_strength: 0.0,
      is_drop: false,
    }
  }
}
