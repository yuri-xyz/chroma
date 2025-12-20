//! Preset 20: Plasma with Dots palette and Monochrome colors (beat reactive)

use crate::params::{ColorMode, PaletteType, PatternType, ShaderParams};

pub fn preset() -> ShaderParams {
  ShaderParams {
    frequency: 12.12,
    amplitude: 1.40,
    speed: 0.923,
    color_shift: 4.362,
    scale: 2.255,
    octaves: 6,
    noise_strength: 0.176,
    distort_amplitude: 0.083,
    noise_scale: 0.005,
    z_rate: 0.021,
    brightness: 1.767,
    contrast: 1.380,
    hue: 0.0,
    saturation: 1.279,
    gamma: 0.989,
    vignette: 0.442,
    vignette_softness: 0.324,
    glyph_sharpness: 0.974,
    palette: PaletteType::Dots,
    color_mode: ColorMode::Monochrome,
    pattern_type: PatternType::Plasma,
    audio_enabled: true,
    bass_influence: 0.577,
    mid_influence: 0.482,
    treble_influence: 0.361,
    beat_distortion_strength: 0.6,
    beat_zoom_strength: 0.5,
    effect_type: 2,
    ..ShaderParams::default()
  }
}
