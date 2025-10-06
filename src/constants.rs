use std::time::Duration;

/// Target frames per second for rendering
pub const TARGET_FPS: u32 = 30;

/// Duration of each frame at target FPS
pub const FRAME_DURATION: Duration = Duration::from_millis(1000 / TARGET_FPS as u64);

/// Minimum brightness threshold for rendering pixels (0-255)
pub const MIN_BRIGHTNESS_THRESHOLD: u8 = 30;

/// Audio silence detection threshold (0.0-1.0)
#[allow(dead_code)]
pub const AUDIO_SILENCE_THRESHOLD: f32 = 0.02;

/// Audio sample detection threshold for "has sound" check
#[allow(dead_code)]
pub const AUDIO_SAMPLE_THRESHOLD: f32 = 0.02;

/// Decay rate for audio parameters when silent (0.0-1.0)
#[allow(dead_code)]
pub const AUDIO_DECAY_RATE: f32 = 0.92;

/// Speed decay rate when audio is silent (0.0-1.0)
#[allow(dead_code)]
pub const AUDIO_SPEED_DECAY_RATE: f32 = 0.88;

/// Number of effect types available
#[allow(dead_code)]
pub const NUM_EFFECT_TYPES: u32 = 7;

/// Effect names for status bar display
pub const EFFECT_NAMES: [&str; 7] = [
  "Circle", "Cross", "Diamond", "Star", "Grid", "Wave", "Octgrams",
];
