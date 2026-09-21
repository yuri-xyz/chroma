//! Timed cycling through the built-in presets (`--preset-interval`).

use rand::{Rng, RngExt};

use super::preset_count;

/// How the next preset is chosen when the interval elapses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PresetOrder {
  /// Walk the presets in numeric order, wrapping after the last one.
  Sequential,
  /// Jump to a random preset that differs from the current one.
  Random,
}

/// Tracks elapsed time and decides which preset comes next.
#[derive(Debug, Clone)]
pub struct PresetCycle {
  interval_seconds: f32,
  elapsed_seconds: f32,
  order: PresetOrder,
  current_index: u32,
}

impl PresetCycle {
  pub fn new(interval_seconds: f32, order: PresetOrder, start_index: u32) -> Self {
    Self {
      interval_seconds,
      elapsed_seconds: 0.0,
      order,
      current_index: start_index % preset_count() as u32,
    }
  }

  /// Advance the timer. Returns the next preset index once the interval has elapsed.
  pub fn tick(&mut self, delta_time: f32) -> Option<u32> {
    self.tick_with_rng(delta_time, &mut rand::rng())
  }

  fn tick_with_rng(&mut self, delta_time: f32, rng: &mut impl Rng) -> Option<u32> {
    self.elapsed_seconds += delta_time;

    if self.elapsed_seconds < self.interval_seconds {
      return None;
    }

    // A long stall (suspended process, blocked stream consumer) still advances one preset.
    self.elapsed_seconds = 0.0;
    self.current_index = next_index(self.current_index, self.order, rng);

    Some(self.current_index)
  }
}

fn next_index(current: u32, order: PresetOrder, rng: &mut impl Rng) -> u32 {
  let count = preset_count() as u32;

  match order {
    PresetOrder::Sequential => (current + 1) % count,
    // Offsetting by 1..count lands on every preset except the current one.
    PresetOrder::Random => (current + rng.random_range(1..count)) % count,
  }
}

#[cfg(test)]
mod tests {
  use rand::SeedableRng;

  use super::*;

  #[test]
  fn test_tick_waits_for_full_interval() {
    let mut cycle = PresetCycle::new(30.0, PresetOrder::Sequential, 0);

    assert_eq!(cycle.tick(29.9), None);
    assert_eq!(cycle.tick(0.2), Some(1));
    assert_eq!(cycle.tick(29.9), None);
  }

  #[test]
  fn test_sequential_order_wraps_after_last_preset() {
    let last = preset_count() as u32 - 1;
    let mut cycle = PresetCycle::new(1.0, PresetOrder::Sequential, last);

    assert_eq!(cycle.tick(1.0), Some(0));
    assert_eq!(cycle.tick(1.0), Some(1));
  }

  #[test]
  fn test_start_index_wraps_like_get_preset() {
    let mut cycle = PresetCycle::new(1.0, PresetOrder::Sequential, preset_count() as u32 + 2);

    assert_eq!(cycle.tick(1.0), Some(3));
  }

  #[test]
  fn test_random_order_never_repeats_current_preset() {
    let mut rng = rand::rngs::StdRng::seed_from_u64(42);
    let mut cycle = PresetCycle::new(1.0, PresetOrder::Random, 5);
    let mut previous = 5;

    for _ in 0..500 {
      let next = cycle
        .tick_with_rng(1.0, &mut rng)
        .expect("interval elapsed");

      assert_ne!(next, previous);
      assert!((next as usize) < preset_count());
      previous = next;
    }
  }

  #[test]
  fn test_long_stall_advances_a_single_preset() {
    let mut cycle = PresetCycle::new(10.0, PresetOrder::Sequential, 0);

    assert_eq!(cycle.tick(95.0), Some(1));
    assert_eq!(cycle.tick(1.0), None);
  }
}
