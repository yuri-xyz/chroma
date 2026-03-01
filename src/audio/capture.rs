#[cfg(feature = "audio")]
use cpal::traits::{DeviceTrait, StreamTrait};
#[cfg(feature = "audio")]
use cpal::{FromSample, Sample, Stream, StreamConfig};
use std::sync::{Arc, Mutex};

use super::device_selector;

#[cfg(debug_assertions)]
use std::fs::OpenOptions;
#[cfg(debug_assertions)]
use std::io::Write;

pub struct AudioCapture {
  #[cfg(feature = "audio")]
  _stream: Option<Stream>,
  pub buffer: Arc<Mutex<Vec<f32>>>,
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
    let buffer = Arc::new(Mutex::new(Vec::with_capacity(4096)));
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
    buffer: Arc<Mutex<Vec<f32>>>,
  ) -> anyhow::Result<Stream>
  where
    T: Sample + cpal::SizedSample,
    f32: FromSample<T>,
  {
    let channels = config.channels as usize;

    let stream = device.build_input_stream(
      config,
      move |data: &[T], _: &cpal::InputCallbackInfo| {
        let mut buf = buffer.lock().unwrap();
        buf.clear();

        // Convert to mono and normalize using cpal's safe conversion
        for frame in data.chunks(channels) {
          let mono_sample: f32 = frame
            .iter()
            .map(|&sample| sample.to_sample::<f32>())
            .sum::<f32>()
            / channels as f32;

          buf.push(mono_sample);
        }

        // Keep buffer size manageable
        const MAX_BUFFER_SIZE: usize = 4096;
        let buf_len = buf.len();
        if buf_len > MAX_BUFFER_SIZE {
          buf.drain(0..buf_len - MAX_BUFFER_SIZE);
        }
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
      buffer: Arc::new(Mutex::new(Vec::new())),
      sample_rate: 44100.0,
    })
  }

  pub fn get_samples(&self) -> Vec<f32> {
    self.buffer.lock().unwrap().clone()
  }
}
