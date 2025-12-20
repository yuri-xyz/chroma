//! Preset 9: Vortex with Braille palette and Chromatic colors

use crate::params::{ColorMode, PaletteType, PatternType, ShaderParams};

pub fn preset() -> ShaderParams {
  ShaderParams {
    frequency: 8.85,
    amplitude: 1.36,
    speed: 0.324,
    color_shift: 1.763,
    scale: 1.999,
    octaves: 4,
    noise_strength: 0.171,
    distort_amplitude: 0.162,
    noise_scale: 0.003,
    z_rate: 0.021,
    brightness: 1.500,
    contrast: 1.581,
    hue: 0.0,
    saturation: 1.357,
    gamma: 1.282,
    vignette: 0.208,
    vignette_softness: 0.478,
    glyph_sharpness: 0.837,
    palette: PaletteType::Braille,
    color_mode: ColorMode::Chromatic,
    pattern_type: PatternType::Vortex,
    audio_enabled: true,
    bass_influence: 0.466,
    mid_influence: 0.473,
    treble_influence: 0.292,
    effect_type: 6,
    ..ShaderParams::default()
  }
}
