use std::{fmt, fmt::Write as _, str::FromStr};

use super::{push_cells, reset_to_base_style, RenderedCell, RenderedFrame, RgbColor, StyleState};

const STREAM_PROTOCOL_MAGIC: &str = "CHROMA_FRAME";
const STREAM_PROTOCOL_VERSION: u8 = 1;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum StreamFormat {
  #[default]
  Ansi,
  Cells,
}

impl fmt::Display for StreamFormat {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    formatter.write_str(match self {
      Self::Ansi => "ansi",
      Self::Cells => "cells",
    })
  }
}

impl FromStr for StreamFormat {
  type Err = String;

  fn from_str(value: &str) -> Result<Self, Self::Err> {
    match value {
      "ansi" => Ok(Self::Ansi),
      "cells" => Ok(Self::Cells),
      _ => Err(format!(
        "Invalid stream format '{}'. Expected one of: ansi, cells",
        value
      )),
    }
  }
}

pub(super) fn to_framed_stream_string(
  frame: &RenderedFrame,
  format: StreamFormat,
  frame_index: u64,
) -> String {
  let payload = to_stream_payload(frame, format);
  let header = stream_frame_header(frame, format, frame_index, payload.len());
  let mut buffer = String::with_capacity(header.len() + payload.len());

  buffer.push_str(&header);
  buffer.push_str(&payload);

  buffer
}

fn stream_frame_header(
  frame: &RenderedFrame,
  format: StreamFormat,
  frame_index: u64,
  payload_bytes: usize,
) -> String {
  format!(
    "{STREAM_PROTOCOL_MAGIC} v={STREAM_PROTOCOL_VERSION} frame={} width={} height={} format={} encoding=utf-8 bytes={}\n",
    frame_index, frame.width, frame.height, format, payload_bytes
  )
}

fn to_stream_payload(frame: &RenderedFrame, format: StreamFormat) -> String {
  match format {
    StreamFormat::Ansi => to_ansi_stream_payload(frame),
    StreamFormat::Cells => to_cells_stream_payload(frame),
  }
}

fn to_ansi_stream_payload(frame: &RenderedFrame) -> String {
  let mut buffer = String::with_capacity(frame.height * frame.width * 25);
  let mut style_state = StyleState {
    foreground: None,
    background: None,
  };

  for row in &frame.rows {
    push_cells(
      &mut buffer,
      row,
      &mut style_state,
      StyleState {
        foreground: None,
        background: None,
      },
    );
    reset_to_base_style(
      &mut buffer,
      &mut style_state,
      StyleState {
        foreground: None,
        background: None,
      },
    );
    buffer.push('\n');
  }

  buffer
}

fn to_cells_stream_payload(frame: &RenderedFrame) -> String {
  let mut buffer = String::with_capacity(frame.height * frame.width * 32);

  for (y, row) in frame.rows.iter().enumerate() {
    let mut x = 0;

    for cell in row {
      push_cell_record(&mut buffer, x, y, cell);
      x += cell.display_width;
    }
  }

  buffer
}

fn push_cell_record(buffer: &mut String, x: usize, y: usize, cell: &RenderedCell) {
  let _ = write!(
    buffer,
    "{}\t{}\t{}\tU+{:04X}\t",
    x, y, cell.display_width, cell.character as u32
  );
  push_optional_rgb(buffer, cell.foreground);
  buffer.push('\t');
  push_optional_rgb(buffer, cell.background);
  buffer.push('\n');
}

fn push_optional_rgb(buffer: &mut String, color: Option<RgbColor>) {
  match color {
    Some((r, g, b)) => {
      let _ = write!(buffer, "{:02X}{:02X}{:02X}", r, g, b);
    }
    None => buffer.push('-'),
  }
}

#[cfg(test)]
mod tests {
  use crossterm::style::Color;

  use super::*;

  #[test]
  fn stream_format_parses_supported_values() {
    assert_eq!("ansi".parse::<StreamFormat>().unwrap(), StreamFormat::Ansi);
    assert_eq!(
      "cells".parse::<StreamFormat>().unwrap(),
      StreamFormat::Cells
    );
  }

  #[test]
  fn stream_format_displays_wire_values() {
    assert_eq!(StreamFormat::Ansi.to_string(), "ansi");
    assert_eq!(StreamFormat::Cells.to_string(), "cells");
  }

  #[test]
  fn stream_format_rejects_unknown_value() {
    let error = "json".parse::<StreamFormat>().unwrap_err();

    assert!(error.contains("Expected one of"));
  }

  #[test]
  fn ansi_stream_output_is_length_prefixed() {
    let ascii_frame = vec![vec![(
      'A',
      Color::Rgb {
        r: 240,
        g: 220,
        b: 200,
      },
    )]];
    let frame = RenderedFrame::from_ascii_frame(&ascii_frame, 1, 1, None, None);
    let payload = "\x1b[38;2;240;220;200mA\x1b[39m\n";

    assert_eq!(
      frame.to_stream_string(StreamFormat::Ansi, 7),
      format!(
        "CHROMA_FRAME v=1 frame=7 width=1 height=1 format=ansi encoding=utf-8 bytes={}\n{}",
        payload.len(),
        payload
      )
    );
  }

  #[test]
  fn cells_stream_output_is_length_prefixed() {
    let ascii_frame = vec![vec![
      (
        'A',
        Color::Rgb {
          r: 240,
          g: 220,
          b: 200,
        },
      ),
      (' ', Color::White),
    ]];
    let frame = RenderedFrame::from_ascii_frame(&ascii_frame, 2, 1, None, None);
    let payload = "0\t0\t1\tU+0041\tF0DCC8\t-\n1\t0\t1\tU+0020\t-\t-\n";

    assert_eq!(
      frame.to_stream_string(StreamFormat::Cells, 11),
      format!(
        "CHROMA_FRAME v=1 frame=11 width=2 height=1 format=cells encoding=utf-8 bytes={}\n{}",
        payload.len(),
        payload
      )
    );
  }

  #[test]
  fn cells_stream_payload_reports_wide_cell_width() {
    let ascii_frame = vec![vec![('界', Color::White)]];
    let frame = RenderedFrame::from_ascii_frame(&ascii_frame, 2, 1, None, None);
    let payload = "0\t0\t2\tU+754C\t-\t-\n";

    assert_eq!(
      frame.to_stream_string(StreamFormat::Cells, 0),
      format!(
        "CHROMA_FRAME v=1 frame=0 width=2 height=1 format=cells encoding=utf-8 bytes={}\n{}",
        payload.len(),
        payload
      )
    );
  }

  #[test]
  fn cells_stream_payload_emits_rows_and_padded_cells() {
    let ascii_frame = vec![vec![('A', Color::White)], vec![('B', Color::White)]];
    let frame = RenderedFrame::from_ascii_frame(&ascii_frame, 2, 2, None, None);
    let payload = concat!(
      "0\t0\t1\tU+0041\t-\t-\n",
      "1\t0\t1\tU+0020\t-\t-\n",
      "0\t1\t1\tU+0042\t-\t-\n",
      "1\t1\t1\tU+0020\t-\t-\n",
    );

    assert_eq!(
      frame.to_stream_string(StreamFormat::Cells, 0),
      format!(
        "CHROMA_FRAME v=1 frame=0 width=2 height=2 format=cells encoding=utf-8 bytes={}\n{}",
        payload.len(),
        payload
      )
    );
  }

  #[test]
  fn cells_stream_payload_emits_background_color_records() {
    let cells = vec![vec![RenderedCell::new(
      'X',
      Some((1, 2, 3)),
      Some((250, 251, 252)),
    )]];
    let frame = RenderedFrame {
      width: 1,
      height: 1,
      rows: cells,
      status_bar: None,
      terminal_background: None,
    };
    let payload = "0\t0\t1\tU+0058\t010203\tFAFBFC\n";

    assert_eq!(
      frame.to_stream_string(StreamFormat::Cells, 0),
      format!(
        "CHROMA_FRAME v=1 frame=0 width=1 height=1 format=cells encoding=utf-8 bytes={}\n{}",
        payload.len(),
        payload
      )
    );
  }
}
