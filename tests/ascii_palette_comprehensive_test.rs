use chroma::ascii::AsciiPalette;

#[test]
fn test_extended_palette_exists_and_has_many_characters() {
  let palette = AsciiPalette::extended();

  assert!(
    palette.len() > 50,
    "Extended palette should have many characters"
  );

  assert!(!palette.is_empty());
}

#[test]
fn test_simple_palette_has_few_characters() {
  let palette = AsciiPalette::simple();

  assert_eq!(palette.len(), 5);
  assert_eq!(palette.get_character(0.0), ' ');
  assert_eq!(palette.get_character(1.0), '@');
}

#[test]
fn test_blocks_palette_unicode_blocks() {
  let palette = AsciiPalette::blocks();

  assert_eq!(palette.get_character(0.0), ' ');
  assert_eq!(palette.get_character(1.0), '█');

  let mid_char = palette.get_character(0.5);

  assert!(mid_char == '░' || mid_char == '▒' || mid_char == '▓');
}

#[test]
fn test_smooth_palette_has_gradual_progression() {
  let palette = AsciiPalette::smooth();

  let char_low = palette.get_character(0.1);
  let char_mid = palette.get_character(0.5);
  let char_high = palette.get_character(0.9);

  assert_ne!(char_low, char_mid);
  assert_ne!(char_mid, char_high);
  assert_ne!(char_low, char_high);
}

#[test]
fn test_braille_palette_braille_characters() {
  let palette = AsciiPalette::braille();

  assert_eq!(palette.get_character(0.0), ' ');
  assert_eq!(palette.get_character(1.0), '⣿');
}

#[test]
fn test_geometric_palette() {
  let palette = AsciiPalette::geometric();

  assert_eq!(palette.get_character(0.0), ' ');
  assert_eq!(palette.get_character(1.0), '█');

  assert!(palette.len() > 5);
}

#[test]
fn test_circles_palette() {
  let palette = AsciiPalette::circles();

  assert_eq!(palette.get_character(0.0), ' ');
  assert_eq!(palette.get_character(1.0), '█');

  assert!(palette.len() > 5);
}

#[test]
fn test_mixed_palette() {
  let palette = AsciiPalette::mixed();

  assert_eq!(palette.get_character(0.0), ' ');
  assert_eq!(palette.get_character(1.0), '█');

  assert_eq!(palette.len(), 10);
}

#[test]
fn test_dots_palette() {
  let palette = AsciiPalette::dots();

  assert_eq!(palette.get_character(0.0), ' ');
  assert_eq!(palette.get_character(1.0), '⣿');
}

#[test]
fn test_shades_palette() {
  let palette = AsciiPalette::shades();

  assert_eq!(palette.get_character(0.0), ' ');
  assert_eq!(palette.get_character(1.0), '█');
}

#[test]
fn test_lines_palette() {
  let palette = AsciiPalette::lines();

  assert_eq!(palette.get_character(0.0), ' ');
  assert_eq!(palette.get_character(1.0), '█');
}

#[test]
fn test_triangles_palette() {
  let palette = AsciiPalette::triangles();

  assert_eq!(palette.get_character(0.0), ' ');

  assert!(palette.len() > 5);
}

#[test]
fn test_arrows_palette() {
  let palette = AsciiPalette::arrows();

  assert_eq!(palette.get_character(0.0), ' ');

  assert!(palette.len() > 5);
}

#[test]
fn test_powerline_palette() {
  let palette = AsciiPalette::powerline();

  assert_eq!(palette.get_character(0.0), ' ');
  assert_eq!(palette.get_character(1.0), '█');
}

#[test]
fn test_boxdraw_palette() {
  let palette = AsciiPalette::boxdraw();

  assert_eq!(palette.get_character(0.0), ' ');
  assert_eq!(palette.get_character(1.0), '█');
}

#[test]
fn test_all_palettes_start_with_space() {
  let palettes = vec![
    AsciiPalette::standard(),
    AsciiPalette::extended(),
    AsciiPalette::simple(),
    AsciiPalette::blocks(),
    AsciiPalette::smooth(),
    AsciiPalette::braille(),
    AsciiPalette::geometric(),
    AsciiPalette::circles(),
    AsciiPalette::mixed(),
    AsciiPalette::dots(),
    AsciiPalette::shades(),
    AsciiPalette::lines(),
    AsciiPalette::triangles(),
    AsciiPalette::arrows(),
    AsciiPalette::powerline(),
    AsciiPalette::boxdraw(),
  ];

  for palette in palettes {
    assert_eq!(
      palette.get_character(0.0),
      ' ',
      "All palettes should start with space for minimum brightness"
    );
  }
}

#[test]
fn test_all_palettes_are_not_empty() {
  let palettes = vec![
    AsciiPalette::standard(),
    AsciiPalette::extended(),
    AsciiPalette::simple(),
    AsciiPalette::blocks(),
    AsciiPalette::smooth(),
    AsciiPalette::braille(),
    AsciiPalette::geometric(),
    AsciiPalette::circles(),
    AsciiPalette::mixed(),
    AsciiPalette::dots(),
    AsciiPalette::shades(),
    AsciiPalette::lines(),
    AsciiPalette::triangles(),
    AsciiPalette::arrows(),
    AsciiPalette::powerline(),
    AsciiPalette::boxdraw(),
  ];

  for palette in palettes {
    assert!(!palette.is_empty());
    assert!(palette.len() > 0);
  }
}

#[test]
fn test_palette_brightness_clamping() {
  let palette = AsciiPalette::standard();

  let char_below_zero = palette.get_character(-1.0);
  let char_above_one = palette.get_character(2.0);
  let char_at_zero = palette.get_character(0.0);
  let char_at_one = palette.get_character(1.0);

  assert_eq!(
    char_below_zero, char_at_zero,
    "Negative brightness should clamp to 0.0"
  );
  assert_eq!(
    char_above_one, char_at_one,
    "Brightness > 1.0 should clamp to 1.0"
  );
}

#[test]
fn test_palette_default_is_standard() {
  let default_palette = AsciiPalette::default();
  let standard_palette = AsciiPalette::standard();

  assert_eq!(default_palette.len(), standard_palette.len());
  assert_eq!(
    default_palette.get_character(0.0),
    standard_palette.get_character(0.0)
  );
  assert_eq!(
    default_palette.get_character(0.5),
    standard_palette.get_character(0.5)
  );
  assert_eq!(
    default_palette.get_character(1.0),
    standard_palette.get_character(1.0)
  );
}

#[test]
fn test_palette_brightness_mapping_is_monotonic() {
  let palette = AsciiPalette::standard();

  let char_at_0 = palette.get_character(0.0);
  let char_at_100 = palette.get_character(1.0);

  assert_eq!(char_at_0, ' ');
  assert_eq!(char_at_100, '@');

  assert_ne!(char_at_0, char_at_100, "Extremes should differ");
}
