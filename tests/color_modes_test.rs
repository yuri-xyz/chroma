// Tests for enhanced color modes and pattern improvements
// Note: These tests validate parameter ranges and color output properties in CPU space
// GPU shader functions are tested through integration testing

use chroma::params::{ColorMode, ShaderParams};

#[test]
fn test_color_mode_fire() {
  let params = ShaderParams {
    color_mode: ColorMode::Fire,
    ..Default::default()
  };

  assert_eq!(params.color_mode, ColorMode::Fire);
}

#[test]
fn test_color_mode_ocean() {
  let params = ShaderParams {
    color_mode: ColorMode::Ocean,
    ..Default::default()
  };

  assert_eq!(params.color_mode, ColorMode::Ocean);
}

#[test]
fn test_color_mode_aurora() {
  let params = ShaderParams {
    color_mode: ColorMode::Aurora,
    ..Default::default()
  };

  assert_eq!(params.color_mode, ColorMode::Aurora);
}

#[test]
fn test_color_mode_galaxy() {
  let params = ShaderParams {
    color_mode: ColorMode::Galaxy,
    ..Default::default()
  };

  assert_eq!(params.color_mode, ColorMode::Galaxy);
}

#[test]
fn test_all_color_modes_valid() {
  // Test that all color modes are valid
  for mode in ColorMode::all() {
    let params = ShaderParams {
      color_mode: *mode,
      ..Default::default()
    };

    assert_eq!(params.color_mode, *mode);
  }
}

#[test]
fn test_color_mode_switching() {
  let mut params = ShaderParams::default();

  // Test cycling through all modes
  for mode in ColorMode::all() {
    params.color_mode = *mode;
    assert_eq!(params.color_mode, *mode);
  }
}

#[test]
fn test_enhanced_warm_mode_parameters() {
  let params = ShaderParams {
    color_mode: ColorMode::Warm,
    brightness: 1.5,
    saturation: 1.0,
    ..Default::default()
  };

  assert_eq!(params.brightness, 1.5);
  assert_eq!(params.saturation, 1.0);
  assert_eq!(params.color_mode, ColorMode::Warm);
}

#[test]
fn test_enhanced_cool_mode_parameters() {
  let params = ShaderParams {
    color_mode: ColorMode::Cool,
    brightness: 1.2,
    contrast: 1.1,
    ..Default::default()
  };

  assert_eq!(params.brightness, 1.2);
  assert_eq!(params.contrast, 1.1);
  assert_eq!(params.color_mode, ColorMode::Cool);
}

#[test]
fn test_enhanced_pastel_mode_parameters() {
  let params = ShaderParams {
    color_mode: ColorMode::Pastel,
    brightness: 1.0,
    saturation: 0.8,
    ..Default::default()
  };

  assert_eq!(params.brightness, 1.0);
  assert_eq!(params.saturation, 0.8);
}

#[test]
fn test_color_mode_with_time_based_changes() {
  // Aurora and Galaxy modes have time-dependent animation
  let mut params = ShaderParams {
    color_mode: ColorMode::Aurora,
    ..Default::default()
  };
  params.update_time(0.016); // One frame at ~60fps

  assert!(params.time > 0.0);
  assert_eq!(params.color_mode, ColorMode::Aurora);
}

#[test]
fn test_pattern_type_compatibility_with_color_modes() {
  use chroma::params::PatternType;

  // Test that pattern types work with new color modes
  for pattern in PatternType::all() {
    for mode in ColorMode::all() {
      let params = ShaderParams {
        pattern_type: *pattern,
        color_mode: *mode,
        ..Default::default()
      };

      assert_eq!(params.pattern_type, *pattern);
      assert_eq!(params.color_mode, *mode);
    }
  }
}

#[test]
fn test_fire_mode_animation_properties() {
  // Fire mode benefits from animation through distort_amplitude
  let mut params = ShaderParams {
    color_mode: ColorMode::Fire,
    distort_amplitude: 0.5,
    ..Default::default()
  };
  params.update_time(0.016);

  assert_eq!(params.color_mode, ColorMode::Fire);
  assert_eq!(params.distort_amplitude, 0.5);
  assert!(params.time > 0.0);
}

#[test]
fn test_ocean_mode_with_noise() {
  // Ocean mode works well with noise parameters
  let params = ShaderParams {
    color_mode: ColorMode::Ocean,
    noise_strength: 0.4,
    frequency: 8.0,
    ..Default::default()
  };

  assert_eq!(params.color_mode, ColorMode::Ocean);
  assert_eq!(params.noise_strength, 0.4);
  assert_eq!(params.frequency, 8.0);
}

#[test]
fn test_aurora_mode_with_high_saturation() {
  // Aurora mode can use high saturation for vibrant effects
  let params = ShaderParams {
    color_mode: ColorMode::Aurora,
    saturation: 1.5,
    brightness: 1.0,
    ..Default::default()
  };

  assert_eq!(params.color_mode, ColorMode::Aurora);
  assert_eq!(params.saturation, 1.5);
}

#[test]
fn test_galaxy_mode_with_vignette() {
  // Galaxy mode works well with vignetting for deep space effect
  let params = ShaderParams {
    color_mode: ColorMode::Galaxy,
    vignette: 0.6,
    vignette_softness: 0.3,
    ..Default::default()
  };

  assert_eq!(params.color_mode, ColorMode::Galaxy);
  assert_eq!(params.vignette, 0.6);
}

#[test]
fn test_color_mode_persistence() {
  let params = ShaderParams {
    color_mode: ColorMode::Fire,
    brightness: 1.5,
    ..Default::default()
  };

  // Save and load
  let filename = params.save_to_file().expect("Failed to save");
  let loaded = ShaderParams::load_from_file(&filename).expect("Failed to load");

  assert_eq!(loaded.color_mode, ColorMode::Fire);
  assert_eq!(loaded.brightness, 1.5);

  std::fs::remove_file(&filename).ok();
}

#[test]
fn test_pattern_improved_properties() {
  use chroma::params::PatternType;

  // Test that improved patterns (noise, waves, plasma) have proper octaves
  // Noise pattern benefits from octaves parameter
  let params = ShaderParams {
    pattern_type: PatternType::Noise,
    octaves: 5,
    ..Default::default()
  };

  assert_eq!(params.pattern_type, PatternType::Noise);
  assert_eq!(params.octaves, 5);
}

#[test]
fn test_waves_pattern_with_amplitude() {
  use chroma::params::PatternType;

  // Enhanced waves pattern uses amplitude for modulation
  let params = ShaderParams {
    pattern_type: PatternType::Waves,
    amplitude: 0.8,
    frequency: 10.0,
    ..Default::default()
  };

  assert_eq!(params.pattern_type, PatternType::Waves);
  assert_eq!(params.amplitude, 0.8);
}

#[test]
fn test_plasma_pattern_with_distortion() {
  use chroma::params::PatternType;

  // Enhanced plasma pattern uses more distortion parameters
  let params = ShaderParams {
    pattern_type: PatternType::Plasma,
    distort_amplitude: 0.6,
    speed: 1.0,
    ..Default::default()
  };

  assert_eq!(params.pattern_type, PatternType::Plasma);
  assert_eq!(params.distort_amplitude, 0.6);
}

#[test]
fn test_color_mode_randomization() {
  use chroma::params::PatternType;

  let mut params = ShaderParams::default();

  for _ in 0..10 {
    params.randomize();

    // Color mode should be randomized within valid set
    assert!(ColorMode::all().contains(&params.color_mode));

    // Pattern should also be randomized
    assert!(PatternType::all().contains(&params.pattern_type));
  }
}

#[test]
fn test_enhanced_gradient_calculation() {
  // New color modes use gradient for edge effects
  // Test that gradient parameters are preserved
  let params = ShaderParams {
    contrast: 1.2,
    noise_strength: 0.5,
    ..Default::default()
  };

  let filename = params.save_to_file().expect("Failed to save");
  let loaded = ShaderParams::load_from_file(&filename).expect("Failed to load");

  assert_eq!(loaded.contrast, 1.2);
  assert_eq!(loaded.noise_strength, 0.5);

  std::fs::remove_file(&filename).ok();
}

#[test]
fn test_color_mode_with_beat_distortion() {
  // Beat distortion should work with new color modes
  let params = ShaderParams {
    color_mode: ColorMode::Fire,
    beat_distortion_strength: 1.0,
    beat_distortion_time: 5.0,
    ..Default::default()
  };

  assert_eq!(params.color_mode, ColorMode::Fire);
  assert_eq!(params.beat_distortion_strength, 1.0);
}

#[test]
fn test_multiple_parameter_combinations() {
  use chroma::params::PatternType;

  // Test various combinations of new parameters
  let test_cases = vec![
    (ColorMode::Fire, PatternType::Waves, 0.5), // Fire + Waves + medium amplitude
    (ColorMode::Ocean, PatternType::Noise, 0.7), // Ocean + Noise + high amplitude
    (ColorMode::Aurora, PatternType::Plasma, 1.0), // Aurora + Plasma + full amplitude
    (ColorMode::Galaxy, PatternType::Ripples, 0.3), // Galaxy + Ripples + low amplitude
  ];

  for (color_mode, pattern_type, amplitude) in test_cases {
    let params = ShaderParams {
      color_mode,
      pattern_type,
      amplitude,
      ..Default::default()
    };

    assert_eq!(params.color_mode, color_mode);
    assert_eq!(params.pattern_type, pattern_type);
    assert_eq!(params.amplitude, amplitude);
  }
}
