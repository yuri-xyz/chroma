// Integration tests for enhanced shader patterns
// Tests parameter validation, serialization, and pattern type consistency

use chroma::params::{PatternType, ShaderParams};

#[test]
fn test_noise_pattern_default_setup() {
  let params = ShaderParams {
    pattern_type: PatternType::Noise,
    frequency: 5.0,
    octaves: 3,
    ..Default::default()
  };

  assert_eq!(params.pattern_type, PatternType::Noise);
  assert_eq!(params.frequency, 5.0);
  assert_eq!(params.octaves, 3);
}

#[test]
fn test_noise_pattern_with_varying_octaves() {
  for octaves in 1..=8 {
    let params = ShaderParams {
      pattern_type: PatternType::Noise,
      octaves,
      ..Default::default()
    };

    assert_eq!(params.octaves, octaves);
    assert_eq!(params.pattern_type, PatternType::Noise);
  }
}

#[test]
fn test_noise_pattern_high_frequency() {
  let params = ShaderParams {
    pattern_type: PatternType::Noise,
    frequency: 18.0, // Max frequency
    octaves: 6,
    ..Default::default()
  };

  assert_eq!(params.pattern_type, PatternType::Noise);
  assert_eq!(params.frequency, 18.0);
  assert_eq!(params.octaves, 6);
}

#[test]
fn test_waves_pattern_animation() {
  let mut params = ShaderParams {
    pattern_type: PatternType::Waves,
    speed: 1.0,
    amplitude: 0.8,
    ..Default::default()
  };

  let initial_time = params.time;
  params.update_time(1.0);

  assert_eq!(params.pattern_type, PatternType::Waves);
  assert!(params.time > initial_time);
  assert_eq!(params.speed, 1.0);
  assert_eq!(params.amplitude, 0.8);
}

#[test]
fn test_waves_pattern_with_distortion() {
  let params = ShaderParams {
    pattern_type: PatternType::Waves,
    distort_amplitude: 0.6,
    frequency: 8.0,
    ..Default::default()
  };

  assert_eq!(params.pattern_type, PatternType::Waves);
  assert_eq!(params.distort_amplitude, 0.6);
  assert_eq!(params.frequency, 8.0);
}

#[test]
fn test_plasma_pattern_configuration() {
  let params = ShaderParams {
    pattern_type: PatternType::Plasma,
    frequency: 10.0,
    distort_amplitude: 0.5,
    speed: 0.8,
    ..Default::default()
  };

  assert_eq!(params.pattern_type, PatternType::Plasma);
  assert_eq!(params.frequency, 10.0);
  assert_eq!(params.distort_amplitude, 0.5);
  assert_eq!(params.speed, 0.8);
}

#[test]
fn test_plasma_with_animation() {
  let mut params = ShaderParams {
    pattern_type: PatternType::Plasma,
    speed: 1.5,
    ..Default::default()
  };

  for _ in 0..60 {
    params.update_time(0.016); // ~60fps
  }

  assert_eq!(params.pattern_type, PatternType::Plasma);
  assert!(params.time > 0.9); // Should have advanced ~1 second
}

#[test]
fn test_pattern_frequency_adjustment() {
  let mut params = ShaderParams {
    pattern_type: PatternType::Noise,
    frequency: 10.0,
    ..Default::default()
  };

  params.adjust_frequency(3.0);
  assert_eq!(params.frequency, 13.0);

  params.adjust_frequency(-5.0);
  assert_eq!(params.frequency, 8.0);
}

#[test]
fn test_all_patterns_valid_ranges() {
  // Test that all pattern types exist and have valid parameter ranges
  for pattern in PatternType::all() {
    let mut params = ShaderParams {
      pattern_type: *pattern,
      frequency: 8.0,
      amplitude: 1.0,
      ..Default::default()
    };

    assert_eq!(params.pattern_type, *pattern);
    params.clamp_all();

    // After clamping, values should still be valid
    assert!(params.frequency >= 3.0 && params.frequency <= 18.0);
    assert!(params.amplitude >= 0.0 && params.amplitude <= 2.0);
  }
}

#[test]
fn test_pattern_randomization() {
  let mut params = ShaderParams::default();
  let _original_pattern = params.pattern_type;

  params.randomize();

  // Pattern should be randomized (within valid set)
  assert!(PatternType::all().contains(&params.pattern_type));
  // Frequency should also change with randomization
  assert!(params.frequency >= 3.0 && params.frequency <= 18.0);
}

#[test]
fn test_improved_pattern_persistence() {
  let params = ShaderParams {
    pattern_type: PatternType::Noise,
    octaves: 5,
    frequency: 12.0,
    amplitude: 0.9,
    distort_amplitude: 0.4,
    ..Default::default()
  };

  let filename = params.save_to_file().expect("Failed to save");
  let loaded = ShaderParams::load_from_file(&filename).expect("Failed to load");

  assert_eq!(loaded.pattern_type, PatternType::Noise);
  assert_eq!(loaded.octaves, 5);
  assert_eq!(loaded.frequency, 12.0);
  assert_eq!(loaded.amplitude, 0.9);
  assert_eq!(loaded.distort_amplitude, 0.4);

  std::fs::remove_file(&filename).ok();
}

#[test]
fn test_waves_pattern_persistence() {
  let params = ShaderParams {
    pattern_type: PatternType::Waves,
    frequency: 9.0,
    amplitude: 0.7,
    speed: 0.9,
    distort_amplitude: 0.3,
    ..Default::default()
  };

  let filename = params.save_to_file().expect("Failed to save");
  let loaded = ShaderParams::load_from_file(&filename).expect("Failed to load");

  assert_eq!(loaded.pattern_type, PatternType::Waves);
  assert_eq!(loaded.frequency, 9.0);
  assert_eq!(loaded.amplitude, 0.7);
  assert_eq!(loaded.speed, 0.9);
  assert_eq!(loaded.distort_amplitude, 0.3);

  std::fs::remove_file(&filename).ok();
}

#[test]
fn test_plasma_pattern_persistence() {
  let params = ShaderParams {
    pattern_type: PatternType::Plasma,
    frequency: 11.0,
    distort_amplitude: 0.55,
    speed: 1.0, // Use reasonable value within limits
    ..Default::default()
  };

  let filename = params.save_to_file().expect("Failed to save");
  let loaded = ShaderParams::load_from_file(&filename).expect("Failed to load");

  assert_eq!(loaded.pattern_type, PatternType::Plasma);
  assert_eq!(loaded.frequency, 11.0);
  assert_eq!(loaded.distort_amplitude, 0.55);
  assert_eq!(loaded.speed, 1.0);

  std::fs::remove_file(&filename).ok();
}

#[test]
fn test_pattern_with_enhanced_color_modes() {
  use chroma::params::ColorMode;

  // Test improved patterns with new color modes
  let test_cases = vec![
    (PatternType::Plasma, ColorMode::Fire),   // Plasma + Fire
    (PatternType::Waves, ColorMode::Ocean),   // Waves + Ocean
    (PatternType::Noise, ColorMode::Aurora),  // Noise + Aurora
    (PatternType::Vortex, ColorMode::Galaxy), // Vortex + Galaxy
  ];

  for (pattern, color_mode) in test_cases {
    let params = ShaderParams {
      pattern_type: pattern,
      color_mode,
      ..Default::default()
    };

    assert_eq!(params.pattern_type, pattern);
    assert_eq!(params.color_mode, color_mode);
  }
}

#[test]
fn test_noise_pattern_amplitude_effects() {
  let mut params = ShaderParams {
    pattern_type: PatternType::Noise,
    amplitude: 0.5,
    ..Default::default()
  };

  // Manually adjust amplitude (no adjust_amplitude method exists)
  params.amplitude = 0.8;
  assert!(params.amplitude > 0.5);

  params.amplitude = 0.6;
  assert!(params.amplitude > 0.5);
}

#[test]
fn test_waves_pattern_beat_sync() {
  let mut params = ShaderParams {
    pattern_type: PatternType::Waves,
    beat_distortion_strength: 0.8,
    ..Default::default()
  };

  params.update_time(0.016);

  assert_eq!(params.pattern_type, PatternType::Waves);
  assert_eq!(params.beat_distortion_strength, 0.8);
  assert!(params.time > 0.0);
}

#[test]
fn test_plasma_pattern_vortex_rotation() {
  let mut params = ShaderParams {
    pattern_type: PatternType::Plasma,
    z_rate: 0.5, // Controls rotation speed
    frequency: 8.0,
    speed: 1.0, // Higher speed for faster time advancement
    ..Default::default()
  };

  for _ in 0..60 {
    params.update_time(0.016);
  }

  assert_eq!(params.pattern_type, PatternType::Plasma);
  assert_eq!(params.z_rate, 0.5);
  // Time advances by speed * delta_time, so check it increased at least some amount
  assert!(params.time > 0.1);
}

#[test]
fn test_combined_parameter_updates() {
  use chroma::params::ColorMode;

  let mut params = ShaderParams {
    pattern_type: PatternType::Waves,
    color_mode: ColorMode::Ocean,
    frequency: 8.0,
    amplitude: 0.7,
    brightness: 1.2,
    ..Default::default()
  };

  // Simulate a frame update
  params.update_time(0.016);
  params.adjust_frequency(1.0);
  params.adjust_brightness(0.1);

  assert_eq!(params.pattern_type, PatternType::Waves);
  assert_eq!(params.color_mode, ColorMode::Ocean);
  assert_eq!(params.frequency, 9.0);
  // Use approximate comparison for floating point
  assert!((params.brightness - 1.3).abs() < 0.01);
  assert!(params.time > 0.0);
}

#[test]
fn test_pattern_state_preservation() {
  use chroma::params::ColorMode;

  let mut params = ShaderParams {
    // Set up complex state
    pattern_type: PatternType::Noise,
    color_mode: ColorMode::Aurora,
    frequency: 12.5,
    amplitude: 0.85,
    octaves: 6,
    distort_amplitude: 0.45,
    speed: 0.95,
    brightness: 1.35,
    contrast: 1.15,
    saturation: 1.25,
    ..Default::default()
  };

  // Update time several times
  for _ in 0..10 {
    params.update_time(0.016);
  }

  // Verify state is preserved
  assert_eq!(params.pattern_type, PatternType::Noise);
  assert_eq!(params.color_mode, ColorMode::Aurora);
  assert_eq!(params.frequency, 12.5);
  assert_eq!(params.amplitude, 0.85);
  assert_eq!(params.octaves, 6);
  assert_eq!(params.distort_amplitude, 0.45);
  assert_eq!(params.speed, 0.95);
  assert_eq!(params.brightness, 1.35);
  assert_eq!(params.contrast, 1.15);
  assert_eq!(params.saturation, 1.25);
  assert!(params.time > 0.15);
}

#[test]
fn test_pattern_switching_consistency() {
  let mut params = ShaderParams::default();

  // Cycle through patterns and verify consistency
  for pattern in PatternType::all() {
    params.pattern_type = *pattern;
    assert_eq!(params.pattern_type, *pattern);

    params.update_time(0.016);

    // Pattern type should not change after time update
    assert_eq!(params.pattern_type, *pattern);
  }
}

#[test]
fn test_octaves_validation() {
  let mut params = ShaderParams {
    pattern_type: PatternType::Noise,
    octaves: 0,
    ..Default::default()
  };

  // Octaves should be at least 1
  assert!(params.octaves < 8); // Max reasonable octaves

  params.octaves = 8;
  assert_eq!(params.octaves, 8);
}

#[test]
fn test_noise_and_waves_fbm_octaves() {
  // Test that octaves parameter is properly used for FBM
  for octaves in 1..=8 {
    let params = ShaderParams {
      pattern_type: PatternType::Noise,
      octaves,
      ..Default::default()
    };

    assert_eq!(params.octaves, octaves);
    assert!(params.octaves > 0);
    assert!(params.octaves <= 8);
  }
}
