use serde::{Deserialize, Serialize};

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
            Self::Dots => Self::Extended,
            Self::Extended => Self::Simple,
            Self::Simple => Self::Standard,
        }
    }

    pub fn previous(self) -> Self {
        match self {
            Self::Standard => Self::Simple,
            Self::Simple => Self::Extended,
            Self::Extended => Self::Dots,
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
            Self::Standard => "Standard",
            Self::Blocks => "Blocks",
            Self::Circles => "Circles",
            Self::Smooth => "Smooth",
            Self::Braille => "Braille",
            Self::Geometric => "Geometric",
            Self::Mixed => "Mixed",
            Self::Dots => "Dots",
            Self::Extended => "Extended",
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
    
    pub audio_enabled: bool,
    pub bass_influence: f32,
    pub mid_influence: f32,
    pub treble_influence: f32,
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
            
            audio_enabled: false,
            bass_influence: 0.5,
            mid_influence: 0.3,
            treble_influence: 0.2,
        }
    }
}

impl ShaderParams {
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

        self.frequency = rng.gen_range(3.0..=18.0);
        self.amplitude = rng.gen_range(0.5..=2.0);
        self.speed = rng.gen_range(0.1..=1.0);
        self.scale = rng.gen_range(0.5..=3.0);
        self.color_shift = rng.gen_range(0.0..=6.28);

        self.noise_strength = rng.gen_range(0.0..=0.3);
        self.distort_amplitude = rng.gen_range(0.0..=1.5);
        self.noise_scale = rng.gen_range(0.001..=0.008);
        self.z_rate = rng.gen_range(0.01..=0.05);

        self.brightness = rng.gen_range(0.8..=1.8);
        self.contrast = rng.gen_range(0.5..=1.8);
        self.hue = rng.gen_range(0.0..360.0);
        self.saturation = rng.gen_range(0.6..=1.5);
        self.gamma = rng.gen_range(0.8..=1.3);

        self.vignette = if rng.gen_bool(0.3) {
            rng.gen_range(0.1..=0.5)
        } else {
            0.0
        };
        self.vignette_softness = rng.gen_range(0.3..=0.8);
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
