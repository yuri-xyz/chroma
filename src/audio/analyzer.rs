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

    // Extract frequency bands
    let bass = self.get_band_energy(&buffer, 20.0, 250.0, freq_resolution);
    let mid = self.get_band_energy(&buffer, 250.0, 2000.0, freq_resolution);
    let treble = self.get_band_energy(&buffer, 2000.0, 8000.0, freq_resolution);

    // Overall energy
    let overall = (bass + mid + treble) / 3.0;

    // Beat detection (based on bass energy sudden increase)
    let bass_diff = bass - self.previous_bass;
    let beat_strength = (bass_diff * 10.0).clamp(0.0, 1.0);

    // Track bass history for drop detection
    self.bass_history.push(bass);
    if self.bass_history.len() > 30 {
      self.bass_history.remove(0);
    }

    // Detect bass drop (significant increase from recent average)
    let avg_bass = self.bass_history.iter().sum::<f32>() / self.bass_history.len() as f32;
    let is_drop = if self.drop_cooldown <= 0.0 {
      let drop_detected = bass > avg_bass * 2.0 && bass_diff > 0.1;
      if drop_detected {
        self.drop_cooldown = 1.0; // 1 second cooldown
      }
      drop_detected
    } else {
      self.drop_cooldown -= delta_time;
      false
    };

    self.previous_bass = bass;

    AudioFeatures {
      bass,
      mid,
      treble,
      overall,
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
}
