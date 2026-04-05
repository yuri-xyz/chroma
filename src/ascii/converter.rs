use crossterm::style::Color;

use super::palette::AsciiPalette;

pub struct AsciiConverter {
  palette: AsciiPalette,
  use_color: bool,
}

impl Default for AsciiConverter {
  fn default() -> Self {
    Self {
      palette: AsciiPalette::default(),
      use_color: true,
    }
  }
}

impl AsciiConverter {
  pub fn new(palette: AsciiPalette, use_color: bool) -> Self {
    Self { palette, use_color }
  }

  pub fn convert_frame(&self, pixels: &[u8], width: u32, height: u32) -> Vec<Vec<(char, Color)>> {
    if width == 0 || height == 0 {
      return Vec::new();
    }

    let row_stride = width as usize * 4;
    let mut result = Vec::with_capacity(height as usize);
    let mut rows = pixels.chunks_exact(row_stride);

    for _ in 0..height {
      let mut row = Vec::with_capacity(width as usize);
      let row_pixels = rows.next().unwrap_or(&[]);

      for pixel in row_pixels.chunks_exact(4) {
        let red = pixel[0];
        let green = pixel[1];
        let blue = pixel[2];
        let brightness = Self::calculate_brightness(red, green, blue);
        let character = self.palette.get_character_for_brightness(brightness);

        let color = if self.use_color {
          Color::Rgb {
            r: red,
            g: green,
            b: blue,
          }
        } else {
          Color::White
        };

        row.push((character, color));
      }

      result.push(row);
    }

    result
  }

  fn calculate_brightness(red: u8, green: u8, blue: u8) -> u8 {
    ((299 * red as u32 + 587 * green as u32 + 114 * blue as u32 + 500) / 1000) as u8
  }

  pub fn set_palette(&mut self, palette: AsciiPalette) {
    self.palette = palette;
  }

  pub fn set_use_color(&mut self, use_color: bool) {
    self.use_color = use_color;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_brightness_calculation() {
    let white_brightness = AsciiConverter::calculate_brightness(255, 255, 255);

    assert_eq!(white_brightness, 255);

    let black_brightness = AsciiConverter::calculate_brightness(0, 0, 0);

    assert_eq!(black_brightness, 0);
  }

  #[test]
  fn test_brightness_calculation_matches_expected_luma_rounding() {
    let brightness = AsciiConverter::calculate_brightness(255, 0, 0);

    assert_eq!(brightness, 76);
  }

  #[test]
  fn test_convert_frame() {
    let converter = AsciiConverter::default();
    let pixels: Vec<u8> = vec![255, 255, 255, 255, 0, 0, 0, 255];
    let result = converter.convert_frame(&pixels, 2, 1);

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].len(), 2);
  }
}
