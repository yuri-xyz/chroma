//! Preset 22: Waves with Simple palette and Chromatic colors (high beat sensitivity)

use crate::params::{ColorMode, PaletteType, PatternType, ShaderParams};

pub fn preset() -> ShaderParams {
  ShaderParams {
    frequency: 18.14,
    amplitude: 1.63,
    speed: 0.572,
    color_shift: 4.857,
    scale: 1.160,
    octaves: 3,
    noise_strength: 0.055,
    distort_amplitude: 0.475,
    noise_scale: 0.007,
    z_rate: 0.021,
    brightness: 1.327,
    contrast: 0.969,
    hue: 0.0,
    saturation: 1.2,
    gamma: 0.990,
    vignette: 0.441,
    vignette_softness: 0.753,
    glyph_sharpness: 1.178,
    palette: PaletteType::Simple,
    color_mode: ColorMode::Chromatic,
    pattern_type: PatternType::Waves,
    audio_enabled: true,
    bass_influence: 0.742,
    mid_influence: 0.595,
    treble_influence: 0.477,
    beat_sensitivity: 1.315,
    beat_distortion_strength: 0.85,
    beat_zoom_strength: 0.7,
    effect_time: 3.69,
    effect_type: 4,
    ..ShaderParams::default()
  }
}
