use chroma::params::{ColorMode, PatternType, ShaderParams};
use chroma::shader::ShaderUniforms;

#[test]
fn test_uniforms_all_basic_fields_mapped() {
  let mut params = ShaderParams::default();

  params.time = 5.5;
  params.frequency = 12.5;
  params.amplitude = 1.5;
  params.speed = 0.7;
  params.color_shift = 2.5;
  params.scale = 2.0;
  params.octaves = 6;

  let uniforms = ShaderUniforms::from_params(&params);

  assert_eq!(uniforms.time, 5.5);
  assert_eq!(uniforms.frequency, 12.5);
  assert_eq!(uniforms.amplitude, 1.5);
  assert_eq!(uniforms.speed, 0.7);
  assert_eq!(uniforms.color_shift, 2.5);
  assert_eq!(uniforms.scale, 2.0);
  assert_eq!(uniforms.octaves, 6);
}

#[test]
fn test_uniforms_resolution_conversion() {
  let mut params = ShaderParams::default();

  params.set_resolution(1920, 1080);

  let uniforms = ShaderUniforms::from_params(&params);

  assert_eq!(uniforms.resolution[0], 1920.0);
  assert_eq!(uniforms.resolution[1], 1080.0);
}

#[test]
fn test_uniforms_noise_parameters_mapped() {
  let mut params = ShaderParams::default();

  params.noise_strength = 0.25;
  params.distort_amplitude = 1.2;
  params.noise_scale = 0.008;
  params.z_rate = 0.05;

  let uniforms = ShaderUniforms::from_params(&params);

  assert_eq!(uniforms.noise_strength, 0.25);
  assert_eq!(uniforms.distort_amplitude, 1.2);
  assert_eq!(uniforms.noise_scale, 0.008);
  assert_eq!(uniforms.z_rate, 0.05);
}

#[test]
fn test_uniforms_color_correction_mapped() {
  let mut params = ShaderParams::default();

  params.brightness = 1.5;
  params.contrast = 0.9;
  params.hue = 180.0;
  params.saturation = 1.3;
  params.gamma = 1.1;

  let uniforms = ShaderUniforms::from_params(&params);

  assert_eq!(uniforms.brightness, 1.5);
  assert_eq!(uniforms.contrast, 0.9);
  assert_eq!(uniforms.hue, 180.0);
  assert_eq!(uniforms.saturation, 1.3);
  assert_eq!(uniforms.gamma, 1.1);
}

#[test]
fn test_uniforms_effect_parameters_mapped() {
  let mut params = ShaderParams::default();

  params.vignette = 0.4;
  params.vignette_softness = 0.7;
  params.glyph_sharpness = 1.2;

  let uniforms = ShaderUniforms::from_params(&params);

  assert_eq!(uniforms.vignette, 0.4);
  assert_eq!(uniforms.vignette_softness, 0.7);
  assert_eq!(uniforms.glyph_sharpness, 1.2);
}

#[test]
fn test_uniforms_background_tint_mapped() {
  let mut params = ShaderParams::default();

  params.background_tint_r = 0.1;
  params.background_tint_g = 0.2;
  params.background_tint_b = 0.3;

  let uniforms = ShaderUniforms::from_params(&params);

  assert_eq!(uniforms.background_tint[0], 0.1);
  assert_eq!(uniforms.background_tint[1], 0.2);
  assert_eq!(uniforms.background_tint[2], 0.3);
}

#[test]
fn test_uniforms_color_mode_to_u32() {
  let mut params = ShaderParams::default();

  params.color_mode = ColorMode::Rainbow;

  let uniforms = ShaderUniforms::from_params(&params);

  assert_eq!(uniforms.color_mode, 0);

  params.color_mode = ColorMode::Neon;

  let uniforms = ShaderUniforms::from_params(&params);

  assert_eq!(uniforms.color_mode, 5);
}

#[test]
fn test_uniforms_pattern_type_to_u32() {
  let mut params = ShaderParams::default();

  params.pattern_type = PatternType::Plasma;

  let uniforms = ShaderUniforms::from_params(&params);

  assert_eq!(uniforms.pattern_type, 0);

  params.pattern_type = PatternType::Fractal;

  let uniforms = ShaderUniforms::from_params(&params);

  assert_eq!(uniforms.pattern_type, 10);
}

#[test]
fn test_uniforms_effect_time_and_type_mapped() {
  let mut params = ShaderParams::default();

  params.effect_time = 42.5;
  params.effect_type = 3;

  let uniforms = ShaderUniforms::from_params(&params);

  assert_eq!(uniforms.effect_time, 42.5);
  assert_eq!(uniforms.effect_type, 3);
}

#[test]
fn test_uniforms_from_default_params() {
  let params = ShaderParams::default();
  let uniforms = ShaderUniforms::from_params(&params);

  assert_eq!(uniforms.time, 0.0);
  assert_eq!(uniforms.frequency, 10.0);
  assert_eq!(uniforms.amplitude, 1.0);
  assert_eq!(uniforms.brightness, 1.2);
  assert_eq!(uniforms.contrast, 1.0);
}

#[test]
fn test_uniforms_from_audio_reactive_defaults() {
  let params = ShaderParams::with_audio_reactive_defaults();
  let uniforms = ShaderUniforms::from_params(&params);

  assert_eq!(uniforms.speed, 0.05);
  assert_eq!(uniforms.brightness, 0.6);
  assert_eq!(uniforms.contrast, 0.8);
  assert_eq!(uniforms.amplitude, 0.4);
  assert_eq!(uniforms.frequency, 6.0);
}

#[test]
fn test_uniforms_extreme_values() {
  let mut params = ShaderParams::default();

  params.frequency = 18.0;
  params.brightness = 2.0;
  params.contrast = 2.0;
  params.saturation = 2.0;

  let uniforms = ShaderUniforms::from_params(&params);

  assert_eq!(uniforms.frequency, 18.0);
  assert_eq!(uniforms.brightness, 2.0);
  assert_eq!(uniforms.contrast, 2.0);
  assert_eq!(uniforms.saturation, 2.0);
}

#[test]
fn test_uniforms_zero_values() {
  let mut params = ShaderParams::default();

  params.time = 0.0;
  params.amplitude = 0.0;
  params.brightness = 0.0;
  params.vignette = 0.0;

  let uniforms = ShaderUniforms::from_params(&params);

  assert_eq!(uniforms.time, 0.0);
  assert_eq!(uniforms.amplitude, 0.0);
  assert_eq!(uniforms.brightness, 0.0);
  assert_eq!(uniforms.vignette, 0.0);
}
