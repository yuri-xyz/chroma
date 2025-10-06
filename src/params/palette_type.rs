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
  Shades,
  Lines,
  Triangles,
  Arrows,
  Powerline,
  BoxDraw,
}

impl PaletteType {
  pub const fn all() -> &'static [Self] {
    &[
      Self::Standard,
      Self::Blocks,
      Self::Circles,
      Self::Smooth,
      Self::Braille,
      Self::Geometric,
      Self::Mixed,
      Self::Dots,
      Self::Extended,
      Self::Simple,
      Self::Shades,
      Self::Lines,
      Self::Triangles,
      Self::Arrows,
      Self::Powerline,
      Self::BoxDraw,
    ]
  }

  pub fn full_name(self) -> &'static str {
    match self {
      Self::Standard => "standard",
      Self::Blocks => "blocks",
      Self::Circles => "circles",
      Self::Smooth => "smooth",
      Self::Braille => "braille",
      Self::Geometric => "geometric",
      Self::Mixed => "mixed",
      Self::Dots => "dots",
      Self::Extended => "extended",
      Self::Simple => "simple",
      Self::Shades => "shades",
      Self::Lines => "lines",
      Self::Triangles => "triangles",
      Self::Arrows => "arrows",
      Self::Powerline => "powerline",
      Self::BoxDraw => "boxdraw",
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
}
