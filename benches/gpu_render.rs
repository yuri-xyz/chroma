//! GPU dispatch plus readback of one frame, the part of the render loop that
//! blocks on the device. Needs a working GPU adapter; without one the bench
//! reports that it was skipped. Run with `cargo bench --bench gpu_render`.

use std::{hint::black_box, io};

use chroma::{
  params::ShaderParams,
  shader::{ShaderPipeline, ShaderUniforms},
};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

const SIZES: [(u32, u32); 3] = [(80, 24), (200, 60), (400, 110)];

fn bench_render(c: &mut Criterion) {
  let mut pipeline = match pollster::block_on(ShaderPipeline::new(1, 1, None, &mut io::sink())) {
    Ok(pipeline) => pipeline,
    Err(error) => {
      eprintln!("skipping gpu_render benchmarks: {error}");
      return;
    }
  };

  let mut group = c.benchmark_group("gpu_render");

  for (width, height) in SIZES {
    pipeline
      .resize(width, height)
      .expect("benchmark sizes are non-zero");

    let mut params = ShaderParams::with_audio_reactive_defaults();
    params.set_resolution(width, height);
    params.time = 3.5;
    let uniforms = ShaderUniforms::from_params(&params);

    group.throughput(Throughput::Elements((width * height) as u64));
    // Reuse one pixel buffer across iterations, as the render loop does.
    let mut pixels = Vec::new();
    group.bench_function(
      BenchmarkId::from_parameter(format!("{width}x{height}")),
      |b| {
        b.iter(|| {
          pipeline
            .render_into(black_box(&uniforms), &mut pixels)
            .expect("render succeeds")
        })
      },
    );
  }

  group.finish();
}

criterion_group!(benches, bench_render);
criterion_main!(benches);
