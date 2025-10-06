use serde::{Deserialize, Serialize};

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
