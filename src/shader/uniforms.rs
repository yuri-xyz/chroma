use bytemuck::{Pod, Zeroable};

use crate::params::ShaderParams;

#[derive(Clone, Copy, Debug, Default)]
pub struct InteractionUniforms {
  pub gravity_offset: [f32; 2],
  pub mouse_position: [f32; 2],
  pub mouse_influence: f32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct ShaderUniforms {
  pub time: f32,
  _padding1: u32,
  pub resolution: [f32; 2],

  pub frequency: f32,
  pub amplitude: f32,
  pub speed: f32,
  pub color_shift: f32,
  pub scale: f32,
  pub octaves: u32,

  pub noise_strength: f32,
  pub distort_amplitude: f32,
  pub noise_scale: f32,
  pub z_rate: f32,

  pub brightness: f32,
  pub contrast: f32,
  pub hue: f32,
  pub saturation: f32,

  pub gamma: f32,
  pub vignette: f32,
  pub vignette_softness: f32,
  pub glyph_sharpness: f32,

  pub color_mode: u32,
  pub pattern_type: u32,
  pub effect_time: f32,
  pub effect_type: u32,

  pub beat_distortion_time: f32,
  pub beat_distortion_strength: f32,
  pub beat_zoom_strength: f32,
  pub gravity: f32,

  pub gravity_offset: [f32; 2],
  pub mouse_position: [f32; 2],

  pub mouse_influence: f32,
  // Align background_tint (vec3) to a 16-byte boundary.
  _padding2: u32,
  pub background_tint: [f32; 3],
  _padding3: u32,
}

impl ShaderUniforms {
  pub fn from_params(params: &ShaderParams) -> Self {
    Self::from_params_with_interaction(params, InteractionUniforms::default())
  }

  pub fn from_params_with_interaction(
    params: &ShaderParams,
    interaction: InteractionUniforms,
  ) -> Self {
    Self {
      time: params.time,
      _padding1: 0,
      resolution: [
        params.resolution_width as f32,
        params.resolution_height as f32,
      ],

      frequency: params.frequency,
      amplitude: params.amplitude,
      speed: params.speed,
      color_shift: params.color_shift,
      scale: params.scale,
      octaves: params.octaves,

      noise_strength: params.noise_strength,
      distort_amplitude: params.distort_amplitude,
      noise_scale: params.noise_scale,
      z_rate: params.z_rate,

      brightness: params.brightness,
      contrast: params.contrast,
      hue: params.hue,
      saturation: params.saturation,

      gamma: params.gamma,
      vignette: params.vignette,
      vignette_softness: params.vignette_softness,
      glyph_sharpness: params.glyph_sharpness,

      color_mode: params.color_mode.to_u32(),
      pattern_type: params.pattern_type.to_u32(),
      effect_time: params.effect_time,
      effect_type: params.effect_type,

      beat_distortion_time: params.beat_distortion_time,
      beat_distortion_strength: params.beat_distortion_strength,
      beat_zoom_strength: params.beat_zoom_strength,
      gravity: params.gravity,

      gravity_offset: interaction.gravity_offset,
      mouse_position: interaction.mouse_position,

      mouse_influence: interaction.mouse_influence,
      _padding2: 0,
      background_tint: [
        params.background_tint_r,
        params.background_tint_g,
        params.background_tint_b,
      ],
      _padding3: 0,
    }
  }
}

#[cfg(test)]
mod tests {
  use std::mem;

  use super::*;
  use crate::params::{ColorMode, PatternType, ShaderParams};

  #[test]
  fn test_uniforms_from_params() {
    let params = ShaderParams::default();
    let uniforms = ShaderUniforms::from_params(&params);

    assert_eq!(uniforms.time, 0.0);
    assert_eq!(uniforms.resolution[0], 80.0);
    assert_eq!(uniforms.resolution[1], 24.0);
    assert_eq!(uniforms.gravity, 0.0);
  }

  #[test]
  fn test_uniforms_from_params_maps_all_selected_fields() {
    let params = ShaderParams {
      time: 12.5,
      resolution_width: 132,
      resolution_height: 41,
      frequency: 3.5,
      amplitude: 1.25,
      speed: 0.75,
      color_shift: 1.2,
      scale: 2.2,
      octaves: 5,
      noise_strength: 0.45,
      distort_amplitude: 0.33,
      noise_scale: 1.7,
      z_rate: 0.2,
      brightness: 1.4,
      contrast: 0.9,
      hue: 0.6,
      saturation: 1.1,
      gamma: 1.8,
      vignette: 0.3,
      vignette_softness: 0.7,
      glyph_sharpness: 1.5,
      color_mode: ColorMode::Galaxy,
      pattern_type: PatternType::Tunnel,
      effect_time: 8.0,
      effect_type: 4,
      beat_distortion_time: 6.5,
      beat_distortion_strength: 0.95,
      beat_zoom_strength: 0.55,
      gravity: 0.8,
      mouse_fight: 0.7,
      background_tint_r: 0.1,
      background_tint_g: 0.2,
      background_tint_b: 0.3,
      ..ShaderParams::default()
    };

    let interaction = InteractionUniforms {
      gravity_offset: [0.15, 0.25],
      mouse_position: [0.3, 0.4],
      mouse_influence: 0.55,
    };
    let uniforms = ShaderUniforms::from_params_with_interaction(&params, interaction);

    assert_eq!(uniforms.time, 12.5);
    assert_eq!(uniforms.resolution, [132.0, 41.0]);
    assert_eq!(uniforms.frequency, 3.5);
    assert_eq!(uniforms.amplitude, 1.25);
    assert_eq!(uniforms.speed, 0.75);
    assert_eq!(uniforms.color_shift, 1.2);
    assert_eq!(uniforms.scale, 2.2);
    assert_eq!(uniforms.octaves, 5);
    assert_eq!(uniforms.noise_strength, 0.45);
    assert_eq!(uniforms.distort_amplitude, 0.33);
    assert_eq!(uniforms.noise_scale, 1.7);
    assert_eq!(uniforms.z_rate, 0.2);
    assert_eq!(uniforms.brightness, 1.4);
    assert_eq!(uniforms.contrast, 0.9);
    assert_eq!(uniforms.hue, 0.6);
    assert_eq!(uniforms.saturation, 1.1);
    assert_eq!(uniforms.gamma, 1.8);
    assert_eq!(uniforms.vignette, 0.3);
    assert_eq!(uniforms.vignette_softness, 0.7);
    assert_eq!(uniforms.glyph_sharpness, 1.5);
    assert_eq!(uniforms.color_mode, ColorMode::Galaxy.to_u32());
    assert_eq!(uniforms.pattern_type, PatternType::Tunnel.to_u32());
    assert_eq!(uniforms.effect_time, 8.0);
    assert_eq!(uniforms.effect_type, 4);
    assert_eq!(uniforms.beat_distortion_time, 6.5);
    assert_eq!(uniforms.beat_distortion_strength, 0.95);
    assert_eq!(uniforms.beat_zoom_strength, 0.55);
    assert_eq!(uniforms.gravity, 0.8);
    assert_eq!(uniforms.gravity_offset, [0.15, 0.25]);
    assert_eq!(uniforms.mouse_position, [0.3, 0.4]);
    assert_eq!(uniforms.mouse_influence, 0.55);
    assert_eq!(uniforms.background_tint, [0.1, 0.2, 0.3]);
  }

  #[test]
  fn test_uniforms_layout_matches_expected_alignment() {
    assert_eq!(mem::size_of::<ShaderUniforms>(), 160);
    assert_eq!(mem::align_of::<ShaderUniforms>(), 4);
    assert_eq!(mem::size_of::<ShaderUniforms>() % 16, 0);
  }
}
