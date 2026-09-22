//! Number formatting for frame serialization. Frames emit a colour escape for
//! nearly every cell, and going through `core::fmt` for each one dominated the
//! CPU cost of a frame.

use super::RgbColor;

const HEX_DIGITS: &[u8; 16] = b"0123456789ABCDEF";

/// Push `value` in decimal without leading zeros.
pub(super) fn push_decimal(buffer: &mut String, mut value: usize) {
  let mut digits = [0_u8; 20];
  let mut start = digits.len();

  loop {
    start -= 1;
    digits[start] = b'0' + (value % 10) as u8;
    value /= 10;

    if value == 0 {
      break;
    }
  }

  push_ascii(buffer, &digits[start..]);
}

/// Push `value` as uppercase hex, zero-padded to at least `min_digits`.
pub(super) fn push_hex(buffer: &mut String, value: u32, min_digits: usize) {
  let significant_digits = (u32::BITS - value.leading_zeros()).div_ceil(4) as usize;
  let digit_count = significant_digits.max(min_digits).min(8);
  // Hex fields are at most six digits; pushing them one at a time measured
  // faster than assembling them on the stack.
  for nibble in (0..digit_count).rev() {
    buffer.push(HEX_DIGITS[((value >> (nibble * 4)) & 0xF) as usize] as char);
  }
}

/// Push an SGR true-colour sequence such as `\x1b[38;2;R;G;Bm`, where
/// `introducer` is everything up to the first channel.
pub(super) fn push_rgb_sequence(buffer: &mut String, introducer: &str, (r, g, b): RgbColor) {
  buffer.push_str(introducer);

  // Assemble the channels on the stack and append them once, instead of
  // paying a capacity check per byte. Three channels need at most 12 bytes.
  let mut sequence = [0_u8; 12];
  let mut len = 0;

  for (channel, terminator) in [(r, b';'), (g, b';'), (b, b'm')] {
    if channel >= 100 {
      sequence[len] = b'0' + channel / 100;
      len += 1;
    }
    if channel >= 10 {
      sequence[len] = b'0' + channel / 10 % 10;
      len += 1;
    }
    sequence[len] = b'0' + channel % 10;
    sequence[len + 1] = terminator;
    len += 2;
  }

  push_ascii(buffer, &sequence[..len]);
}

fn push_ascii(buffer: &mut String, bytes: &[u8]) {
  buffer.push_str(std::str::from_utf8(bytes).expect("digit buffers only hold ASCII"));
}

#[cfg(test)]
mod tests {
  use super::*;

  fn decimal(value: usize) -> String {
    let mut buffer = String::new();
    push_decimal(&mut buffer, value);
    buffer
  }

  fn hex(value: u32, min_digits: usize) -> String {
    let mut buffer = String::new();
    push_hex(&mut buffer, value, min_digits);
    buffer
  }

  #[test]
  fn decimal_matches_display_formatting() {
    for value in [0, 1, 9, 10, 99, 100, 255, 1_000, 65_535, usize::MAX] {
      assert_eq!(decimal(value), value.to_string());
    }
  }

  #[test]
  fn decimal_matches_display_for_every_byte_value() {
    for value in 0..=u8::MAX as usize {
      assert_eq!(decimal(value), value.to_string());
    }
  }

  #[test]
  fn hex_matches_padded_upper_hex_formatting() {
    for value in [
      0,
      0x7,
      0x41,
      0xFF,
      0x754C,
      0xE0B0,
      0x1_F600,
      0x10_FFFF,
      u32::MAX,
    ] {
      assert_eq!(hex(value, 4), format!("{value:04X}"));
      assert_eq!(hex(value, 2), format!("{value:02X}"));
    }
  }

  #[test]
  fn rgb_sequence_matches_format_macro_output() {
    let mut buffer = String::new();

    push_rgb_sequence(&mut buffer, "\x1b[38;2;", (0, 128, 255));

    assert_eq!(buffer, format!("\x1b[38;2;{};{};{}m", 0, 128, 255));
  }
}
