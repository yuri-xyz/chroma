use chroma::constants::EFFECT_NAMES;
use chroma::params::ShaderParams;
use chroma::render::{RenderedCell, RgbColor};
use chroma::utils::color::hue_to_pastel_rgb;
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
  status_text: &str,
  available_cols: usize,
  has_sound: bool,
  time: f32,
) -> Vec<RenderedCell> {
  let fitted_status = fit_status_text(status_text, available_cols);

  if has_sound {
    apply_audio_gradient(&fitted_status, time)
  } else {
    styled_text_cells(&fitted_status, Some((0, 0, 0)), Some((255, 255, 255)))
  }
}

fn fit_status_text(status: &str, available_cols: usize) -> String {
  let status_visual_len = display_width(status);

  if status_visual_len > available_cols {
    truncate_status(status, available_cols)
  } else {
    let padding = " ".repeat(available_cols - status_visual_len);

    format!("{}{}", status, padding)
  }
}

fn display_width(text: &str) -> usize {
  text.chars().map(|c| c.width().unwrap_or(1)).sum()
}

/// Truncate status text to fit available columns
fn truncate_status(status: &str, available_cols: usize) -> String {
  if available_cols <= 3 {
    return ".".repeat(available_cols);
  }

  let target_len = available_cols - 3;
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
fn apply_audio_gradient(status: &str, time: f32) -> Vec<RenderedCell> {
  let gradient_offset = (time * 2.0) % std::f32::consts::TAU;

  map_text_cells(status, |char_pos, ch| {
    let hue = (gradient_offset + (char_pos as f32 * 0.1)) % std::f32::consts::TAU;
    let (r, g, b) = hue_to_pastel_rgb(hue);

    RenderedCell::new(ch, Some((0, 0, 0)), Some((r, g, b)))
  })
}

fn styled_text_cells(
  text: &str,
  foreground: Option<RgbColor>,
  background: Option<RgbColor>,
) -> Vec<RenderedCell> {
  map_text_cells(text, |_, character| {
    RenderedCell::new(character, foreground, background)
  })
}

fn map_text_cells<F>(text: &str, mut build_cell: F) -> Vec<RenderedCell>
where
  F: FnMut(usize, char) -> RenderedCell,
{
  text
    .chars()
    .enumerate()
    .map(|(char_pos, character)| build_cell(char_pos, character))
    .collect()
}

#[cfg(test)]
mod tests {
  use super::*;
  use chroma::params::{ColorMode, PaletteType, PatternType};

  fn test_params() -> ShaderParams {
    ShaderParams {
      frequency: 12.3,
      color_mode: ColorMode::Fire,
      pattern_type: PatternType::Waves,
      palette: PaletteType::Braille,
      ..ShaderParams::default()
    }
  }

  #[test]
  fn test_build_status_text_includes_compact_mode_markers() {
    let text = build_status_text(&test_params(), 3);

    assert_eq!(
      text,
      "Star WFB  F:12.3  Q:quit R:random S:save A:audio N:effect C:color P:palette"
    );
  }

  #[test]
  fn test_format_status_bar_pads_to_available_width() {
    let cells = format_status_bar("Hi", 5, false, 0.0);
    let rendered = cells.iter().map(|cell| cell.character).collect::<String>();

    assert_eq!(cells.len(), 5);
    assert_eq!(rendered, "Hi   ");
    assert_eq!(cells[0].character, 'H');
    assert_eq!(cells[1].character, 'i');
    assert_eq!(cells[4].character, ' ');
    assert_eq!(cells[0].foreground, Some((0, 0, 0)));
    assert_eq!(cells[0].background, Some((255, 255, 255)));
  }

  #[test]
  fn test_format_status_bar_truncates_when_needed() {
    let cells = format_status_bar("Hello world", 6, false, 0.0);
    let rendered = cells.iter().map(|cell| cell.character).collect::<String>();

    assert_eq!(rendered, "Hel...");
  }

  #[test]
  fn test_format_status_bar_respects_tiny_widths() {
    let cells = format_status_bar("Hello world", 2, false, 0.0);
    let rendered = cells.iter().map(|cell| cell.character).collect::<String>();

    assert_eq!(rendered, "..");
  }

  #[test]
  fn test_format_status_bar_pads_wide_characters_to_visual_width() {
    let cells = format_status_bar("界", 3, false, 0.0);
    let rendered = cells.iter().map(|cell| cell.character).collect::<String>();
    let display_width: usize = cells.iter().map(|cell| cell.display_width).sum();

    assert_eq!(rendered, "界 ");
    assert_eq!(display_width, 3);
  }

  #[test]
  fn test_format_status_bar_truncates_wide_characters_without_overflow() {
    let cells = format_status_bar("界界界", 5, false, 0.0);
    let rendered = cells.iter().map(|cell| cell.character).collect::<String>();
    let display_width: usize = cells.iter().map(|cell| cell.display_width).sum();

    assert_eq!(rendered, "界...");
    assert_eq!(display_width, 5);
  }

  #[test]
  fn test_audio_status_bar_applies_per_cell_background_gradient() {
    let cells = format_status_bar("AB", 2, true, 0.0);

    assert_eq!(cells.len(), 2);
    assert_eq!(cells[0].character, 'A');
    assert_eq!(cells[1].character, 'B');
    assert_eq!(cells[0].foreground, Some((0, 0, 0)));
    assert_eq!(cells[1].foreground, Some((0, 0, 0)));
    assert_eq!(cells[0].background, Some(hue_to_pastel_rgb(0.0)));
    assert_eq!(cells[1].background, Some(hue_to_pastel_rgb(0.1)));
  }
}
