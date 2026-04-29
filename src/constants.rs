/// Default target frames per second for rendering (can be overridden via --fps CLI flag)
pub const DEFAULT_FPS: u32 = 60;

/// Minimum brightness threshold for rendering pixels (0-255)
pub const MIN_BRIGHTNESS_THRESHOLD: u8 = 30;

/// Audio silence detection threshold (0.0-1.0)
pub const AUDIO_SILENCE_THRESHOLD: f32 = 0.02;

/// Audio sample detection threshold for "has sound" check
pub const AUDIO_SAMPLE_THRESHOLD: f32 = 0.02;

/// Decay rate for audio parameters when silent (0.0-1.0)
pub const AUDIO_DECAY_RATE: f32 = 0.92;

/// Speed decay rate when audio is silent (0.0-1.0)
pub const AUDIO_SPEED_DECAY_RATE: f32 = 0.88;

/// Effect names for status bar display
pub const EFFECT_NAMES: [&str; 7] = [
  "Circle", "Cross", "Diamond", "Star", "Grid", "Wave", "Octgrams",
];
