use serde::{Deserialize, Serialize};
use std::str::FromStr;

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
  Kaleidoscope,
  Tunnel,
  Metaballs,
  World,
  Fluid,
}

impl PatternType {
  pub const fn all() -> &'static [Self] {
    &[
      Self::Plasma,
      Self::Waves,
      Self::Ripples,
      Self::Vortex,
      Self::Noise,
      Self::Geometric,
      Self::Voronoi,
      Self::Truchet,
      Self::Hexagonal,
      Self::Interference,
      Self::Fractal,
      Self::Glitch,
      Self::Spiral,
      Self::Rings,
      Self::Grid,
      Self::Diamonds,
      Self::Sphere,
      Self::Octgrams,
      Self::WarpedFbm,
      Self::Kaleidoscope,
      Self::Tunnel,
      Self::Metaballs,
      Self::World,
      Self::Fluid,
    ]
  }

  pub fn full_name(self) -> &'static str {
    match self {
      Self::Plasma => "plasma",
      Self::Waves => "waves",
      Self::Ripples => "ripples",
      Self::Vortex => "vortex",
      Self::Noise => "noise",
      Self::Geometric => "geometric",
      Self::Voronoi => "voronoi",
      Self::Truchet => "truchet",
      Self::Hexagonal => "hexagonal",
      Self::Interference => "interference",
      Self::Fractal => "fractal",
      Self::Glitch => "glitch",
      Self::Spiral => "spiral",
      Self::Rings => "rings",
      Self::Grid => "grid",
      Self::Diamonds => "diamonds",
      Self::Sphere => "sphere",
      Self::Octgrams => "octgrams",
      Self::WarpedFbm => "warped",
      Self::Kaleidoscope => "kaleidoscope",
      Self::Tunnel => "tunnel",
      Self::Metaballs => "metaballs",
      Self::World => "world",
      Self::Fluid => "fluid",
    }
  }

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
      Self::Kaleidoscope => 19,
      Self::Tunnel => 20,
      Self::Metaballs => 21,
      Self::World => 22,
      Self::Fluid => 23,
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
      Self::Kaleidoscope => "Kaleido",
      Self::Tunnel => "Tunnel",
      Self::Metaballs => "Metaball",
      Self::World => "World",
      Self::Fluid => "Fluid",
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
      Self::WarpedFbm => Self::Kaleidoscope,
      Self::Kaleidoscope => Self::Tunnel,
      Self::Tunnel => Self::Metaballs,
      Self::Metaballs => Self::World,
      Self::World => Self::Fluid,
      Self::Fluid => Self::Plasma,
    }
  }

  pub fn previous(self) -> Self {
    match self {
      Self::Plasma => Self::Fluid,
      Self::Fluid => Self::World,
      Self::World => Self::Metaballs,
      Self::Metaballs => Self::Tunnel,
      Self::Tunnel => Self::Kaleidoscope,
      Self::Kaleidoscope => Self::WarpedFbm,
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

impl FromStr for PatternType {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.to_lowercase().as_str() {
      "plasma" => Ok(Self::Plasma),
      "waves" => Ok(Self::Waves),
      "ripples" => Ok(Self::Ripples),
      "vortex" => Ok(Self::Vortex),
      "noise" => Ok(Self::Noise),
      "geometric" | "geo" => Ok(Self::Geometric),
      "voronoi" => Ok(Self::Voronoi),
      "truchet" => Ok(Self::Truchet),
      "hexagonal" | "hexagon" | "hex" => Ok(Self::Hexagonal),
      "interference" | "interf" => Ok(Self::Interference),
      "fractal" => Ok(Self::Fractal),
      "glitch" => Ok(Self::Glitch),
      "spiral" => Ok(Self::Spiral),
      "rings" => Ok(Self::Rings),
      "grid" => Ok(Self::Grid),
      "diamonds" | "diamond" => Ok(Self::Diamonds),
      "sphere" => Ok(Self::Sphere),
      "octgrams" | "octgram" => Ok(Self::Octgrams),
      "warped" | "warpedfbm" => Ok(Self::WarpedFbm),
      "kaleidoscope" | "kaleido" | "kal" => Ok(Self::Kaleidoscope),
      "tunnel" | "tun" => Ok(Self::Tunnel),
      "metaballs" | "metaball" | "meta" | "blobs" => Ok(Self::Metaballs),
      "world" | "globe" | "earth" => Ok(Self::World),
      "fluid" | "water" | "caustics" => Ok(Self::Fluid),
      _ => Err(format!("Unknown pattern type: {}", s)),
    }
  }
}
