use chroma::ascii::{AsciiConverter, AsciiPalette};
use crossterm::style::Color;

#[test]
fn test_convert_empty_frame() {
  let converter = AsciiConverter::default();
  let pixels: Vec<u8> = vec![];
  let result = converter.convert_frame(&pixels, 0, 0);

  assert_eq!(result.len(), 0);
}

#[test]
fn test_convert_single_pixel() {
  let converter = AsciiConverter::default();
  let pixels: Vec<u8> = vec![128, 64, 192, 255];
  let result = converter.convert_frame(&pixels, 1, 1);

  assert_eq!(result.len(), 1);
  assert_eq!(result[0].len(), 1);
}

#[test]
fn test_convert_pure_colors() {
  let converter = AsciiConverter::new(AsciiPalette::standard(), true);

  let red_pixels: Vec<u8> = vec![255, 0, 0, 255];
  let result_red = converter.convert_frame(&red_pixels, 1, 1);

  let (_, color) = result_red[0][0];

  if let Color::Rgb { r, g, b } = color {
    assert_eq!(r, 255);
    assert_eq!(g, 0);
    assert_eq!(b, 0);
  } else {
    panic!("Expected RGB color");
  }

  let green_pixels: Vec<u8> = vec![0, 255, 0, 255];
  let result_green = converter.convert_frame(&green_pixels, 1, 1);

  let (_, color) = result_green[0][0];

  if let Color::Rgb { r, g, b } = color {
    assert_eq!(r, 0);
    assert_eq!(g, 255);
    assert_eq!(b, 0);
  } else {
    panic!("Expected RGB color");
  }

  let blue_pixels: Vec<u8> = vec![0, 0, 255, 255];
  let result_blue = converter.convert_frame(&blue_pixels, 1, 1);

  let (_, color) = result_blue[0][0];

  if let Color::Rgb { r, g, b } = color {
    assert_eq!(r, 0);
    assert_eq!(g, 0);
    assert_eq!(b, 255);
  } else {
    panic!("Expected RGB color");
  }
}

#[test]
fn test_convert_with_color_disabled() {
  let converter = AsciiConverter::new(AsciiPalette::standard(), false);
  let pixels: Vec<u8> = vec![255, 0, 0, 255];
  let result = converter.convert_frame(&pixels, 1, 1);

  let (_, color) = result[0][0];

  assert_eq!(
    color,
    Color::White,
    "Color should be white when use_color is false"
  );
}

#[test]
fn test_set_palette_changes_output() {
  let mut converter = AsciiConverter::new(AsciiPalette::standard(), true);
  let pixels: Vec<u8> = vec![255, 255, 255, 255];
  let result_standard = converter.convert_frame(&pixels, 1, 1);

  let (char_standard, _) = result_standard[0][0];

  converter.set_palette(AsciiPalette::simple());

  let result_simple = converter.convert_frame(&pixels, 1, 1);
  let (char_simple, _) = result_simple[0][0];

  assert_eq!(char_standard, '@');
  assert_eq!(char_simple, '@');
}

#[test]
fn test_set_use_color_changes_output() {
  let mut converter = AsciiConverter::new(AsciiPalette::standard(), true);
  let pixels: Vec<u8> = vec![255, 0, 0, 255];
  let result_color = converter.convert_frame(&pixels, 1, 1);

  let (_, color_enabled) = result_color[0][0];

  converter.set_use_color(false);

  let result_no_color = converter.convert_frame(&pixels, 1, 1);
  let (_, color_disabled) = result_no_color[0][0];

  assert!(matches!(color_enabled, Color::Rgb { .. }));
  assert_eq!(color_disabled, Color::White);
}

#[test]
fn test_convert_rectangular_frame() {
  let converter = AsciiConverter::default();
  let width = 5;
  let height = 3;
  let pixels: Vec<u8> = vec![128; (width * height * 4) as usize];
  let result = converter.convert_frame(&pixels, width, height);

  assert_eq!(result.len(), height as usize);

  for row in result {
    assert_eq!(row.len(), width as usize);
  }
}

#[test]
fn test_brightness_calculation_weighted() {
  let converter = AsciiConverter::default();

  let red_pixels: Vec<u8> = vec![255, 0, 0, 255];
  let green_pixels: Vec<u8> = vec![0, 255, 0, 255];
  let blue_pixels: Vec<u8> = vec![0, 0, 255, 255];

  let result_red = converter.convert_frame(&red_pixels, 1, 1);
  let result_green = converter.convert_frame(&green_pixels, 1, 1);
  let result_blue = converter.convert_frame(&blue_pixels, 1, 1);

  let (char_red, _) = result_red[0][0];
  let (char_green, _) = result_green[0][0];
  let (char_blue, _) = result_blue[0][0];

  let palette = AsciiPalette::standard();
  let brightness_red = 0.299;
  let brightness_green = 0.587;
  let brightness_blue = 0.114;

  assert_eq!(char_red, palette.get_character(brightness_red));
  assert_eq!(char_green, palette.get_character(brightness_green));
  assert_eq!(char_blue, palette.get_character(brightness_blue));
}

#[test]
fn test_black_and_white_extremes() {
  let converter = AsciiConverter::new(AsciiPalette::standard(), true);

  let black_pixels: Vec<u8> = vec![0, 0, 0, 255];
  let result_black = converter.convert_frame(&black_pixels, 1, 1);

  let (char_black, _) = result_black[0][0];

  assert_eq!(char_black, ' ');

  let white_pixels: Vec<u8> = vec![255, 255, 255, 255];
  let result_white = converter.convert_frame(&white_pixels, 1, 1);

  let (char_white, _) = result_white[0][0];

  assert_eq!(char_white, '@');
}
