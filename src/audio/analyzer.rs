use super::AudioFeatures;

#[cfg(feature = "audio")]
use rustfft::{num_complex::Complex, FftPlanner};

pub struct AudioAnalyzer {
  sample_rate: f32,
  #[cfg(feature = "audio")]
  fft_planner: FftPlanner<f32>,
  previous_bass: f32,
  bass_history: Vec<f32>,
  drop_cooldown: f32,

  // Peak envelope tracking for each band
  bass_peak: f32,
  mid_peak: f32,
  treble_peak: f32,

  // Beat pulse for rhythmic pop
  beat_pulse: f32,

  // Energy variance tracking
  energy_history: Vec<f32>,
}

impl AudioAnalyzer {
  pub fn new(sample_rate: f32) -> Self {
    Self {
      sample_rate,
      #[cfg(feature = "audio")]
      fft_planner: FftPlanner::new(),
      previous_bass: 0.0,
      bass_history: Vec::with_capacity(30),
      drop_cooldown: 0.0,
      bass_peak: 0.0,
      mid_peak: 0.0,
      treble_peak: 0.0,
      beat_pulse: 0.0,
      energy_history: Vec::with_capacity(60),
    }
  }

  pub fn analyze(&mut self, samples: &[f32], delta_time: f32) -> AudioFeatures {
    if samples.is_empty() {
      return AudioFeatures::default();
    }

    #[cfg(feature = "audio")]
    {
      self.analyze_with_fft(samples, delta_time)
    }

    #[cfg(not(feature = "audio"))]
    {
      let _ = delta_time;
      AudioFeatures::default()
    }
  }

  #[cfg(feature = "audio")]
  fn analyze_with_fft(&mut self, samples: &[f32], delta_time: f32) -> AudioFeatures {
    // Use power of 2 size for FFT
    let fft_size = samples.len().min(2048).next_power_of_two();
    let fft_size = fft_size.max(256);

    // Prepare input buffer with windowing (Hann window)
    let mut buffer: Vec<Complex<f32>> = samples[..fft_size.min(samples.len())]
      .iter()
      .enumerate()
      .map(|(i, &sample)| {
        let window = 0.5 * (1.0 - (2.0 * std::f32::consts::PI * i as f32 / fft_size as f32).cos());
        Complex::new(sample * window, 0.0)
      })
      .collect();

    // Pad if necessary
    while buffer.len() < fft_size {
      buffer.push(Complex::new(0.0, 0.0));
    }

    // Perform FFT
    let fft = self.fft_planner.plan_fft_forward(fft_size);
    fft.process(&mut buffer);

    // Calculate frequency bins
    let freq_resolution = self.sample_rate / fft_size as f32;

    // Extract raw frequency bands
    let bass_raw = self.get_band_energy(&buffer, 20.0, 250.0, freq_resolution);
    let mid_raw = self.get_band_energy(&buffer, 250.0, 2000.0, freq_resolution);
    let treble_raw = self.get_band_energy(&buffer, 2000.0, 8000.0, freq_resolution);

    // Apply envelope following for peak detection (fast attack, slow release)
    const ATTACK_RATE: f32 = 0.98; // How fast to respond to increases
    const RELEASE_RATE: f32 = 0.85; // How fast to decay

    self.bass_peak = Self::apply_envelope(self.bass_peak, bass_raw, ATTACK_RATE, RELEASE_RATE);
    self.mid_peak = Self::apply_envelope(self.mid_peak, mid_raw, ATTACK_RATE, RELEASE_RATE);
    self.treble_peak =
      Self::apply_envelope(self.treble_peak, treble_raw, ATTACK_RATE, RELEASE_RATE);

    // Dynamic range expansion: emphasize differences from the peak envelope
    // This creates the "pop" effect by making transients more prominent
    let bass = self.apply_dynamics(bass_raw, self.bass_peak);
    let mid = self.apply_dynamics(mid_raw, self.mid_peak);
    let treble = self.apply_dynamics(treble_raw, self.treble_peak);

    // Overall energy with dynamics
    let overall = (bass + mid + treble) / 3.0;

    // Track energy history for variance-based reactivity
    self.energy_history.push(overall);
    if self.energy_history.len() > 60 {
      self.energy_history.remove(0);
    }

    // Calculate energy variance (higher variance = more dynamic music)
    let energy_variance = self.calculate_energy_variance();

    // Beat detection (based on bass energy sudden increase)
    let bass_diff = bass_raw - self.previous_bass;
    let mut beat_strength = (bass_diff * 10.0).clamp(0.0, 1.0);

    // Trigger beat pulse on strong beats
    if beat_strength > 0.3 {
      self.beat_pulse = 1.0;
    }

    // Decay beat pulse
    self.beat_pulse *= 0.85;

    // Add beat pulse to beat strength for rhythmic pop
    beat_strength = (beat_strength + self.beat_pulse * 0.5).min(1.0);

    // Track bass history for drop detection
    self.bass_history.push(bass_raw);
    if self.bass_history.len() > 30 {
      self.bass_history.remove(0);
    }

    // Detect bass drop (significant increase from recent average)
    let avg_bass = self.bass_history.iter().sum::<f32>() / self.bass_history.len() as f32;
    let is_drop = if self.drop_cooldown <= 0.0 {
      let drop_detected = bass_raw > avg_bass * 2.0 && bass_diff > 0.1;
      if drop_detected {
        self.drop_cooldown = 1.0; // 1 second cooldown
      }
      drop_detected
    } else {
      self.drop_cooldown -= delta_time;
      false
    };

    self.previous_bass = bass_raw;

    // Apply variance boost: make everything more reactive when music is dynamic
    let variance_multiplier = 1.0 + energy_variance * 0.5;

    AudioFeatures {
      bass: bass * variance_multiplier,
      mid: mid * variance_multiplier,
      treble: treble * variance_multiplier,
      overall: overall * variance_multiplier,
      beat_strength,
      is_drop,
    }
  }

  #[cfg(feature = "audio")]
  fn get_band_energy(
    &self,
    fft_buffer: &[Complex<f32>],
    freq_min: f32,
    freq_max: f32,
    freq_resolution: f32,
  ) -> f32 {
    let bin_min = (freq_min / freq_resolution) as usize;
    let bin_max = ((freq_max / freq_resolution) as usize).min(fft_buffer.len() / 2);

    if bin_min >= bin_max {
      return 0.0;
    }

    let energy: f32 = fft_buffer[bin_min..bin_max]
      .iter()
      .map(|c| (c.re * c.re + c.im * c.im).sqrt())
      .sum();

    let normalized = energy / (bin_max - bin_min) as f32;
    normalized.min(1.0)
  }

  /// Apply envelope following (fast attack, slow release)
  #[cfg(feature = "audio")]
  fn apply_envelope(current_peak: f32, new_value: f32, attack_rate: f32, release_rate: f32) -> f32 {
    if new_value > current_peak {
      // Fast attack: quickly follow increases
      current_peak * attack_rate + new_value * (1.0 - attack_rate)
    } else {
      // Slow release: gradually decay
      current_peak * release_rate
    }
  }

  /// Apply dynamic range expansion to make variations more visible
  #[cfg(feature = "audio")]
  fn apply_dynamics(&self, raw_value: f32, peak: f32) -> f32 {
    if peak < 0.01 {
      return raw_value;
    }

    // Calculate ratio of current value to peak
    let ratio = raw_value / peak;

    // Apply expansion curve: emphasize values close to peak, de-emphasize quiet parts
    // This creates the "pop" effect
    let expanded = ratio.powf(0.7); // Compression exponent < 1 expands dynamic range

    // Apply a boost to transients (values close to peak)
    let transient_boost = if ratio > 0.85 {
      1.0 + (ratio - 0.85) * 2.0 // Boost peaks by up to 30%
    } else {
      1.0
    };

    (expanded * peak * transient_boost).min(1.0)
  }

  /// Calculate energy variance to detect dynamic vs. consistent music sections
  #[cfg(feature = "audio")]
  fn calculate_energy_variance(&self) -> f32 {
    if self.energy_history.len() < 10 {
      return 0.0;
    }

    let mean = self.energy_history.iter().sum::<f32>() / self.energy_history.len() as f32;

    let variance = self
      .energy_history
      .iter()
      .map(|&x| {
        let diff = x - mean;
        diff * diff
      })
      .sum::<f32>()
      / self.energy_history.len() as f32;

    // Normalize variance to 0-1 range (using sqrt for better scaling)
    variance.sqrt().min(1.0)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_apply_envelope_attack() {
    let current_peak = 0.5;
    let new_value = 0.8;
    let attack_rate = 0.9;
    let release_rate = 0.85;

    let result = AudioAnalyzer::apply_envelope(current_peak, new_value, attack_rate, release_rate);

    // Should move toward new_value (attack)
    assert!(result > current_peak);
    assert!(result < new_value); // But not fully reach it (smoothing)
    assert!((result - 0.53).abs() < 0.01); // Should be ~0.53 (0.5*0.9 + 0.8*0.1)
  }

  #[test]
  fn test_apply_envelope_release() {
    let current_peak = 0.8;
    let new_value = 0.3;
    let attack_rate = 0.9;
    let release_rate = 0.85;

    let result = AudioAnalyzer::apply_envelope(current_peak, new_value, attack_rate, release_rate);

    // Should decay (release)
    assert!(result < current_peak);
    assert!((result - 0.68).abs() < 0.01); // Should be ~0.68 (0.8*0.85)
  }

  #[test]
  fn test_apply_dynamics_with_zero_peak() {
    let analyzer = AudioAnalyzer::new(44100.0);
    let raw_value = 0.5;
    let peak = 0.0;

    let result = analyzer.apply_dynamics(raw_value, peak);

    // Should return raw value when peak is too low
    assert_eq!(result, raw_value);
  }

  #[test]
  fn test_apply_dynamics_expansion() {
    let analyzer = AudioAnalyzer::new(44100.0);
    let raw_value = 0.5;
    let peak = 1.0;

    let result = analyzer.apply_dynamics(raw_value, peak);

    // Should apply dynamics expansion
    assert!(result > 0.0);
    assert!(result <= 1.0);

    // With ratio 0.5 and power 0.7: 0.5^0.7 ≈ 0.6095
    assert!((result - 0.61).abs() < 0.05);
  }

  #[test]
  fn test_apply_dynamics_transient_boost() {
    let analyzer = AudioAnalyzer::new(44100.0);
    let raw_value_high = 0.9; // Close to peak
    let raw_value_low = 0.5; // Further from peak
    let peak = 1.0;

    let result_high = analyzer.apply_dynamics(raw_value_high, peak);
    let result_low = analyzer.apply_dynamics(raw_value_low, peak);

    // High ratio (transient) should get extra boost
    assert!(result_high > result_low);
  }

  #[test]
  fn test_calculate_energy_variance_not_enough_data() {
    let mut analyzer = AudioAnalyzer::new(44100.0);

    // Add less than 10 samples
    analyzer.energy_history.push(0.5);
    analyzer.energy_history.push(0.6);

    let variance = analyzer.calculate_energy_variance();

    // Should return 0 when not enough data
    assert_eq!(variance, 0.0);
  }

  #[test]
  fn test_calculate_energy_variance_constant_signal() {
    let mut analyzer = AudioAnalyzer::new(44100.0);

    // Add constant values
    for _ in 0..20 {
      analyzer.energy_history.push(0.5);
    }

    let variance = analyzer.calculate_energy_variance();

    // Constant signal should have very low variance
    assert!(variance < 0.01);
  }

  #[test]
  fn test_calculate_energy_variance_dynamic_signal() {
    let mut analyzer = AudioAnalyzer::new(44100.0);

    // Add varying values
    for i in 0..30 {
      let value = if i % 2 == 0 { 0.2 } else { 0.8 };
      analyzer.energy_history.push(value);
    }

    let variance = analyzer.calculate_energy_variance();

    // Dynamic signal should have higher variance
    assert!(variance > 0.2);
  }

  #[test]
  fn test_analyzer_initialization() {
    let analyzer = AudioAnalyzer::new(44100.0);

    assert_eq!(analyzer.sample_rate, 44100.0);
    assert_eq!(analyzer.previous_bass, 0.0);
    assert_eq!(analyzer.bass_peak, 0.0);
    assert_eq!(analyzer.mid_peak, 0.0);
    assert_eq!(analyzer.treble_peak, 0.0);
    assert_eq!(analyzer.beat_pulse, 0.0);
    assert_eq!(analyzer.drop_cooldown, 0.0);
  }

  #[test]
  fn test_analyze_empty_samples() {
    let mut analyzer = AudioAnalyzer::new(44100.0);
    let samples: Vec<f32> = vec![];

    let features = analyzer.analyze(&samples, 0.016);

    // Should return default features for empty input
    assert_eq!(features.bass, 0.0);
    assert_eq!(features.mid, 0.0);
    assert_eq!(features.treble, 0.0);
    assert_eq!(features.overall, 0.0);
    assert_eq!(features.beat_strength, 0.0);
    assert!(!features.is_drop);
  }

  #[cfg(feature = "audio")]
  #[test]
  fn test_get_band_energy_invalid_range() {
    let analyzer = AudioAnalyzer::new(44100.0);
    let buffer = vec![rustfft::num_complex::Complex::new(0.0, 0.0); 512];

    // Test with invalid frequency range (min >= max)
    let energy = analyzer.get_band_energy(&buffer, 1000.0, 500.0, 10.0);

    assert_eq!(energy, 0.0);
  }

  #[cfg(feature = "audio")]
  #[test]
  fn test_get_band_energy_normalization() {
    let analyzer = AudioAnalyzer::new(44100.0);

    // Create buffer with some energy
    let mut buffer = vec![rustfft::num_complex::Complex::new(0.0, 0.0); 512];
    for i in 0..10 {
      buffer[i] = rustfft::num_complex::Complex::new(1.0, 1.0);
    }

    let energy = analyzer.get_band_energy(&buffer, 0.0, 100.0, 10.0);

    // Energy should be clamped to 1.0
    assert!(energy <= 1.0);
    assert!(energy >= 0.0);
  }
}
