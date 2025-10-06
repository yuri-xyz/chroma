use super::DebugLog;
use crate::constants::MIN_BRIGHTNESS_THRESHOLD;
use crate::utils::color::calculate_brightness;
use anyhow::Result;
use chroma::ascii::AsciiConverter;
use chroma::shader::{ShaderPipeline, ShaderUniforms};
use crossterm::style::Color;
use std::io::{stdout, Write};
use unicode_width::UnicodeWidthChar;

/// Render a complete frame to the terminal
pub fn render_frame(
  pipeline: &ShaderPipeline,
  converter: &AsciiConverter,
  uniforms: &ShaderUniforms,
  status_bar: Option<String>,
  terminal_bg_color: Option<(u8, u8, u8)>,
  debug_log: &mut DebugLog,
) -> Result<()> {
  // Generate pixel data from shader
  let pixel_data = pipeline.render(uniforms)?;

  log_pixel_data(&pixel_data, pipeline, debug_log)?;

  // Convert to ASCII art
  let ascii_frame = converter.convert_frame(&pixel_data, pipeline.width(), pipeline.height());

  log_ascii_frame(&ascii_frame, debug_log)?;

  // Build and display frame
  let frame_buffer = build_frame_buffer(
    ascii_frame,
    status_bar,
    terminal_bg_color,
    pipeline.width() as usize,
    pipeline.height() as usize,
    debug_log,
  )?;

  // Output to terminal
  let mut stdout = stdout();

  write!(stdout, "{}", frame_buffer)?;
  stdout.flush()?;

  Ok(())
}

/// Build the complete frame buffer including content and optional status bar
fn build_frame_buffer(
  ascii_frame: Vec<Vec<(char, Color)>>,
  status_bar: Option<String>,
  terminal_bg_color: Option<(u8, u8, u8)>,
  expected_cols: usize,
  expected_rows: usize,
  debug_log: &mut DebugLog,
) -> Result<String> {
  let mut buffer = String::with_capacity(expected_rows * expected_cols * 25);

  // Initialize buffer (hide cursor, move to home, reset colors)
  buffer.push_str("\x1b[?25l\x1b[H\x1b[0m");

  // Set terminal background color if specified
  if let Some((r, g, b)) = terminal_bg_color {
    buffer.push_str(&format!("\x1b[48;2;{};{};{}m", r, g, b));
  } else {
    buffer.push_str("\x1b[49m");
  }

  // Render ASCII art rows
  let rows_to_render = ascii_frame.len().min(expected_rows);

  for (row_idx, row) in ascii_frame.iter().enumerate().take(rows_to_render) {
    render_row(
      row,
      &mut buffer,
      terminal_bg_color,
      expected_cols,
      row_idx,
      debug_log,
    )?;

    // Only add newline if not the last row, or if there's a status bar
    // Keep background color across lines
    if row_idx < rows_to_render - 1 || status_bar.is_some() {
      if terminal_bg_color.is_some() {
        buffer.push_str("\r\n");
      } else {
        buffer.push_str("\x1b[0m\r\n");
      }
    }
  }

  // Add status bar if enabled
  if let Some(status) = status_bar {
    buffer.push_str("\x1b[0m\x1b[49m");
    buffer.push_str(&status);
  }

  log_frame_stats(
    rows_to_render,
    &ascii_frame,
    expected_rows,
    expected_cols,
    &buffer,
    debug_log,
  )?;

  Ok(buffer)
}

/// Render a single row of ASCII art
fn render_row(
  row: &[(char, Color)],
  buffer: &mut String,
  terminal_bg_color: Option<(u8, u8, u8)>,
  expected_cols: usize,
  _row_idx: usize,
  debug_log: &mut DebugLog,
) -> Result<()> {
  let mut current_col = 0;
  let mut col_idx = 0;

  while col_idx < row.len() && current_col < expected_cols {
    let (character, color) = &row[col_idx];
    let char_width = character.width().unwrap_or(1);

    // Check if character would overflow
    if current_col + char_width > expected_cols {
      writeln!(
        debug_log,
        "WARNING: Character '{}' (width={}) at col {} would overflow (limit={}), skipping rest of row",
        character, char_width, current_col, expected_cols
      )?;
      break;
    }

    // Skip spaces (optimization) - but preserve background color
    if *character == ' ' {
      buffer.push(' ');
      current_col += 1;
      col_idx += 1;
      continue;
    }

    // Skip very dark pixels - but preserve background color
    let brightness = extract_brightness(color);

    if brightness < MIN_BRIGHTNESS_THRESHOLD {
      buffer.push(' ');
      current_col += 1;
      col_idx += 1;
      continue;
    }

    // Render colored character with background
    if let Color::Rgb { r, g, b } = color {
      // Set foreground color
      buffer.push_str(&format!("\x1b[38;2;{};{};{}m", r, g, b));
      buffer.push(*character);
      // Reset foreground but keep background
      buffer.push_str("\x1b[39m");
    } else {
      buffer.push(*character);
    }

    current_col += char_width;
    col_idx += 1;
  }

  Ok(())
}

/// Extract brightness value from color
fn extract_brightness(color: &Color) -> u8 {
  if let Color::Rgb { r, g, b } = color {
    calculate_brightness(*r, *g, *b)
  } else {
    128
  }
}

/// Log pixel data statistics for debugging
fn log_pixel_data(
  pixel_data: &[u8],
  pipeline: &ShaderPipeline,
  debug_log: &mut DebugLog,
) -> Result<()> {
  writeln!(debug_log, "DEBUG: pixel_data length: {}", pixel_data.len())?;
  writeln!(
    debug_log,
    "DEBUG: Expected size: {}",
    pipeline.width() * pipeline.height() * 4
  )?;

  // Log first few pixels
  writeln!(debug_log, "DEBUG: First few pixels RGB values:")?;
  for i in 0..4.min(pixel_data.len() / 4) {
    let idx = i * 4;
    writeln!(
      debug_log,
      "  Pixel {}: R={}, G={}, B={}, A={}",
      i,
      pixel_data[idx],
      pixel_data[idx + 1],
      pixel_data[idx + 2],
      pixel_data[idx + 3]
    )?;
  }

  // Calculate brightness range
  let mut min_brightness = 255u8;
  let mut max_brightness = 0u8;

  for i in 0..(pixel_data.len() / 4).min(100) {
    let idx = i * 4;
    let avg = calculate_brightness(pixel_data[idx], pixel_data[idx + 1], pixel_data[idx + 2]);
    min_brightness = min_brightness.min(avg);
    max_brightness = max_brightness.max(avg);
  }

  writeln!(
    debug_log,
    "DEBUG: Brightness range in first 100 pixels: {} to {}",
    min_brightness, max_brightness
  )?;

  Ok(())
}

/// Log ASCII frame statistics for debugging
fn log_ascii_frame(ascii_frame: &[Vec<(char, Color)>], debug_log: &mut DebugLog) -> Result<()> {
  writeln!(debug_log, "DEBUG: ascii_frame rows: {}", ascii_frame.len())?;

  if !ascii_frame.is_empty() {
    writeln!(
      debug_log,
      "DEBUG: first row length: {}",
      ascii_frame[0].len()
    )?;

    if !ascii_frame[0].is_empty() {
      let (ch, col) = &ascii_frame[0][0];
      writeln!(
        debug_log,
        "DEBUG: first character: '{}' color: {:?}",
        ch, col
      )?;
    }
  }

  Ok(())
}

/// Log frame rendering statistics
fn log_frame_stats(
  rows_rendered: usize,
  ascii_frame: &[Vec<(char, Color)>],
  expected_rows: usize,
  expected_cols: usize,
  buffer: &str,
  debug_log: &mut DebugLog,
) -> Result<()> {
  writeln!(
    debug_log,
    "DEBUG: frame rendered {} rows x {} cols (expected {}x{}), buffer size: {}",
    rows_rendered,
    if ascii_frame.is_empty() {
      0
    } else {
      ascii_frame[0].len().min(expected_cols)
    },
    expected_rows,
    expected_cols,
    buffer.len()
  )?;

  Ok(())
}
