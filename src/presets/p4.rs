//! Preset 4: Waves with Dots palette and Chromatic colors

use crate::params::{ColorMode, PaletteType, PatternType, ShaderParams};

pub fn preset() -> ShaderParams {
    ShaderParams {
        frequency: 15.94,
        amplitude: 1.52,
        speed: 0.364,
        color_shift: 6.03,
        scale: 2.717,
        octaves: 3,
        noise_strength: 0.077,
        distort_amplitude: 1.102,
        noise_scale: 0.002,
        z_rate: 0.025,
        brightness: 0.921,
        contrast: 1.234,
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
        effect_type: 4,
        ..ShaderParams::default()
    }
}
