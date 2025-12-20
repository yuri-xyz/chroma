//! Preset 0: Plasma with Simple palette and Chromatic colors

use crate::params::{ColorMode, PaletteType, PatternType, ShaderParams};

pub fn preset() -> ShaderParams {
  ShaderParams {
    frequency: 12.32,
    amplitude: 1.30,
    speed: 0.596,
    color_shift: 4.011,
    scale: 1.0,
    octaves: 4,
    noise_strength: 0.334,
    distort_amplitude: 0.2,
    noise_scale: 0.005,
    z_rate: 0.02,
    brightness: 1.099,
    contrast: 1.058,
    hue: 0.0,
    saturation: 1.0,
    gamma: 1.0,
    vignette: 0.3,
    vignette_softness: 0.5,
    glyph_sharpness: 1.0,
    palette: PaletteType::Simple,
    color_mode: ColorMode::Chromatic,
    pattern_type: PatternType::Plasma,
    audio_enabled: true,
    bass_influence: 0.5,
    mid_influence: 0.3,
    treble_influence: 0.2,
    ..ShaderParams::default()
  }
}
