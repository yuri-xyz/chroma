use chroma::params::{ColorMode, PaletteType, PatternType};

#[test]
fn test_pattern_type_to_u32_all_variants() {
  assert_eq!(PatternType::Plasma.to_u32(), 0);
  assert_eq!(PatternType::Waves.to_u32(), 1);
  assert_eq!(PatternType::Ripples.to_u32(), 2);
  assert_eq!(PatternType::Vortex.to_u32(), 3);
  assert_eq!(PatternType::Noise.to_u32(), 4);
  assert_eq!(PatternType::Geometric.to_u32(), 5);
  assert_eq!(PatternType::Voronoi.to_u32(), 6);
  assert_eq!(PatternType::Truchet.to_u32(), 7);
  assert_eq!(PatternType::Hexagonal.to_u32(), 8);
  assert_eq!(PatternType::Interference.to_u32(), 9);
  assert_eq!(PatternType::Fractal.to_u32(), 10);
  assert_eq!(PatternType::Glitch.to_u32(), 11);
  assert_eq!(PatternType::Spiral.to_u32(), 12);
  assert_eq!(PatternType::Rings.to_u32(), 13);
  assert_eq!(PatternType::Grid.to_u32(), 14);
  assert_eq!(PatternType::Diamonds.to_u32(), 15);
  assert_eq!(PatternType::Sphere.to_u32(), 16);
  assert_eq!(PatternType::Octgrams.to_u32(), 17);
  assert_eq!(PatternType::WarpedFbm.to_u32(), 18);
}

#[test]
fn test_pattern_type_name_all_variants() {
  assert_eq!(PatternType::Plasma.name(), "Plasma");
  assert_eq!(PatternType::Waves.name(), "Waves");
  assert_eq!(PatternType::Ripples.name(), "Ripples");
  assert_eq!(PatternType::Vortex.name(), "Vortex");
  assert_eq!(PatternType::Noise.name(), "Noise");
  assert_eq!(PatternType::Geometric.name(), "Geo");
  assert_eq!(PatternType::Voronoi.name(), "Voronoi");
  assert_eq!(PatternType::Truchet.name(), "Truchet");
  assert_eq!(PatternType::Hexagonal.name(), "Hexagon");
  assert_eq!(PatternType::Interference.name(), "Interf");
  assert_eq!(PatternType::Fractal.name(), "Fractal");
  assert_eq!(PatternType::Glitch.name(), "Glitch");
  assert_eq!(PatternType::Spiral.name(), "Spiral");
  assert_eq!(PatternType::Rings.name(), "Rings");
  assert_eq!(PatternType::Grid.name(), "Grid");
  assert_eq!(PatternType::Diamonds.name(), "Diamond");
  assert_eq!(PatternType::Sphere.name(), "Sphere");
  assert_eq!(PatternType::Octgrams.name(), "Octgram");
  assert_eq!(PatternType::WarpedFbm.name(), "Warped");
}

#[test]
fn test_pattern_type_next_cycles_through_all() {
  let mut current = PatternType::Plasma;
  let start = current;

  for _ in 0..18 {
    current = current.next();
    assert_ne!(current, start, "Should not cycle back too early");
  }

  current = current.next();
  assert_eq!(current, start, "Should cycle back to start after 19 steps");
}

#[test]
fn test_pattern_type_previous_cycles_through_all() {
  let mut current = PatternType::Plasma;
  let start = current;

  for _ in 0..18 {
    current = current.previous();
    assert_ne!(current, start, "Should not cycle back too early");
  }

  current = current.previous();
  assert_eq!(current, start, "Should cycle back to start after 19 steps");
}

#[test]
fn test_pattern_type_next_then_previous_returns_original() {
  let pattern = PatternType::Vortex;

  assert_eq!(pattern.next().previous(), pattern);
}

#[test]
fn test_pattern_type_previous_then_next_returns_original() {
  let pattern = PatternType::Geometric;

  assert_eq!(pattern.previous().next(), pattern);
}

#[test]
fn test_color_mode_to_u32_all_variants() {
  assert_eq!(ColorMode::Rainbow.to_u32(), 0);
  assert_eq!(ColorMode::Monochrome.to_u32(), 1);
  assert_eq!(ColorMode::Duotone.to_u32(), 2);
  assert_eq!(ColorMode::Warm.to_u32(), 3);
  assert_eq!(ColorMode::Cool.to_u32(), 4);
  assert_eq!(ColorMode::Neon.to_u32(), 5);
  assert_eq!(ColorMode::Pastel.to_u32(), 6);
  assert_eq!(ColorMode::Cyberpunk.to_u32(), 7);
  assert_eq!(ColorMode::Warped.to_u32(), 8);
  assert_eq!(ColorMode::Chromatic.to_u32(), 9);
}

#[test]
fn test_color_mode_name_all_variants() {
  assert_eq!(ColorMode::Rainbow.name(), "Rainbow");
  assert_eq!(ColorMode::Monochrome.name(), "Mono");
  assert_eq!(ColorMode::Duotone.name(), "Duotone");
  assert_eq!(ColorMode::Warm.name(), "Warm");
  assert_eq!(ColorMode::Cool.name(), "Cool");
  assert_eq!(ColorMode::Neon.name(), "Neon");
  assert_eq!(ColorMode::Pastel.name(), "Pastel");
  assert_eq!(ColorMode::Cyberpunk.name(), "Cyber");
  assert_eq!(ColorMode::Warped.name(), "Warped");
  assert_eq!(ColorMode::Chromatic.name(), "Chrome");
}

#[test]
fn test_color_mode_next_cycles_through_all() {
  let mut current = ColorMode::Rainbow;
  let start = current;

  for _ in 0..9 {
    current = current.next();
    assert_ne!(current, start, "Should not cycle back too early");
  }

  current = current.next();
  assert_eq!(current, start, "Should cycle back to start after 10 steps");
}

#[test]
fn test_color_mode_previous_cycles_through_all() {
  let mut current = ColorMode::Rainbow;
  let start = current;

  for _ in 0..9 {
    current = current.previous();
    assert_ne!(current, start, "Should not cycle back too early");
  }

  current = current.previous();
  assert_eq!(current, start, "Should cycle back to start after 10 steps");
}

#[test]
fn test_color_mode_next_then_previous_returns_original() {
  let mode = ColorMode::Neon;

  assert_eq!(mode.next().previous(), mode);
}

#[test]
fn test_color_mode_previous_then_next_returns_original() {
  let mode = ColorMode::Pastel;

  assert_eq!(mode.previous().next(), mode);
}

#[test]
fn test_palette_type_name_all_variants() {
  assert_eq!(PaletteType::Standard.name(), "Std");
  assert_eq!(PaletteType::Blocks.name(), "Block");
  assert_eq!(PaletteType::Circles.name(), "Circle");
  assert_eq!(PaletteType::Smooth.name(), "Smooth");
  assert_eq!(PaletteType::Braille.name(), "Braille");
  assert_eq!(PaletteType::Geometric.name(), "Geo");
  assert_eq!(PaletteType::Mixed.name(), "Mixed");
  assert_eq!(PaletteType::Dots.name(), "Dots");
  assert_eq!(PaletteType::Shades.name(), "Shade");
  assert_eq!(PaletteType::Lines.name(), "Lines");
  assert_eq!(PaletteType::Triangles.name(), "Tri");
  assert_eq!(PaletteType::Arrows.name(), "Arrow");
  assert_eq!(PaletteType::Powerline.name(), "Power");
  assert_eq!(PaletteType::BoxDraw.name(), "Box");
  assert_eq!(PaletteType::Extended.name(), "Extend");
  assert_eq!(PaletteType::Simple.name(), "Simple");
}

#[test]
fn test_palette_type_next_cycles_through_all() {
  let mut current = PaletteType::Standard;
  let start = current;

  for _ in 0..15 {
    current = current.next();
    assert_ne!(current, start, "Should not cycle back too early");
  }

  current = current.next();
  assert_eq!(current, start, "Should cycle back to start after 16 steps");
}

#[test]
fn test_palette_type_previous_cycles_through_all() {
  let mut current = PaletteType::Standard;
  let start = current;

  for _ in 0..15 {
    current = current.previous();
    assert_ne!(current, start, "Should not cycle back too early");
  }

  current = current.previous();
  assert_eq!(current, start, "Should cycle back to start after 16 steps");
}

#[test]
fn test_palette_type_next_then_previous_returns_original() {
  let palette = PaletteType::Braille;

  assert_eq!(palette.next().previous(), palette);
}

#[test]
fn test_palette_type_previous_then_next_returns_original() {
  let palette = PaletteType::Geometric;

  assert_eq!(palette.previous().next(), palette);
}
