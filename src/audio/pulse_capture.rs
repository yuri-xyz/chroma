use std::{
  sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
  },
  thread::{self, JoinHandle},
};

use anyhow::Context as _;
use libpulse_binding as pulse;
use libpulse_simple_binding::Simple;
use pulse::{
  callbacks::ListResult,
  context::{Context, FlagSet, State as ContextState},
  def::BufferAttr,
  mainloop::standard::{IterateResult, Mainloop},
  operation,
  sample::{Format, Spec},
  stream::Direction,
};

use super::capture::SharedSampleBuffer;
use crate::debug::append_debug_line;

const PULSE_APP_NAME: &str = "Chroma";
const PULSE_STREAM_NAME: &str = "Chroma system audio";
const PULSE_SAMPLE_RATE: u32 = 48_000;
const PULSE_CHANNELS: u8 = 2;
const PULSE_READ_FRAMES: usize = 1_024;
const PULSE_BUFFER_FRAGMENTS: u32 = 4;

pub(super) struct PulseCapture {
  _reader: JoinHandle<()>,
  stop: Arc<AtomicBool>,
  pub(super) sample_rate: f32,
  pub(super) source_name: String,
}

impl PulseCapture {
  pub(super) fn new(
    device_name: Option<&str>,
    buffer: Arc<Mutex<SharedSampleBuffer>>,
  ) -> anyhow::Result<Self> {
    let source_name = match device_name {
      Some(name) => name.to_string(),
      None => default_monitor_source_name()?.with_context(|| {
        "PulseAudio/PipeWire did not expose a monitor source for the default sink"
      })?,
    };

    append_debug_line(
      "audio",
      format!("Opening PulseAudio/PipeWire source '{source_name}'"),
    );

    let spec = Spec {
      format: Format::FLOAT32NE,
      rate: PULSE_SAMPLE_RATE,
      channels: PULSE_CHANNELS,
    };
    anyhow::ensure!(spec.is_valid(), "invalid PulseAudio sample specification");

    let fragment_bytes = pulse_fragment_size_bytes();
    let buffer_attr = BufferAttr {
      maxlength: fragment_bytes * PULSE_BUFFER_FRAGMENTS,
      tlength: u32::MAX,
      prebuf: u32::MAX,
      minreq: u32::MAX,
      fragsize: fragment_bytes,
    };

    append_debug_line(
      "audio",
      format!(
        "PulseAudio capture buffer: fragment_bytes={}, target_fragment_ms={:.1}",
        buffer_attr.fragsize,
        pulse_fragment_duration_ms()
      ),
    );

    let stream = Simple::new(
      None,
      PULSE_APP_NAME,
      Direction::Record,
      Some(&source_name),
      PULSE_STREAM_NAME,
      &spec,
      None,
      Some(&buffer_attr),
    )
    .map_err(|error| {
      anyhow::anyhow!("failed to open PulseAudio source '{source_name}': {error}")
    })?;
    if let Ok(latency) = stream.get_latency() {
      append_debug_line(
        "audio",
        format!("PulseAudio reported initial latency: {} usec", latency.0),
      );
    }

    let stop = Arc::new(AtomicBool::new(false));
    let reader_stop = Arc::clone(&stop);
    let reader_source_name = source_name.clone();
    let reader = thread::Builder::new()
      .name("chroma-pulse-capture".to_string())
      .spawn(move || {
        read_pulse_samples(stream, reader_source_name, reader_stop, buffer);
      })
      .context("failed to spawn PulseAudio capture thread")?;

    Ok(Self {
      _reader: reader,
      stop,
      sample_rate: PULSE_SAMPLE_RATE as f32,
      source_name,
    })
  }
}

fn pulse_fragment_size_bytes() -> u32 {
  (PULSE_READ_FRAMES * PULSE_CHANNELS as usize * std::mem::size_of::<f32>()) as u32
}

fn pulse_fragment_duration_ms() -> f32 {
  PULSE_READ_FRAMES as f32 / PULSE_SAMPLE_RATE as f32 * 1_000.0
}

impl Drop for PulseCapture {
  fn drop(&mut self) {
    self.stop.store(true, Ordering::Relaxed);
  }
}

pub(super) fn print_pulse_sources() {
  match list_monitor_source_names() {
    Ok(sources) if sources.is_empty() => {
      println!("\n=== PulseAudio/PipeWire Monitor Sources ===");
      println!("  (none)");
    }
    Ok(sources) => {
      println!("\n=== PulseAudio/PipeWire Monitor Sources ===");
      for source in sources {
        println!("  * {source} [SYSTEM AUDIO]");
      }
      println!("  Chroma will prefer the default sink monitor on Linux when available.");
    }
    Err(error) => {
      println!("\n=== PulseAudio/PipeWire Monitor Sources ===");
      println!("  Unavailable: {error}");
      println!("  Install/run PulseAudio or pipewire-pulse, or use a CPAL-listed input device.");
    }
  }
}

fn read_pulse_samples(
  stream: Simple,
  source_name: String,
  stop: Arc<AtomicBool>,
  buffer: Arc<Mutex<SharedSampleBuffer>>,
) {
  let channels = PULSE_CHANNELS as usize;
  let byte_count = PULSE_READ_FRAMES * channels * std::mem::size_of::<f32>();
  let mut bytes = vec![0_u8; byte_count];
  let mut samples = Vec::with_capacity(PULSE_READ_FRAMES * channels);

  while !stop.load(Ordering::Relaxed) {
    if let Err(error) = stream.read(&mut bytes) {
      append_debug_line(
        "audio",
        format!("PulseAudio read error for '{source_name}': {error}"),
      );
      break;
    }

    samples.clear();
    samples.extend(bytes.chunks_exact(4).map(|chunk| {
      let bytes = [chunk[0], chunk[1], chunk[2], chunk[3]];
      f32::from_ne_bytes(bytes)
    }));

    if let Some(summary) = buffer
      .lock()
      .unwrap()
      .push_interleaved(samples.as_slice(), channels)
    {
      append_debug_line(
        "audio",
        format!(
          "PulseAudio callback #{} for '{source_name}': frames={}, max_abs_sample={:.5}, buffered_samples={}",
          summary.callback_count,
          summary.frame_count,
          summary.max_abs_sample,
          summary.buffered_samples
        ),
      );
    }
  }
}

fn default_monitor_source_name() -> anyhow::Result<Option<String>> {
  let mut connection = PulseConnection::connect()?;
  let default_sink = connection.default_sink_name()?;

  if let Some(sink_name) = default_sink.as_deref() {
    let monitor_name = format!("{sink_name}.monitor");
    if connection.source_exists(&monitor_name)? {
      append_debug_line(
        "audio",
        format!("PulseAudio default sink monitor detected: '{monitor_name}'"),
      );
      return Ok(Some(monitor_name));
    }
  }

  let fallback = connection.first_monitor_source_name(default_sink.as_deref())?;
  if let Some(source_name) = &fallback {
    append_debug_line(
      "audio",
      format!("PulseAudio fallback monitor source detected: '{source_name}'"),
    );
  }
  Ok(fallback)
}

fn list_monitor_source_names() -> anyhow::Result<Vec<String>> {
  let mut connection = PulseConnection::connect()?;
  connection.monitor_source_names()
}

struct PulseConnection {
  mainloop: Mainloop,
  context: Context,
}

impl Drop for PulseConnection {
  fn drop(&mut self) {
    self.context.disconnect();
  }
}

impl PulseConnection {
  fn connect() -> anyhow::Result<Self> {
    let mut mainloop = Mainloop::new().context("failed to create PulseAudio mainloop")?;
    let mut context =
      Context::new(&mainloop, PULSE_APP_NAME).context("failed to create PulseAudio context")?;

    context
      .connect(None, FlagSet::NOFLAGS, None)
      .map_err(|error| anyhow::anyhow!("failed to connect to PulseAudio server: {error}"))?;

    loop {
      Self::iterate(&mut mainloop)?;
      match context.get_state() {
        ContextState::Ready => break,
        ContextState::Failed | ContextState::Terminated => {
          return Err(anyhow::anyhow!(
            "PulseAudio context failed while connecting: {}",
            context.errno()
          ));
        }
        _ => {}
      }
    }

    Ok(Self { mainloop, context })
  }

  fn default_sink_name(&mut self) -> anyhow::Result<Option<String>> {
    let result = Arc::new(Mutex::new(None));
    let callback_result = Arc::clone(&result);
    let introspector = self.context.introspect();
    let operation = introspector.get_server_info(move |info| {
      *callback_result.lock().unwrap() =
        info.default_sink_name.as_ref().map(|name| name.to_string());
    });

    self.wait_for_operation(&operation)?;
    let default_sink_name = result.lock().unwrap().clone();
    Ok(default_sink_name)
  }

  fn source_exists(&mut self, source_name: &str) -> anyhow::Result<bool> {
    let result = Arc::new(Mutex::new(None));
    let callback_result = Arc::clone(&result);
    let expected_name = source_name.to_string();
    let introspector = self.context.introspect();
    let operation = introspector.get_source_info_by_name(source_name, move |entry| match entry {
      ListResult::Item(source) => {
        let matches = source.name.as_deref() == Some(expected_name.as_str());
        if matches {
          *callback_result.lock().unwrap() = Some(true);
        }
      }
      ListResult::End => {
        let mut result = callback_result.lock().unwrap();
        if result.is_none() {
          *result = Some(false);
        }
      }
      ListResult::Error => {
        *callback_result.lock().unwrap() = Some(false);
      }
    });

    self.wait_for_operation(&operation)?;
    let source_exists = result.lock().unwrap().unwrap_or(false);
    Ok(source_exists)
  }

  fn first_monitor_source_name(
    &mut self,
    preferred_sink_name: Option<&str>,
  ) -> anyhow::Result<Option<String>> {
    let result = Arc::new(Mutex::new(MonitorSearchResult::default()));
    let callback_result = Arc::clone(&result);
    let preferred_sink_name = preferred_sink_name.map(str::to_string);
    let introspector = self.context.introspect();
    let operation = introspector.get_source_info_list(move |entry| match entry {
      ListResult::Item(source) => {
        let Some(source_name) = source.name.as_ref().map(|name| name.to_string()) else {
          return;
        };
        let is_monitor = source.monitor_of_sink_name.is_some() || source_name.ends_with(".monitor");
        if !is_monitor {
          return;
        }

        let mut result = callback_result.lock().unwrap();
        if result.first.is_none() {
          result.first = Some(source_name.clone());
        }

        if preferred_sink_name.as_deref() == source.monitor_of_sink_name.as_deref() {
          result.preferred = Some(source_name);
        }
      }
      ListResult::End => {}
      ListResult::Error => {
        callback_result.lock().unwrap().error = true;
      }
    });

    self.wait_for_operation(&operation)?;
    let result = result.lock().unwrap();
    if result.error {
      return Err(anyhow::anyhow!("failed to list PulseAudio source devices"));
    }

    Ok(result.preferred.clone().or_else(|| result.first.clone()))
  }

  fn monitor_source_names(&mut self) -> anyhow::Result<Vec<String>> {
    let result = Arc::new(Mutex::new(MonitorListResult::default()));
    let callback_result = Arc::clone(&result);
    let introspector = self.context.introspect();
    let operation = introspector.get_source_info_list(move |entry| match entry {
      ListResult::Item(source) => {
        let Some(source_name) = source.name.as_ref().map(|name| name.to_string()) else {
          return;
        };
        if source.monitor_of_sink_name.is_some() || source_name.ends_with(".monitor") {
          callback_result.lock().unwrap().sources.push(source_name);
        }
      }
      ListResult::End => {}
      ListResult::Error => {
        callback_result.lock().unwrap().error = true;
      }
    });

    self.wait_for_operation(&operation)?;
    let result = result.lock().unwrap();
    if result.error {
      return Err(anyhow::anyhow!("failed to list PulseAudio source devices"));
    }

    Ok(result.sources.clone())
  }

  fn wait_for_operation<T: ?Sized>(
    &mut self,
    operation: &operation::Operation<T>,
  ) -> anyhow::Result<()> {
    while operation.get_state() == operation::State::Running {
      Self::iterate(&mut self.mainloop)?;
    }

    match operation.get_state() {
      operation::State::Done => Ok(()),
      operation::State::Cancelled => Err(anyhow::anyhow!("PulseAudio operation was cancelled")),
      state => Err(anyhow::anyhow!(
        "PulseAudio operation finished unexpectedly: {state:?}"
      )),
    }
  }

  fn iterate(mainloop: &mut Mainloop) -> anyhow::Result<()> {
    match mainloop.iterate(true) {
      IterateResult::Success(_) => Ok(()),
      IterateResult::Quit(retval) => Err(anyhow::anyhow!(
        "PulseAudio mainloop quit unexpectedly: {retval:?}"
      )),
      IterateResult::Err(error) => Err(anyhow::anyhow!("PulseAudio mainloop error: {error}")),
    }
  }
}

#[derive(Default)]
struct MonitorSearchResult {
  first: Option<String>,
  preferred: Option<String>,
  error: bool,
}

#[derive(Default)]
struct MonitorListResult {
  sources: Vec<String>,
  error: bool,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn pulse_sample_byte_count_matches_interleaved_f32_layout() {
    assert_eq!(pulse_fragment_size_bytes(), 8_192);
  }

  #[test]
  fn pulse_fragment_duration_stays_interactive() {
    assert!(pulse_fragment_duration_ms() < 25.0);
  }
}
