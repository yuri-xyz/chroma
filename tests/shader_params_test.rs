use chroma::params::ShaderParams;

#[test]
fn test_default_params() {
  let params = ShaderParams::default();

  assert_eq!(params.time, 0.0);
  assert_eq!(params.frequency, 10.0);
  assert_eq!(params.amplitude, 1.0);
  assert_eq!(params.brightness, 1.2);
  assert_eq!(params.contrast, 1.0);
}

#[test]
fn test_update_time() {
  let mut params = ShaderParams {
    speed: 2.0,
    ..Default::default()
  };

  params.update_time(1.0);

  assert_eq!(params.time, 2.0);
}

#[test]
fn test_set_resolution() {
  let mut params = ShaderParams::default();

  params.set_resolution(100, 50);

  assert_eq!(params.resolution_width, 100);
  assert_eq!(params.resolution_height, 50);
}

#[test]
fn test_randomize() {
  let mut params = ShaderParams::default();
  let original_frequency = params.frequency;

  params.randomize();

  assert!(params.frequency >= 3.0 && params.frequency <= 18.0);
  assert!(params.brightness >= 0.8 && params.brightness <= 1.8);
  assert!(params.contrast >= 0.5 && params.contrast <= 1.8);
  assert!(params.hue >= 0.0 && params.hue < 360.0);

  assert_ne!(params.frequency, original_frequency);
}

#[test]
fn test_clamp_all() {
  let mut params = ShaderParams {
    frequency: 100.0,
    brightness: 10.0,
    hue: 400.0,
    ..Default::default()
  };

  params.clamp_all();

  assert!(params.frequency <= 18.0);
  assert!(params.brightness <= 2.0);
  assert!(params.hue < 360.0);
}

#[test]
fn test_audio_reactive_defaults() {
  let params = ShaderParams::with_audio_reactive_defaults();

  assert!(params.audio_enabled);
  assert_eq!(params.speed, 0.05);
  assert_eq!(params.brightness, 0.6);
  assert_eq!(params.contrast, 0.8);
  assert_eq!(params.amplitude, 0.4);
  assert_eq!(params.frequency, 6.0);
  assert_eq!(params.beat_distortion_time, -100.0);
  assert_eq!(params.beat_distortion_strength, 0.8);
  assert_eq!(params.beat_zoom_strength, 0.0);
}

#[test]
fn test_beat_distortion_parameters_default() {
  let params = ShaderParams::default();

  assert_eq!(params.beat_distortion_time, -100.0);
  assert_eq!(params.beat_distortion_strength, 0.6);
  assert_eq!(params.beat_zoom_strength, 0.5);
}

#[test]
fn test_beat_distortion_parameters_persistence() {
  let mut params = ShaderParams::default();

  params.beat_distortion_time = 5.0;
  params.beat_distortion_strength = 1.2;
  params.beat_zoom_strength = 0.8;

  assert_eq!(params.beat_distortion_time, 5.0);
  assert_eq!(params.beat_distortion_strength, 1.2);
  assert_eq!(params.beat_zoom_strength, 0.8);
}

#[test]
fn test_apply_audio_data() {
  let mut params = ShaderParams {
    audio_enabled: true,
    bass_influence: 0.5,
    mid_influence: 0.3,
    treble_influence: 0.2,
    ..Default::default()
  };

  params.apply_audio_data(0.8, 0.6, 0.4);

  // Bass affects amplitude
  assert_eq!(params.amplitude, 1.0 + 0.8 * 0.5);
  // Mid affects color_shift
  assert_eq!(params.color_shift, 0.6 * 0.3);
  // Treble affects frequency
  assert_eq!(params.frequency, 1.0 + 0.4 * 0.2);
}

#[test]
fn test_apply_audio_data_disabled() {
  let mut params = ShaderParams {
    audio_enabled: false,
    amplitude: 1.0,
    color_shift: 0.0,
    frequency: 10.0,
    ..Default::default()
  };

  params.apply_audio_data(0.8, 0.6, 0.4);

  // Should not change when audio disabled
  assert_eq!(params.amplitude, 1.0);
  assert_eq!(params.color_shift, 0.0);
  assert_eq!(params.frequency, 10.0);
}

#[test]
fn test_adjust_frequency() {
  let mut params = ShaderParams::default();

  params.adjust_frequency(5.0);
  assert_eq!(params.frequency, 15.0);

  // Test clamping at upper bound
  params.adjust_frequency(10.0);
  assert_eq!(params.frequency, 18.0); // Clamped to max

  // Test clamping at lower bound
  params.adjust_frequency(-20.0);
  assert_eq!(params.frequency, 3.0); // Clamped to min
}

#[test]
fn test_adjust_brightness() {
  let mut params = ShaderParams::default();

  params.adjust_brightness(0.5);
  assert_eq!(params.brightness, 1.7);

  // Test clamping at upper bound
  params.adjust_brightness(1.0);
  assert_eq!(params.brightness, 2.0); // Clamped to max

  // Test clamping at lower bound
  params.adjust_brightness(-5.0);
  assert_eq!(params.brightness, 0.0); // Clamped to min
}

#[test]
fn test_adjust_contrast() {
  let mut params = ShaderParams::default();

  params.adjust_contrast(0.3);
  assert_eq!(params.contrast, 1.3);

  // Test clamping
  params.adjust_contrast(-2.0);
  assert_eq!(params.contrast, 0.2); // Clamped to min
}

#[test]
fn test_adjust_saturation() {
  let mut params = ShaderParams::default();

  params.adjust_saturation(0.5);
  assert_eq!(params.saturation, 1.5);

  // Test clamping
  params.adjust_saturation(1.0);
  assert_eq!(params.saturation, 2.0); // Clamped to max
}

#[test]
fn test_adjust_hue_wrapping() {
  let mut params = ShaderParams::default();

  params.adjust_hue(100.0);
  assert_eq!(params.hue, 100.0);

  // Test wrapping at 360
  params.adjust_hue(300.0);
  assert_eq!(params.hue, 40.0); // 400 % 360 = 40

  // Test negative wrapping
  params.adjust_hue(-50.0);
  assert_eq!(params.hue, 350.0); // Wraps around
}

#[test]
fn test_file_persistence() {
  let mut params = ShaderParams::default();
  params.frequency = 15.0;
  params.brightness = 1.5;
  params.beat_distortion_strength = 0.9;

  // Save to file
  let filename = params.save_to_file().expect("Failed to save");

  // Load from file
  let loaded = ShaderParams::load_from_file(&filename).expect("Failed to load");

  assert_eq!(loaded.frequency, 15.0);
  assert_eq!(loaded.brightness, 1.5);
  assert_eq!(loaded.beat_distortion_strength, 0.9);

  // Cleanup
  std::fs::remove_file(&filename).ok();
}
