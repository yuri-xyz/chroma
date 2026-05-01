use serde::{Deserialize, Serialize};

define_named_enum!(
  #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
  pub enum PatternType {
    Plasma => {
      full: "plasma",
      display: "Plasma",
      aliases: []
    },
    Waves => {
      full: "waves",
      display: "Waves",
      aliases: []
    },
    Ripples => {
      full: "ripples",
      display: "Ripples",
      aliases: []
    },
    Vortex => {
      full: "vortex",
      display: "Vortex",
      aliases: []
    },
    // Reduce in randomizer
    Noise => {
      full: "noise",
      display: "Noise",
      aliases: []
    },
    Geometric => {
      full: "geometric",
      display: "Geo",
      aliases: ["geo"]
    },
    Voronoi => {
      full: "voronoi",
      display: "Voronoi",
      aliases: []
    },
    Truchet => {
      full: "truchet",
      display: "Truchet",
      aliases: []
    },
    Hexagonal => {
      full: "hexagonal",
      display: "Hexagon",
      aliases: ["hexagon", "hex"]
    },
    Interference => {
      full: "interference",
      display: "Interf",
      aliases: ["interf"]
    },
    Fractal => {
      full: "fractal",
      display: "Fractal",
      aliases: []
    },
    Glitch => {
      full: "glitch",
      display: "Glitch",
      aliases: []
    },
    Spiral => {
      full: "spiral",
      display: "Spiral",
      aliases: []
    },
    Rings => {
      full: "rings",
      display: "Rings",
      aliases: []
    },
    Grid => {
      full: "grid",
      display: "Grid",
      aliases: []
    },
    Diamonds => {
      full: "diamonds",
      display: "Diamond",
      aliases: ["diamond"]
    },
    Sphere => {
      full: "sphere",
      display: "Sphere",
      aliases: []
    },
    Octgrams => {
      full: "octgrams",
      display: "Octgram",
      aliases: ["octgram"]
    },
    WarpedFbm => {
      full: "warped",
      display: "Warped",
      aliases: ["warpedfbm"]
    },
    Kaleidoscope => {
      full: "kaleidoscope",
      display: "Kaleido",
      aliases: ["kaleido", "kal"]
    },
    Tunnel => {
      full: "tunnel",
      display: "Tunnel",
      aliases: ["tun"]
    },
    Metaballs => {
      full: "metaballs",
      display: "Metaball",
      aliases: ["metaball", "meta", "blobs"]
    },
    World => {
      full: "world",
      display: "World",
      aliases: ["globe", "earth"]
    },
    Fluid => {
      full: "fluid",
      display: "Fluid",
      aliases: ["water", "caustics"]
    },
    Pyramid => {
      full: "pyramid",
      display: "Pyramid",
      aliases: ["pyr", "obelisk"]
    }
  },
  error_label: "pattern type"
);
