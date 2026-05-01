use std::io::{self, stdout, Write as IoWrite};

use anyhow::Result;
use chroma::{
  ascii::AsciiConverter,
  debug::frame_logging_enabled,
  render::{RenderedCell, RenderedFrame, StreamFormat},
  shader::{ShaderPipeline, ShaderUniforms},
};
use crossterm::style::Color;

use super::DebugLog;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreamRenderStatus {
  Continue,
  ConsumerClosed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StdoutWriteStatus {
  Written,
  BrokenPipe,
}

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

fn write_output(writer: &mut impl IoWrite, buffer: &str) -> Result<StdoutWriteStatus> {
  if let Err(error) = writer.write_all(buffer.as_bytes()) {
    return handle_write_error(error);
  }

  if let Err(error) = writer.flush() {
    return handle_write_error(error);
  }

  Ok(StdoutWriteStatus::Written)
}

fn handle_write_error(error: io::Error) -> Result<StdoutWriteStatus> {
  if error.kind() == io::ErrorKind::BrokenPipe {
    return Ok(StdoutWriteStatus::BrokenPipe);
  }

  Err(error.into())
}

fn write_stdout(buffer: &str) -> Result<StdoutWriteStatus> {
  let mut stdout = stdout();
  write_output(&mut stdout, buffer)
}

fn write_stream_output(writer: &mut impl IoWrite, buffer: &str) -> Result<StreamRenderStatus> {
  match write_output(writer, buffer)? {
    StdoutWriteStatus::Written => Ok(StreamRenderStatus::Continue),
    StdoutWriteStatus::BrokenPipe => Ok(StreamRenderStatus::ConsumerClosed),
  }
}

fn write_stream_stdout(buffer: &str) -> Result<StreamRenderStatus> {
  let mut stdout = stdout();
  write_stream_output(&mut stdout, buffer)
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
  stream_format: StreamFormat,
  stream_frame_index: u64,
  debug_log: &mut DebugLog,
) -> Result<StreamRenderStatus> {
  let ascii_frame = render_ascii_frame(pipeline, converter, uniforms, debug_log)?;
  let frame = build_rendered_frame(pipeline, &ascii_frame, None, None);

  write_stream_stdout(&frame.to_stream_string(stream_format, stream_frame_index))
}

#[cfg(test)]
mod tests {
  use super::*;

  struct BrokenPipeWriter;

  impl IoWrite for BrokenPipeWriter {
    fn write(&mut self, _buffer: &[u8]) -> io::Result<usize> {
      Err(io::Error::new(io::ErrorKind::BrokenPipe, "consumer closed"))
    }

    fn flush(&mut self) -> io::Result<()> {
      Ok(())
    }
  }

  struct FlushBrokenPipeWriter {
    bytes: Vec<u8>,
  }

  impl IoWrite for FlushBrokenPipeWriter {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
      self.bytes.extend_from_slice(buffer);
      Ok(buffer.len())
    }

    fn flush(&mut self) -> io::Result<()> {
      Err(io::Error::new(io::ErrorKind::BrokenPipe, "consumer closed"))
    }
  }

  struct FailingWriter;

  impl IoWrite for FailingWriter {
    fn write(&mut self, _buffer: &[u8]) -> io::Result<usize> {
      Err(io::Error::other("disk full"))
    }

    fn flush(&mut self) -> io::Result<()> {
      Ok(())
    }
  }

  #[test]
  fn write_output_reports_successful_write() {
    let mut output = Vec::new();

    let status = write_output(&mut output, "frame").unwrap();

    assert_eq!(status, StdoutWriteStatus::Written);
    assert_eq!(output, b"frame");
  }

  #[test]
  fn write_output_treats_write_broken_pipe_as_closed_consumer() {
    let mut writer = BrokenPipeWriter;

    let status = write_output(&mut writer, "frame").unwrap();

    assert_eq!(status, StdoutWriteStatus::BrokenPipe);
  }

  #[test]
  fn write_output_treats_flush_broken_pipe_as_closed_consumer() {
    let mut writer = FlushBrokenPipeWriter { bytes: Vec::new() };

    let status = write_output(&mut writer, "frame").unwrap();

    assert_eq!(status, StdoutWriteStatus::BrokenPipe);
    assert_eq!(writer.bytes, b"frame");
  }

  #[test]
  fn write_output_preserves_non_broken_pipe_errors() {
    let mut writer = FailingWriter;

    let error = write_output(&mut writer, "frame").unwrap_err();

    assert!(error.to_string().contains("disk full"));
  }

  #[test]
  fn write_stream_output_reports_continue_after_successful_write() {
    let mut output = Vec::new();

    let status = write_stream_output(&mut output, "frame").unwrap();

    assert_eq!(status, StreamRenderStatus::Continue);
    assert_eq!(output, b"frame");
  }

  #[test]
  fn write_stream_output_reports_consumer_closed_for_write_broken_pipe() {
    let mut writer = BrokenPipeWriter;

    let status = write_stream_output(&mut writer, "frame").unwrap();

    assert_eq!(status, StreamRenderStatus::ConsumerClosed);
  }

  #[test]
  fn write_stream_output_reports_consumer_closed_for_flush_broken_pipe() {
    let mut writer = FlushBrokenPipeWriter { bytes: Vec::new() };

    let status = write_stream_output(&mut writer, "frame").unwrap();

    assert_eq!(status, StreamRenderStatus::ConsumerClosed);
    assert_eq!(writer.bytes, b"frame");
  }

  #[test]
  fn write_stream_output_preserves_non_broken_pipe_errors() {
    let mut writer = FailingWriter;

    let error = write_stream_output(&mut writer, "frame").unwrap_err();

    assert!(error.to_string().contains("disk full"));
  }
}
