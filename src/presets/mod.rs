//! Built-in presets for chroma visualizer.
//!
//! Each preset is defined in its own file and returns a ShaderParams instance.
//! These presets are also available as TOML files in the examples/ folder.

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
mod p3;
mod p4;
mod p5;
mod p6;
mod p7;
mod p8;
mod p9;

use rand::Rng;
#[cfg(test)]
use rand::SeedableRng;

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
];

/// Get a preset by index. Wraps around if index exceeds the number of presets.
pub fn get_preset(index: u32) -> ShaderParams {
  let wrapped = (index as usize) % PRESETS.len();
  PRESETS[wrapped]()
}

/// Get a random preset.
pub fn get_random_preset() -> ShaderParams {
  let index = rand::thread_rng().gen_range(0..PRESETS.len());

  PRESETS[index]()
}

#[cfg(test)]
fn get_random_preset_with_seed(seed: u64) -> ShaderParams {
  let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
  let index = rng.gen_range(0..PRESETS.len());

  PRESETS[index]()
}

/// Get the total number of available presets.
pub fn preset_count() -> usize {
  PRESETS.len()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_preset_count() {
    assert_eq!(preset_count(), 25);
  }

  #[test]
  fn test_get_preset_valid_indices() {
    for i in 0..25 {
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
    let preset_0 = get_preset(0);
    let preset_25 = get_preset(25);
    let preset_50 = get_preset(50);

    assert_eq!(
      preset_0.frequency, preset_25.frequency,
      "Preset 25 should wrap to preset 0"
    );
    assert_eq!(
      preset_0.frequency, preset_50.frequency,
      "Preset 50 should wrap to preset 0"
    );
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
