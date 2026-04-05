#[cfg(feature = "audio")]
use cpal::traits::{DeviceTrait, StreamTrait};
#[cfg(feature = "audio")]
use cpal::{FromSample, Sample, Stream, StreamConfig};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use super::device_selector;

#[cfg(debug_assertions)]
use std::fs::OpenOptions;
#[cfg(debug_assertions)]
use std::io::Write;

const MAX_PENDING_SAMPLES: usize = 8_192;

struct SharedSampleBuffer {
  samples: VecDeque<f32>,
  max_len: usize,
}

impl SharedSampleBuffer {
  fn with_max_len(max_len: usize) -> Self {
    Self {
      samples: VecDeque::with_capacity(max_len),
      max_len,
    }
  }

  fn push_interleaved<T>(&mut self, data: &[T], channels: usize)
  where
    T: Sample,
    f32: FromSample<T>,
  {
    for frame in data.chunks(channels) {
      let mono_sample: f32 = frame
        .iter()
        .map(|&sample| sample.to_sample::<f32>())
        .sum::<f32>()
        / channels as f32;

      if self.samples.len() == self.max_len {
        self.samples.pop_front();
      }

      self.samples.push_back(mono_sample);
    }
  }

  fn drain_samples(&mut self) -> Vec<f32> {
    self.samples.drain(..).collect()
  }
}

pub struct AudioCapture {
  #[cfg(feature = "audio")]
  _stream: Option<Stream>,
  buffer: Arc<Mutex<SharedSampleBuffer>>,
  pub sample_rate: f32,
}

impl AudioCapture {
  /// List all available audio devices across all hosts
  #[cfg(feature = "audio")]
  pub fn list_devices() -> anyhow::Result<()> {
    // Try to use the best host, fall back to default
    let host = match device_selector::find_system_audio_auto() {
      Ok((host, _)) => host,
      Err(_) => cpal::default_host(),
    };

    device_selector::list_devices(&host)
  }

  /// Create audio capture with optional device name
  #[cfg(feature = "audio")]
  pub fn new(device_name: Option<&str>) -> anyhow::Result<Self> {
    #[cfg(debug_assertions)]
    {
      let mut log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("audio_debug.log")?;

      writeln!(log_file, "\n=== Audio Capture Initialization ===")?;
    }

    // Find the device - either by name or auto-detect system audio
    let device = if let Some(name) = device_name {
      #[cfg(debug_assertions)]
      {
        let mut log_file = OpenOptions::new()
          .create(true)
          .append(true)
          .open("audio_debug.log")?;
        writeln!(log_file, "Looking for specific device: {}", name)?;
      }
      let host = cpal::default_host();
      device_selector::find_device_by_name(&host, name)?
    } else {
      // Auto-detect system audio across all available hosts
      #[cfg(debug_assertions)]
      {
        let mut log_file = OpenOptions::new()
          .create(true)
          .append(true)
          .open("audio_debug.log")?;
        writeln!(log_file, "Auto-detecting system audio source...")?;
      }
      let (_host, device) = device_selector::find_system_audio_auto()?;
      device
    };

    #[cfg(debug_assertions)]
    {
      if let Ok(desc) = device.description() {
        let mut log_file = OpenOptions::new()
          .create(true)
          .append(true)
          .open("audio_debug.log")?;
        writeln!(log_file, "Using device: {}", desc.name())?;
      }
    }

    // Get config - try input first, then output for loopback (macOS 14.2+)
    #[cfg(target_os = "macos")]
    let config = match device.default_input_config() {
      Ok(config) => config,
      Err(_) => device
        .default_output_config()
        .map_err(|e| anyhow::anyhow!("Failed to get device config: {}", e))?,
    };

    #[cfg(not(target_os = "macos"))]
    let config = device
      .default_input_config()
      .map_err(|e| anyhow::anyhow!("Failed to get device config: {}", e))?;

    #[cfg(debug_assertions)]
    {
      let mut log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("audio_debug.log")?;
      writeln!(
        log_file,
        "Config: sample_rate={}, channels={}",
        config.sample_rate(),
        config.channels()
      )?;
    }

    let sample_rate = config.sample_rate() as f32;
    let buffer = Arc::new(Mutex::new(SharedSampleBuffer::with_max_len(
      MAX_PENDING_SAMPLES,
    )));
    let buffer_clone = Arc::clone(&buffer);

    let stream = match config.sample_format() {
      cpal::SampleFormat::F32 => Self::build_stream::<f32>(&device, &config.into(), buffer_clone)?,
      cpal::SampleFormat::I16 => Self::build_stream::<i16>(&device, &config.into(), buffer_clone)?,
      cpal::SampleFormat::U16 => Self::build_stream::<u16>(&device, &config.into(), buffer_clone)?,
      _ => return Err(anyhow::anyhow!("Unsupported sample format")),
    };

    stream.play()?;

    #[cfg(debug_assertions)]
    {
      let mut log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("audio_debug.log")?;
      writeln!(log_file, "Audio stream started successfully")?;
    }

    Ok(Self {
      _stream: Some(stream),
      buffer,
      sample_rate,
    })
  }

  #[cfg(feature = "audio")]
  fn build_stream<T>(
    device: &cpal::Device,
    config: &StreamConfig,
    buffer: Arc<Mutex<SharedSampleBuffer>>,
  ) -> anyhow::Result<Stream>
  where
    T: Sample + cpal::SizedSample,
    f32: FromSample<T>,
  {
    let channels = config.channels as usize;

    let stream = device.build_input_stream(
      config,
      move |data: &[T], _: &cpal::InputCallbackInfo| {
        buffer.lock().unwrap().push_interleaved(data, channels);
      },
      |err| {
        // Log audio stream errors to file (debug only)
        #[cfg(debug_assertions)]
        {
          if let Ok(mut log_file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open("audio_debug.log")
          {
            writeln!(log_file, "Audio stream error: {}", err).ok();
          }
        }
        // Suppress warning about err being unused in release mode
        #[cfg(not(debug_assertions))]
        {
          let _ = err;
        }
      },
      None,
    )?;

    Ok(stream)
  }

  #[cfg(not(feature = "audio"))]
  pub fn new(_device_name: Option<&str>) -> anyhow::Result<Self> {
    Ok(Self {
      buffer: Arc::new(Mutex::new(SharedSampleBuffer::with_max_len(
        MAX_PENDING_SAMPLES,
      ))),
      sample_rate: 44100.0,
    })
  }

  pub fn drain_samples(&self) -> Vec<f32> {
    self.buffer.lock().unwrap().drain_samples()
  }
}

#[cfg(all(test, feature = "audio"))]
mod tests {
  use super::SharedSampleBuffer;

  #[test]
  fn test_shared_sample_buffer_accumulates_across_pushes() {
    let mut buffer = SharedSampleBuffer::with_max_len(8);

    buffer.push_interleaved(&[0.2_f32, 0.4_f32, 0.6_f32, 0.8_f32], 2);
    buffer.push_interleaved(&[1.0_f32, 0.0_f32, 0.5_f32, 0.5_f32], 2);

    let samples = buffer.drain_samples();
    let expected = [0.3_f32, 0.7, 0.5, 0.5];

    assert_eq!(samples.len(), expected.len());
    for (actual, expected) in samples.into_iter().zip(expected) {
      assert!((actual - expected).abs() < 1e-6);
    }
  }

  #[test]
  fn test_shared_sample_buffer_enforces_bounded_history() {
    let mut buffer = SharedSampleBuffer::with_max_len(3);

    buffer.push_interleaved(&[0.1_f32, 0.2_f32, 0.3_f32, 0.4_f32], 1);

    assert_eq!(buffer.drain_samples(), vec![0.2, 0.3, 0.4]);
  }

  #[test]
  fn test_shared_sample_buffer_drain_clears_pending_samples() {
    let mut buffer = SharedSampleBuffer::with_max_len(4);

    buffer.push_interleaved(&[0.25_f32, 0.75_f32], 1);
    assert_eq!(buffer.drain_samples(), vec![0.25, 0.75]);
    assert!(buffer.drain_samples().is_empty());
  }
}
