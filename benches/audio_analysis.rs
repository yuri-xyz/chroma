//! Cost of the per-frame audio analysis. Run with
//! `cargo bench --bench audio_analysis`.

use std::{f32::consts::TAU, hint::black_box};

use chroma::audio::AudioAnalyzer;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

const SAMPLE_RATE: f32 = 44_100.0;

/// A bass tone with a mid overtone: enough spectral content that every band
/// and the beat detector do real work.
fn signal(len: usize) -> Vec<f32> {
  (0..len)
    .map(|index| {
      let t = index as f32 / SAMPLE_RATE;
      0.6 * (TAU * 60.0 * t).sin() + 0.25 * (TAU * 880.0 * t).sin()
    })
    .collect()
}

fn bench_analyze(c: &mut Criterion) {
  let mut group = c.benchmark_group("audio_analyze");

  // One frame's worth of samples at 60 FPS and at 30 FPS.
  for batch in [SAMPLE_RATE as usize / 60, SAMPLE_RATE as usize / 30] {
    let samples = signal(batch);
    let mut analyzer = AudioAnalyzer::new(SAMPLE_RATE);
    group.throughput(Throughput::Elements(batch as u64));
    group.bench_with_input(
      BenchmarkId::new("samples_per_call", batch),
      &samples,
      |b, samples| b.iter(|| analyzer.analyze(black_box(samples), 1.0 / 60.0)),
    );
  }

  group.finish();
}

criterion_group!(benches, bench_analyze);
criterion_main!(benches);
