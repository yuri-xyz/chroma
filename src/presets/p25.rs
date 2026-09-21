//! Preset 25: Preset 18's corner vortex filling the whole screen
//!
//! Preset 18 puts the eye of the vortex near the bottom-right corner, where its
//! vignette dims it and fades the arms into an oval. This keeps everything else
//! and drops the vignette, so the arms run edge to edge.

use crate::params::ShaderParams;

pub fn preset() -> ShaderParams {
  ShaderParams {
    vignette: 0.0,
    ..super::p18::preset()
  }
}
