// Color conversion and manipulation utilities

/// Convert a hue value (in radians) to a pastel RGB color
///
/// # Arguments
/// * `hue` - Hue value in radians (0.0 to 2π)
///
/// # Returns
/// RGB tuple (r, g, b) with values 0-255
pub fn hue_to_pastel_rgb(hue: f32) -> (u8, u8, u8) {
  let hue_normalized = (hue / 6.28) % 1.0;
  let h = hue_normalized * 6.0;
  let c = 1.0;
  let x = 1.0 - ((h % 2.0) - 1.0).abs();

  let (r, g, b) = if h < 1.0 {
    (c, x, 0.0)
  } else if h < 2.0 {
    (x, c, 0.0)
  } else if h < 3.0 {
    (0.0, c, x)
  } else if h < 4.0 {
    (0.0, x, c)
  } else if h < 5.0 {
    (x, 0.0, c)
  } else {
    (c, 0.0, x)
  };

  const LIGHTNESS: f32 = 0.35;
  const SATURATION: f32 = 0.85;

  let pastel_r = ((r * SATURATION + LIGHTNESS) * 255.0).min(255.0) as u8;
  let pastel_g = ((g * SATURATION + LIGHTNESS) * 255.0).min(255.0) as u8;
  let pastel_b = ((b * SATURATION + LIGHTNESS) * 255.0).min(255.0) as u8;

  (pastel_r, pastel_g, pastel_b)
}

/// Calculate brightness from RGB color
///
/// # Arguments
/// * `r`, `g`, `b` - RGB color components (0-255)
///
/// # Returns
/// Brightness value (0-255)
pub fn calculate_brightness(r: u8, g: u8, b: u8) -> u8 {
  ((r as u32 + g as u32 + b as u32) / 3) as u8
}
