//! Built-in presets for chroma visualizer.
//!
//! Each preset is defined in its own file and returns a ShaderParams instance.
//! These presets are also available as TOML files in the examples/ folder.

mod cycle;
mod p0;
mod p1;
mod p10;
mod p11;
mod p12;
mod p13;
mod p14;
mod p15;
mod p16;
mod p17;
mod p18;
mod p19;
mod p2;
mod p20;
mod p21;
mod p22;
mod p23;
mod p24;
mod p25;
mod p3;
mod p4;
mod p5;
mod p6;
mod p7;
mod p8;
mod p9;

pub use cycle::{PresetCycle, PresetOrder};
use rand::{Rng, RngExt};

use crate::params::ShaderParams;

/// All preset functions
const PRESETS: &[fn() -> ShaderParams] = &[
  p0::preset,
  p1::preset,
  p2::preset,
  p3::preset,
  p4::preset,
  p5::preset,
  p6::preset,
  p7::preset,
  p8::preset,
  p9::preset,
  p10::preset,
  p11::preset,
  p12::preset,
  p13::preset,
  p14::preset,
  p15::preset,
  p16::preset,
  p17::preset,
  p18::preset,
  p19::preset,
  p20::preset,
  p21::preset,
  p22::preset,
  p23::preset,
  p24::preset,
  p25::preset,
];

/// Get a preset by index. Wraps around if index exceeds the number of presets.
pub fn get_preset(index: u32) -> ShaderParams {
  let wrapped = (index as usize) % PRESETS.len();
  PRESETS[wrapped]()
}

/// Pick a random preset index, for use with `get_preset`.
pub fn random_preset_index() -> u32 {
  random_preset_index_with_rng(&mut rand::rng())
}

fn random_preset_index_with_rng(rng: &mut impl Rng) -> u32 {
  rng.random_range(0..PRESETS.len()) as u32
}

/// Get the total number of available presets.
pub fn preset_count() -> usize {
  PRESETS.len()
}

#[cfg(test)]
mod tests {
  use rand::SeedableRng;

  use super::*;
  use crate::params::PatternType;

  fn get_random_preset_with_seed(seed: u64) -> ShaderParams {
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);

    get_preset(random_preset_index_with_rng(&mut rng))
  }

  #[test]
  fn test_preset_count() {
    assert_eq!(preset_count(), 26);
  }

  #[test]
  fn test_get_preset_valid_indices() {
    for i in 0..preset_count() as u32 {
      let preset = get_preset(i);
      // Just verify it doesn't panic and returns valid params
      assert!(
        preset.frequency > 0.0,
        "Preset {} should have positive frequency",
        i
      );
    }
  }

  #[test]
  fn test_get_preset_wraparound() {
    let count = preset_count() as u32;
    let preset_0 = get_preset(0);

    assert_eq!(
      preset_0.frequency,
      get_preset(count).frequency,
      "The first index past the end should wrap to preset 0"
    );
    assert_eq!(
      preset_0.frequency,
      get_preset(count * 2).frequency,
      "Wrapping should keep working past the second lap"
    );
  }

  #[test]
  fn test_full_screen_corner_vortex_only_drops_the_vignette() {
    let bubble = get_preset(18);
    let full_screen = get_preset(25);

    assert_eq!(full_screen.vignette, 0.0);
    assert!(bubble.vignette > 0.0);
    assert_eq!(full_screen.pattern_type, PatternType::VortexCorner);
    assert_eq!(full_screen.scale, bubble.scale);
    assert_eq!(full_screen.color_mode, bubble.color_mode);
    assert_eq!(full_screen.palette, bubble.palette);
  }

  #[test]
  fn test_get_random_preset_with_seed() {
    let preset_a = get_random_preset_with_seed(42);
    let preset_b = get_random_preset_with_seed(42);
    let preset_c = get_random_preset_with_seed(7);

    assert_eq!(preset_a.frequency, preset_b.frequency);
    assert_eq!(preset_a.pattern_type, preset_b.pattern_type);
    assert_eq!(preset_a.color_mode, preset_b.color_mode);
    assert!(
      preset_a.frequency != preset_c.frequency
        || preset_a.pattern_type != preset_c.pattern_type
        || preset_a.color_mode != preset_c.color_mode
    );
  }

  #[test]
  fn test_all_presets_produce_clamped_valid_ranges() {
    for i in 0..preset_count() {
      let preset = get_preset(i as u32);

      assert!((0.1..=20.0).contains(&preset.frequency));
      assert!((0.0..=2.0).contains(&preset.amplitude));
      assert!((0.0..=2.0).contains(&preset.speed));
      assert!((0.1..=5.0).contains(&preset.scale));
      assert!(preset.octaves >= 1);
      assert!(preset.octaves <= 8);
    }
  }
}
