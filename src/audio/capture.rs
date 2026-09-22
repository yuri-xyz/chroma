use std::{
  collections::VecDeque,
  sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex, MutexGuard, PoisonError,
  },
  time::{Duration, Instant},
};

use cpal::{
  traits::{DeviceTrait, StreamTrait},
  FromSample, Sample, Stream, StreamConfig, I24,
};

#[cfg(target_os = "linux")]
use super::pulse_capture;
use super::{analyzer::sample_rate_scale, device_selector};
use crate::debug::append_debug_line;

const MAX_PENDING_SAMPLES: usize = 8_192;
const EMPTY_DRAIN_LOG_INTERVAL: u64 = 120;
const POPULATED_DRAIN_LOG_INTERVAL: u64 = 60;
const SILENT_CALLBACK_WARNING_THRESHOLD: u64 = 180;
/// How often a stream the backend invalidated is rebuilt while rebuilding fails.
const RECOVERY_RETRY_INTERVAL: Duration = Duration::from_secs(1);
/// Why output-device loopback can deliver only zeros, and what to do about it.
#[cfg(target_os = "macos")]
const SILENT_LOOPBACK_HINT: &str = "CoreAudio loopback stays silent on some output devices; install/select BlackHole or another loopback-capable source.";
#[cfg(target_os = "windows")]
const SILENT_LOOPBACK_HINT: &str = "WASAPI loopback hears only shared-mode playback, so the playing applications are outputting silence or hold the device in exclusive mode.";
#[cfg(not(any(target_os = "macos", target_os = "windows")))]
const SILENT_LOOPBACK_HINT: &str =
  "The selected output device is not providing system-audio loopback.";

#[derive(Debug, Clone, Copy)]
pub(super) struct CallbackLogSummary {
  pub(super) callback_count: u64,
  pub(super) frame_count: usize,
  pub(super) max_abs_sample: f32,
  pub(super) buffered_samples: usize,
}

pub(super) struct SharedSampleBuffer {
  samples: VecDeque<f32>,
  max_len: usize,
  callback_count: u64,
  drain_count: u64,
  total_samples_received: u64,
  total_samples_drained: u64,
  all_zero_callback_streak: u64,
  emitted_silence_warning: bool,
  last_samples_at: Instant,
}

impl SharedSampleBuffer {
  pub(super) fn with_max_len(max_len: usize) -> Self {
    Self {
      samples: VecDeque::with_capacity(max_len),
      max_len,
      callback_count: 0,
      drain_count: 0,
      total_samples_received: 0,
      total_samples_drained: 0,
      all_zero_callback_streak: 0,
      emitted_silence_warning: false,
      last_samples_at: Instant::now(),
    }
  }

  pub(super) fn push_interleaved<T>(
    &mut self,
    data: &[T],
    channels: usize,
  ) -> Option<CallbackLogSummary>
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
      || (max_abs_sample > 0.01 && self.callback_count.is_multiple_of(60))
      || self.callback_count.is_multiple_of(240);

    should_log.then_some(CallbackLogSummary {
      callback_count: self.callback_count,
      frame_count,
      max_abs_sample,
      buffered_samples: self.samples.len(),
    })
  }

  fn drain_samples(&mut self, now: Instant) -> (Vec<f32>, u64, u64, u64) {
    self.drain_count += 1;
    let drain_count = self.drain_count;
    let callback_count = self.callback_count;
    let drained = self.samples.drain(..).collect::<Vec<_>>();
    self.total_samples_drained += drained.len() as u64;
    if !drained.is_empty() {
      self.last_samples_at = now;
    }

    (
      drained,
      drain_count,
      callback_count,
      self.total_samples_received,
    )
  }

  /// WASAPI loopback delivers no packets at all while nothing is playing, so a
  /// stretch without samples means silence rather than a slow callback.
  fn time_without_samples(&self, now: Instant) -> Duration {
    now.saturating_duration_since(self.last_samples_at)
  }

  fn take_silence_warning(&mut self) -> bool {
    if self.all_zero_callback_streak >= SILENT_CALLBACK_WARNING_THRESHOLD
      && !self.emitted_silence_warning
    {
      self.emitted_silence_warning = true;
      return true;
    }

    false
  }
}

/// Errors after which the stream delivers nothing until it is rebuilt: the
/// device was unplugged, or WASAPI saw the default device change (it never
/// moves an existing stream to the new default).
fn invalidates_stream(kind: cpal::ErrorKind) -> bool {
  matches!(
    kind,
    cpal::ErrorKind::StreamInvalidated
      | cpal::ErrorKind::DeviceNotAvailable
      | cpal::ErrorKind::HostUnavailable
  )
}

/// A panic while the buffer was locked leaves at worst a partial batch of
/// samples, so keep capturing instead of cascading the panic into rendering.
pub(super) fn lock_samples(
  buffer: &Mutex<SharedSampleBuffer>,
) -> MutexGuard<'_, SharedSampleBuffer> {
  buffer.lock().unwrap_or_else(PoisonError::into_inner)
}

pub struct AudioCapture {
  _stream: Option<Stream>,
  #[cfg(target_os = "linux")]
  _pulse_capture: Option<pulse_capture::PulseCapture>,
  buffer: Arc<Mutex<SharedSampleBuffer>>,
  pub sample_rate: f32,
  using_output_config_fallback: bool,
  requested_device: Option<String>,
  stream_invalidated: Arc<AtomicBool>,
  last_recovery_attempt: Option<Instant>,
}

impl AudioCapture {
  /// List all available audio devices across all hosts
  pub fn list_devices() -> anyhow::Result<()> {
    // Try to use the best host, fall back to default
    let host = match device_selector::find_system_audio_auto() {
      Ok((host, _)) => host,
      Err(_) => cpal::default_host(),
    };

    device_selector::list_devices(&host)?;

    #[cfg(target_os = "linux")]
    pulse_capture::print_pulse_sources();

    Ok(())
  }

  /// Create audio capture with optional device name
  pub fn new(device_name: Option<&str>) -> anyhow::Result<Self> {
    append_debug_line("audio", "=== Audio Capture Initialization ===");
    let requested_device = device_name;

    #[cfg(target_os = "linux")]
    match Self::new_pulse(device_name) {
      Ok(capture) => return Ok(capture),
      Err(error) => append_debug_line(
        "audio",
        format!("Linux PulseAudio/PipeWire backend unavailable, falling back to CPAL: {error}"),
      ),
    }

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

    // Get config - try input first, then output for loopback (macOS CoreAudio, Windows WASAPI)
    #[cfg(any(target_os = "macos", target_os = "windows"))]
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

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
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
        format!("Capturing output device '{device_name}' through loopback with its output config"),
      );
    }

    let sample_rate = config.sample_rate() as f32;
    let buffer = Arc::new(Mutex::new(SharedSampleBuffer::with_max_len(
      MAX_PENDING_SAMPLES * sample_rate_scale(sample_rate),
    )));
    let stream_invalidated = Arc::new(AtomicBool::new(false));
    let shared = (Arc::clone(&buffer), Arc::clone(&stream_invalidated));
    let stream_config = config.into();

    let stream = match config.sample_format() {
      cpal::SampleFormat::F32 => Self::build_stream::<f32>(&device, stream_config, shared)?,
      cpal::SampleFormat::I32 => Self::build_stream::<i32>(&device, stream_config, shared)?,
      cpal::SampleFormat::I24 => Self::build_stream::<I24>(&device, stream_config, shared)?,
      cpal::SampleFormat::I16 => Self::build_stream::<i16>(&device, stream_config, shared)?,
      cpal::SampleFormat::U16 => Self::build_stream::<u16>(&device, stream_config, shared)?,
      format => return Err(anyhow::anyhow!("Unsupported sample format {format:?}")),
    };

    stream.play()?;

    append_debug_line("audio", "Audio stream started successfully");

    Ok(Self {
      _stream: Some(stream),
      #[cfg(target_os = "linux")]
      _pulse_capture: None,
      buffer,
      sample_rate,
      using_output_config_fallback,
      requested_device: requested_device.map(str::to_string),
      stream_invalidated,
      last_recovery_attempt: None,
    })
  }

  #[cfg(target_os = "linux")]
  fn new_pulse(device_name: Option<&str>) -> anyhow::Result<Self> {
    append_debug_line(
      "audio",
      "Trying Linux PulseAudio/PipeWire monitor backend...",
    );

    let buffer = Arc::new(Mutex::new(SharedSampleBuffer::with_max_len(
      MAX_PENDING_SAMPLES,
    )));
    let pulse_capture = pulse_capture::PulseCapture::new(device_name, Arc::clone(&buffer))?;
    let sample_rate = pulse_capture.sample_rate;

    append_debug_line(
      "audio",
      format!(
        "Linux PulseAudio/PipeWire monitor backend started from '{}'",
        pulse_capture.source_name
      ),
    );

    Ok(Self {
      _stream: None,
      _pulse_capture: Some(pulse_capture),
      buffer,
      sample_rate,
      using_output_config_fallback: false,
      requested_device: device_name.map(str::to_string),
      stream_invalidated: Arc::new(AtomicBool::new(false)),
      last_recovery_attempt: None,
    })
  }

  fn build_stream<T>(
    device: &cpal::Device,
    config: StreamConfig,
    (buffer, stream_invalidated): (Arc<Mutex<SharedSampleBuffer>>, Arc<AtomicBool>),
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
        channels, config.sample_rate
      ),
    );
    let callback_device_name = device_name.clone();
    let error_device_name = device_name.clone();

    let stream = device.build_input_stream(
      config,
      move |data: &[T], _: &cpal::InputCallbackInfo| {
        if let Some(summary) = lock_samples(&buffer).push_interleaved(data, channels) {
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
      move |err: cpal::Error| {
        let invalidated = invalidates_stream(err.kind());
        if invalidated {
          stream_invalidated.store(true, Ordering::Relaxed);
        }
        append_debug_line(
          "audio",
          format!("Audio stream error for '{error_device_name}' (needs rebuild: {invalidated}): {err}"),
        );
      },
      None,
    )?;

    Ok(stream)
  }

  pub fn drain_samples(&self) -> Vec<f32> {
    let (samples, drain_count, callback_count, total_samples_received, should_warn_zero_stream) = {
      let mut buffer = lock_samples(&self.buffer);
      let (samples, drain_count, callback_count, total_samples_received) =
        buffer.drain_samples(Instant::now());
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
        format!(
          "WARNING: received a long run of all-zero callbacks from output-device loopback. Either nothing is playing, or: {SILENT_LOOPBACK_HINT}"
        ),
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

  /// How long capture has gone without delivering a sample.
  pub fn time_without_samples(&self) -> Duration {
    lock_samples(&self.buffer).time_without_samples(Instant::now())
  }

  /// Rebuild the stream after the backend invalidated it, for example when a
  /// Windows user switches the default output or unplugs the captured device.
  /// Auto-detection runs again, so the rebuilt stream follows the new default.
  /// Returns whether a new stream replaced the old one; failed attempts are
  /// retried at most once per `RECOVERY_RETRY_INTERVAL`.
  pub fn recover_if_invalidated(&mut self) -> anyhow::Result<bool> {
    if !self.stream_invalidated.load(Ordering::Relaxed) {
      return Ok(false);
    }

    let now = Instant::now();
    if self
      .last_recovery_attempt
      .is_some_and(|attempt| now.duration_since(attempt) < RECOVERY_RETRY_INTERVAL)
    {
      return Ok(false);
    }
    self.last_recovery_attempt = Some(now);

    append_debug_line("audio", "Rebuilding invalidated audio stream");
    let requested_device = self.requested_device.clone();
    // Release the old endpoint first so the rebuild does not race it for the device.
    self._stream = None;
    *self = Self::new(requested_device.as_deref())?;

    Ok(true)
  }
}

#[cfg(test)]
mod tests {
  use std::time::{Duration, Instant};

  use super::{invalidates_stream, SharedSampleBuffer};

  #[test]
  fn test_shared_sample_buffer_accumulates_across_pushes() {
    let mut buffer = SharedSampleBuffer::with_max_len(8);

    buffer.push_interleaved(&[0.2_f32, 0.4_f32, 0.6_f32, 0.8_f32], 2);
    buffer.push_interleaved(&[1.0_f32, 0.0_f32, 0.5_f32, 0.5_f32], 2);

    let (samples, ..) = buffer.drain_samples(Instant::now());
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

    assert_eq!(buffer.drain_samples(Instant::now()).0, vec![0.2, 0.3, 0.4]);
  }

  #[test]
  fn test_shared_sample_buffer_drain_clears_pending_samples() {
    let mut buffer = SharedSampleBuffer::with_max_len(4);

    buffer.push_interleaved(&[0.25_f32, 0.75_f32], 1);
    assert_eq!(buffer.drain_samples(Instant::now()).0, vec![0.25, 0.75]);
    assert!(buffer.drain_samples(Instant::now()).0.is_empty());
  }

  #[test]
  fn test_time_without_samples_resets_only_when_samples_arrive() {
    let mut buffer = SharedSampleBuffer::with_max_len(4);
    let start = Instant::now();
    let later = start + Duration::from_millis(500);

    buffer.drain_samples(start);
    buffer.drain_samples(later);
    assert!(buffer.time_without_samples(later) >= Duration::from_millis(500));

    buffer.push_interleaved(&[0.5_f32], 1);
    buffer.drain_samples(later);
    assert_eq!(buffer.time_without_samples(later), Duration::ZERO);
  }

  #[test]
  fn test_only_lost_streams_need_a_rebuild() {
    assert!(invalidates_stream(cpal::ErrorKind::StreamInvalidated));
    assert!(invalidates_stream(cpal::ErrorKind::DeviceNotAvailable));
    assert!(invalidates_stream(cpal::ErrorKind::HostUnavailable));
    assert!(!invalidates_stream(cpal::ErrorKind::DeviceChanged));
    assert!(!invalidates_stream(cpal::ErrorKind::Xrun));
  }
}
