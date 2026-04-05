use rand::Rng;

use super::{PaletteType, PatternType, ShaderParams};

/// Probability of applying vignette effect during randomization
const VIGNETTE_PROBABILITY: f64 = 0.3;
/// Probability of applying background tint during randomization
const BACKGROUND_TINT_PROBABILITY: f64 = 0.2;

/// Weighted pattern selection for randomization
/// Favors visually appealing patterns, reduces less interesting ones
const PATTERN_WEIGHTS: &[(PatternType, u32)] = &[
  // Visually rich patterns get higher weight
  (PatternType::Plasma, 3),
  (PatternType::Waves, 3),
  (PatternType::Ripples, 3),
  (PatternType::Vortex, 3),
  (PatternType::Geometric, 2),
  (PatternType::Voronoi, 2),
  (PatternType::Truchet, 2),
  (PatternType::Hexagonal, 2),
  (PatternType::Interference, 2),
  (PatternType::Fractal, 2),
  (PatternType::Glitch, 1),
  (PatternType::Spiral, 2),
  (PatternType::Rings, 2),
  (PatternType::Grid, 1),
  (PatternType::Diamonds, 2),
  (PatternType::Sphere, 2),
  (PatternType::Octgrams, 1),
  (PatternType::WarpedFbm, 2),
  (PatternType::Kaleidoscope, 2),
  (PatternType::Tunnel, 2),
  (PatternType::Metaballs, 2),
  (PatternType::World, 2),
  (PatternType::Fluid, 2),
  // Simpler patterns get lower weight
  (PatternType::Noise, 1),
];

/// Weighted palette selection for randomization
const PALETTE_WEIGHTS: &[(PaletteType, u32)] = &[
  (PaletteType::Circles, 4),
  (PaletteType::Braille, 3),
  (PaletteType::Dots, 3),
  (PaletteType::Lines, 2),
  (PaletteType::Triangles, 2),
  (PaletteType::Arrows, 1),
  (PaletteType::Powerline, 1),
  (PaletteType::BoxDraw, 1),
  (PaletteType::Extended, 1),
  (PaletteType::Mixed, 1),
];

fn select_weighted<T: Copy>(weights: &[(T, u32)], rng: &mut impl Rng) -> T {
  debug_assert!(!weights.is_empty(), "weights slice must not be empty");

  let total: u32 = weights.iter().map(|(_, w)| w).sum();
  let mut choice = rng.gen_range(0..total);

  for (item, weight) in weights {
    if choice < *weight {
      return *item;
    }
    choice -= weight;
  }

  // Fallback (should not reach due to algorithm correctness)
  weights[0].0
}

/// Randomize shader parameters with weighted selection for patterns and palettes
pub fn randomize(params: &mut ShaderParams) {
  let mut rng = rand::thread_rng();

  randomize_with_rng(params, &mut rng);
}

pub fn randomize_with_rng(params: &mut ShaderParams, rng: &mut impl Rng) {
  params.pattern_type = select_weighted(PATTERN_WEIGHTS, rng);
  params.palette = select_weighted(PALETTE_WEIGHTS, rng);

  params.effect_type = rng.gen_range(2..=6);

  params.frequency = rng.gen_range(3.0..=18.0);
  params.amplitude = rng.gen_range(0.5..=2.0);
  params.speed = rng.gen_range(0.1..=1.0);
  params.scale = rng.gen_range(0.5..=3.0);
  params.color_shift = rng.gen_range(0.0..=std::f32::consts::TAU);
  params.octaves = rng.gen_range(2..=6);

  params.noise_strength = rng.gen_range(0.0..=0.3);
  params.distort_amplitude = rng.gen_range(0.0..=1.5);
  params.noise_scale = rng.gen_range(0.001..=0.008);
  params.z_rate = rng.gen_range(0.01..=0.05);

  params.brightness = rng.gen_range(0.8..=1.8);
  params.contrast = rng.gen_range(0.5..=1.8);
  params.saturation = rng.gen_range(0.6..=1.5);
  params.gamma = rng.gen_range(0.8..=1.3);

  params.vignette = if rng.gen_bool(VIGNETTE_PROBABILITY) {
    rng.gen_range(0.1..=0.5)
  } else {
    0.0
  };

  params.vignette_softness = rng.gen_range(0.3..=0.8);
  params.glyph_sharpness = rng.gen_range(0.7..=1.5);

  if rng.gen_bool(BACKGROUND_TINT_PROBABILITY) {
    params.background_tint_r = rng.gen_range(0.0..=0.3);
    params.background_tint_g = rng.gen_range(0.0..=0.3);
    params.background_tint_b = rng.gen_range(0.0..=0.3);
  } else {
    params.background_tint_r = 0.0;
    params.background_tint_g = 0.0;
    params.background_tint_b = 0.0;
  }

  params.bass_influence = rng.gen_range(0.3..=0.8);
  params.mid_influence = rng.gen_range(0.2..=0.6);
  params.treble_influence = rng.gen_range(0.1..=0.5);
  params.beat_sensitivity = rng.gen_range(0.5..=2.0);
}
