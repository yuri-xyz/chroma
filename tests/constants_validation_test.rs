use chroma::constants::*;

#[test]
fn test_default_fps_reasonable() {
  assert!(DEFAULT_FPS > 0, "Default FPS must be positive");
  assert!(
    DEFAULT_FPS <= 120,
    "Default FPS should be reasonable for terminal rendering"
  );
}

#[test]
fn test_min_brightness_threshold_valid_range() {
  assert!(
    MIN_BRIGHTNESS_THRESHOLD < 128,
    "Brightness threshold should be relatively low for dark pixels"
  );
}

#[cfg(feature = "audio")]
#[test]
fn test_audio_thresholds_normalized() {
  assert!(
    AUDIO_SILENCE_THRESHOLD >= 0.0 && AUDIO_SILENCE_THRESHOLD <= 1.0,
    "Silence threshold should be normalized"
  );
  assert!(
    AUDIO_SAMPLE_THRESHOLD >= 0.0 && AUDIO_SAMPLE_THRESHOLD <= 1.0,
    "Sample threshold should be normalized"
  );
}

#[cfg(feature = "audio")]
#[test]
fn test_audio_decay_rates_valid() {
  assert!(
    AUDIO_DECAY_RATE > 0.0 && AUDIO_DECAY_RATE < 1.0,
    "Decay rate should be between 0 and 1 for proper decay"
  );
  assert!(
    AUDIO_SPEED_DECAY_RATE > 0.0 && AUDIO_SPEED_DECAY_RATE < 1.0,
    "Speed decay rate should be between 0 and 1 for proper decay"
  );
}

#[test]
fn test_effect_names_not_empty() {
  for (index, name) in EFFECT_NAMES.iter().enumerate() {
    assert!(
      !name.is_empty(),
      "Effect name at index {} should not be empty",
      index
    );
  }
}

#[test]
fn test_effect_names_all_unique() {
  use std::collections::HashSet;

  let unique_names: HashSet<_> = EFFECT_NAMES.iter().collect();

  assert_eq!(
    unique_names.len(),
    EFFECT_NAMES.len(),
    "All effect names should be unique"
  );
}

#[test]
fn test_effect_names_reasonable_length() {
  for name in &EFFECT_NAMES {
    assert!(
      name.len() <= 20,
      "Effect name '{}' should be reasonably short for display",
      name
    );
  }
}

#[cfg(feature = "audio")]
#[test]
fn test_audio_decay_rate_slower_than_speed_decay() {
  assert!(
    AUDIO_SPEED_DECAY_RATE < AUDIO_DECAY_RATE,
    "Speed should decay faster than other audio parameters"
  );
}

#[test]
fn test_effect_names_contents() {
  assert_eq!(EFFECT_NAMES[0], "Circle");
  assert_eq!(EFFECT_NAMES[1], "Cross");
  assert_eq!(EFFECT_NAMES[2], "Diamond");
  assert_eq!(EFFECT_NAMES[3], "Star");
  assert_eq!(EFFECT_NAMES[4], "Grid");
  assert_eq!(EFFECT_NAMES[5], "Wave");
  assert_eq!(EFFECT_NAMES[6], "Octgrams");
}
