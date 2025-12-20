//! Preset 12: Plasma with BoxDraw palette and Chromatic colors

use crate::params::{ColorMode, PaletteType, PatternType, ShaderParams};

pub fn preset() -> ShaderParams {
  ShaderParams {
    frequency: 5.28,
    amplitude: 0.90,
    speed: 0.283,
    color_shift: 2.472,
    scale: 2.619,
    octaves: 5,
    noise_strength: 0.278,
    distort_amplitude: 0.270,
    noise_scale: 0.007,
    z_rate: 0.046,
    brightness: 0.862,
    contrast: 1.404,
    hue: 0.0,
    saturation: 1.461,
    gamma: 1.242,
    vignette: 0.246,
    vignette_softness: 0.658,
    glyph_sharpness: 1.321,
    palette: PaletteType::BoxDraw,
    color_mode: ColorMode::Chromatic,
    pattern_type: PatternType::Plasma,
    audio_enabled: true,
    bass_influence: 0.732,
    mid_influence: 0.475,
    treble_influence: 0.244,
    effect_type: 6,
    ..ShaderParams::default()
  }
}
