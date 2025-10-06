use bytemuck::{Pod, Zeroable};

use crate::params::ShaderParams;

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
  _padding2: [u32; 3], // Need 12 bytes padding to align vec3 to 16-byte boundary
  pub background_tint: [f32; 3],
  _padding3: u32,
}

impl ShaderUniforms {
  pub fn from_params(params: &ShaderParams) -> Self {
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
      _padding2: [0; 3],
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
  use super::*;

  #[test]
  fn test_uniforms_from_params() {
    let params = ShaderParams::default();
    let uniforms = ShaderUniforms::from_params(&params);

    assert_eq!(uniforms.time, 0.0);
    assert_eq!(uniforms.resolution[0], 80.0);
    assert_eq!(uniforms.resolution[1], 24.0);
  }
}
