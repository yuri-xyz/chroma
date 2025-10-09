use crate::constants::EFFECT_NAMES;
use crate::utils::color::hue_to_pastel_rgb;
use chroma::params::ShaderParams;
use unicode_width::UnicodeWidthChar;

/// Build status bar text with current parameters
pub fn build_status_text(params: &ShaderParams, effect_type: u32) -> String {
  let effect_name = EFFECT_NAMES[effect_type as usize % 7];
  let pattern_initial = params.pattern_type.name().chars().next().unwrap_or('?');
  let color_initial = params.color_mode.name().chars().next().unwrap_or('?');
  let palette_initial = params.palette.name().chars().next().unwrap_or('?');

  format!(
    "{} {}{}{}  F:{:.1}  Q:quit R:random S:save A:audio N:effect C:color P:palette",
    effect_name, pattern_initial, color_initial, palette_initial, params.frequency
  )
}

/// Format status bar with optional audio gradient
pub fn format_status_bar(
  status_text: String,
  available_cols: usize,
  has_sound: bool,
  time: f32,
) -> String {
  let status_visual_len: usize = status_text.chars().map(|c| c.width().unwrap_or(1)).sum();

  let truncated_status = if status_visual_len > available_cols {
    truncate_status(status_text, available_cols)
  } else {
    let padding = " ".repeat(available_cols - status_visual_len);

    format!("{}{}", status_text, padding)
  };

  if has_sound {
    apply_audio_gradient(truncated_status, time)
  } else {
    format!("\x1b[47m\x1b[30m{}\x1b[0m", truncated_status)
  }
}

/// Truncate status text to fit available columns
fn truncate_status(status: String, available_cols: usize) -> String {
  let target_len = available_cols.saturating_sub(3);
  let mut current_width = 0;
  let mut truncated = String::new();

  for ch in status.chars() {
    let char_width = ch.width().unwrap_or(1);

    if current_width + char_width > target_len {
      break;
    }

    truncated.push(ch);
    current_width += char_width;
  }

  format!("{}...", truncated)
}

/// Apply animated gradient to status bar when audio is active
fn apply_audio_gradient(status: String, time: f32) -> String {
  let gradient_offset = (time * 2.0) % std::f32::consts::TAU;
  let mut formatted_status = String::new();

  for (char_pos, ch) in status.chars().enumerate() {
    let hue = (gradient_offset + (char_pos as f32 * 0.1)) % std::f32::consts::TAU;
    let (r, g, b) = hue_to_pastel_rgb(hue);

    formatted_status.push_str(&format!(
      "\x1b[48;2;{};{};{}m\x1b[30m{}\x1b[49m\x1b[39m",
      r, g, b, ch
    ));
  }

  formatted_status
}
