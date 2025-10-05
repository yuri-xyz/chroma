// Keyboard input handling

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::Duration;
use term_shaders::ascii::{AsciiConverter, AsciiPalette};
use term_shaders::params::{PaletteType, ShaderParams};

/// Handle keyboard input events
pub fn handle_input(
  params: &mut ShaderParams,
  converter: &mut AsciiConverter,
  running: &mut bool,
  debug_log: &mut BufWriter<File>,
) -> Result<()> {
  if !event::poll(Duration::from_millis(0))? {
    return Ok(());
  }

  match event::read()? {
    Event::Key(KeyEvent {
      code,
      kind: KeyEventKind::Press,
      ..
    }) => {
      handle_key_press(code, params, converter, running, debug_log)?;
    }
    _ => {}
  }

  Ok(())
}

/// Handle individual key press events
fn handle_key_press(
  code: KeyCode,
  params: &mut ShaderParams,
  converter: &mut AsciiConverter,
  running: &mut bool,
  debug_log: &mut BufWriter<File>,
) -> Result<()> {
  match code {
    // Application control
    KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
      *running = false;
    }

    // Parameter adjustments
    KeyCode::Up => params.frequency += 0.1,
    KeyCode::Down => params.frequency = (params.frequency - 0.1).max(0.1),
    KeyCode::Right => params.speed += 0.1,
    KeyCode::Left => params.speed = (params.speed - 0.1).max(0.1),
    KeyCode::Char('+') | KeyCode::Char('=') => params.amplitude += 0.1,
    KeyCode::Char('-') | KeyCode::Char('_') => params.amplitude = (params.amplitude - 0.1).max(0.1),
    KeyCode::Char('[') => params.scale = (params.scale - 0.1).max(0.1),
    KeyCode::Char(']') => params.scale += 0.1,

    // Pattern selection
    KeyCode::Char('t') | KeyCode::Char('T') => {
      params.pattern_type = params.pattern_type.next();
    }
    KeyCode::Char('y') | KeyCode::Char('Y') => {
      params.pattern_type = params.pattern_type.previous();
    }

    // Color mode
    KeyCode::Char('c') | KeyCode::Char('C') => {
      params.color_mode = params.color_mode.next();
    }

    // Palette selection
    KeyCode::Char('p') | KeyCode::Char('P') => {
      params.palette = params.palette.next();
      let new_palette = palette_from_type(params.palette);
      converter.set_palette(new_palette);
    }
    KeyCode::Char('o') | KeyCode::Char('O') => {
      params.palette = params.palette.previous();
      let new_palette = palette_from_type(params.palette);
      converter.set_palette(new_palette);
    }

    // Randomization
    KeyCode::Char('r') | KeyCode::Char('R') => {
      params.randomize();
      let new_palette = palette_from_type(params.palette);
      converter.set_palette(new_palette);
    }

    // Effects
    KeyCode::Char('e') | KeyCode::Char('E') | KeyCode::Char(' ') => {
      params.effect_time = params.time;
      writeln!(
        debug_log,
        "EFFECT: Triggered effect type {} at time {:.2}",
        params.effect_type, params.effect_time
      )?;
    }
    KeyCode::Char('n') | KeyCode::Char('N') => {
      params.effect_type = (params.effect_type + 1) % 7;
      writeln!(
        debug_log,
        "EFFECT: Switched to effect type {}",
        params.effect_type
      )?;
    }

    // Audio toggle
    KeyCode::Char('a') | KeyCode::Char('A') => {
      #[cfg(feature = "audio")]
      {
        params.audio_enabled = !params.audio_enabled;
        writeln!(
          debug_log,
          "AUDIO: Audio reactivity {}",
          if params.audio_enabled {
            "enabled"
          } else {
            "disabled"
          }
        )?;
      }
    }

    // Save configuration
    KeyCode::Char('s') | KeyCode::Char('S') => match params.save_to_file() {
      Ok(filename) => {
        writeln!(debug_log, "CONFIG: Saved configuration to {}", filename)?;
      }
      Err(error) => {
        writeln!(debug_log, "CONFIG: Failed to save configuration: {}", error)?;
      }
    },

    _ => {}
  }

  Ok(())
}

/// Convert palette type to ASCII palette
fn palette_from_type(palette_type: PaletteType) -> AsciiPalette {
  match palette_type {
    PaletteType::Standard => AsciiPalette::standard(),
    PaletteType::Blocks => AsciiPalette::blocks(),
    PaletteType::Circles => AsciiPalette::circles(),
    PaletteType::Smooth => AsciiPalette::smooth(),
    PaletteType::Braille => AsciiPalette::braille(),
    PaletteType::Geometric => AsciiPalette::geometric(),
    PaletteType::Mixed => AsciiPalette::mixed(),
    PaletteType::Dots => AsciiPalette::dots(),
    PaletteType::Shades => AsciiPalette::shades(),
    PaletteType::Lines => AsciiPalette::lines(),
    PaletteType::Triangles => AsciiPalette::triangles(),
    PaletteType::Arrows => AsciiPalette::arrows(),
    PaletteType::Powerline => AsciiPalette::powerline(),
    PaletteType::BoxDraw => AsciiPalette::boxdraw(),
    PaletteType::Extended => AsciiPalette::extended(),
    PaletteType::Simple => AsciiPalette::simple(),
  }
}
