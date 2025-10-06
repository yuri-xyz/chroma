// Color conversion and manipulation utilities

/// Convert a hue value (in radians) to a pastel RGB color
///
/// # Arguments
/// * `hue` - Hue value in radians (0.0 to 2π)
///
/// # Returns
/// RGB tuple (r, g, b) with values 0-255
pub fn hue_to_pastel_rgb(hue: f32) -> (u8, u8, u8) {
  let hue_normalized = (hue / std::f32::consts::TAU) % 1.0;
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

/// Parse a hex color string to normalized RGB values (0.0-1.0)
///
/// # Arguments
/// * `hex` - Hex color string (e.g. "#FF0000", "FF0000", "#F00", "F00")
///
/// # Returns
/// Result with tuple (r, g, b) with values 0.0-1.0, or error if parsing fails
pub fn parse_hex_color(hex: &str) -> Result<(f32, f32, f32), String> {
  let hex = hex.trim_start_matches('#');

  let (r_str, g_str, b_str) = match hex.len() {
    3 => {
      let chars: Vec<char> = hex.chars().collect();
      (
        format!("{}{}", chars[0], chars[0]),
        format!("{}{}", chars[1], chars[1]),
        format!("{}{}", chars[2], chars[2]),
      )
    }
    6 => (
      hex[0..2].to_string(),
      hex[2..4].to_string(),
      hex[4..6].to_string(),
    ),
    _ => {
      return Err(format!(
        "Invalid hex color format: '{}' (expected 3 or 6 hex digits)",
        hex
      ))
    }
  };

  let r = u8::from_str_radix(&r_str, 16)
    .map_err(|_| format!("Invalid red component in hex color: '{}'", r_str))?;
  let g = u8::from_str_radix(&g_str, 16)
    .map_err(|_| format!("Invalid green component in hex color: '{}'", g_str))?;
  let b = u8::from_str_radix(&b_str, 16)
    .map_err(|_| format!("Invalid blue component in hex color: '{}'", b_str))?;

  Ok((r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_hue_to_pastel_rgb_red() {
    let (r, _g, b) = hue_to_pastel_rgb(0.0);

    assert!(r > 200, "Red component should be high for hue 0");
    assert!(b < 150, "Blue component should be low for hue 0");
  }

  #[test]
  fn test_hue_to_pastel_rgb_green() {
    let hue = 2.09;
    let (r, g, b) = hue_to_pastel_rgb(hue);

    assert!(g > r, "Green should dominate for green hue");
    assert!(g > b, "Green should dominate for green hue");
  }

  #[test]
  fn test_hue_to_pastel_rgb_blue() {
    let hue = 4.18;
    let (_r, _g, b) = hue_to_pastel_rgb(hue);

    assert!(b > 150, "Blue component should be high for blue hue");
  }

  #[test]
  fn test_hue_to_pastel_rgb_wrapping() {
    let (r1, g1, b1) = hue_to_pastel_rgb(0.0);
    let (r2, g2, b2) = hue_to_pastel_rgb(std::f32::consts::TAU);
    let (r3, g3, b3) = hue_to_pastel_rgb(std::f32::consts::TAU * 2.0);

    assert_eq!(r1, r2, "Hue should wrap at 2π");
    assert_eq!(g1, g2, "Hue should wrap at 2π");
    assert_eq!(b1, b2, "Hue should wrap at 2π");

    assert!((r1 as i16 - r3 as i16).abs() <= 2, "Hue should wrap at 4π");
    assert!((g1 as i16 - g3 as i16).abs() <= 2, "Hue should wrap at 4π");
    assert!((b1 as i16 - b3 as i16).abs() <= 2, "Hue should wrap at 4π");
  }

  #[test]
  fn test_hue_to_pastel_rgb_produces_pastel_colors() {
    for hue_steps in 0..12 {
      let hue = (hue_steps as f32) * (std::f32::consts::TAU / 12.0);
      let (r, g, b) = hue_to_pastel_rgb(hue);

      assert!(r >= 89, "Pastel colors should have minimum brightness");
      assert!(g >= 89, "Pastel colors should have minimum brightness");
      assert!(b >= 89, "Pastel colors should have minimum brightness");
    }
  }

  #[test]
  fn test_calculate_brightness_black() {
    let brightness = calculate_brightness(0, 0, 0);

    assert_eq!(brightness, 0, "Black should have zero brightness");
  }

  #[test]
  fn test_calculate_brightness_white() {
    let brightness = calculate_brightness(255, 255, 255);

    assert_eq!(brightness, 255, "White should have maximum brightness");
  }

  #[test]
  fn test_calculate_brightness_gray() {
    let brightness = calculate_brightness(128, 128, 128);

    assert_eq!(brightness, 128, "Mid-gray should have mid brightness");
  }

  #[test]
  fn test_calculate_brightness_red() {
    let brightness = calculate_brightness(255, 0, 0);

    assert_eq!(brightness, 85, "Pure red brightness should be 255/3");
  }

  #[test]
  fn test_calculate_brightness_averaging() {
    let brightness = calculate_brightness(100, 150, 200);
    let expected = ((100_u32 + 150_u32 + 200_u32) / 3) as u8;

    assert_eq!(brightness, expected, "Should average RGB components");
  }

  #[test]
  fn test_calculate_brightness_symmetric() {
    let b1 = calculate_brightness(100, 150, 200);
    let b2 = calculate_brightness(200, 150, 100);
    let b3 = calculate_brightness(150, 100, 200);

    assert_eq!(b1, b2, "Brightness should be symmetric to color order");
    assert_eq!(b1, b3, "Brightness should be symmetric to color order");
  }

  #[test]
  fn test_hue_spectrum_produces_different_colors() {
    let color1 = hue_to_pastel_rgb(0.0);
    let color2 = hue_to_pastel_rgb(2.09);
    let color3 = hue_to_pastel_rgb(4.18);

    assert_ne!(
      color1, color2,
      "Different hues should produce different colors"
    );
    assert_ne!(
      color2, color3,
      "Different hues should produce different colors"
    );
    assert_ne!(
      color1, color3,
      "Different hues should produce different colors"
    );
  }

  #[test]
  fn test_parse_hex_color_6_digit() {
    let (r, g, b) = parse_hex_color("FF0000").unwrap();
    assert!((r - 1.0).abs() < 0.01, "Red should be 1.0");
    assert!(g.abs() < 0.01, "Green should be 0.0");
    assert!(b.abs() < 0.01, "Blue should be 0.0");
  }

  #[test]
  fn test_parse_hex_color_with_hash() {
    let (r, g, b) = parse_hex_color("#00FF00").unwrap();
    assert!(r.abs() < 0.01, "Red should be 0.0");
    assert!((g - 1.0).abs() < 0.01, "Green should be 1.0");
    assert!(b.abs() < 0.01, "Blue should be 0.0");
  }

  #[test]
  fn test_parse_hex_color_3_digit() {
    let (r, g, b) = parse_hex_color("F00").unwrap();
    assert!((r - 1.0).abs() < 0.01, "Red should be 1.0");
    assert!(g.abs() < 0.01, "Green should be 0.0");
    assert!(b.abs() < 0.01, "Blue should be 0.0");
  }

  #[test]
  fn test_parse_hex_color_3_digit_with_hash() {
    let (r, g, b) = parse_hex_color("#ABC").unwrap();
    let expected_r = 0xAA as f32 / 255.0;
    let expected_g = 0xBB as f32 / 255.0;
    let expected_b = 0xCC as f32 / 255.0;
    assert!((r - expected_r).abs() < 0.01, "Red component mismatch");
    assert!((g - expected_g).abs() < 0.01, "Green component mismatch");
    assert!((b - expected_b).abs() < 0.01, "Blue component mismatch");
  }

  #[test]
  fn test_parse_hex_color_black() {
    let (r, g, b) = parse_hex_color("000000").unwrap();
    assert!(r.abs() < 0.01, "Red should be 0.0");
    assert!(g.abs() < 0.01, "Green should be 0.0");
    assert!(b.abs() < 0.01, "Blue should be 0.0");
  }

  #[test]
  fn test_parse_hex_color_white() {
    let (r, g, b) = parse_hex_color("FFFFFF").unwrap();
    assert!((r - 1.0).abs() < 0.01, "Red should be 1.0");
    assert!((g - 1.0).abs() < 0.01, "Green should be 1.0");
    assert!((b - 1.0).abs() < 0.01, "Blue should be 1.0");
  }

  #[test]
  fn test_parse_hex_color_invalid_length() {
    let result = parse_hex_color("FF00");
    assert!(result.is_err(), "Should fail for 4 digit hex");
  }

  #[test]
  fn test_parse_hex_color_invalid_chars() {
    let result = parse_hex_color("GGGGGG");
    assert!(result.is_err(), "Should fail for invalid hex characters");
  }

  #[test]
  fn test_parse_hex_color_lowercase() {
    let (r, g, b) = parse_hex_color("ff8800").unwrap();
    assert!((r - 1.0).abs() < 0.01, "Red should be 1.0");
    assert!((g - (0x88 as f32 / 255.0)).abs() < 0.01, "Green mismatch");
    assert!(b.abs() < 0.01, "Blue should be 0.0");
  }
}
