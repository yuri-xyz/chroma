//! Preset 1: Waves with Circles palette and Warped colors

use crate::params::{ColorMode, PaletteType, PatternType, ShaderParams};

pub fn preset() -> ShaderParams {
  ShaderParams {
    frequency: 13.57,
    amplitude: 1.21,
    speed: 0.959,
    color_shift: 5.70,
    scale: 0.593,
    octaves: 6,
    noise_strength: 0.135,
    distort_amplitude: 1.441,
    noise_scale: 0.003,
    z_rate: 0.021,
    brightness: 1.241,
    contrast: 0.736,
    hue: 0.0,
    saturation: 1.146,
    gamma: 0.943,
    vignette: 0.241,
    vignette_softness: 0.567,
    glyph_sharpness: 1.488,
    palette: PaletteType::Circles,
    color_mode: ColorMode::Warped,
    pattern_type: PatternType::Waves,
    audio_enabled: true,
    bass_influence: 0.774,
    mid_influence: 0.304,
    treble_influence: 0.460,
    effect_type: 2,
    ..ShaderParams::default()
  }
}
