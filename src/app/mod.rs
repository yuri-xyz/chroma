mod audio;
mod config_watcher;
mod input;
mod rendering;
mod status_bar;

#[cfg(feature = "audio")]
use crate::constants::AUDIO_SAMPLE_THRESHOLD;
use anyhow::Result;
use chroma::ascii::{AsciiConverter, AsciiPalette};
#[cfg(feature = "audio")]
use chroma::audio::{AudioAnalyzer, AudioCapture};
use chroma::params::{PaletteType, ShaderParams};
use chroma::shader::{ShaderPipeline, ShaderUniforms};
use crossterm::terminal;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::Instant;

#[cfg(debug_assertions)]
pub(crate) type DebugLog = BufWriter<File>;

#[cfg(not(debug_assertions))]
pub(crate) type DebugLog = BufWriter<std::io::Sink>;

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
  #[cfg(feature = "audio")]
  audio_capture: Option<AudioCapture>,
  #[cfg(feature = "audio")]
  audio_analyzer: Option<AudioAnalyzer>,
}

impl App {
  /// Create a new application instance
  pub async fn new(
    loaded_config: Option<ShaderParams>,
    show_status_bar: bool,
    stream_dimensions: Option<crate::cli::StreamDimensions>,
    config_path: Option<String>,
    #[cfg(feature = "audio")] audio_device: Option<String>,
    custom_shader: Option<String>,
    target_fps: u32,
  ) -> Result<Self> {
    #[cfg(debug_assertions)]
    let mut debug_log = {
      let debug_file = File::create("debug.log")?;
      BufWriter::new(debug_file)
    };

    #[cfg(not(debug_assertions))]
    let mut debug_log = BufWriter::new(std::io::sink());

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

    let shader_width = terminal_width as u32;

    let shader_height = if show_status_bar && !stream_mode {
      (terminal_height - 1) as u32
    } else {
      terminal_height as u32
    };

    writeln!(
      debug_log,
      "DEBUG: Shader size: {}x{}",
      shader_width, shader_height
    )?;

    let mut params = loaded_config.unwrap_or_else(|| {
      #[cfg(feature = "audio")]
      {
        ShaderParams::with_audio_reactive_defaults()
      }
      #[cfg(not(feature = "audio"))]
      {
        ShaderParams::default()
      }
    });

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

    let palette = Self::palette_from_type(params.palette);
    let converter = AsciiConverter::new(palette, true);

    #[cfg(feature = "audio")]
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
      #[cfg(feature = "audio")]
      audio_capture,
      #[cfg(feature = "audio")]
      audio_analyzer,
    })
  }

  /// Initialize audio capture and analyzer
  #[cfg(feature = "audio")]
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

  /// Convert palette type to ASCII palette
  fn palette_from_type(palette_type: PaletteType) -> AsciiPalette {
    match palette_type {
      PaletteType::Standard => AsciiPalette::standard(),
      PaletteType::Blocks => AsciiPalette::blocks(),
      PaletteType::Circles => AsciiPalette::circles(),
      PaletteType::Smooth => AsciiPalette::smooth(),
      PaletteType::Braille => AsciiPalette::braille(),
      PaletteType::Geometric => AsciiPalette::geometric(),
      PaletteType::Mixed => AsciiPalette::mixed(),
      PaletteType::Dots => AsciiPalette::dots(),
      PaletteType::Shades => AsciiPalette::shades(),
      PaletteType::Lines => AsciiPalette::lines(),
      PaletteType::Triangles => AsciiPalette::triangles(),
      PaletteType::Arrows => AsciiPalette::arrows(),
      PaletteType::Powerline => AsciiPalette::powerline(),
      PaletteType::BoxDraw => AsciiPalette::boxdraw(),
      PaletteType::Extended => AsciiPalette::extended(),
      PaletteType::Simple => AsciiPalette::simple(),
    }
  }

  /// Update application state
  fn update(&mut self) {
    let current_time = Instant::now();
    let delta_time = current_time
      .duration_since(self.last_frame_time)
      .as_secs_f32();

    self.params.update_time(delta_time);

    #[cfg(feature = "audio")]
    audio::update_audio_reactive(
      &mut self.params,
      &self.audio_capture,
      &mut self.audio_analyzer,
      delta_time,
      &mut self.debug_log,
    );

    self.check_and_apply_config_reload();

    self.last_frame_time = current_time;
  }

  /// Check for config file changes and apply them if valid
  fn check_and_apply_config_reload(&mut self) {
    if let Some(ref watcher) = self.config_watcher {
      if let Some(mut new_params) = watcher.try_receive_config() {
        let current_time = self.params.time;
        let current_width = self.params.resolution_width;
        let current_height = self.params.resolution_height;

        new_params.time = current_time;
        new_params.set_resolution(current_width, current_height);

        if new_params.palette != self.params.palette {
          let new_palette = Self::palette_from_type(new_params.palette);
          self.converter = AsciiConverter::new(new_palette, true);
        }

        self.params = new_params;

        let _ = writeln!(self.debug_log, "Config reloaded successfully");
      }
    }
  }

  /// Render current frame
  fn render(&mut self) -> Result<()> {
    let uniforms = ShaderUniforms::from_params(&self.params);

    writeln!(
      self.debug_log,
      "DEBUG: Uniforms - time: {}, freq: {}, amp: {}, scale: {}",
      self.params.time, self.params.frequency, self.params.amplitude, self.params.scale
    )?;
    writeln!(
      self.debug_log,
      "DEBUG: Resolution in uniforms: {}x{}",
      self.params.resolution_width, self.params.resolution_height
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

    // Convert terminal background color from normalized floats to u8
    let terminal_bg = if self.params.terminal_bg_r > 0.0
      || self.params.terminal_bg_g > 0.0
      || self.params.terminal_bg_b > 0.0
    {
      Some((
        (self.params.terminal_bg_r * 255.0) as u8,
        (self.params.terminal_bg_g * 255.0) as u8,
        (self.params.terminal_bg_b * 255.0) as u8,
      ))
    } else {
      None
    };

    rendering::render_frame(
      &self.pipeline,
      &self.converter,
      &uniforms,
      status_bar,
      terminal_bg,
      &mut self.debug_log,
    )?;

    self.debug_log.flush()?;
    Ok(())
  }

  /// Check if audio is currently active
  fn check_audio_activity(&self) -> bool {
    #[cfg(feature = "audio")]
    {
      if self.params.audio_enabled {
        if let (Some(capture), Some(_)) = (&self.audio_capture, &self.audio_analyzer) {
          let samples = capture.get_samples();
          return !samples.is_empty() && samples.iter().any(|s| s.abs() > AUDIO_SAMPLE_THRESHOLD);
        }
      }
    }
    false
  }

  /// Build status bar string
  fn build_status_bar(&self, has_sound: bool) -> String {
    let (current_width, _) = terminal::size().unwrap_or((80, 24));
    let available_cols = current_width as usize;
    let status_text = status_bar::build_status_text(&self.params, self.params.effect_type);

    status_bar::format_status_bar(status_text, available_cols, has_sound, self.params.time)
  }

  /// Handle window resize
  async fn handle_resize(&mut self, new_width: u16, new_height: u16) -> Result<()> {
    writeln!(
      self.debug_log,
      "RESIZE: Terminal resized to {}x{} (was {}x{})",
      new_width, new_height, self.last_terminal_size.0, self.last_terminal_size.1
    )?;

    let shader_width = new_width as u32;
    let shader_height = if self.show_status_bar {
      (new_height - 1) as u32
    } else {
      new_height as u32
    };

    self.params.set_resolution(shader_width, shader_height);

    self.pipeline = ShaderPipeline::new(
      shader_width,
      shader_height,
      self.custom_shader.clone(),
      &mut self.debug_log,
    )
    .await?;

    self.last_terminal_size = (new_width, new_height);

    writeln!(
      self.debug_log,
      "RESIZE: Pipeline recreated with dimensions {}x{}",
      shader_width, shader_height
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
