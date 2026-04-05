#[cfg(feature = "audio")]
use cpal::traits::{DeviceTrait, StreamTrait};
#[cfg(feature = "audio")]
use cpal::{FromSample, Sample, Stream, StreamConfig};
use crate::debug::append_debug_line;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use super::device_selector;

const MAX_PENDING_SAMPLES: usize = 8_192;
const EMPTY_DRAIN_LOG_INTERVAL: u64 = 120;
const POPULATED_DRAIN_LOG_INTERVAL: u64 = 60;
const SILENT_CALLBACK_WARNING_THRESHOLD: u64 = 180;

#[derive(Debug, Clone, Copy)]
struct CallbackLogSummary {
  callback_count: u64,
  frame_count: usize,
  max_abs_sample: f32,
  buffered_samples: usize,
}

struct SharedSampleBuffer {
  samples: VecDeque<f32>,
  max_len: usize,
  callback_count: u64,
  drain_count: u64,
  total_samples_received: u64,
  total_samples_drained: u64,
  all_zero_callback_streak: u64,
  emitted_silence_warning: bool,
}

impl SharedSampleBuffer {
  fn with_max_len(max_len: usize) -> Self {
    Self {
      samples: VecDeque::with_capacity(max_len),
      max_len,
      callback_count: 0,
      drain_count: 0,
      total_samples_received: 0,
      total_samples_drained: 0,
      all_zero_callback_streak: 0,
      emitted_silence_warning: false,
    }
  }

  fn push_interleaved<T>(&mut self, data: &[T], channels: usize) -> Option<CallbackLogSummary>
  where
    T: Sample,
    f32: FromSample<T>,
  {
    self.callback_count += 1;
    let mut max_abs_sample = 0.0_f32;

    for frame in data.chunks(channels) {
      let mono_sample: f32 = frame
        .iter()
        .map(|&sample| sample.to_sample::<f32>())
        .sum::<f32>()
        / channels as f32;
      max_abs_sample = max_abs_sample.max(mono_sample.abs());

      if self.samples.len() == self.max_len {
        self.samples.pop_front();
      }

      self.samples.push_back(mono_sample);
    }

    let frame_count = data.len() / channels;
    self.total_samples_received += frame_count as u64;

    if max_abs_sample <= f32::EPSILON {
      self.all_zero_callback_streak += 1;
    } else {
      self.all_zero_callback_streak = 0;
      self.emitted_silence_warning = false;
    }

    let should_log = self.callback_count <= 3
      || max_abs_sample > 0.01
      || self.callback_count % 240 == 0;

    should_log.then_some(CallbackLogSummary {
      callback_count: self.callback_count,
      frame_count,
      max_abs_sample,
      buffered_samples: self.samples.len(),
    })
  }

  fn drain_samples(&mut self) -> (Vec<f32>, u64, u64, u64) {
    self.drain_count += 1;
    let drain_count = self.drain_count;
    let callback_count = self.callback_count;
    let drained = self.samples.drain(..).collect::<Vec<_>>();
    self.total_samples_drained += drained.len() as u64;

    (
      drained,
      drain_count,
      callback_count,
      self.total_samples_received,
    )
  }

  fn take_silence_warning(&mut self) -> bool {
    if self.all_zero_callback_streak >= SILENT_CALLBACK_WARNING_THRESHOLD && !self.emitted_silence_warning {
      self.emitted_silence_warning = true;
      return true;
    }

    false
  }
}

pub struct AudioCapture {
  #[cfg(feature = "audio")]
  _stream: Option<Stream>,
  buffer: Arc<Mutex<SharedSampleBuffer>>,
  pub sample_rate: f32,
  #[cfg(feature = "audio")]
  using_output_config_fallback: bool,
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
    append_debug_line("audio", "=== Audio Capture Initialization ===");

    // Find the device - either by name or auto-detect system audio
    let (host, device) = if let Some(name) = device_name {
      append_debug_line("audio", format!("Looking for specific device: {name}"));
      device_selector::find_device_by_name_auto(name)?
    } else {
      // Auto-detect system audio across all available hosts
      append_debug_line("audio", "Auto-detecting system audio source...");
      device_selector::find_system_audio_auto()?
    };

    let device_name = device
      .description()
      .ok()
      .map(|desc| desc.name().to_string())
      .unwrap_or_else(|| "<unnamed-device>".to_string());
    append_debug_line(
      "audio",
      format!("Using host {:?} and device '{device_name}'", host.id()),
    );

    let input_config_result = device.default_input_config();
    let output_config_result = device.default_output_config();

    append_debug_line(
      "audio",
      format!(
        "Device config availability for '{device_name}': input={}, output={}",
        input_config_result
          .as_ref()
          .map(|config| format!(
            "ok(sample_rate={}, channels={}, format={:?})",
            config.sample_rate(),
            config.channels(),
            config.sample_format()
          ))
          .unwrap_or_else(|error| format!("err({error})")),
        output_config_result
          .as_ref()
          .map(|config| format!(
            "ok(sample_rate={}, channels={}, format={:?})",
            config.sample_rate(),
            config.channels(),
            config.sample_format()
          ))
          .unwrap_or_else(|error| format!("err({error})")),
      ),
    );

    // Get config - try input first, then output for loopback (macOS)
    #[cfg(target_os = "macos")]
    let (config, config_source) = match input_config_result {
      Ok(config) => (config, "default-input-config"),
      Err(input_error) => match output_config_result {
        Ok(config) => {
          append_debug_line(
            "audio",
            format!(
              "Falling back to output config for '{device_name}' after input config error: {input_error}"
            ),
          );
          (config, "default-output-config")
        }
        Err(output_error) => {
          return Err(anyhow::anyhow!(
            "Failed to get device config. input_error={input_error}, output_error={output_error}"
          ));
        }
      },
    };

    #[cfg(not(target_os = "macos"))]
    let (config, config_source) = (
      input_config_result.map_err(|e| anyhow::anyhow!("Failed to get device config: {}", e))?,
      "default-input-config",
    );

    append_debug_line(
      "audio",
      format!(
        "Using {config_source} for '{device_name}': sample_rate={}, channels={}, format={:?}",
        config.sample_rate(),
        config.channels(),
        config.sample_format()
      ),
    );
    let using_output_config_fallback = config_source == "default-output-config";
    if using_output_config_fallback {
      append_debug_line(
        "audio",
        format!(
          "Using output-config fallback for '{device_name}'. On macOS this may still produce silent buffers unless the device truly supports loopback."
        ),
      );
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

    append_debug_line("audio", "Audio stream started successfully");

    Ok(Self {
      _stream: Some(stream),
      buffer,
      sample_rate,
      using_output_config_fallback,
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
    let device_name = device
      .description()
      .ok()
      .map(|description| description.name().to_string())
      .unwrap_or_else(|| "<unnamed-device>".to_string());

    append_debug_line(
      "audio",
      format!(
        "Building input stream for '{device_name}': channels={}, sample_rate={}",
        channels,
        config.sample_rate
      ),
    );
    let callback_device_name = device_name.clone();
    let error_device_name = device_name.clone();

    let stream = device.build_input_stream(
      config,
      move |data: &[T], _: &cpal::InputCallbackInfo| {
        if let Some(summary) = buffer.lock().unwrap().push_interleaved(data, channels) {
          append_debug_line(
            "audio",
            format!(
              "Input callback #{} for '{callback_device_name}': frames={}, max_abs_sample={:.5}, buffered_samples={}",
              summary.callback_count,
              summary.frame_count,
              summary.max_abs_sample,
              summary.buffered_samples
            ),
          );
        }
      },
      move |err| {
        append_debug_line(
          "audio",
          format!("Audio stream error for '{error_device_name}': {err}"),
        );
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
      #[cfg(feature = "audio")]
      using_output_config_fallback: false,
    })
  }

  pub fn drain_samples(&self) -> Vec<f32> {
    let (samples, drain_count, callback_count, total_samples_received, should_warn_zero_stream) = {
      let mut buffer = self.buffer.lock().unwrap();
      let (samples, drain_count, callback_count, total_samples_received) = buffer.drain_samples();
      let should_warn_zero_stream = buffer.take_silence_warning();

      (
        samples,
        drain_count,
        callback_count,
        total_samples_received,
        should_warn_zero_stream,
      )
    };

    if should_warn_zero_stream && self.using_output_config_fallback {
      append_debug_line(
        "audio",
        "WARNING: received a long run of all-zero callbacks while using macOS output-config fallback. This usually means the selected output device is not providing real system-audio loopback. Install/select BlackHole or another loopback-capable source.",
      );
    }

    if samples.is_empty() {
      if drain_count <= 3 || drain_count % EMPTY_DRAIN_LOG_INTERVAL == 0 {
        append_debug_line(
          "audio",
          format!(
            "Drain #{drain_count}: no samples available yet (callbacks={callback_count}, total_received={total_samples_received})"
          ),
        );
      }
    } else if drain_count <= 3 || drain_count % POPULATED_DRAIN_LOG_INTERVAL == 0 {
      let max_abs_sample = samples
        .iter()
        .fold(0.0_f32, |max_value, sample| max_value.max(sample.abs()));
      append_debug_line(
        "audio",
        format!(
          "Drain #{drain_count}: drained {} samples (callbacks={callback_count}, total_received={total_samples_received}, max_abs_sample={max_abs_sample:.5})",
          samples.len()
        ),
      );
    }

    samples
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

    let (samples, ..) = buffer.drain_samples();
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

    assert_eq!(buffer.drain_samples().0, vec![0.2, 0.3, 0.4]);
  }

  #[test]
  fn test_shared_sample_buffer_drain_clears_pending_samples() {
    let mut buffer = SharedSampleBuffer::with_max_len(4);

    buffer.push_interleaved(&[0.25_f32, 0.75_f32], 1);
    assert_eq!(buffer.drain_samples().0, vec![0.25, 0.75]);
    assert!(buffer.drain_samples().0.is_empty());
  }
}
