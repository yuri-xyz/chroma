// Application main module

mod audio;
mod input;
mod rendering;
mod status_bar;

#[cfg(feature = "audio")]
use crate::constants::AUDIO_SAMPLE_THRESHOLD;
use crate::constants::FRAME_DURATION;
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
  last_frame_time: Instant,
  debug_log: DebugLog,
  last_terminal_size: (u16, u16),
  #[cfg(feature = "audio")]
  audio_capture: Option<AudioCapture>,
  #[cfg(feature = "audio")]
  audio_analyzer: Option<AudioAnalyzer>,
}

impl App {
  /// Create a new application instance
  pub async fn new(
    loaded_config: Option<ShaderParams>,
    #[cfg(feature = "audio")] audio_device: Option<String>,
  ) -> Result<Self> {
    #[cfg(debug_assertions)]
    let mut debug_log = {
      let debug_file = File::create("debug.log")?;
      BufWriter::new(debug_file)
    };

    #[cfg(not(debug_assertions))]
    let mut debug_log = BufWriter::new(std::io::sink());

    let (terminal_width, terminal_height) = terminal::size()?;
    writeln!(
      debug_log,
      "DEBUG: Terminal size: {}x{}",
      terminal_width, terminal_height
    )?;

    let shader_width = terminal_width as u32;
    let shader_height = (terminal_height - 1) as u32;
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

    let pipeline = ShaderPipeline::new(shader_width, shader_height).await?;
    let palette = Self::palette_from_type(params.palette);
    let converter = AsciiConverter::new(palette, true);

    #[cfg(feature = "audio")]
    let (audio_capture, audio_analyzer) =
      Self::init_audio(&mut debug_log, audio_device.as_deref())?;

    Ok(Self {
      params,
      pipeline,
      converter,
      running: true,
      last_frame_time: Instant::now(),
      debug_log,
      last_terminal_size: (terminal_width, terminal_height),
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

    self.last_frame_time = current_time;
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

    let has_sound = self.check_audio_activity();
    let status_bar = self.build_status_bar(has_sound);

    rendering::render_frame(
      &self.pipeline,
      &self.converter,
      &uniforms,
      status_bar,
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
    let shader_height = (new_height - 1) as u32;

    self.params.set_resolution(shader_width, shader_height);
    self.pipeline = ShaderPipeline::new(shader_width, shader_height).await?;
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

      // Check for window resize
      let (current_width, current_height) = terminal::size()?;
      if (current_width, current_height) != self.last_terminal_size {
        pollster::block_on(async { self.handle_resize(current_width, current_height).await })?;
      }

      // Handle input, update state, and render
      input::handle_input(
        &mut self.params,
        &mut self.converter,
        &mut self.running,
        &mut self.debug_log,
      )?;

      self.update();
      self.render()?;

      // Frame rate limiting
      let frame_time = frame_start.elapsed();
      if frame_time < FRAME_DURATION {
        std::thread::sleep(FRAME_DURATION - frame_time);
      }
    }

    Ok(())
  }
}
