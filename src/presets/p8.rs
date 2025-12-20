//! Preset 8: Waves with Circles palette and Warped colors

use crate::params::{ColorMode, PaletteType, PatternType, ShaderParams};

pub fn preset() -> ShaderParams {
  ShaderParams {
    frequency: 13.00,
    amplitude: 1.71,
    speed: 0.836,
    color_shift: 1.717,
    scale: 2.063,
    octaves: 3,
    noise_strength: 0.061,
    distort_amplitude: 1.108,
    noise_scale: 0.008,
    z_rate: 0.014,
    brightness: 1.021,
    contrast: 0.879,
    hue: 0.0,
    saturation: 0.881,
    gamma: 1.244,
    vignette: 0.307,
    vignette_softness: 0.676,
    glyph_sharpness: 1.156,
    background_tint_r: 0.205,
    background_tint_g: 0.258,
    background_tint_b: 0.082,
    palette: PaletteType::Circles,
    color_mode: ColorMode::Warped,
    pattern_type: PatternType::Waves,
    audio_enabled: true,
    bass_influence: 0.383,
    mid_influence: 0.528,
    treble_influence: 0.496,
    effect_type: 5,
    ..ShaderParams::default()
  }
}
