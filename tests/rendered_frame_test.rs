use chroma::{
  ascii::{AsciiConverter, AsciiPalette},
  render::{RenderedCell, RenderedFrame, StreamFormat},
};
use crossterm::style::Color;

fn frame_from_pixels(
  pixels: &[u8],
  width: u32,
  height: u32,
  palette: AsciiPalette,
  use_color: bool,
) -> RenderedFrame {
  let converter = AsciiConverter::new(palette, use_color);
  let ascii_frame = converter.convert_frame(pixels, width, height);

  RenderedFrame::from_ascii_frame(&ascii_frame, width as usize, height as usize, None, None)
}

#[test]
fn test_rendered_frame_supports_position_assertions() {
  let ascii_frame = vec![
    vec![
      (' ', Color::White),
      ('@', Color::Rgb { r: 255, g: 0, b: 0 }),
      (
        '#',
        Color::Rgb {
          r: 10,
          g: 10,
          b: 10,
        },
      ),
    ],
    vec![
      ('A', Color::Rgb { r: 0, g: 255, b: 0 }),
      ('B', Color::White),
      ('C', Color::Rgb { r: 0, g: 0, b: 255 }),
    ],
  ];

  let frame = RenderedFrame::from_ascii_frame(&ascii_frame, 3, 2, None, None);

  assert_eq!(frame.cell(0, 0).map(|cell| cell.character), Some(' '));
  assert_eq!(frame.cell(1, 0).map(|cell| cell.character), Some('@'));
  assert_eq!(
    frame.cell(1, 0).and_then(|cell| cell.foreground),
    Some((255, 0, 0))
  );
  assert_eq!(frame.cell(2, 0).map(|cell| cell.character), Some(' '));

  assert_eq!(frame.cell(0, 1).map(|cell| cell.character), Some('A'));
  assert_eq!(frame.cell(1, 1).map(|cell| cell.character), Some('B'));
  assert_eq!(frame.cell(1, 1).and_then(|cell| cell.foreground), None);
  assert_eq!(frame.cell(2, 1).map(|cell| cell.character), Some('C'));
  assert_eq!(
    frame.cell(2, 1).and_then(|cell| cell.foreground),
    Some((0, 0, 255))
  );
}

#[test]
fn test_terminal_string_preserves_status_bar_and_cell_styles() {
  let ascii_frame = vec![vec![(
    'X',
    Color::Rgb {
      r: 120,
      g: 160,
      b: 200,
    },
  )]];
  let status_bar = vec![
    RenderedCell::new('O', Some((0, 0, 0)), Some((255, 255, 255))),
    RenderedCell::new('K', Some((0, 0, 0)), Some((255, 255, 255))),
  ];
  let frame =
    RenderedFrame::from_ascii_frame(&ascii_frame, 1, 1, Some(status_bar), Some((9, 8, 7)));
  let output = frame.to_terminal_string();

  assert_eq!(
    output,
    "\x1b[?25l\x1b[H\x1b[0m\x1b[48;2;9;8;7m\x1b[38;2;120;160;200mX\x1b[39m\r\n\x1b[0m\x1b[49m\x1b[48;2;255;255;255m\x1b[38;2;0;0;0mOK\x1b[49m\x1b[39m"
  );
  assert_eq!(frame.status_cell(0).map(|cell| cell.character), Some('O'));
  assert_eq!(frame.status_cell(1).map(|cell| cell.character), Some('K'));
}

#[test]
fn test_stream_string_uses_legacy_blank_line_delimiter() {
  let ascii_frame = vec![vec![('A', Color::White), ('B', Color::White)]];
  let frame = RenderedFrame::from_ascii_frame(&ascii_frame, 2, 1, None, None);

  assert_eq!(frame.to_stream_string(StreamFormat::Legacy, 2), "AB\n\n");
}

#[test]
fn test_framed_ansi_stream_string_uses_length_prefixed_frame_header() {
  let ascii_frame = vec![vec![('A', Color::White), ('B', Color::White)]];
  let frame = RenderedFrame::from_ascii_frame(&ascii_frame, 2, 1, None, None);

  assert_eq!(
    frame.to_stream_string(StreamFormat::Ansi, 2),
    "CHROMA_FRAME v=1 frame=2 width=2 height=1 format=ansi encoding=utf-8 bytes=3\nAB\n"
  );
}

#[test]
fn test_stream_string_resets_foreground_style_at_row_boundaries() {
  let ascii_frame = vec![
    vec![(
      'A',
      Color::Rgb {
        r: 250,
        g: 200,
        b: 150,
      },
    )],
    vec![('B', Color::White)],
  ];
  let frame = RenderedFrame::from_ascii_frame(&ascii_frame, 1, 2, None, None);
  let payload = "\x1b[38;2;250;200;150mA\x1b[39m\nB\n";

  assert_eq!(
    frame.to_stream_string(StreamFormat::Ansi, 3),
    format!(
      "CHROMA_FRAME v=1 frame=3 width=1 height=2 format=ansi encoding=utf-8 bytes={}\n{}",
      payload.len(),
      payload
    )
  );
}

#[test]
fn test_terminal_string_resets_between_rows_without_terminal_background() {
  let ascii_frame = vec![
    vec![('A', Color::White), ('B', Color::White)],
    vec![('C', Color::White), ('D', Color::White)],
  ];
  let frame = RenderedFrame::from_ascii_frame(&ascii_frame, 2, 2, None, None);
  let output = frame.to_terminal_string();

  assert_eq!(output, "\x1b[?25l\x1b[H\x1b[0m\x1b[49mAB\x1b[0m\r\nCD");
}

#[test]
fn test_terminal_string_only_emits_foreground_changes_when_style_changes() {
  let ascii_frame = vec![vec![
    ('A', Color::Rgb { r: 255, g: 0, b: 0 }),
    ('B', Color::Rgb { r: 0, g: 0, b: 255 }),
    ('C', Color::Rgb { r: 0, g: 0, b: 255 }),
  ]];
  let frame = RenderedFrame::from_ascii_frame(&ascii_frame, 3, 1, None, None);

  assert_eq!(
    frame.to_terminal_string(),
    "\x1b[?25l\x1b[H\x1b[0m\x1b[49m\x1b[38;2;255;0;0mA\x1b[38;2;0;0;255mBC"
  );
}

#[test]
fn test_out_of_bounds_position_queries_return_none() {
  let ascii_frame = vec![vec![('A', Color::White)]];
  let frame = RenderedFrame::from_ascii_frame(&ascii_frame, 1, 1, None, None);

  assert_eq!(frame.cell(1, 0), None);
  assert_eq!(frame.cell(0, 1), None);
  assert_eq!(frame.status_cell(0), None);
}

#[test]
fn test_status_bar_position_queries_respect_wide_characters() {
  let ascii_frame = vec![vec![('A', Color::White)]];
  let status_bar = vec![RenderedCell::new(
    '界',
    Some((0, 0, 0)),
    Some((255, 255, 255)),
  )];
  let frame = RenderedFrame::from_ascii_frame(&ascii_frame, 1, 1, Some(status_bar), None);

  assert_eq!(frame.status_cell(0).map(|cell| cell.character), Some('界'));
  assert_eq!(frame.status_cell(1).map(|cell| cell.character), Some('界'));
  assert_eq!(frame.status_cell(2), None);
}

#[test]
fn test_pixel_fixture_flows_through_converter_into_rendered_frame() {
  let pixels = vec![
    0, 0, 0, 255, 128, 128, 128, 255, 255, 255, 255, 255, 255, 0, 0, 255,
  ];
  let frame = frame_from_pixels(&pixels, 2, 2, AsciiPalette::simple(), true);

  assert_eq!(frame.cell(0, 0).map(|cell| cell.character), Some(' '));
  assert_eq!(frame.cell(0, 0).and_then(|cell| cell.foreground), None);

  assert_eq!(frame.cell(1, 0).map(|cell| cell.character), Some('o'));
  assert_eq!(
    frame.cell(1, 0).and_then(|cell| cell.foreground),
    Some((128, 128, 128))
  );

  assert_eq!(frame.cell(0, 1).map(|cell| cell.character), Some('@'));
  assert_eq!(
    frame.cell(0, 1).and_then(|cell| cell.foreground),
    Some((255, 255, 255))
  );

  assert_eq!(frame.cell(1, 1).map(|cell| cell.character), Some('.'));
  assert_eq!(
    frame.cell(1, 1).and_then(|cell| cell.foreground),
    Some((255, 0, 0))
  );
}

#[test]
fn test_pipeline_fixture_without_color_keeps_characters_but_drops_foreground() {
  let pixels = vec![255, 255, 255, 255, 128, 128, 128, 255];
  let frame = frame_from_pixels(&pixels, 2, 1, AsciiPalette::simple(), false);

  assert_eq!(frame.cell(0, 0).map(|cell| cell.character), Some('@'));
  assert_eq!(frame.cell(0, 0).and_then(|cell| cell.foreground), None);

  assert_eq!(frame.cell(1, 0).map(|cell| cell.character), Some('o'));
  assert_eq!(frame.cell(1, 0).and_then(|cell| cell.foreground), None);
}

#[test]
fn test_pipeline_fixture_terminal_output_matches_semantic_cells() {
  let pixels = vec![255, 255, 255, 255, 0, 0, 255, 255];
  let frame = frame_from_pixels(&pixels, 2, 1, AsciiPalette::simple(), true);
  let output = frame.to_terminal_string();

  assert_eq!(frame.cell(0, 0).map(|cell| cell.character), Some('@'));
  assert_eq!(frame.cell(1, 0).map(|cell| cell.character), Some(' '));
  assert_eq!(
    output,
    "\x1b[?25l\x1b[H\x1b[0m\x1b[49m\x1b[38;2;255;255;255m@\x1b[39m "
  );
}
