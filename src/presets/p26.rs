//! Preset 26: Borealis aurora curtains over a starlit sky
//!
//! Aurora colours on the Circles palette, whose soft rings read as glowing
//! light. Gamma lifts the mostly green and blue aurora colours into the middle
//! of the glyph range; brightness and contrast would not help because audio
//! reactivity rewrites them every frame. No vignette, so the curtains span the
//! whole sky; strong bass influence so the rays stretch upward on the beat, and
//! a softer beat ripple and zoom so the curtains sway rather than jolt.

use crate::params::{ColorMode, PaletteType, PatternType, ShaderParams};

pub fn preset() -> ShaderParams {
  ShaderParams {
    frequency: 7.0,
    amplitude: 1.0,
    speed: 0.35,
    noise_strength: 0.05,
    brightness: 1.3,
    contrast: 1.1,
    saturation: 1.2,
    gamma: 1.3,
    vignette: 0.0,
    palette: PaletteType::Circles,
    color_mode: ColorMode::Aurora,
    pattern_type: PatternType::Borealis,
    audio_enabled: true,
    bass_influence: 0.75,
    mid_influence: 0.35,
    treble_influence: 0.2,
    beat_distortion_strength: 0.5,
    beat_zoom_strength: 0.3,
    effect_type: 6,
    ..ShaderParams::default()
  }
}
