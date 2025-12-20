//! Preset 24: Waves with Arrows palette and Chromatic colors (with background tint)

use crate::params::{ColorMode, PaletteType, PatternType, ShaderParams};

pub fn preset() -> ShaderParams {
  ShaderParams {
    frequency: 11.37,
    amplitude: 1.53,
    speed: 0.808,
    color_shift: 2.129,
    scale: 0.529,
    octaves: 4,
    noise_strength: 0.259,
    distort_amplitude: 0.395,
    noise_scale: 0.005,
    z_rate: 0.049,
    brightness: 2.069,
    contrast: 1.404,
    hue: 0.0,
    saturation: 1.2,
    gamma: 1.292,
    vignette: 0.481,
    vignette_softness: 0.449,
    glyph_sharpness: 1.401,
    background_tint_r: 0.092,
    background_tint_g: 0.108,
    background_tint_b: 0.056,
    terminal_bg_r: 0.133,
    terminal_bg_g: 0.133,
    terminal_bg_b: 0.133,
    palette: PaletteType::Arrows,
    color_mode: ColorMode::Chromatic,
    pattern_type: PatternType::Waves,
    audio_enabled: true,
    bass_influence: 0.652,
    mid_influence: 0.209,
    treble_influence: 0.119,
    beat_sensitivity: 1.093,
    beat_distortion_strength: 0.85,
    beat_zoom_strength: 0.7,
    effect_time: -100.0,
    effect_type: 5,
    ..ShaderParams::default()
  }
}
