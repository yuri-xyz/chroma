use std::io::{stdout, Write as IoWrite};

use anyhow::Result;
use chroma::{
  ascii::AsciiConverter,
  debug::frame_logging_enabled,
  render::{RenderedCell, RenderedFrame},
  shader::{ShaderPipeline, ShaderUniforms},
};
use crossterm::style::Color;

use super::DebugLog;

fn build_rendered_frame(
  pipeline: &ShaderPipeline,
  ascii_frame: &[Vec<(char, Color)>],
  status_bar: Option<Vec<RenderedCell>>,
  terminal_bg_color: Option<(u8, u8, u8)>,
) -> RenderedFrame {
  RenderedFrame::from_ascii_frame(
    ascii_frame,
    pipeline.width() as usize,
    pipeline.height() as usize,
    status_bar,
    terminal_bg_color,
  )
}

fn write_stdout(buffer: &str) -> Result<()> {
  let mut stdout = stdout();
  write!(stdout, "{}", buffer)?;
  stdout.flush()?;

  Ok(())
}

/// Render a complete frame to the terminal
pub fn render_frame(
  pipeline: &ShaderPipeline,
  converter: &AsciiConverter,
  uniforms: &ShaderUniforms,
  status_bar: Option<Vec<RenderedCell>>,
  terminal_bg_color: Option<(u8, u8, u8)>,
  debug_log: &mut DebugLog,
) -> Result<()> {
  let ascii_frame = render_ascii_frame(pipeline, converter, uniforms, debug_log)?;
  let frame = build_rendered_frame(pipeline, &ascii_frame, status_bar, terminal_bg_color);
  let frame_buffer = frame.to_terminal_string();

  log_frame_stats(&frame, &ascii_frame, &frame_buffer, debug_log)?;

  write_stdout(&frame_buffer)?;

  Ok(())
}

fn render_ascii_frame(
  pipeline: &ShaderPipeline,
  converter: &AsciiConverter,
  uniforms: &ShaderUniforms,
  debug_log: &mut DebugLog,
) -> Result<Vec<Vec<(char, Color)>>> {
  let pixel_data = pipeline.render(uniforms)?;

  log_pixel_data(&pixel_data, pipeline, debug_log)?;

  let ascii_frame = converter.convert_frame(&pixel_data, pipeline.width(), pipeline.height());

  log_ascii_frame(&ascii_frame, debug_log)?;

  Ok(ascii_frame)
}

/// Log pixel data statistics for debugging
fn log_pixel_data(
  pixel_data: &[u8],
  pipeline: &ShaderPipeline,
  debug_log: &mut DebugLog,
) -> Result<()> {
  if !frame_logging_enabled() {
    return Ok(());
  }

  debug_logln!(debug_log, "DEBUG: pixel_data length: {}", pixel_data.len())?;
  debug_logln!(
    debug_log,
    "DEBUG: Expected size: {}",
    pipeline.width() * pipeline.height() * 4
  )?;

  debug_logln!(debug_log, "DEBUG: First few pixels RGB values:")?;
  for i in 0..4.min(pixel_data.len() / 4) {
    let idx = i * 4;
    debug_logln!(
      debug_log,
      "  Pixel {}: R={}, G={}, B={}, A={}",
      i,
      pixel_data[idx],
      pixel_data[idx + 1],
      pixel_data[idx + 2],
      pixel_data[idx + 3]
    )?;
  }

  let mut min_brightness = 255u8;
  let mut max_brightness = 0u8;

  for i in 0..(pixel_data.len() / 4).min(100) {
    let idx = i * 4;
    let avg = ((pixel_data[idx] as u32 + pixel_data[idx + 1] as u32 + pixel_data[idx + 2] as u32)
      / 3) as u8;
    min_brightness = min_brightness.min(avg);
    max_brightness = max_brightness.max(avg);
  }

  debug_logln!(
    debug_log,
    "DEBUG: Brightness range in first 100 pixels: {} to {}",
    min_brightness,
    max_brightness
  )?;

  Ok(())
}

/// Log ASCII frame statistics for debugging
fn log_ascii_frame(ascii_frame: &[Vec<(char, Color)>], debug_log: &mut DebugLog) -> Result<()> {
  if !frame_logging_enabled() {
    return Ok(());
  }

  debug_logln!(debug_log, "DEBUG: ascii_frame rows: {}", ascii_frame.len())?;

  if !ascii_frame.is_empty() {
    debug_logln!(
      debug_log,
      "DEBUG: first row length: {}",
      ascii_frame[0].len()
    )?;

    if !ascii_frame[0].is_empty() {
      let (ch, col) = &ascii_frame[0][0];
      debug_logln!(
        debug_log,
        "DEBUG: first character: '{}' color: {:?}",
        ch,
        col
      )?;
    }
  }

  Ok(())
}

/// Log frame rendering statistics
fn log_frame_stats(
  frame: &RenderedFrame,
  ascii_frame: &[Vec<(char, Color)>],
  buffer: &str,
  debug_log: &mut DebugLog,
) -> Result<()> {
  if !frame_logging_enabled() {
    return Ok(());
  }

  debug_logln!(
    debug_log,
    "DEBUG: frame rendered {} rows x {} cols (expected {}x{}), buffer size: {}",
    frame.rows().len(),
    if ascii_frame.is_empty() {
      0
    } else {
      ascii_frame[0].len().min(frame.width())
    },
    frame.height(),
    frame.width(),
    buffer.len()
  )?;

  Ok(())
}

/// Render a frame optimized for stream mode (no terminal control codes, just raw output)
pub fn render_stream_frame(
  pipeline: &ShaderPipeline,
  converter: &AsciiConverter,
  uniforms: &ShaderUniforms,
  debug_log: &mut DebugLog,
) -> Result<()> {
  let ascii_frame = render_ascii_frame(pipeline, converter, uniforms, debug_log)?;
  let frame = build_rendered_frame(pipeline, &ascii_frame, None, None);

  write_stdout(&frame.to_stream_string())?;

  Ok(())
}
