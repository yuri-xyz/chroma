//! Preset 18: Vortex with Dots palette and Chromatic colors

use crate::params::{ColorMode, PaletteType, PatternType, ShaderParams};

pub fn preset() -> ShaderParams {
    ShaderParams {
        frequency: 4.26,
        amplitude: 0.55,
        speed: 0.462,
        color_shift: 4.019,
        scale: 0.563,
        octaves: 4,
        noise_strength: 0.166,
        distort_amplitude: 0.193,
        noise_scale: 0.005,
        z_rate: 0.023,
        brightness: 1.697,
        contrast: 1.154,
        hue: 0.0,
        saturation: 1.440,
        gamma: 1.180,
        vignette: 0.192,
        vignette_softness: 0.506,
        glyph_sharpness: 1.402,
        palette: PaletteType::Dots,
        color_mode: ColorMode::Chromatic,
        pattern_type: PatternType::Vortex,
        audio_enabled: true,
        bass_influence: 0.441,
        mid_influence: 0.238,
        treble_influence: 0.125,
        effect_type: 4,
        ..ShaderParams::default()
    }
}
