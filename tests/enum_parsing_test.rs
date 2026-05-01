use std::str::FromStr;

use chroma::params::{ColorMode, PaletteType, PatternType};

#[test]
fn test_pattern_type_parses_aliases_and_round_trips_full_names() {
  let aliases = [
    ("geo", PatternType::Geometric),
    ("hex", PatternType::Hexagonal),
    ("interf", PatternType::Interference),
    ("diamond", PatternType::Diamonds),
    ("warpedfbm", PatternType::WarpedFbm),
    ("kaleido", PatternType::Kaleidoscope),
    ("tun", PatternType::Tunnel),
    ("meta", PatternType::Metaballs),
    ("earth", PatternType::World),
    ("water", PatternType::Fluid),
    ("pyr", PatternType::Pyramid),
    ("loop", PatternType::Infinity),
  ];

  for (alias, expected) in aliases {
    assert_eq!(PatternType::from_str(alias).unwrap(), expected);
  }

  for pattern in PatternType::all() {
    assert_eq!(
      PatternType::from_str(pattern.full_name()).unwrap(),
      *pattern
    );
  }
}

#[test]
fn test_color_mode_parses_aliases_and_round_trips_full_names() {
  let aliases = [
    ("mono", ColorMode::Monochrome),
    ("cyber", ColorMode::Cyberpunk),
    ("chrome", ColorMode::Chromatic),
  ];

  for (alias, expected) in aliases {
    assert_eq!(ColorMode::from_str(alias).unwrap(), expected);
  }

  for mode in ColorMode::all() {
    assert_eq!(ColorMode::from_str(mode.full_name()).unwrap(), *mode);
  }
}

#[test]
fn test_palette_type_parses_aliases_and_round_trips_full_names() {
  let aliases = [
    ("std", PaletteType::Standard),
    ("block", PaletteType::Blocks),
    ("circle", PaletteType::Circles),
    ("geo", PaletteType::Geometric),
    ("shade", PaletteType::Shades),
    ("tri", PaletteType::Triangles),
    ("arrow", PaletteType::Arrows),
    ("power", PaletteType::Powerline),
    ("box", PaletteType::BoxDraw),
    ("extend", PaletteType::Extended),
  ];

  for (alias, expected) in aliases {
    assert_eq!(PaletteType::from_str(alias).unwrap(), expected);
  }

  for palette in PaletteType::all() {
    assert_eq!(
      PaletteType::from_str(palette.full_name()).unwrap(),
      *palette
    );
  }
}

#[test]
fn test_enum_parsers_reject_unknown_values_with_clear_errors() {
  let pattern_error = PatternType::from_str("unknown-pattern").unwrap_err();
  let color_error = ColorMode::from_str("unknown-mode").unwrap_err();
  let palette_error = PaletteType::from_str("unknown-palette").unwrap_err();

  assert!(pattern_error.contains("Unknown pattern type"));
  assert!(color_error.contains("Unknown color mode"));
  assert!(palette_error.contains("Unknown palette type"));
}
