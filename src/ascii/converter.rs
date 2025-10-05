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
    let mut result = Vec::with_capacity(height as usize);

    for y in 0..height {
      let mut row = Vec::with_capacity(width as usize);

      for x in 0..width {
        let pixel_index = ((y * width + x) * 4) as usize;

        let red = pixels[pixel_index] as f32 / 255.0;
        let green = pixels[pixel_index + 1] as f32 / 255.0;
        let blue = pixels[pixel_index + 2] as f32 / 255.0;

        let brightness = self.calculate_brightness(red, green, blue);
        let character = self.palette.get_character(brightness);

        let color = if self.use_color {
          Color::Rgb {
            r: (red * 255.0) as u8,
            g: (green * 255.0) as u8,
            b: (blue * 255.0) as u8,
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

  fn calculate_brightness(&self, red: f32, green: f32, blue: f32) -> f32 {
    0.299 * red + 0.587 * green + 0.114 * blue
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
    let converter = AsciiConverter::default();
    let white_brightness = converter.calculate_brightness(1.0, 1.0, 1.0);

    assert!((white_brightness - 1.0).abs() < 0.001);

    let black_brightness = converter.calculate_brightness(0.0, 0.0, 0.0);

    assert!((black_brightness - 0.0).abs() < 0.001);
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
