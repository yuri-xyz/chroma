//! Preset 13: Plasma with Mixed palette and Neon colors

use crate::params::{ColorMode, PaletteType, PatternType, ShaderParams};

pub fn preset() -> ShaderParams {
    ShaderParams {
        frequency: 6.52,
        amplitude: 1.75,
        speed: 0.996,
        color_shift: 5.826,
        scale: 0.873,
        octaves: 3,
        noise_strength: 0.135,
        distort_amplitude: 0.720,
        noise_scale: 0.004,
        z_rate: 0.019,
        brightness: 1.465,
        contrast: 1.528,
        hue: 0.0,
        saturation: 1.493,
        gamma: 0.840,
        vignette: 0.161,
        vignette_softness: 0.712,
        glyph_sharpness: 0.735,
        palette: PaletteType::Mixed,
        color_mode: ColorMode::Neon,
        pattern_type: PatternType::Plasma,
        audio_enabled: true,
        bass_influence: 0.658,
        mid_influence: 0.296,
        treble_influence: 0.335,
        effect_type: 5,
        ..ShaderParams::default()
    }
}
