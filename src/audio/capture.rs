#[cfg(feature = "audio")]
use cpal::traits::{DeviceTrait, StreamTrait};
#[cfg(feature = "audio")]
use cpal::{Stream, StreamConfig};
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::{Arc, Mutex};

use super::device_selector;

pub struct AudioCapture {
  #[cfg(feature = "audio")]
  _stream: Option<Stream>,
  pub buffer: Arc<Mutex<Vec<f32>>>,
  pub sample_rate: f32,
}

impl AudioCapture {
  /// List all available audio devices
  #[cfg(feature = "audio")]
  pub fn list_devices() -> anyhow::Result<()> {
    let host = cpal::default_host();

    device_selector::list_devices(&host)
  }

  /// Create audio capture with optional device name
  #[cfg(feature = "audio")]
  pub fn new(device_name: Option<&str>) -> anyhow::Result<Self> {
    let mut log_file = OpenOptions::new()
      .create(true)
      .append(true)
      .open("audio_debug.log")?;

    writeln!(log_file, "\n=== Audio Capture Initialization ===")?;

    let host = cpal::default_host();

    writeln!(log_file, "CPAL host: {:?}", host.id())?;

    // Try to find the audio device
    let device = if let Some(name) = device_name {
      writeln!(log_file, "Looking for specific device: {}", name)?;
      device_selector::find_device_by_name(&host, name)?
    } else {
      // Auto-detect system audio device (monitor source)
      device_selector::find_system_audio_device(&host)?
    };

    if let Ok(device_name) = device.name() {
      writeln!(log_file, "Using device: {}", device_name)?;
    }

    let config = device
      .default_input_config()
      .map_err(|e| anyhow::anyhow!("Failed to get device config: {}", e))?;

    writeln!(
      log_file,
      "Config: sample_rate={}, channels={}",
      config.sample_rate().0,
      config.channels()
    )?;

    let sample_rate = config.sample_rate().0 as f32;
    let buffer = Arc::new(Mutex::new(Vec::with_capacity(4096)));
    let buffer_clone = Arc::clone(&buffer);

    let stream = match config.sample_format() {
      cpal::SampleFormat::F32 => Self::build_stream::<f32>(&device, &config.into(), buffer_clone)?,
      cpal::SampleFormat::I16 => Self::build_stream::<i16>(&device, &config.into(), buffer_clone)?,
      cpal::SampleFormat::U16 => Self::build_stream::<u16>(&device, &config.into(), buffer_clone)?,
      _ => return Err(anyhow::anyhow!("Unsupported sample format")),
    };

    stream.play()?;
    writeln!(log_file, "Audio stream started successfully")?;

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
    T: cpal::Sample + cpal::SizedSample,
  {
    let channels = config.channels as usize;

    let stream = device.build_input_stream(
      config,
      move |data: &[T], _: &cpal::InputCallbackInfo| {
        let mut buf = buffer.lock().unwrap();
        buf.clear();

        // Convert to mono and normalize
        for frame in data.chunks(channels) {
          let mono_sample: f32 = frame.iter().fold(0.0f32, |acc, &sample| {
            // Convert sample to f32 using cpal's conversion
            let s = if std::mem::size_of::<T>() == std::mem::size_of::<f32>() {
              unsafe { std::mem::transmute_copy(&sample) }
            } else if std::mem::size_of::<T>() == std::mem::size_of::<i16>() {
              let i: i16 = unsafe { std::mem::transmute_copy(&sample) };

              i as f32 / i16::MAX as f32
            } else if std::mem::size_of::<T>() == std::mem::size_of::<u16>() {
              let u: u16 = unsafe { std::mem::transmute_copy(&sample) };

              (u as f32 / u16::MAX as f32) * 2.0 - 1.0
            } else {
              0.0f32
            };

            acc + s
          }) / channels as f32;

          buf.push(mono_sample);
        }

        // Keep buffer size manageable
        let buf_len = buf.len();

        if buf_len > 4096 {
          buf.drain(0..buf_len - 4096);
        }
      },
      |err| {
        // Log audio stream errors to file instead of stderr
        if let Ok(mut log_file) = OpenOptions::new()
          .create(true)
          .append(true)
          .open("audio_debug.log")
        {
          writeln!(log_file, "Audio stream error: {}", err).ok();
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
