use std::f32::consts::PI;

use chroma::audio::{AudioAnalyzer, AudioFeatures, ANALYSIS_WINDOW_SIZE};

const SAMPLE_RATE: f32 = 44_100.0;
const ANALYSIS_WINDOW: usize = ANALYSIS_WINDOW_SIZE;

fn sine_wave(frequency_hz: f32, amplitude: f32, sample_count: usize) -> Vec<f32> {
  (0..sample_count)
    .map(|index| {
      let phase = 2.0 * PI * frequency_hz * index as f32 / SAMPLE_RATE;
      phase.sin() * amplitude
    })
    .collect()
}

fn layered_signal(sample_count: usize) -> Vec<f32> {
  let bass = sine_wave(80.0, 0.9, sample_count);
  let mid = sine_wave(900.0, 0.5, sample_count);
  let treble = sine_wave(4_200.0, 0.25, sample_count);

  bass
    .into_iter()
    .zip(mid)
    .zip(treble)
    .map(|((bass, mid), treble)| (bass + mid + treble).clamp(-1.0, 1.0))
    .collect()
}

fn bass_pulse_train(
  sample_count: usize,
  start_offset: usize,
  pulse_stride: usize,
  pulse_width: usize,
) -> Vec<f32> {
  let mut samples = vec![0.0; sample_count];

  for pulse_start in (start_offset..sample_count).step_by(pulse_stride) {
    for offset in 0..pulse_width.min(sample_count.saturating_sub(pulse_start)) {
      let index = pulse_start + offset;
      let envelope = 1.0 - (offset as f32 / pulse_width as f32);
      let phase = 2.0 * PI * 70.0 * index as f32 / SAMPLE_RATE;
      samples[index] = (phase.sin() * envelope).clamp(-1.0, 1.0);
    }
  }

  samples
}

fn analyze_in_chunks(samples: &[f32], chunk_sizes: &[usize]) -> AudioFeatures {
  let mut analyzer = AudioAnalyzer::new(SAMPLE_RATE);
  let mut features = AudioFeatures::default();
  let mut offset = 0;

  for &chunk_size in chunk_sizes {
    if offset >= samples.len() {
      break;
    }

    let end = (offset + chunk_size).min(samples.len());
    let chunk = &samples[offset..end];
    features = analyzer.analyze(chunk, chunk.len() as f32 / SAMPLE_RATE);
    offset = end;
  }

  if offset < samples.len() {
    let chunk = &samples[offset..];
    features = analyzer.analyze(chunk, chunk.len() as f32 / SAMPLE_RATE);
  }

  features
}

fn assert_close(actual: f32, expected: f32, tolerance: f32, label: &str) {
  assert!(
    (actual - expected).abs() <= tolerance,
    "{label} mismatch: expected {expected:.4}, got {actual:.4}, tolerance {tolerance:.4}"
  );
}

#[test]
fn test_analyzer_waits_for_full_window_before_emitting_features() {
  let mut analyzer = AudioAnalyzer::new(SAMPLE_RATE);
  let fragment = sine_wave(80.0, 0.9, ANALYSIS_WINDOW / 8);

  let features = analyzer.analyze(&fragment, fragment.len() as f32 / SAMPLE_RATE);

  assert_eq!(features.bass, 0.0);
  assert_eq!(features.mid, 0.0);
  assert_eq!(features.treble, 0.0);
  assert_eq!(features.overall, 0.0);
  assert_eq!(features.beat_strength, 0.0);
  assert!(!features.is_drop);
}

#[test]
fn test_bass_tone_produces_bass_dominant_features() {
  let samples = sine_wave(80.0, 0.9, ANALYSIS_WINDOW * 2);
  let features = analyze_in_chunks(&samples, &[ANALYSIS_WINDOW]);

  assert!(
    features.bass > 0.15,
    "expected meaningful bass energy, got bass={:.4} mid={:.4} treble={:.4}",
    features.bass,
    features.mid,
    features.treble
  );
  assert!(
    features.bass > features.mid * 3.0,
    "bass should dominate mids, got bass={:.4} mid={:.4}",
    features.bass,
    features.mid
  );
  assert!(
    features.bass > features.treble * 4.0,
    "bass should dominate treble, got bass={:.4} treble={:.4}",
    features.bass,
    features.treble
  );
}

#[test]
fn test_mid_tone_produces_mid_dominant_features() {
  let samples = sine_wave(900.0, 0.9, ANALYSIS_WINDOW * 2);
  let features = analyze_in_chunks(&samples, &[ANALYSIS_WINDOW]);

  assert!(
    features.mid > 0.15,
    "expected meaningful mid energy, got bass={:.4} mid={:.4} treble={:.4}",
    features.bass,
    features.mid,
    features.treble
  );
  assert!(
    features.mid > features.bass * 3.0,
    "mid should dominate bass, got bass={:.4} mid={:.4}",
    features.bass,
    features.mid
  );
  assert!(
    features.mid > features.treble * 2.0,
    "mid should dominate treble, got mid={:.4} treble={:.4}",
    features.mid,
    features.treble
  );
}

#[test]
fn test_treble_tone_produces_treble_dominant_features() {
  let samples = sine_wave(4_200.0, 0.9, ANALYSIS_WINDOW * 2);
  let features = analyze_in_chunks(&samples, &[ANALYSIS_WINDOW]);

  assert!(
    features.treble > 0.15,
    "expected meaningful treble energy, got bass={:.4} mid={:.4} treble={:.4}",
    features.bass,
    features.mid,
    features.treble
  );
  assert!(
    features.treble > features.mid * 2.0,
    "treble should dominate mids, got mid={:.4} treble={:.4}",
    features.mid,
    features.treble
  );
  assert!(
    features.treble > features.bass * 4.0,
    "treble should dominate bass, got bass={:.4} treble={:.4}",
    features.bass,
    features.treble
  );
}

#[test]
fn test_chunk_boundaries_do_not_materially_change_features() {
  let samples = layered_signal(ANALYSIS_WINDOW * 3);

  let whole = analyze_in_chunks(&samples, &[samples.len()]);
  let chunked = analyze_in_chunks(&samples, &[173, 257, 389, 521, 233, 611, 467, 1_024, 901]);

  assert_close(chunked.bass, whole.bass, 0.08, "bass");
  assert_close(chunked.mid, whole.mid, 0.08, "mid");
  assert_close(chunked.treble, whole.treble, 0.08, "treble");
  assert_close(chunked.overall, whole.overall, 0.08, "overall");
}

#[test]
fn test_steady_tone_does_not_false_trigger_beats_after_warmup() {
  let samples = sine_wave(80.0, 0.9, ANALYSIS_WINDOW * 4);
  let mut analyzer = AudioAnalyzer::new(SAMPLE_RATE);
  let mut max_beat_strength = 0.0_f32;

  for chunk in samples.chunks(256) {
    let features = analyzer.analyze(chunk, chunk.len() as f32 / SAMPLE_RATE);
    max_beat_strength = max_beat_strength.max(features.beat_strength);
  }

  assert!(
    max_beat_strength < 0.35,
    "steady tone should not look like repeated beats"
  );
}

#[test]
fn test_bass_pulses_trigger_beat_detection() {
  let samples = bass_pulse_train(
    ANALYSIS_WINDOW * 6,
    ANALYSIS_WINDOW * 2,
    ANALYSIS_WINDOW,
    192,
  );
  let mut analyzer = AudioAnalyzer::new(SAMPLE_RATE);
  let mut max_beat_strength = 0.0_f32;

  for chunk in samples.chunks(256) {
    let features = analyzer.analyze(chunk, chunk.len() as f32 / SAMPLE_RATE);
    max_beat_strength = max_beat_strength.max(features.beat_strength);
  }

  assert!(
    max_beat_strength > 0.45,
    "expected bass pulses to register as beats, got max beat strength {:.4}",
    max_beat_strength
  );
}

#[test]
fn test_partial_tail_is_preserved_across_successive_analyze_calls() {
  let samples = layered_signal(ANALYSIS_WINDOW * 3);
  let split_at = ANALYSIS_WINDOW + (ANALYSIS_WINDOW / 3);

  let whole = analyze_in_chunks(&samples, &[samples.len()]);

  let mut analyzer = AudioAnalyzer::new(SAMPLE_RATE);
  let _ = analyzer.analyze(&samples[..split_at], split_at as f32 / SAMPLE_RATE);
  let resumed = analyzer.analyze(
    &samples[split_at..],
    (samples.len() - split_at) as f32 / SAMPLE_RATE,
  );

  assert_close(resumed.bass, whole.bass, 0.08, "bass");
  assert_close(resumed.mid, whole.mid, 0.08, "mid");
  assert_close(resumed.treble, whole.treble, 0.08, "treble");
  assert_close(resumed.overall, whole.overall, 0.08, "overall");
}

#[test]
fn test_many_small_chunks_match_single_chunk_for_same_signal() {
  let samples = layered_signal(ANALYSIS_WINDOW * 2 + ANALYSIS_WINDOW / 2);

  let whole = analyze_in_chunks(&samples, &[samples.len()]);
  let byte_sized = analyze_in_chunks(&samples, &vec![1; samples.len()]);

  assert_close(byte_sized.bass, whole.bass, 0.08, "bass");
  assert_close(byte_sized.mid, whole.mid, 0.08, "mid");
  assert_close(byte_sized.treble, whole.treble, 0.08, "treble");
  assert_close(byte_sized.overall, whole.overall, 0.08, "overall");
}
