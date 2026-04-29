macro_rules! debug_logln {
  ($writer:expr, $($arg:tt)*) => {{
    use std::io::Write as _;
    writeln!($writer, $($arg)*)
  }};
}

mod audio;
mod config_watcher;
mod input;
mod rendering;
mod status_bar;

use anyhow::Result;
use chroma::ascii::{AsciiConverter, AsciiPalette};
use chroma::audio::{AudioAnalyzer, AudioCapture, AudioFeatures};
use chroma::constants::AUDIO_SILENCE_THRESHOLD;
use chroma::debug::DebugLog;
use chroma::params::ShaderParams;
use chroma::render::RenderedCell;
use chroma::shader::{ShaderPipeline, ShaderUniforms};
use crossterm::terminal;
use std::io::Write;
use std::time::Instant;

fn shader_dimensions(
  terminal_width: u16,
  terminal_height: u16,
  show_status_bar: bool,
  stream_mode: bool,
) -> (u32, u32) {
  let shader_width = terminal_width as u32;
  let shader_height = if show_status_bar && !stream_mode {
    terminal_height.saturating_sub(1) as u32
  } else {
    terminal_height as u32
  };

  (shader_width, shader_height)
}

fn create_converter(params: &ShaderParams) -> AsciiConverter {
  AsciiConverter::new(AsciiPalette::from(params.palette), true)
}

fn terminal_background_color(params: &ShaderParams) -> Option<(u8, u8, u8)> {
  if params.terminal_bg_r > 0.0 || params.terminal_bg_g > 0.0 || params.terminal_bg_b > 0.0 {
    Some((
      (params.terminal_bg_r * 255.0) as u8,
      (params.terminal_bg_g * 255.0) as u8,
      (params.terminal_bg_b * 255.0) as u8,
    ))
  } else {
    None
  }
}

fn prepare_reloaded_params(
  current_params: &ShaderParams,
  mut new_params: ShaderParams,
) -> ShaderParams {
  new_params.time = current_params.time;
  new_params.audio_enabled = true;
  new_params.set_resolution(
    current_params.resolution_width,
    current_params.resolution_height,
  );
  new_params
}

/// Main application state
pub struct App {
  params: ShaderParams,
  pipeline: ShaderPipeline,
  converter: AsciiConverter,
  running: bool,
  show_status_bar: bool,
  stream_mode: bool,
  last_frame_time: Instant,
  debug_log: DebugLog,
  last_terminal_size: (u16, u16),
  config_watcher: Option<config_watcher::ConfigWatcher>,
  custom_shader: Option<String>,
  target_fps: u32,
  audio_capture: Option<AudioCapture>,
  audio_analyzer: Option<AudioAnalyzer>,
  latest_audio_features: AudioFeatures,
}

impl App {
  /// Create a new application instance
  pub async fn new(
    loaded_config: Option<ShaderParams>,
    show_status_bar: bool,
    stream_dimensions: Option<crate::cli::StreamDimensions>,
    config_path: Option<String>,
    audio_device: Option<String>,
    custom_shader: Option<String>,
    target_fps: u32,
  ) -> Result<Self> {
    let mut debug_log = DebugLog::create_default()?;

    let stream_mode = stream_dimensions.is_some();

    let (terminal_width, terminal_height) = if let Some(dims) = stream_dimensions {
      // Stream mode: use fixed dimensions
      (dims.width, dims.height)
    } else {
      // Normal mode: get terminal size
      terminal::size()?
    };

    writeln!(
      debug_log,
      "DEBUG: Terminal size: {}x{} (stream_mode: {})",
      terminal_width, terminal_height, stream_mode
    )?;

    let (shader_width, shader_height) = shader_dimensions(
      terminal_width,
      terminal_height,
      show_status_bar,
      stream_mode,
    );

    writeln!(
      debug_log,
      "DEBUG: Shader size: {}x{}",
      shader_width, shader_height
    )?;

    let mut params = loaded_config.unwrap_or_else(ShaderParams::with_audio_reactive_defaults);
    params.audio_enabled = true;

    params.set_resolution(shader_width, shader_height);

    if custom_shader.is_some() {
      writeln!(
        debug_log,
        "DEBUG: Using custom shader (overrides pattern selection)"
      )?;
    }

    let pipeline = ShaderPipeline::new(
      shader_width,
      shader_height,
      custom_shader.clone(),
      &mut debug_log,
    )
    .await?;

    let converter = create_converter(&params);

    let (audio_capture, audio_analyzer) =
      Self::init_audio(&mut debug_log, audio_device.as_deref())?;

    let config_watcher = Self::init_config_watcher(&config_path, &mut debug_log)?;

    Ok(Self {
      params,
      pipeline,
      converter,
      running: true,
      show_status_bar,
      stream_mode,
      last_frame_time: Instant::now(),
      debug_log,
      last_terminal_size: (terminal_width, terminal_height),
      config_watcher,
      custom_shader,
      target_fps,
      audio_capture,
      audio_analyzer,
      latest_audio_features: AudioFeatures::default(),
    })
  }

  /// Initialize audio capture and analyzer
  fn init_audio(
    debug_log: &mut DebugLog,
    device_name: Option<&str>,
  ) -> Result<(Option<AudioCapture>, Option<AudioAnalyzer>)> {
    match AudioCapture::new(device_name) {
      Ok(capture) => {
        writeln!(
          debug_log,
          "Audio capture initialized successfully at {} Hz",
          capture.sample_rate
        )?;
        let analyzer = AudioAnalyzer::new(capture.sample_rate);
        Ok((Some(capture), Some(analyzer)))
      }
      Err(e) => {
        writeln!(debug_log, "Failed to initialize audio: {}", e)?;
        Ok((None, None))
      }
    }
  }

  /// Initialize config file watcher if config path is provided
  fn init_config_watcher(
    config_path: &Option<String>,
    debug_log: &mut DebugLog,
  ) -> Result<Option<config_watcher::ConfigWatcher>> {
    if let Some(path) = config_path {
      match config_watcher::ConfigWatcher::new(path) {
        Ok(watcher) => {
          writeln!(debug_log, "Config file watcher initialized for: {}", path)?;
          Ok(Some(watcher))
        }
        Err(e) => {
          writeln!(debug_log, "Failed to initialize config watcher: {}", e)?;
          Ok(None)
        }
      }
    } else {
      Ok(None)
    }
  }

  /// Update application state
  fn update(&mut self) {
    let current_time = Instant::now();
    let delta_time = current_time
      .duration_since(self.last_frame_time)
      .as_secs_f32();

    self.params.update_time(delta_time);

    let features = audio::update_audio_reactive(
      &mut self.params,
      &self.audio_capture,
      &mut self.audio_analyzer,
      delta_time,
      &mut self.debug_log,
    );
    self.latest_audio_features = features;

    self.check_and_apply_config_reload();

    self.last_frame_time = current_time;
  }

  /// Check for config file changes and apply them if valid
  fn check_and_apply_config_reload(&mut self) {
    if let Some(ref watcher) = self.config_watcher {
      if let Some(new_params) = watcher.try_receive_config() {
        let new_params = prepare_reloaded_params(&self.params, new_params);

        if new_params.palette != self.params.palette {
          self.converter = create_converter(&new_params);
        }

        self.params = new_params;

        let _ = debug_logln!(self.debug_log, "Config reloaded successfully");
      }
    }
  }

  /// Render current frame
  fn render(&mut self) -> Result<()> {
    let uniforms = ShaderUniforms::from_params(&self.params);

    debug_logln!(
      self.debug_log,
      "DEBUG: Uniforms - time: {}, freq: {}, amp: {}, scale: {}",
      self.params.time,
      self.params.frequency,
      self.params.amplitude,
      self.params.scale
    )?;
    debug_logln!(
      self.debug_log,
      "DEBUG: Resolution in uniforms: {}x{}",
      self.params.resolution_width,
      self.params.resolution_height
    )?;

    // Stream mode: use simplified rendering
    if self.stream_mode {
      rendering::render_stream_frame(
        &self.pipeline,
        &self.converter,
        &uniforms,
        &mut self.debug_log,
      )?;

      self.debug_log.flush()?;
      return Ok(());
    }

    // Normal mode: full rendering with status bar
    let has_sound = self.check_audio_activity();

    let status_bar = if self.show_status_bar {
      Some(self.build_status_bar(has_sound))
    } else {
      None
    };

    rendering::render_frame(
      &self.pipeline,
      &self.converter,
      &uniforms,
      status_bar,
      terminal_background_color(&self.params),
      &mut self.debug_log,
    )?;

    self.debug_log.flush()?;
    Ok(())
  }

  /// Check if audio is currently active
  fn check_audio_activity(&self) -> bool {
    self.latest_audio_features.overall >= AUDIO_SILENCE_THRESHOLD
  }

  /// Build status bar cells
  fn build_status_bar(&self, has_sound: bool) -> Vec<RenderedCell> {
    let available_cols = self.last_terminal_size.0 as usize;
    let status_text = status_bar::build_status_text(&self.params, self.params.effect_type);

    status_bar::format_status_bar(&status_text, available_cols, has_sound, self.params.time)
  }

  /// Handle window resize
  async fn handle_resize(&mut self, new_width: u16, new_height: u16) -> Result<()> {
    debug_logln!(
      self.debug_log,
      "RESIZE: Terminal resized to {}x{} (was {}x{})",
      new_width,
      new_height,
      self.last_terminal_size.0,
      self.last_terminal_size.1
    )?;

    let (shader_width, shader_height) = shader_dimensions(
      new_width,
      new_height,
      self.show_status_bar,
      self.stream_mode,
    );

    self.params.set_resolution(shader_width, shader_height);

    self.pipeline = ShaderPipeline::new(
      shader_width,
      shader_height,
      self.custom_shader.clone(),
      &mut self.debug_log,
    )
    .await?;

    self.last_terminal_size = (new_width, new_height);

    debug_logln!(
      self.debug_log,
      "RESIZE: Pipeline recreated with dimensions {}x{}",
      shader_width,
      shader_height
    )?;

    Ok(())
  }

  /// Main application loop
  pub fn run(&mut self) -> Result<()> {
    while self.running {
      let frame_start = Instant::now();

      // Skip input and resize handling in stream mode
      if !self.stream_mode {
        // Check for window resize
        let (current_width, current_height) = terminal::size()?;
        if (current_width, current_height) != self.last_terminal_size {
          pollster::block_on(async { self.handle_resize(current_width, current_height).await })?;
        }

        // Handle input
        input::handle_input(
          &mut self.params,
          &mut self.converter,
          &mut self.running,
          &mut self.debug_log,
        )?;
      }

      self.update();
      self.render()?;

      // Frame rate limiting
      let frame_time = frame_start.elapsed();
      let frame_duration = std::time::Duration::from_micros((1_000_000 / self.target_fps) as u64);

      if frame_time < frame_duration {
        std::thread::sleep(frame_duration - frame_time);
      }
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use chroma::params::PaletteType;

  #[test]
  fn test_shader_dimensions_only_reserve_status_row_outside_stream_mode() {
    assert_eq!(shader_dimensions(80, 24, true, false), (80, 23));
    assert_eq!(shader_dimensions(80, 24, true, true), (80, 24));
    assert_eq!(shader_dimensions(80, 24, false, false), (80, 24));
  }

  #[test]
  fn test_shader_dimensions_saturate_when_terminal_height_is_tiny() {
    assert_eq!(shader_dimensions(80, 0, true, false), (80, 0));
    assert_eq!(shader_dimensions(80, 1, true, false), (80, 0));
  }

  #[test]
  fn test_terminal_background_color_converts_normalized_channels() {
    let params = ShaderParams {
      terminal_bg_r: 1.0,
      terminal_bg_g: 0.5,
      terminal_bg_b: 0.25,
      ..ShaderParams::default()
    };

    assert_eq!(terminal_background_color(&params), Some((255, 127, 63)));
  }

  #[test]
  fn test_terminal_background_color_returns_none_for_black_background() {
    let params = ShaderParams::default();

    assert_eq!(terminal_background_color(&params), None);
  }

  #[test]
  fn test_terminal_background_color_returns_some_when_any_channel_is_non_zero() {
    let params = ShaderParams {
      terminal_bg_b: 0.1,
      ..ShaderParams::default()
    };

    assert_eq!(terminal_background_color(&params), Some((0, 0, 25)));
  }

  #[test]
  fn test_create_converter_uses_selected_palette() {
    let params = ShaderParams {
      palette: PaletteType::Simple,
      ..ShaderParams::default()
    };
    let converter = create_converter(&params);
    let pixels = vec![128, 128, 128, 255];
    let frame = converter.convert_frame(&pixels, 1, 1);

    assert_eq!(frame[0][0].0, 'o');
  }

  #[test]
  fn test_prepare_reloaded_params_preserves_runtime_state() {
    let current = ShaderParams {
      time: 12.5,
      resolution_width: 120,
      resolution_height: 40,
      palette: PaletteType::Braille,
      ..ShaderParams::default()
    };
    let incoming = ShaderParams {
      time: 1.0,
      resolution_width: 10,
      resolution_height: 10,
      palette: PaletteType::Lines,
      frequency: 14.0,
      ..ShaderParams::default()
    };

    let prepared = prepare_reloaded_params(&current, incoming);

    assert_eq!(prepared.time, current.time);
    assert_eq!(prepared.resolution_width, current.resolution_width);
    assert_eq!(prepared.resolution_height, current.resolution_height);
    assert_eq!(prepared.palette, PaletteType::Lines);
    assert_eq!(prepared.frequency, 14.0);
  }

  #[test]
  fn test_prepare_reloaded_params_keeps_incoming_non_runtime_fields() {
    let current = ShaderParams {
      time: 5.0,
      resolution_width: 100,
      resolution_height: 30,
      ..ShaderParams::default()
    };
    let incoming = ShaderParams {
      terminal_bg_r: 0.5,
      audio_enabled: false,
      beat_sensitivity: 2.5,
      ..ShaderParams::default()
    };

    let prepared = prepare_reloaded_params(&current, incoming);

    assert_eq!(prepared.time, 5.0);
    assert_eq!(prepared.resolution_width, 100);
    assert_eq!(prepared.resolution_height, 30);
    assert_eq!(prepared.terminal_bg_r, 0.5);
    assert!(prepared.audio_enabled);
    assert_eq!(prepared.beat_sensitivity, 2.5);
  }
}
