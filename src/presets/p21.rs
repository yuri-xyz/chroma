//! Preset 21: Waves with Dots palette and Chromatic colors (slow, beat reactive)

use crate::params::{ColorMode, PaletteType, PatternType, ShaderParams};

pub fn preset() -> ShaderParams {
  ShaderParams {
    frequency: 6.0,
    amplitude: 0.40,
    speed: 0.001,
    color_shift: 4.438,
    scale: 2.717,
    octaves: 3,
    noise_strength: 0.001,
    distort_amplitude: 0.001,
    noise_scale: 0.002,
    z_rate: 0.025,
    brightness: 0.6,
    contrast: 0.8,
    hue: 0.0,
    saturation: 1.148,
    gamma: 1.060,
    vignette: 0.499,
    vignette_softness: 0.714,
    glyph_sharpness: 1.409,
    palette: PaletteType::Dots,
    color_mode: ColorMode::Chromatic,
    pattern_type: PatternType::Waves,
    audio_enabled: true,
    bass_influence: 0.668,
    mid_influence: 0.572,
    treble_influence: 0.200,
    beat_distortion_strength: 0.6,
    beat_zoom_strength: 0.5,
    effect_time: 21.42,
    effect_type: 4,
    ..ShaderParams::default()
  }
}
