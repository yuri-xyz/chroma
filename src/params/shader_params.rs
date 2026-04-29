use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use rand::{rngs::StdRng, SeedableRng};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::{randomizer, ColorMode, PaletteType, PatternType};

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

  pub terminal_bg_r: f32,
  pub terminal_bg_g: f32,
  pub terminal_bg_b: f32,

  pub palette: PaletteType,
  pub color_mode: ColorMode,
  pub pattern_type: PatternType,

  pub audio_enabled: bool,
  pub bass_influence: f32,
  pub mid_influence: f32,
  pub treble_influence: f32,
  pub beat_sensitivity: f32,

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

      terminal_bg_r: 0.0,
      terminal_bg_g: 0.0,
      terminal_bg_b: 0.0,

      palette: PaletteType::Simple,
      color_mode: ColorMode::Chromatic,
      pattern_type: PatternType::Plasma,

      audio_enabled: true,
      bass_influence: 0.5,
      mid_influence: 0.3,
      treble_influence: 0.2,
      beat_sensitivity: 1.0, // Default balanced sensitivity

      effect_time: -100.0,
      effect_type: 0,

      beat_distortion_time: -100.0,
      beat_distortion_strength: 0.85,
      beat_zoom_strength: 0.7,
    }
  }
}

impl ShaderParams {
  fn adjust_clamped(current: &mut f32, delta: f32, min: f32, max: f32) {
    *current = (*current + delta).clamp(min, max);
  }

  fn normalized_hue(hue: f32) -> f32 {
    let normalized = hue % 360.0;

    if normalized < 0.0 {
      normalized + 360.0
    } else {
      normalized
    }
  }

  fn config_hash(&self) -> String {
    let toml_string = toml::to_string(self).unwrap_or_default();
    let mut hasher = Sha256::new();

    hasher.update(toml_string.as_bytes());

    let result = hasher.finalize();

    format!("{:x}", result)[..12].to_string()
  }

  fn config_filename(&self) -> String {
    format!("config_{}.toml", self.config_hash())
  }

  fn config_path_in<P: AsRef<Path>>(&self, directory: P) -> PathBuf {
    directory.as_ref().join(self.config_filename())
  }

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
    self.audio_enabled = true;
    self.amplitude = 1.0 + bass * self.bass_influence;
    self.color_shift = mid * self.mid_influence;
    self.frequency = 1.0 + treble * self.treble_influence;
  }

  pub fn clamp_all(&mut self) {
    self.audio_enabled = true;

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
    self.hue = Self::normalized_hue(self.hue);

    self.saturation = self.saturation.clamp(0.0, 2.0);
    self.gamma = self.gamma.clamp(0.5, 2.0);

    self.vignette = self.vignette.clamp(0.0, 1.0);
    self.vignette_softness = self.vignette_softness.clamp(0.0, 1.0);
    self.glyph_sharpness = self.glyph_sharpness.clamp(0.5, 2.0);

    self.background_tint_r = self.background_tint_r.clamp(0.0, 1.0);
    self.background_tint_g = self.background_tint_g.clamp(0.0, 1.0);
    self.background_tint_b = self.background_tint_b.clamp(0.0, 1.0);

    self.terminal_bg_r = self.terminal_bg_r.clamp(0.0, 1.0);
    self.terminal_bg_g = self.terminal_bg_g.clamp(0.0, 1.0);
    self.terminal_bg_b = self.terminal_bg_b.clamp(0.0, 1.0);

    self.bass_influence = self.bass_influence.clamp(0.0, 1.0);
    self.mid_influence = self.mid_influence.clamp(0.0, 1.0);
    self.treble_influence = self.treble_influence.clamp(0.0, 1.0);
    self.beat_sensitivity = self.beat_sensitivity.clamp(0.1, 3.0);
  }

  pub fn adjust_frequency(&mut self, delta: f32) {
    Self::adjust_clamped(&mut self.frequency, delta, 3.0, 18.0);
  }

  pub fn adjust_amplitude(&mut self, delta: f32) {
    Self::adjust_clamped(&mut self.amplitude, delta, 0.0, 2.0);
  }

  pub fn adjust_speed(&mut self, delta: f32) {
    Self::adjust_clamped(&mut self.speed, delta, 0.0, 1.0);
  }

  pub fn adjust_scale(&mut self, delta: f32) {
    Self::adjust_clamped(&mut self.scale, delta, 0.1, 5.0);
  }

  pub fn adjust_brightness(&mut self, delta: f32) {
    Self::adjust_clamped(&mut self.brightness, delta, 0.0, 2.0);
  }

  pub fn adjust_contrast(&mut self, delta: f32) {
    Self::adjust_clamped(&mut self.contrast, delta, 0.2, 2.0);
  }

  pub fn adjust_saturation(&mut self, delta: f32) {
    Self::adjust_clamped(&mut self.saturation, delta, 0.0, 2.0);
  }

  pub fn adjust_hue(&mut self, delta: f32) {
    self.hue = Self::normalized_hue(self.hue + delta);
  }

  pub fn randomize(&mut self) {
    randomizer::randomize(self);
  }

  pub fn randomize_with_seed(&mut self, seed: u64) {
    let mut rng = StdRng::seed_from_u64(seed);

    randomizer::randomize_with_rng(self, &mut rng);
  }

  pub fn save_to_file(&self) -> Result<String> {
    let filename = self.config_filename();

    self.save_to_file_in(".")?;

    Ok(filename)
  }

  pub fn save_to_file_in<P: AsRef<Path>>(&self, directory: P) -> Result<PathBuf> {
    let path = self.config_path_in(&directory);

    if path.exists() {
      return Ok(path);
    }

    let toml_content =
      toml::to_string_pretty(self).context("Failed to serialize configuration to TOML")?;

    fs::create_dir_all(directory.as_ref()).context(format!(
      "Failed to create config directory: {}",
      directory.as_ref().display()
    ))?;
    fs::write(&path, toml_content)
      .context(format!("Failed to write config file: {}", path.display()))?;

    Ok(path)
  }

  pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
    let content = fs::read_to_string(path.as_ref()).context(format!(
      "Failed to read config file: {}",
      path.as_ref().display()
    ))?;

    Self::load_from_str(&content)
  }

  /// Load shader params from a TOML string, merging with defaults for missing fields.
  pub fn load_from_str(content: &str) -> Result<Self> {
    // Start with defaults, then deserialize on top (missing fields keep defaults)
    let default_params = Self::default();
    let default_toml = toml::to_string(&default_params)?;
    let mut default_value: toml::Value = toml::from_str(&default_toml)?;

    // Parse the loaded config
    let loaded_value: toml::Value =
      toml::from_str(content).context("Failed to parse config as TOML")?;

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
