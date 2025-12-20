//! Preset 14: Plasma with Braille palette and Chromatic colors

use crate::params::{ColorMode, PaletteType, PatternType, ShaderParams};

pub fn preset() -> ShaderParams {
    ShaderParams {
        frequency: 12.32,
        amplitude: 1.30,
        speed: 0.446,
        color_shift: 0.714,
        scale: 1.0,
        octaves: 4,
        noise_strength: 0.417,
        distort_amplitude: 0.2,
        noise_scale: 0.005,
        z_rate: 0.02,
        brightness: 0.988,
        contrast: 0.886,
        hue: 0.0,
        saturation: 1.0,
        gamma: 1.0,
        vignette: 0.0,
        vignette_softness: 0.5,
        glyph_sharpness: 1.0,
        palette: PaletteType::Braille,
        color_mode: ColorMode::Chromatic,
        pattern_type: PatternType::Plasma,
        audio_enabled: true,
        bass_influence: 0.5,
        mid_influence: 0.3,
        treble_influence: 0.2,
        ..ShaderParams::default()
    }
}
