#![allow(dead_code)]

use std::f32::consts::PI;

use chroma::audio::{AudioAnalyzer, AudioFeatures, ANALYSIS_HOP_SIZE, ANALYSIS_WINDOW_SIZE};

pub const SAMPLE_RATE: f32 = 44_100.0;
pub const ANALYSIS_WINDOW: usize = ANALYSIS_WINDOW_SIZE;
pub const ANALYSIS_HOP: usize = ANALYSIS_HOP_SIZE;

#[derive(Debug, Clone, Copy)]
pub struct FixtureSegment {
  pub label: &'static str,
  pub start_sample: usize,
  pub end_sample: usize,
}

impl FixtureSegment {
  pub fn len(&self) -> usize {
    self.end_sample - self.start_sample
  }
}

#[derive(Debug)]
pub struct AuthoredFixture {
  pub samples: Vec<f32>,
  pub segments: Vec<FixtureSegment>,
}

impl AuthoredFixture {
  pub fn segment(&self, label: &str) -> &FixtureSegment {
    self
      .segments
      .iter()
      .find(|segment| segment.label == label)
      .unwrap_or_else(|| panic!("unknown fixture segment: {label}"))
  }
}

#[derive(Debug, Clone, Copy)]
pub struct AnalyzerFrame {
  pub sample_end: usize,
  pub features: AudioFeatures,
}

pub struct PulseLayer {
  pub stride: usize,
  pub width: usize,
  pub amplitude: f32,
  pub frequency_hz: f32,
}

pub struct FixtureBuilder {
  samples: Vec<f32>,
  segments: Vec<FixtureSegment>,
}

impl FixtureBuilder {
  pub fn new() -> Self {
    Self {
      samples: Vec::new(),
      segments: Vec::new(),
    }
  }

  pub fn silence(mut self, label: &'static str, windows: usize) -> Self {
    self.push_segment(label, vec![0.0; windows * ANALYSIS_WINDOW]);
    self
  }

  pub fn tone(
    mut self,
    label: &'static str,
    frequency_hz: f32,
    amplitude: f32,
    windows: usize,
  ) -> Self {
    self.push_segment(
      label,
      sine_wave(frequency_hz, amplitude, windows * ANALYSIS_WINDOW),
    );
    self
  }

  pub fn layers(mut self, label: &'static str, components: &[(f32, f32)], windows: usize) -> Self {
    let sample_count = windows * ANALYSIS_WINDOW;
    let mut samples = vec![0.0; sample_count];

    for &(frequency_hz, amplitude) in components {
      for (index, value) in sine_wave(frequency_hz, amplitude, sample_count)
        .into_iter()
        .enumerate()
      {
        samples[index] += value;
      }
    }

    for sample in &mut samples {
      *sample = sample.clamp(-1.0, 1.0);
    }

    self.push_segment(label, samples);
    self
  }

  pub fn kick_pulses(
    mut self,
    label: &'static str,
    windows: usize,
    pulse_stride: usize,
    pulse_width: usize,
    amplitude: f32,
    frequency_hz: f32,
  ) -> Self {
    self.push_segment(
      label,
      pulse_train(
        windows * ANALYSIS_WINDOW,
        pulse_stride,
        pulse_width,
        amplitude,
        frequency_hz,
      ),
    );
    self
  }

  pub fn pulse_train(
    mut self,
    label: &'static str,
    windows: usize,
    pulse_stride: usize,
    pulse_width: usize,
    amplitude: f32,
    frequency_hz: f32,
  ) -> Self {
    self.push_segment(
      label,
      pulse_train(
        windows * ANALYSIS_WINDOW,
        pulse_stride,
        pulse_width,
        amplitude,
        frequency_hz,
      ),
    );
    self
  }

  pub fn alternating_pulses(
    mut self,
    label: &'static str,
    windows: usize,
    pulse_stride: usize,
    pulse_width: usize,
    low_frequency_hz: f32,
    high_frequency_hz: f32,
  ) -> Self {
    let sample_count = windows * ANALYSIS_WINDOW;
    let mut samples = vec![0.0; sample_count];
    let mut use_low_frequency = true;

    for pulse_start in (0..sample_count).step_by(pulse_stride) {
      let frequency_hz = if use_low_frequency {
        low_frequency_hz
      } else {
        high_frequency_hz
      };

      for offset in 0..pulse_width.min(sample_count.saturating_sub(pulse_start)) {
        let index = pulse_start + offset;
        let envelope = 1.0 - (offset as f32 / pulse_width as f32);
        let phase = 2.0 * PI * frequency_hz * index as f32 / SAMPLE_RATE;
        samples[index] = (phase.sin() * envelope).clamp(-1.0, 1.0);
      }

      use_low_frequency = !use_low_frequency;
    }

    self.push_segment(label, samples);
    self
  }

  pub fn layered_pulse_train(
    mut self,
    label: &'static str,
    windows: usize,
    bed_components: &[(f32, f32)],
    pulse: PulseLayer,
  ) -> Self {
    let sample_count = windows * ANALYSIS_WINDOW;
    let mut samples = vec![0.0; sample_count];

    for &(frequency_hz, amplitude) in bed_components {
      for (index, value) in sine_wave(frequency_hz, amplitude, sample_count)
        .into_iter()
        .enumerate()
      {
        samples[index] += value;
      }
    }

    for (sample, pulse) in samples.iter_mut().zip(pulse_train(
      sample_count,
      pulse.stride,
      pulse.width,
      pulse.amplitude,
      pulse.frequency_hz,
    )) {
      *sample = (*sample + pulse).clamp(-1.0, 1.0);
    }

    self.push_segment(label, samples);
    self
  }

  pub fn build(self) -> AuthoredFixture {
    AuthoredFixture {
      samples: self.samples,
      segments: self.segments,
    }
  }

  fn push_segment(&mut self, label: &'static str, samples: Vec<f32>) {
    let start_sample = self.samples.len();
    let end_sample = start_sample + samples.len();

    self.samples.extend(samples);
    self.segments.push(FixtureSegment {
      label,
      start_sample,
      end_sample,
    });
  }
}

pub fn analyze_fixture(fixture: &AuthoredFixture, chunk_size: usize) -> Vec<AnalyzerFrame> {
  let mut analyzer = AudioAnalyzer::new(SAMPLE_RATE);
  let mut trace = Vec::new();
  let mut sample_end = 0;

  for chunk in fixture.samples.chunks(chunk_size) {
    sample_end += chunk.len();
    let features = analyzer.analyze(chunk, chunk.len() as f32 / SAMPLE_RATE);
    trace.push(AnalyzerFrame {
      sample_end,
      features,
    });
  }

  trace
}

pub fn analyze_fixture_with_chunk_schedule(
  fixture: &AuthoredFixture,
  chunk_schedule: &[usize],
) -> Vec<AnalyzerFrame> {
  assert!(
    !chunk_schedule.is_empty(),
    "chunk schedule must contain at least one chunk size"
  );

  let mut analyzer = AudioAnalyzer::new(SAMPLE_RATE);
  let mut trace = Vec::new();
  let mut sample_end = 0;
  let mut offset = 0;
  let mut schedule_index = 0;

  while offset < fixture.samples.len() {
    let chunk_size = chunk_schedule[schedule_index % chunk_schedule.len()];
    let end = (offset + chunk_size).min(fixture.samples.len());
    let chunk = &fixture.samples[offset..end];

    sample_end += chunk.len();
    let features = analyzer.analyze(chunk, chunk.len() as f32 / SAMPLE_RATE);
    trace.push(AnalyzerFrame {
      sample_end,
      features,
    });

    offset = end;
    schedule_index += 1;
  }

  trace
}

pub fn sine_wave(frequency_hz: f32, amplitude: f32, sample_count: usize) -> Vec<f32> {
  (0..sample_count)
    .map(|index| {
      let phase = 2.0 * PI * frequency_hz * index as f32 / SAMPLE_RATE;
      phase.sin() * amplitude
    })
    .collect()
}

fn pulse_train(
  sample_count: usize,
  pulse_stride: usize,
  pulse_width: usize,
  amplitude: f32,
  frequency_hz: f32,
) -> Vec<f32> {
  let mut samples = vec![0.0; sample_count];

  for pulse_start in (0..sample_count).step_by(pulse_stride) {
    for offset in 0..pulse_width.min(sample_count.saturating_sub(pulse_start)) {
      let index = pulse_start + offset;
      let envelope = 1.0 - (offset as f32 / pulse_width as f32);
      let phase = 2.0 * PI * frequency_hz * index as f32 / SAMPLE_RATE;
      samples[index] = (phase.sin() * envelope * amplitude).clamp(-1.0, 1.0);
    }
  }

  samples
}
