use chroma::params::ShaderParams;

#[test]
fn test_adjust_frequency_within_bounds() {
  let mut params = ShaderParams::default();

  params.adjust_frequency(2.0);

  assert_eq!(params.frequency, 12.0);

  params.adjust_frequency(-5.0);

  assert_eq!(params.frequency, 7.0);
}

#[test]
fn test_adjust_frequency_clamps_at_maximum() {
  let mut params = ShaderParams::default();

  params.adjust_frequency(100.0);

  assert_eq!(params.frequency, 18.0);
}

#[test]
fn test_adjust_frequency_clamps_at_minimum() {
  let mut params = ShaderParams::default();

  params.adjust_frequency(-100.0);

  assert_eq!(params.frequency, 3.0);
}

#[test]
fn test_adjust_brightness_within_bounds() {
  let mut params = ShaderParams::default();

  params.adjust_brightness(0.3);

  assert_eq!(params.brightness, 1.5);

  params.adjust_brightness(-0.5);

  assert_eq!(params.brightness, 1.0);
}

#[test]
fn test_adjust_brightness_clamps_at_maximum() {
  let mut params = ShaderParams::default();

  params.adjust_brightness(100.0);

  assert_eq!(params.brightness, 2.0);
}

#[test]
fn test_adjust_brightness_clamps_at_minimum() {
  let mut params = ShaderParams::default();

  params.adjust_brightness(-100.0);

  assert_eq!(params.brightness, 0.0);
}

#[test]
fn test_adjust_contrast_within_bounds() {
  let mut params = ShaderParams::default();

  params.adjust_contrast(0.3);

  assert!((params.contrast - 1.3).abs() < 0.001);

  params.adjust_contrast(-0.5);

  assert!((params.contrast - 0.8).abs() < 0.001);
}

#[test]
fn test_adjust_contrast_clamps_at_maximum() {
  let mut params = ShaderParams::default();

  params.adjust_contrast(100.0);

  assert_eq!(params.contrast, 2.0);
}

#[test]
fn test_adjust_contrast_clamps_at_minimum() {
  let mut params = ShaderParams::default();

  params.adjust_contrast(-100.0);

  assert_eq!(params.contrast, 0.2);
}

#[test]
fn test_adjust_saturation_within_bounds() {
  let mut params = ShaderParams::default();

  params.adjust_saturation(0.3);

  assert!((params.saturation - 1.3).abs() < 0.001);

  params.adjust_saturation(-0.5);

  assert!((params.saturation - 0.8).abs() < 0.001);
}

#[test]
fn test_adjust_saturation_clamps_at_maximum() {
  let mut params = ShaderParams::default();

  params.adjust_saturation(100.0);

  assert_eq!(params.saturation, 2.0);
}

#[test]
fn test_adjust_saturation_clamps_at_minimum() {
  let mut params = ShaderParams::default();

  params.adjust_saturation(-100.0);

  assert_eq!(params.saturation, 0.0);
}

#[test]
fn test_adjust_hue_wraps_around() {
  let mut params = ShaderParams::default();

  params.adjust_hue(180.0);

  assert_eq!(params.hue, 180.0);

  params.adjust_hue(200.0);

  assert_eq!(params.hue, 20.0);
}

#[test]
fn test_adjust_hue_wraps_negative() {
  let mut params = ShaderParams::default();

  params.adjust_hue(-50.0);

  assert_eq!(params.hue, 310.0);
}

#[test]
fn test_adjust_hue_multiple_wraps() {
  let mut params = ShaderParams::default();

  params.adjust_hue(720.0);

  assert_eq!(params.hue, 0.0);

  params.adjust_hue(-720.0);

  assert_eq!(params.hue, 0.0);
}

#[test]
fn test_apply_audio_data_when_enabled() {
  let mut params = ShaderParams {
    audio_enabled: true,
    bass_influence: 0.5,
    mid_influence: 0.3,
    treble_influence: 0.2,
    ..Default::default()
  };

  let original_amplitude = params.amplitude;
  let original_color_shift = params.color_shift;
  let original_frequency = params.frequency;

  params.apply_audio_data(0.8, 0.6, 0.4);

  assert_eq!(params.amplitude, 1.0 + 0.8 * 0.5);
  assert_eq!(params.color_shift, 0.6 * 0.3);
  assert_eq!(params.frequency, 1.0 + 0.4 * 0.2);

  assert_ne!(params.amplitude, original_amplitude);
  assert_ne!(params.color_shift, original_color_shift);
  assert_ne!(params.frequency, original_frequency);
}

#[test]
fn test_apply_audio_data_when_disabled() {
  let mut params = ShaderParams {
    audio_enabled: false,
    ..Default::default()
  };

  let original_amplitude = params.amplitude;
  let original_color_shift = params.color_shift;
  let original_frequency = params.frequency;

  params.apply_audio_data(0.8, 0.6, 0.4);

  assert_eq!(params.amplitude, original_amplitude);
  assert_eq!(params.color_shift, original_color_shift);
  assert_eq!(params.frequency, original_frequency);
}

#[test]
fn test_apply_audio_data_zero_values() {
  let mut params = ShaderParams {
    audio_enabled: true,
    bass_influence: 0.5,
    mid_influence: 0.3,
    treble_influence: 0.2,
    ..Default::default()
  };

  params.apply_audio_data(0.0, 0.0, 0.0);

  assert_eq!(params.amplitude, 1.0);
  assert_eq!(params.color_shift, 0.0);
  assert_eq!(params.frequency, 1.0);
}

#[test]
fn test_with_audio_reactive_defaults() {
  let params = ShaderParams::with_audio_reactive_defaults();

  assert_eq!(params.speed, 0.05);
  assert_eq!(params.brightness, 0.6);
  assert_eq!(params.contrast, 0.8);
  assert_eq!(params.amplitude, 0.4);
  assert_eq!(params.frequency, 6.0);
  assert!(params.audio_enabled);
  assert_eq!(params.effect_time, -100.0);
}

#[test]
fn test_with_audio_reactive_defaults_differs_from_default() {
  let default_params = ShaderParams::default();
  let audio_params = ShaderParams::with_audio_reactive_defaults();

  assert_ne!(default_params.speed, audio_params.speed);
  assert_ne!(default_params.brightness, audio_params.brightness);
  assert_ne!(default_params.contrast, audio_params.contrast);
  assert_ne!(default_params.amplitude, audio_params.amplitude);
  assert_ne!(default_params.frequency, audio_params.frequency);

  #[cfg(not(feature = "audio"))]
  assert_ne!(default_params.audio_enabled, audio_params.audio_enabled);
}
