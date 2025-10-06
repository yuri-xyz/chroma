use serde::{Deserialize, Serialize};

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
  pub const fn all() -> &'static [Self] {
    &[
      Self::Rainbow,
      Self::Monochrome,
      Self::Duotone,
      Self::Warm,
      Self::Cool,
      Self::Neon,
      Self::Pastel,
      Self::Cyberpunk,
      Self::Warped,
      Self::Chromatic,
    ]
  }

  pub fn full_name(self) -> &'static str {
    match self {
      Self::Rainbow => "rainbow",
      Self::Monochrome => "monochrome",
      Self::Duotone => "duotone",
      Self::Warm => "warm",
      Self::Cool => "cool",
      Self::Neon => "neon",
      Self::Pastel => "pastel",
      Self::Cyberpunk => "cyberpunk",
      Self::Warped => "warped",
      Self::Chromatic => "chromatic",
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
}
