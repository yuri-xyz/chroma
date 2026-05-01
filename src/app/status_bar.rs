use chroma::{
  constants::EFFECT_NAMES,
  params::ShaderParams,
  render::{RenderedCell, RgbColor},
  utils::color::hue_to_pastel_rgb,
};
use unicode_width::UnicodeWidthChar;

const MUSIC_SYMBOL_DELAY_SECONDS: f32 = 2.0;
const MUSIC_SYMBOL_MIN_SPARE_COLS: usize = 10;
const MUSIC_SYMBOL_SPACING_COLS: i32 = 5;
const MUSIC_SYMBOL_SPEED_COLS_PER_SECOND: f32 = 16.0;
const MUSIC_SYMBOL_MAX_TEXT_CLEARANCE_COLS: usize = 15;
const MUSIC_SYMBOL_TEXT_FADE_COLS: f32 = 5.0;
const MUSIC_SYMBOLS: [char; 32] = [
  '♪', '♫', '♬', '♩', '♭', '♯', '☺', '☻', '☼', '★', '☆', '✦', '✧', '✺', '❋', '◆', '◇', '●', '○',
  '◌', '◍', '◎', '◈', '✹', '✷', '✶', '✵', '✴', '✳', '✲', '✱', '✻',
];

pub fn music_symbol_delay_seconds() -> f32 {
  MUSIC_SYMBOL_DELAY_SECONDS
}

/// Build status bar text with current parameters
pub fn build_status_text(params: &ShaderParams, effect_type: u32) -> String {
  let effect_name = EFFECT_NAMES[effect_type as usize % 7];
  let pattern_initial = params.pattern_type.name().chars().next().unwrap_or('?');
  let color_initial = params.color_mode.name().chars().next().unwrap_or('?');
  let palette_initial = params.palette.name().chars().next().unwrap_or('?');

  format!(
    "{} {}{}{}  F:{:.1}  Q:quit R:random S:save N:effect C:color P:palette",
    effect_name, pattern_initial, color_initial, palette_initial, params.frequency
  )
}

/// Format status bar with optional audio gradient
pub fn format_status_bar(
  status_text: &str,
  available_cols: usize,
  audio_gradient_active: bool,
  time: f32,
  audio_flow_elapsed: f32,
  audio_production_elapsed: f32,
) -> Vec<RenderedCell> {
  let fitted_status = fit_status_text(status_text, available_cols);
  let status_width = display_width(status_text);

  let mut cells = if audio_gradient_active {
    apply_audio_gradient(&fitted_status, time)
  } else {
    styled_text_cells(&fitted_status, Some((0, 0, 0)), Some((255, 255, 255)))
  };

  apply_music_symbol_flow(
    &mut cells,
    status_width.min(available_cols),
    audio_flow_elapsed,
    audio_production_elapsed,
  );

  cells
}

pub fn music_symbol_drain_seconds(status_text: &str, available_cols: usize) -> f32 {
  let status_width = display_width(status_text).min(available_cols);
  if status_width >= available_cols {
    return 0.0;
  }

  let spare_cols = available_cols - status_width;
  if spare_cols < MUSIC_SYMBOL_MIN_SPARE_COLS {
    return 0.0;
  }

  let movement_cols = spare_cols.saturating_sub(music_symbol_text_clearance(spare_cols));
  if movement_cols < MUSIC_SYMBOL_MIN_SPARE_COLS {
    return 0.0;
  }

  (movement_cols as f32 + 1.0) / MUSIC_SYMBOL_SPEED_COLS_PER_SECOND
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

fn apply_music_symbol_flow(
  cells: &mut [RenderedCell],
  status_width: usize,
  audio_flow_elapsed: f32,
  audio_production_elapsed: f32,
) {
  if audio_flow_elapsed < MUSIC_SYMBOL_DELAY_SECONDS
    || audio_production_elapsed < MUSIC_SYMBOL_DELAY_SECONDS
    || status_width >= cells.len()
  {
    return;
  }

  let spare_cols = cells.len() - status_width;
  if spare_cols < MUSIC_SYMBOL_MIN_SPARE_COLS {
    return;
  }

  let text_clearance_cols = music_symbol_text_clearance(spare_cols);
  let movement_cols = spare_cols.saturating_sub(text_clearance_cols);
  if movement_cols < MUSIC_SYMBOL_MIN_SPARE_COLS {
    return;
  }

  let flow_time = audio_flow_elapsed - MUSIC_SYMBOL_DELAY_SECONDS;
  let production_time = audio_production_elapsed - MUSIC_SYMBOL_DELAY_SECONDS;
  let current_step = (flow_time * MUSIC_SYMBOL_SPEED_COLS_PER_SECOND).floor() as i32;
  let production_step = (production_time * MUSIC_SYMBOL_SPEED_COLS_PER_SECOND).floor() as i32;
  let latest_slot = production_step / MUSIC_SYMBOL_SPACING_COLS;
  let earliest_visible_slot =
    ((current_step - movement_cols as i32).div_euclid(MUSIC_SYMBOL_SPACING_COLS) + 1).max(0);

  for slot_index in earliest_visible_slot.max(0)..=latest_slot {
    let symbol_distance = current_step - slot_index * MUSIC_SYMBOL_SPACING_COLS;
    if symbol_distance < 0 || symbol_distance >= movement_cols as i32 {
      continue;
    }

    let x = movement_cols - 1 - symbol_distance as usize;
    let symbol_hash = status_symbol_hash(slot_index as u32);
    let symbol = MUSIC_SYMBOLS[symbol_hash as usize % MUSIC_SYMBOLS.len()];
    draw_music_symbol(
      cells,
      status_width,
      spare_cols,
      text_clearance_cols + x,
      text_clearance_cols,
      symbol,
    );
  }
}

fn music_symbol_text_clearance(spare_cols: usize) -> usize {
  MUSIC_SYMBOL_MAX_TEXT_CLEARANCE_COLS.min(spare_cols.saturating_sub(MUSIC_SYMBOL_MIN_SPARE_COLS))
}

fn draw_music_symbol(
  cells: &mut [RenderedCell],
  status_width: usize,
  spare_cols: usize,
  spare_position: usize,
  text_clearance_cols: usize,
  symbol: char,
) {
  if spare_position >= spare_cols {
    return;
  }

  let alpha = music_symbol_alpha(spare_position, spare_cols, text_clearance_cols);
  if alpha < 0.08 {
    return;
  }

  let cell_index = status_width + spare_position;
  let background = cells[cell_index].background.unwrap_or((255, 255, 255));

  cells[cell_index] = RenderedCell::new(
    symbol,
    Some(blend_rgb(background, (0, 0, 0), alpha)),
    Some(background),
  );
}

fn music_symbol_alpha(spare_x: usize, spare_cols: usize, text_clearance_cols: usize) -> f32 {
  let fade_cols = 4.0_f32.min((spare_cols as f32 / 2.0).max(1.0));
  let fade_in = ((spare_cols - 1 - spare_x) as f32 / fade_cols).clamp(0.0, 1.0);
  let fade_out =
    ((spare_x as f32 - text_clearance_cols as f32) / MUSIC_SYMBOL_TEXT_FADE_COLS).clamp(0.0, 1.0);

  fade_in.min(fade_out) * 0.92
}

fn blend_rgb(from: RgbColor, to: RgbColor, alpha: f32) -> RgbColor {
  let mix_channel =
    |from: u8, to: u8| -> u8 { (from as f32 + (to as f32 - from as f32) * alpha).round() as u8 };

  (
    mix_channel(from.0, to.0),
    mix_channel(from.1, to.1),
    mix_channel(from.2, to.2),
  )
}

fn status_symbol_hash(value: u32) -> u32 {
  let mut hash = value.wrapping_mul(747_796_405).wrapping_add(2_891_336_453);
  hash = ((hash >> ((hash >> 28) + 4)) ^ hash).wrapping_mul(277_803_737);
  (hash >> 22) ^ hash
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
  use chroma::params::{ColorMode, PaletteType, PatternType};

  use super::*;

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
      "Star WFB  F:12.3  Q:quit R:random S:save N:effect C:color P:palette"
    );
  }

  #[test]
  fn test_format_status_bar_pads_to_available_width() {
    let cells = format_status_bar("Hi", 5, false, 0.0, 0.0, 0.0);
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
    let cells = format_status_bar("Hello world", 6, false, 0.0, 0.0, 0.0);
    let rendered = cells.iter().map(|cell| cell.character).collect::<String>();

    assert_eq!(rendered, "Hel...");
  }

  #[test]
  fn test_format_status_bar_respects_tiny_widths() {
    let cells = format_status_bar("Hello world", 2, false, 0.0, 0.0, 0.0);
    let rendered = cells.iter().map(|cell| cell.character).collect::<String>();

    assert_eq!(rendered, "..");
  }

  #[test]
  fn test_format_status_bar_pads_wide_characters_to_visual_width() {
    let cells = format_status_bar("界", 3, false, 0.0, 0.0, 0.0);
    let rendered = cells.iter().map(|cell| cell.character).collect::<String>();
    let display_width: usize = cells.iter().map(|cell| cell.display_width).sum();

    assert_eq!(rendered, "界 ");
    assert_eq!(display_width, 3);
  }

  #[test]
  fn test_format_status_bar_truncates_wide_characters_without_overflow() {
    let cells = format_status_bar("界界界", 5, false, 0.0, 0.0, 0.0);
    let rendered = cells.iter().map(|cell| cell.character).collect::<String>();
    let display_width: usize = cells.iter().map(|cell| cell.display_width).sum();

    assert_eq!(rendered, "界...");
    assert_eq!(display_width, 5);
  }

  #[test]
  fn test_audio_status_bar_applies_per_cell_background_gradient() {
    let cells = format_status_bar("AB", 2, true, 0.0, 0.0, 0.0);

    assert_eq!(cells.len(), 2);
    assert_eq!(cells[0].character, 'A');
    assert_eq!(cells[1].character, 'B');
    assert_eq!(cells[0].foreground, Some((0, 0, 0)));
    assert_eq!(cells[1].foreground, Some((0, 0, 0)));
    assert_eq!(cells[0].background, Some(hue_to_pastel_rgb(0.0)));
    assert_eq!(cells[1].background, Some(hue_to_pastel_rgb(0.1)));
  }

  #[test]
  fn test_audio_status_bar_waits_before_music_symbols() {
    let cells = format_status_bar(
      "AB",
      20,
      true,
      0.0,
      MUSIC_SYMBOL_DELAY_SECONDS - 0.1,
      MUSIC_SYMBOL_DELAY_SECONDS - 0.1,
    );
    let rendered = cells.iter().map(|cell| cell.character).collect::<String>();

    assert_eq!(rendered, "AB                  ");
  }

  #[test]
  fn test_audio_status_bar_requires_spare_space_for_music_symbols() {
    let cells = format_status_bar(
      "1234567890",
      19,
      true,
      0.0,
      MUSIC_SYMBOL_DELAY_SECONDS + 2.0,
      MUSIC_SYMBOL_DELAY_SECONDS + 2.0,
    );
    let rendered = cells.iter().map(|cell| cell.character).collect::<String>();

    assert_eq!(rendered, "1234567890         ");
  }

  #[test]
  fn test_audio_status_bar_flows_music_symbols_in_spare_space() {
    let cells = format_status_bar(
      "AB",
      20,
      true,
      0.0,
      MUSIC_SYMBOL_DELAY_SECONDS + 1.0,
      MUSIC_SYMBOL_DELAY_SECONDS + 1.0,
    );
    let rendered = cells.iter().map(|cell| cell.character).collect::<String>();
    let status_prefix = rendered.chars().take(2).collect::<String>();
    let music_symbol_count = cells
      .iter()
      .skip(2)
      .filter(|cell| MUSIC_SYMBOLS.contains(&cell.character))
      .count();

    assert_eq!(status_prefix, "AB");
    assert!(music_symbol_count >= 1);
  }

  #[test]
  fn test_audio_status_bar_starts_music_symbols_one_at_a_time() {
    let cells = format_status_bar(
      "AB",
      40,
      true,
      0.0,
      MUSIC_SYMBOL_DELAY_SECONDS + 0.3,
      MUSIC_SYMBOL_DELAY_SECONDS + 0.3,
    );
    let music_symbol_count = cells
      .iter()
      .skip(2)
      .filter(|cell| MUSIC_SYMBOLS.contains(&cell.character))
      .count();

    assert!((1..=2).contains(&music_symbol_count));
  }

  #[test]
  fn test_audio_status_bar_moves_visible_symbols_in_lockstep() {
    let cells_before = format_status_bar(
      "AB",
      60,
      true,
      0.0,
      MUSIC_SYMBOL_DELAY_SECONDS + 4.0,
      MUSIC_SYMBOL_DELAY_SECONDS + 4.0,
    );
    let cells_after = format_status_bar(
      "AB",
      60,
      true,
      0.0,
      MUSIC_SYMBOL_DELAY_SECONDS + 4.0 + (1.0 / MUSIC_SYMBOL_SPEED_COLS_PER_SECOND),
      MUSIC_SYMBOL_DELAY_SECONDS + 4.0 + (1.0 / MUSIC_SYMBOL_SPEED_COLS_PER_SECOND),
    );
    let positions_before = music_symbol_positions(&cells_before);
    let positions_after = music_symbol_positions(&cells_after);

    assert_eq!(positions_before.len(), positions_after.len());
    assert!(!positions_before.is_empty());
    assert!(positions_before
      .iter()
      .zip(positions_after.iter())
      .all(|(before, after)| *after + 1 == *before));
  }

  #[test]
  fn test_audio_status_bar_drain_stops_producing_new_symbols() {
    let production_elapsed = MUSIC_SYMBOL_DELAY_SECONDS + 4.0;
    let cells_at_stop =
      format_status_bar("AB", 80, true, 0.0, production_elapsed, production_elapsed);
    let cells_after_stop = format_status_bar(
      "AB",
      80,
      true,
      0.0,
      production_elapsed + (1.0 / MUSIC_SYMBOL_SPEED_COLS_PER_SECOND),
      production_elapsed,
    );
    let positions_at_stop = music_symbol_positions(&cells_at_stop);
    let positions_after_stop = music_symbol_positions(&cells_after_stop);

    assert_eq!(positions_at_stop.len(), positions_after_stop.len());
    assert!(!positions_at_stop.is_empty());
    assert!(positions_at_stop
      .iter()
      .zip(positions_after_stop.iter())
      .all(|(before, after)| *after + 1 == *before));
  }

  #[test]
  fn test_audio_status_bar_drain_uses_inactive_bar_colors() {
    let production_elapsed = MUSIC_SYMBOL_DELAY_SECONDS + 4.0;
    let cells = format_status_bar(
      "AB",
      80,
      false,
      0.0,
      production_elapsed + (1.0 / MUSIC_SYMBOL_SPEED_COLS_PER_SECOND),
      production_elapsed,
    );

    assert_eq!(cells[0].foreground, Some((0, 0, 0)));
    assert_eq!(cells[0].background, Some((255, 255, 255)));
    assert!(!music_symbol_positions(&cells).is_empty());
  }

  #[test]
  fn test_audio_status_bar_drain_clears_symbols_after_cycle_completes() {
    let production_elapsed = MUSIC_SYMBOL_DELAY_SECONDS + 4.0;
    let drain_seconds = music_symbol_drain_seconds("AB", 80);
    let cells = format_status_bar(
      "AB",
      80,
      true,
      0.0,
      production_elapsed + drain_seconds + 0.5,
      production_elapsed,
    );

    assert!(music_symbol_positions(&cells).is_empty());
  }

  #[test]
  fn test_audio_status_bar_keeps_music_symbols_clear_of_text() {
    let cells = format_status_bar(
      "AB",
      30,
      true,
      0.0,
      MUSIC_SYMBOL_DELAY_SECONDS + 1.0,
      MUSIC_SYMBOL_DELAY_SECONDS + 1.0,
    );
    let expected_clearance = music_symbol_text_clearance(28);
    let post_text_gap = cells
      .iter()
      .skip(2)
      .take(expected_clearance)
      .map(|cell| cell.character)
      .collect::<String>();

    assert_eq!(expected_clearance, MUSIC_SYMBOL_MAX_TEXT_CLEARANCE_COLS);
    assert_eq!(post_text_gap, " ".repeat(expected_clearance));
  }

  fn music_symbol_positions(cells: &[RenderedCell]) -> Vec<usize> {
    cells
      .iter()
      .enumerate()
      .filter_map(|(idx, cell)| MUSIC_SYMBOLS.contains(&cell.character).then_some(idx))
      .collect()
  }
}
