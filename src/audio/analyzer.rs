use std::{collections::VecDeque, ops::Range, sync::Arc};

use rustfft::{num_complex::Complex, Fft, FftPlanner};

use super::AudioFeatures;

pub const ANALYSIS_WINDOW_SIZE: usize = 2_048;
pub const ANALYSIS_HOP_SIZE: usize = 512;

const BASS_HISTORY_SIZE: usize = 30;
const ENERGY_HISTORY_SIZE: usize = 60;
const MIN_WARMUP_WINDOWS: usize = 4;
const BEAT_PULSE_DECAY: f32 = 0.78;
const BASS_DROP_COOLDOWN_SECS: f32 = 1.0;

struct RollingHistory {
  values: VecDeque<f32>,
  capacity: usize,
  sum: f32,
  sum_squares: f32,
}

impl RollingHistory {
  fn with_capacity(capacity: usize) -> Self {
    Self {
      values: VecDeque::with_capacity(capacity),
      capacity,
      sum: 0.0,
      sum_squares: 0.0,
    }
  }

  fn push(&mut self, value: f32) {
    if self.values.len() == self.capacity {
      if let Some(removed) = self.values.pop_front() {
        self.sum -= removed;
        self.sum_squares -= removed * removed;
      }
    }

    self.values.push_back(value);
    self.sum += value;
    self.sum_squares += value * value;
  }

  fn len(&self) -> usize {
    self.values.len()
  }

  fn is_empty(&self) -> bool {
    self.values.is_empty()
  }

  fn mean(&self) -> f32 {
    if self.is_empty() {
      0.0
    } else {
      self.sum / self.len() as f32
    }
  }

  fn stddev(&self) -> f32 {
    if self.is_empty() {
      return 0.0;
    }

    let mean = self.mean();
    let mean_of_squares = self.sum_squares / self.len() as f32;
    let variance = (mean_of_squares - mean * mean).max(0.0);

    variance.sqrt()
  }

  fn iter(&self) -> impl Iterator<Item = &f32> {
    self.values.iter()
  }
}

pub struct AudioAnalyzer {
  sample_rate: f32,
  window_size: usize,
  hop_size: usize,
  sample_buffer: Vec<f32>,
  sample_buffer_start: usize,
  fft: Arc<dyn Fft<f32>>,
  fft_buffer: Vec<Complex<f32>>,
  window_coefficients: Vec<f32>,
  bass_bin_range: Range<usize>,
  mid_bin_range: Range<usize>,
  treble_bin_range: Range<usize>,
  previous_bass: f32,
  bass_history: RollingHistory,
  drop_cooldown: f32,
  bass_peak: f32,
  mid_peak: f32,
  treble_peak: f32,
  beat_pulse: f32,
  energy_history: RollingHistory,
  processed_windows: usize,
  latest_features: AudioFeatures,
}

impl AudioAnalyzer {
  pub fn new(sample_rate: f32) -> Self {
    Self::with_window(sample_rate, ANALYSIS_WINDOW_SIZE, ANALYSIS_HOP_SIZE)
  }

  pub fn with_window(sample_rate: f32, window_size: usize, hop_size: usize) -> Self {
    assert!(window_size > 0, "window size must be positive");
    assert!(hop_size > 0, "hop size must be positive");
    assert!(
      hop_size <= window_size,
      "hop size must not exceed window size"
    );

    let mut fft_planner = FftPlanner::new();
    let fft = fft_planner.plan_fft_forward(window_size);
    let freq_resolution = sample_rate / window_size as f32;

    Self {
      sample_rate,
      window_size,
      hop_size,
      sample_buffer: Vec::with_capacity(window_size * 2),
      sample_buffer_start: 0,
      fft,
      fft_buffer: vec![Complex::new(0.0, 0.0); window_size],
      window_coefficients: Self::build_hann_window(window_size),
      bass_bin_range: Self::band_bin_range(20.0, 250.0, freq_resolution, window_size),
      mid_bin_range: Self::band_bin_range(250.0, 2_000.0, freq_resolution, window_size),
      treble_bin_range: Self::band_bin_range(2_000.0, 8_000.0, freq_resolution, window_size),
      previous_bass: 0.0,
      bass_history: RollingHistory::with_capacity(BASS_HISTORY_SIZE),
      drop_cooldown: 0.0,
      bass_peak: 0.0,
      mid_peak: 0.0,
      treble_peak: 0.0,
      beat_pulse: 0.0,
      energy_history: RollingHistory::with_capacity(ENERGY_HISTORY_SIZE),
      processed_windows: 0,
      latest_features: AudioFeatures::default(),
    }
  }

  pub fn analyze(&mut self, samples: &[f32], _delta_time: f32) -> AudioFeatures {
    if samples.is_empty() {
      return self.latest_features;
    }

    self.sample_buffer.extend_from_slice(samples);

    while self.available_sample_count() >= self.window_size {
      self.populate_fft_buffer_from_samples();
      self.latest_features = self.analyze_window();
      self.processed_windows += 1;
      self.advance_sample_buffer();
    }

    self.latest_features
  }

  fn populate_fft_buffer_from_samples(&mut self) {
    let window =
      &self.sample_buffer[self.sample_buffer_start..self.sample_buffer_start + self.window_size];
    Self::write_window_slice(
      &mut self.fft_buffer[..self.window_size],
      window,
      &self.window_coefficients[..self.window_size],
    );
  }

  fn available_sample_count(&self) -> usize {
    self
      .sample_buffer
      .len()
      .saturating_sub(self.sample_buffer_start)
  }

  fn advance_sample_buffer(&mut self) {
    self.sample_buffer_start += self.hop_size;

    if self.sample_buffer_start >= self.window_size {
      self.compact_sample_buffer();
    }
  }

  fn compact_sample_buffer(&mut self) {
    if self.sample_buffer_start == 0 {
      return;
    }

    self
      .sample_buffer
      .copy_within(self.sample_buffer_start.., 0);
    let remaining_len = self.sample_buffer.len() - self.sample_buffer_start;
    self.sample_buffer.truncate(remaining_len);
    self.sample_buffer_start = 0;
  }

  fn write_window_slice(
    fft_bins: &mut [Complex<f32>],
    samples: &[f32],
    window_coefficients: &[f32],
  ) {
    for (bin, (&sample, &window)) in fft_bins
      .iter_mut()
      .zip(samples.iter().zip(window_coefficients.iter()))
    {
      *bin = Complex::new(sample * window, 0.0);
    }
  }

  fn analyze_window(&mut self) -> AudioFeatures {
    self.fft.process(&mut self.fft_buffer);

    let bass_raw = Self::get_band_energy(&self.fft_buffer, &self.bass_bin_range);
    let mid_raw = Self::get_band_energy(&self.fft_buffer, &self.mid_bin_range);
    let treble_raw = Self::get_band_energy(&self.fft_buffer, &self.treble_bin_range);

    const ATTACK_RATE: f32 = 0.98;
    const RELEASE_RATE: f32 = 0.85;

    self.bass_peak = Self::apply_envelope(self.bass_peak, bass_raw, ATTACK_RATE, RELEASE_RATE);
    self.mid_peak = Self::apply_envelope(self.mid_peak, mid_raw, ATTACK_RATE, RELEASE_RATE);
    self.treble_peak =
      Self::apply_envelope(self.treble_peak, treble_raw, ATTACK_RATE, RELEASE_RATE);

    let bass = self.apply_dynamics(bass_raw, self.bass_peak);
    let mid = self.apply_dynamics(mid_raw, self.mid_peak);
    let treble = self.apply_dynamics(treble_raw, self.treble_peak);
    let overall = (bass + mid + treble) / 3.0;

    self.energy_history.push(overall);
    let energy_variance = self.calculate_energy_variance();

    let beat_strength = self.detect_beat(bass_raw, mid_raw, treble_raw, energy_variance);
    let is_drop = self.detect_drop(bass_raw, mid_raw, treble_raw, beat_strength);
    self.bass_history.push(bass_raw);

    self.previous_bass = bass_raw;

    let variance_multiplier = 1.0 + energy_variance * 0.35;

    AudioFeatures {
      bass: (bass * variance_multiplier).min(1.0),
      mid: (mid * variance_multiplier).min(1.0),
      treble: (treble * variance_multiplier).min(1.0),
      overall: (overall * variance_multiplier).min(1.0),
      beat_strength,
      is_drop,
    }
  }

  fn detect_beat(
    &mut self,
    bass_raw: f32,
    mid_raw: f32,
    treble_raw: f32,
    energy_variance: f32,
  ) -> f32 {
    if self.processed_windows < MIN_WARMUP_WINDOWS {
      self.beat_pulse *= BEAT_PULSE_DECAY;
      return 0.0;
    }

    let average_bass = self.bass_history.mean().max(0.01);
    let bass_flux = (bass_raw - self.previous_bass).max(0.0);
    let relative_flux = bass_flux / average_bass;
    let adaptive_threshold = 0.10 + energy_variance * 0.08;
    let max_non_bass = mid_raw.max(treble_raw);
    let bass_focus = ((bass_raw - max_non_bass * 0.6) / (bass_raw + 1e-4)).clamp(0.0, 1.0);
    let onset_strength = ((relative_flux - adaptive_threshold) / 0.55).clamp(0.0, 1.0) * bass_focus;

    self.beat_pulse = (self.beat_pulse * BEAT_PULSE_DECAY).max(onset_strength);

    (onset_strength * 0.85 + self.beat_pulse * 0.35).min(1.0)
  }

  fn detect_drop(
    &mut self,
    bass_raw: f32,
    mid_raw: f32,
    treble_raw: f32,
    beat_strength: f32,
  ) -> bool {
    let window_delta_time = self.hop_size as f32 / self.sample_rate;

    if self.drop_cooldown > 0.0 {
      self.drop_cooldown = (self.drop_cooldown - window_delta_time).max(0.0);
      return false;
    }

    if self.processed_windows < MIN_WARMUP_WINDOWS || self.bass_history.len() < 8 {
      return false;
    }

    let average_bass = self.bass_history.mean();
    if average_bass < 0.03 {
      return false;
    }
    let active_bass_windows = self
      .bass_history
      .iter()
      .filter(|&&value| value > 0.03)
      .count();
    if active_bass_windows < 4 {
      return false;
    }

    let bass_flux = (bass_raw - self.previous_bass).max(0.0);
    let bass_dominates = bass_raw > mid_raw.max(treble_raw) * 0.85;
    let drop_detected = bass_raw > average_bass * 1.5
      && bass_dominates
      && (bass_flux > average_bass * 0.18 + 0.015 || beat_strength > 0.12);

    if drop_detected {
      self.drop_cooldown = BASS_DROP_COOLDOWN_SECS;
    }

    drop_detected
  }

  fn get_band_energy(fft_buffer: &[Complex<f32>], bin_range: &Range<usize>) -> f32 {
    if bin_range.start >= bin_range.end || bin_range.start >= fft_buffer.len() / 2 {
      return 0.0;
    }

    let bin_end = bin_range.end.min(fft_buffer.len() / 2);
    let power = fft_buffer[bin_range.start..bin_end]
      .iter()
      .map(|value| value.norm_sqr())
      .sum::<f32>();

    let rms = power.sqrt();

    rms / (1.0 + rms)
  }

  fn build_hann_window(window_size: usize) -> Vec<f32> {
    (0..window_size)
      .map(|index| {
        0.5 * (1.0 - (2.0 * std::f32::consts::PI * index as f32 / window_size as f32).cos())
      })
      .collect()
  }

  fn band_bin_range(
    freq_min: f32,
    freq_max: f32,
    freq_resolution: f32,
    fft_size: usize,
  ) -> Range<usize> {
    let bin_min = (freq_min / freq_resolution) as usize;
    let bin_max = ((freq_max / freq_resolution) as usize).min(fft_size / 2);

    bin_min..bin_max
  }

  fn apply_envelope(current_peak: f32, new_value: f32, attack_rate: f32, release_rate: f32) -> f32 {
    if new_value > current_peak {
      current_peak * attack_rate + new_value * (1.0 - attack_rate)
    } else {
      current_peak * release_rate
    }
  }

  fn apply_dynamics(&self, raw_value: f32, peak: f32) -> f32 {
    if peak < 0.01 {
      return raw_value;
    }

    let ratio = (raw_value / peak).clamp(0.0, 1.2);
    let expanded = ratio.powf(0.7);
    let transient_boost = if ratio > 0.85 {
      1.0 + (ratio - 0.85) * 1.5
    } else {
      1.0
    };

    (expanded * peak * transient_boost).min(1.0)
  }

  fn calculate_energy_variance(&self) -> f32 {
    if self.energy_history.len() < 10 {
      return 0.0;
    }

    self.energy_history.stddev().min(1.0)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_apply_envelope_attack() {
    let current_peak = 0.5;
    let new_value = 0.8;
    let result = AudioAnalyzer::apply_envelope(current_peak, new_value, 0.9, 0.85);

    assert!(result > current_peak);
    assert!(result < new_value);
    assert!((result - 0.53).abs() < 0.01);
  }

  #[test]
  fn test_apply_envelope_release() {
    let current_peak = 0.8;
    let new_value = 0.3;
    let result = AudioAnalyzer::apply_envelope(current_peak, new_value, 0.9, 0.85);

    assert!(result < current_peak);
    assert!((result - 0.68).abs() < 0.01);
  }

  #[test]
  fn test_apply_dynamics_with_zero_peak() {
    let analyzer = AudioAnalyzer::new(44_100.0);
    let result = analyzer.apply_dynamics(0.5, 0.0);

    assert_eq!(result, 0.5);
  }

  #[test]
  fn test_apply_dynamics_expansion() {
    let analyzer = AudioAnalyzer::new(44_100.0);
    let result = analyzer.apply_dynamics(0.5, 1.0);

    assert!(result > 0.0);
    assert!(result <= 1.0);
    assert!((result - 0.61).abs() < 0.05);
  }

  #[test]
  fn test_apply_dynamics_transient_boost() {
    let analyzer = AudioAnalyzer::new(44_100.0);
    let result_high = analyzer.apply_dynamics(0.9, 1.0);
    let result_low = analyzer.apply_dynamics(0.5, 1.0);

    assert!(result_high > result_low);
  }

  #[test]
  fn test_calculate_energy_variance_not_enough_data() {
    let mut analyzer = AudioAnalyzer::new(44_100.0);
    analyzer.energy_history.push(0.5);
    analyzer.energy_history.push(0.6);

    assert_eq!(analyzer.calculate_energy_variance(), 0.0);
  }

  #[test]
  fn test_calculate_energy_variance_constant_signal() {
    let mut analyzer = AudioAnalyzer::new(44_100.0);

    for _ in 0..20 {
      analyzer.energy_history.push(0.5);
    }

    assert!(analyzer.calculate_energy_variance() < 0.01);
  }

  #[test]
  fn test_calculate_energy_variance_dynamic_signal() {
    let mut analyzer = AudioAnalyzer::new(44_100.0);

    for index in 0..30 {
      analyzer
        .energy_history
        .push(if index % 2 == 0 { 0.2 } else { 0.8 });
    }

    assert!(analyzer.calculate_energy_variance() > 0.2);
  }

  #[test]
  fn test_analyzer_initialization() {
    let analyzer = AudioAnalyzer::new(44_100.0);

    assert_eq!(analyzer.sample_rate, 44_100.0);
    assert_eq!(analyzer.window_size, ANALYSIS_WINDOW_SIZE);
    assert_eq!(analyzer.hop_size, ANALYSIS_HOP_SIZE);
    assert_eq!(analyzer.previous_bass, 0.0);
    assert_eq!(analyzer.bass_peak, 0.0);
    assert_eq!(analyzer.mid_peak, 0.0);
    assert_eq!(analyzer.treble_peak, 0.0);
    assert_eq!(analyzer.beat_pulse, 0.0);
    assert_eq!(analyzer.drop_cooldown, 0.0);
    assert_eq!(analyzer.processed_windows, 0);
    assert!(analyzer.sample_buffer.is_empty());
    assert_eq!(analyzer.sample_buffer_start, 0);
    assert_eq!(analyzer.fft_buffer.len(), ANALYSIS_WINDOW_SIZE);
    assert_eq!(analyzer.window_coefficients.len(), ANALYSIS_WINDOW_SIZE);
    assert!(analyzer.bass_bin_range.start < analyzer.bass_bin_range.end);
    assert!(analyzer.mid_bin_range.start < analyzer.mid_bin_range.end);
    assert!(analyzer.treble_bin_range.start < analyzer.treble_bin_range.end);
  }

  #[test]
  fn test_analyze_empty_samples_returns_latest_features() {
    let mut analyzer = AudioAnalyzer::new(44_100.0);

    let features = analyzer.analyze(&[], 0.016);

    assert_eq!(features.bass, 0.0);
    assert_eq!(features.mid, 0.0);
    assert_eq!(features.treble, 0.0);
    assert_eq!(features.overall, 0.0);
    assert_eq!(features.beat_strength, 0.0);
    assert!(!features.is_drop);
  }

  #[test]
  fn test_sample_buffer_compacts_after_hop_advance() {
    let mut analyzer = AudioAnalyzer::with_window(44_100.0, 8, 2);
    analyzer.sample_buffer = (0..12).map(|value| value as f32).collect();
    analyzer.sample_buffer_start = 8;

    analyzer.compact_sample_buffer();

    assert_eq!(analyzer.sample_buffer_start, 0);
    assert_eq!(analyzer.sample_buffer, vec![8.0, 9.0, 10.0, 11.0]);
  }

  #[test]
  fn test_available_sample_count_respects_buffer_start_offset() {
    let mut analyzer = AudioAnalyzer::with_window(44_100.0, 8, 2);
    analyzer.sample_buffer = (0..10).map(|value| value as f32).collect();
    analyzer.sample_buffer_start = 3;

    assert_eq!(analyzer.available_sample_count(), 7);
  }

  #[test]
  fn test_advance_sample_buffer_skips_hop_without_immediate_compaction() {
    let mut analyzer = AudioAnalyzer::with_window(44_100.0, 8, 2);
    analyzer.sample_buffer = (0..10).map(|value| value as f32).collect();

    analyzer.advance_sample_buffer();

    assert_eq!(analyzer.sample_buffer_start, 2);
    assert_eq!(analyzer.sample_buffer.len(), 10);
    assert_eq!(analyzer.available_sample_count(), 8);
  }

  #[test]
  fn test_get_band_energy_invalid_range() {
    let buffer = vec![rustfft::num_complex::Complex::new(0.0, 0.0); 512];
    let invalid_range = std::ops::Range {
      start: 100,
      end: 50,
    };

    let energy = AudioAnalyzer::get_band_energy(&buffer, &invalid_range);

    assert_eq!(energy, 0.0);
  }

  #[test]
  fn test_get_band_energy_normalization() {
    let mut buffer = vec![rustfft::num_complex::Complex::new(0.0, 0.0); 512];

    for sample in buffer.iter_mut().take(10) {
      *sample = rustfft::num_complex::Complex::new(1.0, 1.0);
    }

    let energy = AudioAnalyzer::get_band_energy(&buffer, &(0..10));

    assert!((0.0..=1.0).contains(&energy));
  }
}
