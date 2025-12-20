//! Preset 11: Truchet with Dots palette and Chromatic colors

use crate::params::{ColorMode, PaletteType, PatternType, ShaderParams};

pub fn preset() -> ShaderParams {
  ShaderParams {
    frequency: 3.53,
    amplitude: 0.55,
    speed: 0.872,
    color_shift: 4.828,
    scale: 0.663,
    octaves: 6,
    noise_strength: 0.143,
    distort_amplitude: 0.857,
    noise_scale: 0.003,
    z_rate: 0.029,
    brightness: 1.090,
    contrast: 0.767,
    hue: 0.0,
    saturation: 0.628,
    gamma: 0.912,
    vignette: 0.368,
    vignette_softness: 0.383,
    glyph_sharpness: 0.917,
    palette: PaletteType::Dots,
    color_mode: ColorMode::Chromatic,
    pattern_type: PatternType::Truchet,
    audio_enabled: true,
    bass_influence: 0.490,
    mid_influence: 0.550,
    treble_influence: 0.251,
    effect_type: 2,
    ..ShaderParams::default()
  }
}
