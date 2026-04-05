use serde::{Deserialize, Serialize};

define_named_enum!(
  #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
  pub enum PaletteType {
    Standard => {
      full: "standard",
      display: "Std",
      aliases: ["std"]
    },
    Blocks => {
      full: "blocks",
      display: "Block",
      aliases: ["block"]
    },
    Circles => {
      full: "circles",
      display: "Circle",
      aliases: ["circle"]
    },
    Smooth => {
      full: "smooth",
      display: "Smooth",
      aliases: []
    },
    Braille => {
      full: "braille",
      display: "Braille",
      aliases: []
    },
    Geometric => {
      full: "geometric",
      display: "Geo",
      aliases: ["geo"]
    },
    Mixed => {
      full: "mixed",
      display: "Mixed",
      aliases: []
    },
    Dots => {
      full: "dots",
      display: "Dots",
      aliases: []
    },
    Shades => {
      full: "shades",
      display: "Shade",
      aliases: ["shade"]
    },
    Lines => {
      full: "lines",
      display: "Lines",
      aliases: []
    },
    Triangles => {
      full: "triangles",
      display: "Tri",
      aliases: ["tri"]
    },
    Arrows => {
      full: "arrows",
      display: "Arrow",
      aliases: ["arrow"]
    },
    Powerline => {
      full: "powerline",
      display: "Power",
      aliases: ["power"]
    },
    BoxDraw => {
      full: "boxdraw",
      display: "Box",
      aliases: ["box"]
    },
    Extended => {
      full: "extended",
      display: "Extend",
      aliases: ["extend"]
    },
    Simple => {
      full: "simple",
      display: "Simple",
      aliases: []
    }
  },
  error_label: "palette type"
);
