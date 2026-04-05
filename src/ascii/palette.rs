use crate::params::PaletteType;

pub struct AsciiPalette {
  characters: Vec<char>,
  brightness_lookup: Box<[char; 256]>,
}

impl Default for AsciiPalette {
  fn default() -> Self {
    Self::standard()
  }
}

impl AsciiPalette {
  fn from_characters(characters: Vec<char>) -> Self {
    assert!(
      !characters.is_empty(),
      "palette must contain at least one character"
    );

    let last_index = characters.len() - 1;
    let mut brightness_lookup = Box::new([' '; 256]);

    for (brightness, slot) in brightness_lookup.iter_mut().enumerate() {
      let index = ((brightness as f32 / 255.0) * last_index as f32).round() as usize;
      *slot = characters[index.min(last_index)];
    }

    Self {
      characters,
      brightness_lookup,
    }
  }

  pub fn standard() -> Self {
    Self::from_characters(vec![' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'])
  }

  pub fn extended() -> Self {
    Self::from_characters(vec![
      ' ', '.', '\'', '`', '^', '"', ',', ':', ';', 'I', 'l', '!', 'i', '>', '<', '~', '+', '_',
      '-', '?', ']', '[', '}', '{', '1', ')', '(', '|', '\\', '/', 't', 'f', 'j', 'r', 'x', 'n',
      'u', 'v', 'c', 'z', 'X', 'Y', 'U', 'J', 'C', 'L', 'Q', '0', 'O', 'Z', 'm', 'w', 'q', 'p',
      'd', 'b', 'k', 'h', 'a', 'o', '*', '#', 'M', 'W', '&', '8', '%', 'B', '@', '$',
    ])
  }

  pub fn simple() -> Self {
    Self::from_characters(vec![' ', '.', 'o', 'O', '@'])
  }

  pub fn blocks() -> Self {
    Self::from_characters(vec![' ', '░', '▒', '▓', '█'])
  }

  pub fn smooth() -> Self {
    Self::from_characters(vec![' ', '·', '∘', '○', '◌', '◍', '◎', '◉', '●', '█'])
  }

  pub fn braille() -> Self {
    Self::from_characters(vec![' ', '⠁', '⠃', '⠇', '⠏', '⠟', '⠿', '⡿', '⣿'])
  }

  pub fn geometric() -> Self {
    Self::from_characters(vec![' ', '▪', '▫', '▬', '▭', '▮', '▯', '■', '█'])
  }

  pub fn circles() -> Self {
    Self::from_characters(vec![' ', '·', '∘', '○', '◌', '◍', '◎', '◉', '●', '█'])
  }

  pub fn mixed() -> Self {
    Self::from_characters(vec![' ', '·', '∘', '░', '▒', '▓', '●', '◉', '■', '█'])
  }

  pub fn dots() -> Self {
    Self::from_characters(vec![' ', '⡀', '⡄', '⡆', '⡇', '⣇', '⣧', '⣷', '⣿'])
  }

  pub fn shades() -> Self {
    Self::from_characters(vec![' ', '░', '░', '▒', '▒', '▓', '▓', '█', '█'])
  }

  pub fn lines() -> Self {
    Self::from_characters(vec![' ', '╌', '╍', '┄', '┅', '┈', '┉', '━', '█'])
  }

  pub fn triangles() -> Self {
    Self::from_characters(vec![' ', '▵', '▴', '▿', '▾', '◂', '◃', '▸', '▹'])
  }

  pub fn arrows() -> Self {
    Self::from_characters(vec![' ', '›', '»', '⟩', '→', '⇒', '⟹', '⟾', '▶'])
  }

  pub fn powerline() -> Self {
    Self::from_characters(vec![
      ' ', '\u{e0b0}', '\u{e0b1}', '\u{e0b2}', '\u{e0b3}', '\u{e0b4}', '\u{e0b5}', '\u{e0b6}', '█',
    ])
  }

  pub fn boxdraw() -> Self {
    Self::from_characters(vec![' ', '─', '━', '│', '┃', '┼', '╋', '╬', '█'])
  }

  pub fn get_character(&self, brightness: f32) -> char {
    let scaled_brightness = (brightness.clamp(0.0, 1.0) * 255.0).round() as u8;

    self.get_character_for_brightness(scaled_brightness)
  }

  pub fn get_character_for_brightness(&self, brightness: u8) -> char {
    self.brightness_lookup[brightness as usize]
  }

  pub fn len(&self) -> usize {
    self.characters.len()
  }

  pub fn is_empty(&self) -> bool {
    self.characters.is_empty()
  }
}

impl From<PaletteType> for AsciiPalette {
  fn from(palette_type: PaletteType) -> Self {
    match palette_type {
      PaletteType::Standard => Self::standard(),
      PaletteType::Blocks => Self::blocks(),
      PaletteType::Circles => Self::circles(),
      PaletteType::Smooth => Self::smooth(),
      PaletteType::Braille => Self::braille(),
      PaletteType::Geometric => Self::geometric(),
      PaletteType::Mixed => Self::mixed(),
      PaletteType::Dots => Self::dots(),
      PaletteType::Extended => Self::extended(),
      PaletteType::Simple => Self::simple(),
      PaletteType::Shades => Self::shades(),
      PaletteType::Lines => Self::lines(),
      PaletteType::Triangles => Self::triangles(),
      PaletteType::Arrows => Self::arrows(),
      PaletteType::Powerline => Self::powerline(),
      PaletteType::BoxDraw => Self::boxdraw(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_standard_palette() {
    let palette = AsciiPalette::standard();

    assert_eq!(palette.get_character(0.0), ' ');
    assert_eq!(palette.get_character(1.0), '@');
  }

  #[test]
  fn test_brightness_mapping() {
    let palette = AsciiPalette::standard();

    assert_eq!(palette.get_character(0.5), '+');
  }

  #[test]
  fn test_blocks_palette() {
    let palette = AsciiPalette::blocks();

    assert_eq!(palette.get_character(0.0), ' ');
    assert_eq!(palette.get_character(1.0), '█');
  }

  #[test]
  fn test_smooth_palette() {
    let palette = AsciiPalette::smooth();

    assert_eq!(palette.get_character(0.0), ' ');
    assert!(palette.characters.len() > 5);
  }

  #[test]
  fn test_braille_palette() {
    let palette = AsciiPalette::braille();

    assert_eq!(palette.get_character(0.0), ' ');
    assert_eq!(palette.get_character(1.0), '⣿');
  }

  #[test]
  fn test_palette_type_conversion() {
    let palette = AsciiPalette::from(PaletteType::Powerline);

    assert_eq!(palette.get_character(1.0), '█');
  }

  #[test]
  fn test_byte_lookup_matches_float_mapping() {
    let palette = AsciiPalette::standard();

    for brightness in [0_u8, 32, 64, 128, 200, 255] {
      let float_mapped = palette.get_character(brightness as f32 / 255.0);
      let byte_mapped = palette.get_character_for_brightness(brightness);

      assert_eq!(float_mapped, byte_mapped);
    }
  }

  #[test]
  fn test_byte_lookup_matches_float_mapping_for_every_brightness_value() {
    let palette = AsciiPalette::standard();

    for brightness in 0_u8..=255 {
      let float_mapped = palette.get_character(brightness as f32 / 255.0);
      let byte_mapped = palette.get_character_for_brightness(brightness);

      assert_eq!(float_mapped, byte_mapped);
    }
  }
}
