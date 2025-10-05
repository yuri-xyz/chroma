#[cfg(feature = "audio")]
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
#[cfg(feature = "audio")]
use cpal::{Device, Stream, StreamConfig};
use std::sync::{Arc, Mutex};

pub struct AudioCapture {
  #[cfg(feature = "audio")]
  _stream: Option<Stream>,
  pub buffer: Arc<Mutex<Vec<f32>>>,
  pub sample_rate: f32,
}

impl AudioCapture {
  #[cfg(feature = "audio")]
  pub fn new() -> anyhow::Result<Self> {
    let host = cpal::default_host();

    // Try to get default input device (microphone/system audio)
    let device = host
      .default_input_device()
      .ok_or_else(|| anyhow::anyhow!("No input device available. Check system audio settings."))?;

    let config = device
      .default_input_config()
      .map_err(|e| anyhow::anyhow!("Failed to get device config: {}", e))?;

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

    Ok(Self {
      _stream: Some(stream),
      buffer,
      sample_rate,
    })
  }

  #[cfg(feature = "audio")]
  fn build_stream<T>(
    device: &Device,
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
          // Use Sample trait to convert each sample
          // Sample values are in the range of the sample type (e.g., -1.0 to 1.0 for floats)
          let mono_sample: f32 = frame.iter().fold(0.0f32, |acc, &sample| {
            // Convert sample to f32 using cpal's conversion
            let s = if std::mem::size_of::<T>() == std::mem::size_of::<f32>() {
              // If T is f32, transmute directly
              unsafe { std::mem::transmute_copy(&sample) }
            } else if std::mem::size_of::<T>() == std::mem::size_of::<i16>() {
              // If T is i16, convert to f32
              let i: i16 = unsafe { std::mem::transmute_copy(&sample) };
              i as f32 / i16::MAX as f32
            } else if std::mem::size_of::<T>() == std::mem::size_of::<u16>() {
              // If T is u16, convert to f32
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
      |err| eprintln!("Audio stream error: {}", err),
      None,
    )?;

    Ok(stream)
  }

  #[cfg(not(feature = "audio"))]
  pub fn new() -> anyhow::Result<Self> {
    Ok(Self {
      buffer: Arc::new(Mutex::new(Vec::new())),
      sample_rate: 44100.0,
    })
  }

  pub fn get_samples(&self) -> Vec<f32> {
    self.buffer.lock().unwrap().clone()
  }
}
