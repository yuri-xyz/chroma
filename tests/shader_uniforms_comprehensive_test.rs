use chroma::params::{ColorMode, PatternType, ShaderParams};
use chroma::shader::ShaderUniforms;

#[test]
fn test_uniforms_all_basic_fields_mapped() {
  let params = ShaderParams {
    time: 5.5,
    frequency: 12.5,
    amplitude: 1.5,
    speed: 0.7,
    color_shift: 2.5,
    scale: 2.0,
    octaves: 6,
    ..Default::default()
  };

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
  let params = ShaderParams {
    noise_strength: 0.25,
    distort_amplitude: 1.2,
    noise_scale: 0.008,
    z_rate: 0.05,
    ..Default::default()
  };

  let uniforms = ShaderUniforms::from_params(&params);

  assert_eq!(uniforms.noise_strength, 0.25);
  assert_eq!(uniforms.distort_amplitude, 1.2);
  assert_eq!(uniforms.noise_scale, 0.008);
  assert_eq!(uniforms.z_rate, 0.05);
}

#[test]
fn test_uniforms_color_correction_mapped() {
  let params = ShaderParams {
    brightness: 1.5,
    contrast: 0.9,
    hue: 180.0,
    saturation: 1.3,
    gamma: 1.1,
    ..Default::default()
  };

  let uniforms = ShaderUniforms::from_params(&params);

  assert_eq!(uniforms.brightness, 1.5);
  assert_eq!(uniforms.contrast, 0.9);
  assert_eq!(uniforms.hue, 180.0);
  assert_eq!(uniforms.saturation, 1.3);
  assert_eq!(uniforms.gamma, 1.1);
}

#[test]
fn test_uniforms_effect_parameters_mapped() {
  let params = ShaderParams {
    vignette: 0.4,
    vignette_softness: 0.7,
    glyph_sharpness: 1.2,
    ..Default::default()
  };

  let uniforms = ShaderUniforms::from_params(&params);

  assert_eq!(uniforms.vignette, 0.4);
  assert_eq!(uniforms.vignette_softness, 0.7);
  assert_eq!(uniforms.glyph_sharpness, 1.2);
}

#[test]
fn test_uniforms_background_tint_mapped() {
  let params = ShaderParams {
    background_tint_r: 0.1,
    background_tint_g: 0.2,
    background_tint_b: 0.3,
    ..Default::default()
  };

  let uniforms = ShaderUniforms::from_params(&params);

  assert_eq!(uniforms.background_tint[0], 0.1);
  assert_eq!(uniforms.background_tint[1], 0.2);
  assert_eq!(uniforms.background_tint[2], 0.3);
}

#[test]
fn test_uniforms_color_mode_to_u32() {
  let rainbow_params = ShaderParams {
    color_mode: ColorMode::Rainbow,
    ..Default::default()
  };
  let uniforms = ShaderUniforms::from_params(&rainbow_params);

  assert_eq!(uniforms.color_mode, 0);

  let neon_params = ShaderParams {
    color_mode: ColorMode::Neon,
    ..Default::default()
  };
  let uniforms = ShaderUniforms::from_params(&neon_params);

  assert_eq!(uniforms.color_mode, 5);
}

#[test]
fn test_uniforms_pattern_type_to_u32() {
  let plasma_params = ShaderParams {
    pattern_type: PatternType::Plasma,
    ..Default::default()
  };
  let uniforms = ShaderUniforms::from_params(&plasma_params);

  assert_eq!(uniforms.pattern_type, 0);

  let fractal_params = ShaderParams {
    pattern_type: PatternType::Fractal,
    ..Default::default()
  };
  let uniforms = ShaderUniforms::from_params(&fractal_params);

  assert_eq!(uniforms.pattern_type, 10);
}

#[test]
fn test_uniforms_effect_time_and_type_mapped() {
  let params = ShaderParams {
    effect_time: 42.5,
    effect_type: 3,
    ..Default::default()
  };

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
  let params = ShaderParams {
    frequency: 18.0,
    brightness: 2.0,
    contrast: 2.0,
    saturation: 2.0,
    ..Default::default()
  };

  let uniforms = ShaderUniforms::from_params(&params);

  assert_eq!(uniforms.frequency, 18.0);
  assert_eq!(uniforms.brightness, 2.0);
  assert_eq!(uniforms.contrast, 2.0);
  assert_eq!(uniforms.saturation, 2.0);
}

#[test]
fn test_uniforms_zero_values() {
  let params = ShaderParams {
    time: 0.0,
    amplitude: 0.0,
    brightness: 0.0,
    vignette: 0.0,
    ..Default::default()
  };

  let uniforms = ShaderUniforms::from_params(&params);

  assert_eq!(uniforms.time, 0.0);
  assert_eq!(uniforms.amplitude, 0.0);
  assert_eq!(uniforms.brightness, 0.0);
  assert_eq!(uniforms.vignette, 0.0);
}
