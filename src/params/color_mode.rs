use serde::{Deserialize, Serialize};

define_named_enum!(
  #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
  pub enum ColorMode {
    Rainbow => {
      full: "rainbow",
      display: "Rainbow",
      aliases: []
    },
    Monochrome => {
      full: "monochrome",
      display: "Mono",
      aliases: ["mono"]
    },
    Duotone => {
      full: "duotone",
      display: "Duotone",
      aliases: []
    },
    Warm => {
      full: "warm",
      display: "Warm",
      aliases: []
    },
    Cool => {
      full: "cool",
      display: "Cool",
      aliases: []
    },
    Neon => {
      full: "neon",
      display: "Neon",
      aliases: []
    },
    Pastel => {
      full: "pastel",
      display: "Pastel",
      aliases: []
    },
    Cyberpunk => {
      full: "cyberpunk",
      display: "Cyber",
      aliases: ["cyber"]
    },
    Warped => {
      full: "warped",
      display: "Warped",
      aliases: []
    },
    Fire => {
      full: "fire",
      display: "Fire",
      aliases: []
    },
    Ocean => {
      full: "ocean",
      display: "Ocean",
      aliases: []
    },
    Aurora => {
      full: "aurora",
      display: "Aurora",
      aliases: []
    },
    Galaxy => {
      full: "galaxy",
      display: "Galaxy",
      aliases: []
    },
    Chromatic => {
      full: "chromatic",
      display: "Chrome",
      aliases: ["chrome"]
    }
  },
  error_label: "color mode"
);
