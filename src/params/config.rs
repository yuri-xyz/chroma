use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PatternType {
  Plasma,
  Waves,
  Ripples,
  Vortex,
  Noise, // Reduce in randomizer
  Geometric,
  Voronoi,
  Truchet,
  Hexagonal,
  Interference,
  Fractal,
  Glitch,
  Spiral,
  Rings,
  Grid,
  Diamonds,
  Sphere,
  Octgrams,
  WarpedFbm,
}

impl PatternType {
  pub fn to_u32(self) -> u32 {
    match self {
      Self::Plasma => 0,
      Self::Waves => 1,
      Self::Ripples => 2,
      Self::Vortex => 3,
      Self::Noise => 4,
      Self::Geometric => 5,
      Self::Voronoi => 6,
      Self::Truchet => 7,
      Self::Hexagonal => 8,
      Self::Interference => 9,
      Self::Fractal => 10,
      Self::Glitch => 11,
      Self::Spiral => 12,
      Self::Rings => 13,
      Self::Grid => 14,
      Self::Diamonds => 15,
      Self::Sphere => 16,
      Self::Octgrams => 17,
      Self::WarpedFbm => 18,
    }
  }

  pub fn name(self) -> &'static str {
    match self {
      Self::Plasma => "Plasma",
      Self::Waves => "Waves",
      Self::Ripples => "Ripples",
      Self::Vortex => "Vortex",
      Self::Noise => "Noise",
      Self::Geometric => "Geo",
      Self::Voronoi => "Voronoi",
      Self::Truchet => "Truchet",
      Self::Hexagonal => "Hexagon",
      Self::Interference => "Interf",
      Self::Fractal => "Fractal",
      Self::Glitch => "Glitch",
      Self::Spiral => "Spiral",
      Self::Rings => "Rings",
      Self::Grid => "Grid",
      Self::Diamonds => "Diamond",
      Self::Sphere => "Sphere",
      Self::Octgrams => "Octgram",
      Self::WarpedFbm => "Warped",
    }
  }

  pub fn next(self) -> Self {
    match self {
      Self::Plasma => Self::Waves,
      Self::Waves => Self::Ripples,
      Self::Ripples => Self::Vortex,
      Self::Vortex => Self::Noise,
      Self::Noise => Self::Geometric,
      Self::Geometric => Self::Voronoi,
      Self::Voronoi => Self::Truchet,
      Self::Truchet => Self::Hexagonal,
      Self::Hexagonal => Self::Interference,
      Self::Interference => Self::Fractal,
      Self::Fractal => Self::Glitch,
      Self::Glitch => Self::Spiral,
      Self::Spiral => Self::Rings,
      Self::Rings => Self::Grid,
      Self::Grid => Self::Diamonds,
      Self::Diamonds => Self::Sphere,
      Self::Sphere => Self::Octgrams,
      Self::Octgrams => Self::WarpedFbm,
      Self::WarpedFbm => Self::Plasma,
    }
  }

  pub fn previous(self) -> Self {
    match self {
      Self::Plasma => Self::WarpedFbm,
      Self::WarpedFbm => Self::Octgrams,
      Self::Octgrams => Self::Sphere,
      Self::Sphere => Self::Diamonds,
      Self::Diamonds => Self::Grid,
      Self::Grid => Self::Rings,
      Self::Rings => Self::Spiral,
      Self::Spiral => Self::Glitch,
      Self::Glitch => Self::Fractal,
      Self::Fractal => Self::Interference,
      Self::Interference => Self::Hexagonal,
      Self::Hexagonal => Self::Truchet,
      Self::Truchet => Self::Voronoi,
      Self::Voronoi => Self::Geometric,
      Self::Geometric => Self::Noise,
      Self::Noise => Self::Vortex,
      Self::Vortex => Self::Ripples,
      Self::Ripples => Self::Waves,
      Self::Waves => Self::Plasma,
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ColorMode {
  Rainbow,
  Monochrome,
  Duotone,
  Warm,
  Cool,
  Neon,
  Pastel,
  Cyberpunk,
  Warped,
  Chromatic,
}

impl ColorMode {
  pub fn next(self) -> Self {
    match self {
      Self::Rainbow => Self::Monochrome,
      Self::Monochrome => Self::Duotone,
      Self::Duotone => Self::Warm,
      Self::Warm => Self::Cool,
      Self::Cool => Self::Neon,
      Self::Neon => Self::Pastel,
      Self::Pastel => Self::Cyberpunk,
      Self::Cyberpunk => Self::Warped,
      Self::Warped => Self::Chromatic,
      Self::Chromatic => Self::Rainbow,
    }
  }

  pub fn previous(self) -> Self {
    match self {
      Self::Rainbow => Self::Chromatic,
      Self::Chromatic => Self::Warped,
      Self::Warped => Self::Cyberpunk,
      Self::Cyberpunk => Self::Pastel,
      Self::Pastel => Self::Neon,
      Self::Neon => Self::Cool,
      Self::Cool => Self::Warm,
      Self::Warm => Self::Duotone,
      Self::Duotone => Self::Monochrome,
      Self::Monochrome => Self::Rainbow,
    }
  }

  pub fn name(self) -> &'static str {
    match self {
      Self::Rainbow => "Rainbow",
      Self::Monochrome => "Mono",
      Self::Duotone => "Duotone",
      Self::Warm => "Warm",
      Self::Cool => "Cool",
      Self::Neon => "Neon",
      Self::Pastel => "Pastel",
      Self::Cyberpunk => "Cyber",
      Self::Warped => "Warped",
      Self::Chromatic => "Chrome",
    }
  }

  pub fn to_u32(self) -> u32 {
    match self {
      Self::Rainbow => 0,
      Self::Monochrome => 1,
      Self::Duotone => 2,
      Self::Warm => 3,
      Self::Cool => 4,
      Self::Neon => 5,
      Self::Pastel => 6,
      Self::Cyberpunk => 7,
      Self::Warped => 8,
      Self::Chromatic => 9,
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaletteType {
  Standard,
  Blocks,
  Circles,
  Smooth,
  Braille,
  Geometric,
  Mixed,
  Dots,
  Extended,
  Simple,
  Shades,
  Lines,
  Triangles,
  Arrows,
  Powerline,
  BoxDraw,
}

impl PaletteType {
  pub fn next(self) -> Self {
    match self {
      Self::Standard => Self::Blocks,
      Self::Blocks => Self::Circles,
      Self::Circles => Self::Smooth,
      Self::Smooth => Self::Braille,
      Self::Braille => Self::Geometric,
      Self::Geometric => Self::Mixed,
      Self::Mixed => Self::Dots,
      Self::Dots => Self::Shades,
      Self::Shades => Self::Lines,
      Self::Lines => Self::Triangles,
      Self::Triangles => Self::Arrows,
      Self::Arrows => Self::Powerline,
      Self::Powerline => Self::BoxDraw,
      Self::BoxDraw => Self::Extended,
      Self::Extended => Self::Simple,
      Self::Simple => Self::Standard,
    }
  }

  pub fn previous(self) -> Self {
    match self {
      Self::Standard => Self::Simple,
      Self::Simple => Self::Extended,
      Self::Extended => Self::BoxDraw,
      Self::BoxDraw => Self::Powerline,
      Self::Powerline => Self::Arrows,
      Self::Arrows => Self::Triangles,
      Self::Triangles => Self::Lines,
      Self::Lines => Self::Shades,
      Self::Shades => Self::Dots,
      Self::Dots => Self::Mixed,
      Self::Mixed => Self::Geometric,
      Self::Geometric => Self::Braille,
      Self::Braille => Self::Smooth,
      Self::Smooth => Self::Circles,
      Self::Circles => Self::Blocks,
      Self::Blocks => Self::Standard,
    }
  }

  pub fn name(self) -> &'static str {
    match self {
      Self::Standard => "Std",
      Self::Blocks => "Block",
      Self::Circles => "Circle",
      Self::Smooth => "Smooth",
      Self::Braille => "Braille",
      Self::Geometric => "Geo",
      Self::Mixed => "Mixed",
      Self::Dots => "Dots",
      Self::Shades => "Shade",
      Self::Lines => "Lines",
      Self::Triangles => "Tri",
      Self::Arrows => "Arrow",
      Self::Powerline => "Power",
      Self::BoxDraw => "Box",
      Self::Extended => "Extend",
      Self::Simple => "Simple",
    }
  }
}

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

      vignette: 0.0,
      vignette_softness: 0.5,
      glyph_sharpness: 1.0,

      background_tint_r: 0.0,
      background_tint_g: 0.0,
      background_tint_b: 0.0,

      palette: PaletteType::Circles,
      color_mode: ColorMode::Chromatic,
      pattern_type: PatternType::Octgrams,

      audio_enabled: false,
      bass_influence: 0.5,
      mid_influence: 0.3,
      treble_influence: 0.2,

      effect_time: -100.0,
      effect_type: 2,
    }
  }
}

impl ShaderParams {
  /// Configure for audio-reactive mode with calm initial state
  /// Starts nearly still and dimmed, waiting for audio to bring it to life
  pub fn with_audio_reactive_defaults() -> Self {
    Self {
      speed: 0.05,         // Nearly still (vs default 1.0)
      brightness: 0.6,     // Dimmed (vs default 1.2)
      contrast: 0.8,       // Softer (vs default 1.0)
      amplitude: 0.4,      // Minimal (vs default 1.0)
      frequency: 6.0,      // Lower detail (vs default 10.0)
      audio_enabled: true, // Audio reactive mode ON
      effect_time: -100.0, // Far in past to prevent startup wave
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
    self.hue = self.hue % 360.0;

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
    use rand::Rng;
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
    self.color_shift = rng.gen_range(0.0..=6.28);
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

    let mut params: ShaderParams =
      toml::from_str(&content).context("Failed to parse config file")?;

    params.clamp_all();

    Ok(params)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_default_params() {
    let params = ShaderParams::default();

    assert_eq!(params.time, 0.0);
    assert_eq!(params.frequency, 10.0);
    assert_eq!(params.amplitude, 1.0);
    assert_eq!(params.brightness, 1.2);
    assert_eq!(params.contrast, 1.0);
  }

  #[test]
  fn test_update_time() {
    let mut params = ShaderParams::default();
    params.speed = 2.0;

    params.update_time(1.0);

    assert_eq!(params.time, 2.0);
  }

  #[test]
  fn test_set_resolution() {
    let mut params = ShaderParams::default();

    params.set_resolution(100, 50);

    assert_eq!(params.resolution_width, 100);
    assert_eq!(params.resolution_height, 50);
  }

  #[test]
  fn test_randomize() {
    let mut params = ShaderParams::default();
    let original_frequency = params.frequency;

    params.randomize();

    assert!(params.frequency >= 3.0 && params.frequency <= 18.0);
    assert!(params.brightness >= 0.8 && params.brightness <= 1.8);
    assert!(params.contrast >= 0.5 && params.contrast <= 1.8);
    assert!(params.hue >= 0.0 && params.hue < 360.0);

    assert_ne!(params.frequency, original_frequency);
  }

  #[test]
  fn test_clamp_all() {
    let mut params = ShaderParams::default();

    params.frequency = 100.0;
    params.brightness = 10.0;
    params.hue = 400.0;

    params.clamp_all();

    assert!(params.frequency <= 18.0);
    assert!(params.brightness <= 2.0);
    assert!(params.hue < 360.0);
  }
}
