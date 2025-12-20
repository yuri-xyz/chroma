//! Preset 19: Plasma with Geometric palette and Cyberpunk colors

use crate::params::{ColorMode, PaletteType, PatternType, ShaderParams};

pub fn preset() -> ShaderParams {
    ShaderParams {
        frequency: 12.32,
        amplitude: 1.30,
        speed: 0.721,
        color_shift: 2.979,
        scale: 1.0,
        octaves: 4,
        noise_strength: 0.482,
        distort_amplitude: 0.2,
        noise_scale: 0.005,
        z_rate: 0.02,
        brightness: 1.285,
        contrast: 1.206,
        hue: 0.0,
        saturation: 1.0,
        gamma: 1.0,
        vignette: 0.3,
        vignette_softness: 0.5,
        glyph_sharpness: 1.0,
        palette: PaletteType::Geometric,
        color_mode: ColorMode::Cyberpunk,
        pattern_type: PatternType::Plasma,
        audio_enabled: true,
        bass_influence: 0.5,
        mid_influence: 0.3,
        treble_influence: 0.2,
        effect_time: 5.62,
        ..ShaderParams::default()
    }
}
