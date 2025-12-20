//! Preset 5: Octgrams with Braille palette and Monochrome colors

use crate::params::{ColorMode, PaletteType, PatternType, ShaderParams};

pub fn preset() -> ShaderParams {
    ShaderParams {
        frequency: 10.0,
        amplitude: 1.0,
        speed: 0.5,
        color_shift: 0.0,
        scale: 1.0,
        octaves: 4,
        noise_strength: 0.15,
        distort_amplitude: 0.5,
        noise_scale: 0.005,
        z_rate: 0.02,
        brightness: 1.2,
        contrast: 1.0,
        hue: 0.0,
        saturation: 1.0,
        gamma: 1.0,
        vignette: 0.0,
        vignette_softness: 0.5,
        glyph_sharpness: 1.0,
        palette: PaletteType::Braille,
        color_mode: ColorMode::Monochrome,
        pattern_type: PatternType::Octgrams,
        audio_enabled: true,
        bass_influence: 0.5,
        mid_influence: 0.3,
        treble_influence: 0.2,
        ..ShaderParams::default()
    }
}
