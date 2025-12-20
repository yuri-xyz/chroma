//! Preset 23: Waves with Powerline palette and Chromatic colors (high beat sensitivity)

use crate::params::{ColorMode, PaletteType, PatternType, ShaderParams};

pub fn preset() -> ShaderParams {
    ShaderParams {
        frequency: 18.08,
        amplitude: 1.63,
        speed: 0.686,
        color_shift: 5.460,
        scale: 1.160,
        octaves: 3,
        noise_strength: 0.055,
        distort_amplitude: 0.472,
        noise_scale: 0.007,
        z_rate: 0.021,
        brightness: 1.459,
        contrast: 1.078,
        hue: 0.0,
        saturation: 1.2,
        gamma: 0.990,
        vignette: 0.441,
        vignette_softness: 0.753,
        glyph_sharpness: 1.178,
        palette: PaletteType::Powerline,
        color_mode: ColorMode::Chromatic,
        pattern_type: PatternType::Waves,
        audio_enabled: true,
        bass_influence: 0.742,
        mid_influence: 0.595,
        treble_influence: 0.477,
        beat_sensitivity: 1.315,
        beat_distortion_strength: 0.85,
        beat_zoom_strength: 0.7,
        effect_time: 3.69,
        effect_type: 4,
        ..ShaderParams::default()
    }
}
