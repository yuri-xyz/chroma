//! CPU cost of turning shader pixels into terminal output, per stage and end
//! to end. Run with `cargo bench --bench frame_render`.

use std::hint::black_box;

use chroma::{
  ascii::{AsciiConverter, AsciiPalette},
  params::PaletteType,
  render::{RenderedFrame, StreamFormat},
};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

/// Terminal sizes from a small window up to a fullscreen 4K terminal.
const SIZES: [(u32, u32); 3] = [(80, 24), (200, 60), (400, 110)];

/// Deterministic RGBA frame whose colour changes on every cell, like real
/// shader output, so style escapes are emitted for almost every cell.
fn synthetic_pixels(width: u32, height: u32) -> Vec<u8> {
  let mut state = 0x9E37_79B9_u32;
  let mut pixels = Vec::with_capacity((width * height * 4) as usize);

  for y in 0..height {
    for x in 0..width {
      state ^= state << 13;
      state ^= state >> 17;
      state ^= state << 5;
      let noise = (state & 0x3F) as u8;
      let gradient = ((x * 255) / width.max(1)) as u8;
      pixels.push(gradient.wrapping_add(noise));
      pixels.push(((y * 255) / height.max(1)) as u8 ^ noise);
      pixels.push(255 - gradient);
      pixels.push(255);
    }
  }

  pixels
}

fn size_label(width: u32, height: u32) -> String {
  format!("{width}x{height}")
}

fn bench_convert_frame(c: &mut Criterion) {
  let mut group = c.benchmark_group("convert_frame");
  let converter = AsciiConverter::default();

  for (width, height) in SIZES {
    let pixels = synthetic_pixels(width, height);
    group.throughput(Throughput::Elements((width * height) as u64));
    group.bench_with_input(
      BenchmarkId::from_parameter(size_label(width, height)),
      &pixels,
      |b, pixels| b.iter(|| converter.convert_frame(black_box(pixels), width, height)),
    );
  }

  group.finish();
}

fn bench_build_rendered_frame(c: &mut Criterion) {
  let mut group = c.benchmark_group("rendered_frame_build");

  for palette in [PaletteType::Standard, PaletteType::Braille] {
    let converter = AsciiConverter::new(AsciiPalette::from(palette), true);

    for (width, height) in SIZES {
      let ascii = converter.convert_frame(&synthetic_pixels(width, height), width, height);
      group.throughput(Throughput::Elements((width * height) as u64));
      group.bench_with_input(
        BenchmarkId::new(palette.name(), size_label(width, height)),
        &ascii,
        |b, ascii| {
          b.iter(|| {
            RenderedFrame::from_ascii_frame(
              black_box(ascii),
              width as usize,
              height as usize,
              None,
              None,
            )
          })
        },
      );
    }
  }

  group.finish();
}

fn rendered_frame(width: u32, height: u32) -> RenderedFrame {
  let converter = AsciiConverter::default();
  let ascii = converter.convert_frame(&synthetic_pixels(width, height), width, height);
  RenderedFrame::from_ascii_frame(&ascii, width as usize, height as usize, None, None)
}

fn bench_serialize(c: &mut Criterion) {
  let mut group = c.benchmark_group("serialize");

  for (width, height) in SIZES {
    let frame = rendered_frame(width, height);
    let label = size_label(width, height);
    group.throughput(Throughput::Elements((width * height) as u64));

    group.bench_with_input(BenchmarkId::new("terminal", &label), &frame, |b, frame| {
      b.iter(|| black_box(frame).to_terminal_string())
    });

    // The render loop keeps one output buffer alive across frames.
    let mut buffer = String::new();
    group.bench_with_input(
      BenchmarkId::new("terminal_reused_buffer", &label),
      &frame,
      |b, frame| {
        b.iter(|| {
          black_box(frame).write_terminal_string(&mut buffer);
          buffer.len()
        })
      },
    );

    for format in [StreamFormat::Ansi, StreamFormat::Cells] {
      group.bench_with_input(
        BenchmarkId::new(format!("stream_{format}"), &label),
        &frame,
        |b, frame| b.iter(|| black_box(frame).to_stream_string(format, 42)),
      );
      group.bench_with_input(
        BenchmarkId::new(format!("stream_{format}_reused_buffer"), &label),
        &frame,
        |b, frame| {
          b.iter(|| {
            black_box(frame).write_stream_string(format, 42, &mut buffer);
            buffer.len()
          })
        },
      );
    }
  }

  group.finish();
}

/// Everything the render loop does on the CPU after GPU readback, including
/// reusing one output buffer across frames.
fn bench_end_to_end(c: &mut Criterion) {
  let mut group = c.benchmark_group("pixels_to_terminal_string");
  let converter = AsciiConverter::default();
  let mut output = String::new();

  for (width, height) in SIZES {
    let pixels = synthetic_pixels(width, height);
    group.throughput(Throughput::Elements((width * height) as u64));
    group.bench_with_input(
      BenchmarkId::from_parameter(size_label(width, height)),
      &pixels,
      |b, pixels| {
        b.iter(|| {
          let ascii = converter.convert_frame(black_box(pixels), width, height);
          let frame =
            RenderedFrame::from_ascii_frame(&ascii, width as usize, height as usize, None, None);
          frame.write_terminal_string(&mut output);
          output.len()
        })
      },
    );
  }

  group.finish();
}

criterion_group!(
  benches,
  bench_convert_frame,
  bench_build_rendered_frame,
  bench_serialize,
  bench_end_to_end
);
criterion_main!(benches);
