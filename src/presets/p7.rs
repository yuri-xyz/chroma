//! Preset 7: Truchet with Triangles palette and Chromatic colors

use crate::params::{ColorMode, PaletteType, PatternType, ShaderParams};

pub fn preset() -> ShaderParams {
    ShaderParams {
        frequency: 11.85,
        amplitude: 1.30,
        speed: 0.473,
        color_shift: 4.950,
        scale: 1.706,
        octaves: 4,
        noise_strength: 0.088,
        distort_amplitude: 0.2,
        noise_scale: 0.006,
        z_rate: 0.034,
        brightness: 1.286,
        contrast: 0.903,
        hue: 308.47,
        saturation: 1.236,
        gamma: 1.093,
        vignette: 0.442,
        vignette_softness: 0.322,
        glyph_sharpness: 1.0,
        palette: PaletteType::Triangles,
        color_mode: ColorMode::Chromatic,
        pattern_type: PatternType::Truchet,
        audio_enabled: true,
        bass_influence: 0.5,
        mid_influence: 0.3,
        treble_influence: 0.2,
        effect_time: 49.12,
        ..ShaderParams::default()
    }
}
