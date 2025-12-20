//! Preset 3: Vortex with Simple palette and Neon colors

use crate::params::{ColorMode, PaletteType, PatternType, ShaderParams};

pub fn preset() -> ShaderParams {
    ShaderParams {
        frequency: 3.33,
        amplitude: 1.93,
        speed: 0.713,
        color_shift: 1.003,
        scale: 3.0,
        octaves: 4,
        noise_strength: 0.015,
        distort_amplitude: 0.388,
        noise_scale: 0.007,
        z_rate: 0.017,
        brightness: 0.929,
        contrast: 0.801,
        hue: 0.0,
        saturation: 0.605,
        gamma: 0.901,
        vignette: 0.119,
        vignette_softness: 0.738,
        glyph_sharpness: 1.034,
        palette: PaletteType::Simple,
        color_mode: ColorMode::Neon,
        pattern_type: PatternType::Vortex,
        audio_enabled: true,
        bass_influence: 0.575,
        mid_influence: 0.272,
        treble_influence: 0.268,
        effect_type: 5,
        ..ShaderParams::default()
    }
}
