//! Preset 17: Plasma with BoxDraw palette and Chromatic colors

use crate::params::{ColorMode, PaletteType, PatternType, ShaderParams};

pub fn preset() -> ShaderParams {
  ShaderParams {
    frequency: 8.87,
    amplitude: 1.14,
    speed: 0.507,
    color_shift: 5.873,
    scale: 1.767,
    octaves: 4,
    noise_strength: 0.164,
    distort_amplitude: 0.356,
    noise_scale: 0.002,
    z_rate: 0.030,
    brightness: 1.469,
    contrast: 0.974,
    hue: 0.0,
    saturation: 1.159,
    gamma: 0.833,
    vignette: 0.392,
    vignette_softness: 0.648,
    glyph_sharpness: 1.084,
    palette: PaletteType::BoxDraw,
    color_mode: ColorMode::Chromatic,
    pattern_type: PatternType::Plasma,
    audio_enabled: true,
    bass_influence: 0.362,
    mid_influence: 0.510,
    treble_influence: 0.258,
    effect_type: 3,
    ..ShaderParams::default()
  }
}
