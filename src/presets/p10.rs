//! Preset 10: Plasma with Circles palette and Monochrome colors

use crate::params::{ColorMode, PaletteType, PatternType, ShaderParams};

pub fn preset() -> ShaderParams {
    ShaderParams {
        frequency: 8.85,
        amplitude: 1.78,
        speed: 0.816,
        color_shift: 1.682,
        scale: 1.547,
        octaves: 3,
        noise_strength: 0.177,
        distort_amplitude: 0.703,
        noise_scale: 0.007,
        z_rate: 0.016,
        brightness: 1.575,
        contrast: 0.687,
        hue: 0.0,
        saturation: 0.626,
        gamma: 1.104,
        vignette: 0.0,
        vignette_softness: 0.558,
        glyph_sharpness: 1.044,
        palette: PaletteType::Circles,
        color_mode: ColorMode::Monochrome,
        pattern_type: PatternType::Plasma,
        audio_enabled: true,
        bass_influence: 0.477,
        mid_influence: 0.525,
        treble_influence: 0.142,
        effect_type: 2,
        ..ShaderParams::default()
    }
}
