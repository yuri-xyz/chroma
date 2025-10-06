use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use rand::Rng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::{ColorMode, PaletteType, PatternType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShaderParams {
  pub time: f32,
  pub resolution_width: u32,
  pub resolution_height: u32,

  pub frequency: f32,
  pub amplitude: f32,
  pub speed: f32,
  pub color_shift: f32,
  pub scale: f32,
  pub octaves: u32,

  pub noise_strength: f32,
  pub distort_amplitude: f32,
  pub noise_scale: f32,
  pub z_rate: f32,

  pub brightness: f32,
  pub contrast: f32,
  pub hue: f32,
  pub saturation: f32,
  pub gamma: f32,

  pub vignette: f32,
  pub vignette_softness: f32,
  pub glyph_sharpness: f32,

  pub background_tint_r: f32,
  pub background_tint_g: f32,
  pub background_tint_b: f32,

  pub palette: PaletteType,
  pub color_mode: ColorMode,
  pub pattern_type: PatternType,

  pub audio_enabled: bool,
  pub bass_influence: f32,
  pub mid_influence: f32,
  pub treble_influence: f32,

  pub effect_time: f32,
  pub effect_type: u32,

  pub beat_distortion_time: f32,
  pub beat_distortion_strength: f32,
  pub beat_zoom_strength: f32,
}

impl Default for ShaderParams {
  fn default() -> Self {
    Self {
      time: 0.0,
      resolution_width: 80,
      resolution_height: 24,

      frequency: 10.0,
      amplitude: 1.0,
      speed: 0.5,
      color_shift: 0.0,
      scale: 1.0,
      octaves: 4,

      noise_strength: 0.15,
      distort_amplitude: 0.5,
      noise_scale: 0.005,
      z_rate: 0.02,

      brightness: 1.2,
      contrast: 1.0,
      hue: 0.0,
      saturation: 1.0,
      gamma: 1.0,

      vignette: 0.3,
      vignette_softness: 0.5,
      glyph_sharpness: 1.0,

      background_tint_r: 0.0,
      background_tint_g: 0.0,
      background_tint_b: 0.0,

      palette: PaletteType::Simple,
      color_mode: ColorMode::Chromatic,
      pattern_type: PatternType::Plasma,

      audio_enabled: false,
      bass_influence: 0.5,
      mid_influence: 0.3,
      treble_influence: 0.2,

      effect_time: -100.0,
      effect_type: 0,

      beat_distortion_time: -100.0,
      beat_distortion_strength: 0.6, // Default on for all modes
      beat_zoom_strength: 0.5,       // Default zoom enabled
    }
  }
}

impl ShaderParams {
  /// Configure for audio-reactive mode with calm initial state
  /// Starts nearly still and dimmed, waiting for audio to bring it to life
  pub fn with_audio_reactive_defaults() -> Self {
    Self {
      speed: 0.05,                   // Nearly still (vs default 1.0)
      brightness: 0.6,               // Dimmed (vs default 1.2)
      contrast: 0.8,                 // Softer (vs default 1.0)
      amplitude: 0.4,                // Minimal (vs default 1.0)
      frequency: 6.0,                // Lower detail (vs default 10.0)
      audio_enabled: true,           // Audio reactive mode ON
      effect_time: -100.0,           // Far in past to prevent startup wave
      beat_distortion_time: -100.0,  // Far in past to prevent startup distortion
      beat_distortion_strength: 0.8, // Default beat pop strength
      beat_zoom_strength: 0.0,       // Zoom strength (set per-beat)
      ..Default::default()
    }
  }

  pub fn update_time(&mut self, delta_time: f32) {
    self.time += delta_time * self.speed;
  }

  pub fn set_resolution(&mut self, width: u32, height: u32) {
    self.resolution_width = width;
    self.resolution_height = height;
  }

  pub fn apply_audio_data(&mut self, bass: f32, mid: f32, treble: f32) {
    if self.audio_enabled {
      self.amplitude = 1.0 + bass * self.bass_influence;
      self.color_shift = mid * self.mid_influence;
      self.frequency = 1.0 + treble * self.treble_influence;
    }
  }

  pub fn clamp_all(&mut self) {
    self.frequency = self.frequency.clamp(3.0, 18.0);
    self.amplitude = self.amplitude.clamp(0.0, 2.0);
    self.speed = self.speed.clamp(0.0, 1.0);
    self.scale = self.scale.clamp(0.1, 5.0);

    self.noise_strength = self.noise_strength.clamp(0.0, 0.5);
    self.distort_amplitude = self.distort_amplitude.clamp(0.0, 2.0);
    self.noise_scale = self.noise_scale.clamp(0.0, 0.01);
    self.z_rate = self.z_rate.clamp(0.0, 0.1);

    self.brightness = self.brightness.clamp(0.0, 2.0);
    self.contrast = self.contrast.clamp(0.2, 2.0);
    self.hue %= 360.0;

    if self.hue < 0.0 {
      self.hue += 360.0;
    }

    self.saturation = self.saturation.clamp(0.0, 2.0);
    self.gamma = self.gamma.clamp(0.5, 2.0);

    self.vignette = self.vignette.clamp(0.0, 1.0);
    self.vignette_softness = self.vignette_softness.clamp(0.0, 1.0);
    self.glyph_sharpness = self.glyph_sharpness.clamp(0.5, 2.0);

    self.background_tint_r = self.background_tint_r.clamp(0.0, 1.0);
    self.background_tint_g = self.background_tint_g.clamp(0.0, 1.0);
    self.background_tint_b = self.background_tint_b.clamp(0.0, 1.0);

    self.bass_influence = self.bass_influence.clamp(0.0, 1.0);
    self.mid_influence = self.mid_influence.clamp(0.0, 1.0);
    self.treble_influence = self.treble_influence.clamp(0.0, 1.0);
  }

  pub fn adjust_frequency(&mut self, delta: f32) {
    self.frequency = (self.frequency + delta).clamp(3.0, 18.0);
  }

  pub fn adjust_brightness(&mut self, delta: f32) {
    self.brightness = (self.brightness + delta).clamp(0.0, 2.0);
  }

  pub fn adjust_contrast(&mut self, delta: f32) {
    self.contrast = (self.contrast + delta).clamp(0.2, 2.0);
  }

  pub fn adjust_saturation(&mut self, delta: f32) {
    self.saturation = (self.saturation + delta).clamp(0.0, 2.0);
  }

  pub fn adjust_hue(&mut self, delta: f32) {
    self.hue = (self.hue + delta) % 360.0;

    if self.hue < 0.0 {
      self.hue += 360.0;
    }
  }

  pub fn randomize(&mut self) {
    let mut rng = rand::thread_rng();

    // Weighted randomization - favor good-looking patterns, reduce problematic ones
    self.pattern_type = match rng.gen_range(0..20) {
      0..=2 => PatternType::Plasma,
      3..=5 => PatternType::Waves,
      6..=8 => PatternType::Ripples,
      9..=11 => PatternType::Vortex,
      12..=13 => PatternType::Truchet,
      14 => PatternType::Interference,
      15 => PatternType::Fractal,
      16 => PatternType::Spiral,
      17 => PatternType::Rings,
      18 => PatternType::Grid,
      _ => PatternType::Voronoi,
    };

    // Reduced problematic palettes
    self.palette = match rng.gen_range(0..20) {
      0..=3 => PaletteType::Circles,
      4..=6 => PaletteType::Braille,
      7..=9 => PaletteType::Dots,
      10..=11 => PaletteType::Lines,
      12..=13 => PaletteType::Triangles,
      14 => PaletteType::Arrows,
      15 => PaletteType::Powerline,
      16 => PaletteType::BoxDraw,
      17 => PaletteType::Extended,
      18 => PaletteType::Mixed,
      _ => PaletteType::Circles,
    };

    self.effect_type = rng.gen_range(2..=6);

    self.frequency = rng.gen_range(3.0..=18.0);
    self.amplitude = rng.gen_range(0.5..=2.0);
    self.speed = rng.gen_range(0.1..=1.0);
    self.scale = rng.gen_range(0.5..=3.0);
    self.color_shift = rng.gen_range(0.0..=std::f32::consts::TAU);
    self.octaves = rng.gen_range(2..=6);

    self.noise_strength = rng.gen_range(0.0..=0.3);
    self.distort_amplitude = rng.gen_range(0.0..=1.5);
    self.noise_scale = rng.gen_range(0.001..=0.008);
    self.z_rate = rng.gen_range(0.01..=0.05);

    self.brightness = rng.gen_range(0.8..=1.8);
    self.contrast = rng.gen_range(0.5..=1.8);
    self.saturation = rng.gen_range(0.6..=1.5);
    self.gamma = rng.gen_range(0.8..=1.3);

    self.vignette = if rng.gen_bool(0.3) {
      rng.gen_range(0.1..=0.5)
    } else {
      0.0
    };

    self.vignette_softness = rng.gen_range(0.3..=0.8);
    self.glyph_sharpness = rng.gen_range(0.7..=1.5);

    if rng.gen_bool(0.2) {
      self.background_tint_r = rng.gen_range(0.0..=0.3);
      self.background_tint_g = rng.gen_range(0.0..=0.3);
      self.background_tint_b = rng.gen_range(0.0..=0.3);
    } else {
      self.background_tint_r = 0.0;
      self.background_tint_g = 0.0;
      self.background_tint_b = 0.0;
    }

    self.bass_influence = rng.gen_range(0.3..=0.8);
    self.mid_influence = rng.gen_range(0.2..=0.6);
    self.treble_influence = rng.gen_range(0.1..=0.5);
  }

  fn compute_hash(&self) -> String {
    let toml_string = toml::to_string(self).unwrap_or_default();
    let mut hasher = Sha256::new();

    hasher.update(toml_string.as_bytes());

    let result = hasher.finalize();

    format!("{:x}", result)[..12].to_string()
  }

  pub fn save_to_file(&self) -> Result<String> {
    let hash = self.compute_hash();
    let filename = format!("config_{}.toml", hash);

    if Path::new(&filename).exists() {
      return Ok(filename);
    }

    let toml_content =
      toml::to_string_pretty(self).context("Failed to serialize configuration to TOML")?;

    fs::write(&filename, toml_content)
      .context(format!("Failed to write config file: {}", filename))?;

    Ok(filename)
  }

  pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
    let content = fs::read_to_string(path.as_ref()).context(format!(
      "Failed to read config file: {}",
      path.as_ref().display()
    ))?;

    // Start with defaults, then deserialize on top (missing fields keep defaults)
    let default_params = Self::default();
    let default_toml = toml::to_string(&default_params)?;
    let mut default_value: toml::Value = toml::from_str(&default_toml)?;

    // Parse the loaded config
    let loaded_value: toml::Value =
      toml::from_str(&content).context("Failed to parse config file as TOML")?;

    // Merge loaded config into defaults (only overwrites present fields)
    if let (toml::Value::Table(ref mut default_table), toml::Value::Table(loaded_table)) =
      (&mut default_value, loaded_value)
    {
      for (key, value) in loaded_table {
        default_table.insert(key, value);
      }
    }

    // Deserialize merged config
    let mut params: ShaderParams = toml::from_str(&toml::to_string(&default_value)?)?;

    params.clamp_all();

    Ok(params)
  }
}
