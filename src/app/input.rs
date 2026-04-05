use super::DebugLog;
use anyhow::Result;
use chroma::ascii::{AsciiConverter, AsciiPalette};
use chroma::params::ShaderParams;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use std::time::Duration;

const EFFECT_TYPE_COUNT: u32 = 7;
const FIRST_ACTIVE_EFFECT_TYPE: u32 = 2;
const PARAMETER_STEP: f32 = 0.1;

/// Handle keyboard input events
pub fn handle_input(
  params: &mut ShaderParams,
  converter: &mut AsciiConverter,
  running: &mut bool,
  debug_log: &mut DebugLog,
) -> Result<()> {
  if !event::poll(Duration::from_millis(0))? {
    return Ok(());
  }

  if let Event::Key(KeyEvent {
    code,
    modifiers,
    kind: KeyEventKind::Press,
    ..
  }) = event::read()?
  {
    handle_key_press(code, modifiers, params, converter, running, debug_log)?;
  }

  Ok(())
}

fn adjust_if_manual(
  params: &mut ShaderParams,
  delta: f32,
  adjust: impl FnOnce(&mut ShaderParams, f32),
) {
  if !params.audio_enabled {
    adjust(params, delta);
  }
}

fn sync_palette(converter: &mut AsciiConverter, palette: chroma::params::PaletteType) {
  converter.set_palette(AsciiPalette::from(palette));
}

fn next_effect_type(effect_type: u32) -> u32 {
  match (effect_type + 1) % EFFECT_TYPE_COUNT {
    0 | 1 => FIRST_ACTIVE_EFFECT_TYPE,
    next => next,
  }
}

fn cycle_effect(params: &mut ShaderParams, debug_log: &mut DebugLog) -> Result<()> {
  params.effect_type = next_effect_type(params.effect_type);
  params.effect_time = params.time;

  debug_logln!(
    debug_log,
    "EFFECT: Switched to effect type {}",
    params.effect_type
  )?;

  Ok(())
}

#[cfg(feature = "audio")]
fn toggle_audio(params: &mut ShaderParams, debug_log: &mut DebugLog) -> Result<()> {
  params.audio_enabled = !params.audio_enabled;

  debug_logln!(
    debug_log,
    "AUDIO: Audio reactivity {}",
    if params.audio_enabled {
      "enabled"
    } else {
      "disabled"
    }
  )?;

  Ok(())
}

fn save_configuration(params: &ShaderParams, debug_log: &mut DebugLog) -> Result<()> {
  match params.save_to_file() {
    Ok(filename) => {
      debug_logln!(debug_log, "CONFIG: Saved configuration to {}", filename)?;
    }
    Err(error) => {
      debug_logln!(debug_log, "CONFIG: Failed to save configuration: {}", error)?;
    }
  }

  Ok(())
}

/// Handle individual key press events
fn handle_key_press(
  code: KeyCode,
  modifiers: KeyModifiers,
  params: &mut ShaderParams,
  converter: &mut AsciiConverter,
  running: &mut bool,
  debug_log: &mut DebugLog,
) -> Result<()> {
  // Handle Ctrl+C to exit
  if modifiers.contains(KeyModifiers::CONTROL) && matches!(code, KeyCode::Char('c')) {
    *running = false;
    return Ok(());
  }

  match code {
    // Quit
    KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
      *running = false;
    }

    // Parameter adjustments (disabled when audio mode is active)
    KeyCode::Up => {
      adjust_if_manual(params, PARAMETER_STEP, ShaderParams::adjust_frequency);
    }
    KeyCode::Down => {
      adjust_if_manual(params, -PARAMETER_STEP, ShaderParams::adjust_frequency);
    }
    KeyCode::Right => {
      adjust_if_manual(params, PARAMETER_STEP, ShaderParams::adjust_speed);
    }
    KeyCode::Left => {
      adjust_if_manual(params, -PARAMETER_STEP, ShaderParams::adjust_speed);
    }
    KeyCode::Char('+') | KeyCode::Char('=') => {
      adjust_if_manual(params, PARAMETER_STEP, ShaderParams::adjust_amplitude);
    }
    KeyCode::Char('-') | KeyCode::Char('_') => {
      adjust_if_manual(params, -PARAMETER_STEP, ShaderParams::adjust_amplitude);
    }
    KeyCode::Char('[') => params.adjust_scale(-PARAMETER_STEP),
    KeyCode::Char(']') => params.adjust_scale(PARAMETER_STEP),

    // Pattern selection
    KeyCode::Char('t') | KeyCode::Char('T') => {
      params.pattern_type = params.pattern_type.next();
    }

    // Color mode selection
    KeyCode::Char('c') | KeyCode::Char('C') => {
      params.color_mode = params.color_mode.next();
    }

    // Palette type selection
    KeyCode::Char('p') | KeyCode::Char('P') => {
      params.palette = params.palette.next();
      sync_palette(converter, params.palette);
    }

    // Randomization
    KeyCode::Char('r') | KeyCode::Char('R') => {
      params.randomize();
      sync_palette(converter, params.palette);
    }

    // Cycle through effects
    KeyCode::Char('n') | KeyCode::Char('N') => {
      cycle_effect(params, debug_log)?;
    }

    // Audio toggle
    KeyCode::Char('a') | KeyCode::Char('A') => {
      #[cfg(feature = "audio")]
      {
        toggle_audio(params, debug_log)?;
      }
    }

    // Save configuration
    KeyCode::Char('s') | KeyCode::Char('S') => save_configuration(params, debug_log)?,

    _ => {}
  }

  params.clamp_all();

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;
  use chroma::params::{ColorMode, PaletteType, PatternType};
  use std::fs::File;
  use std::io::BufWriter;
  use std::time::{SystemTime, UNIX_EPOCH};

  fn test_debug_log() -> DebugLog {
    #[cfg(debug_assertions)]
    {
      let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
      let path = std::env::temp_dir().join(format!("chroma-input-test-{timestamp}.log"));

      BufWriter::new(File::create(path).unwrap())
    }

    #[cfg(not(debug_assertions))]
    {
      BufWriter::new(std::io::sink())
    }
  }

  fn converter_output(converter: &AsciiConverter) -> Vec<Vec<(char, crossterm::style::Color)>> {
    converter.convert_frame(&[128, 128, 128, 255], 1, 1)
  }

  fn invoke_key(
    code: KeyCode,
    modifiers: KeyModifiers,
    params: &mut ShaderParams,
    converter: &mut AsciiConverter,
    running: &mut bool,
    debug_log: &mut DebugLog,
  ) {
    handle_key_press(code, modifiers, params, converter, running, debug_log).unwrap();
  }

  #[test]
  fn test_ctrl_c_stops_running() {
    let mut params = ShaderParams::default();
    let mut converter = AsciiConverter::new(AsciiPalette::from(params.palette), true);
    let mut running = true;
    let mut debug_log = test_debug_log();

    invoke_key(
      KeyCode::Char('c'),
      KeyModifiers::CONTROL,
      &mut params,
      &mut converter,
      &mut running,
      &mut debug_log,
    );

    assert!(!running);
  }

  #[test]
  fn test_quit_keys_stop_running() {
    for code in [KeyCode::Char('q'), KeyCode::Char('Q'), KeyCode::Esc] {
      let mut params = ShaderParams::default();
      let mut converter = AsciiConverter::new(AsciiPalette::from(params.palette), true);
      let mut running = true;
      let mut debug_log = test_debug_log();

      invoke_key(
        code,
        KeyModifiers::NONE,
        &mut params,
        &mut converter,
        &mut running,
        &mut debug_log,
      );

      assert!(!running);
    }
  }

  #[test]
  fn test_arrow_keys_adjust_params_when_audio_is_disabled() {
    let mut params = ShaderParams::default();
    params.audio_enabled = false;
    params.frequency = 10.0;
    params.speed = 0.5;
    let mut converter = AsciiConverter::new(AsciiPalette::from(params.palette), true);
    let mut running = true;
    let mut debug_log = test_debug_log();

    invoke_key(
      KeyCode::Up,
      KeyModifiers::NONE,
      &mut params,
      &mut converter,
      &mut running,
      &mut debug_log,
    );
    invoke_key(
      KeyCode::Right,
      KeyModifiers::NONE,
      &mut params,
      &mut converter,
      &mut running,
      &mut debug_log,
    );

    assert_eq!(params.frequency, 10.1);
    assert_eq!(params.speed, 0.6);
    assert!(running);
  }

  #[test]
  fn test_arrow_keys_do_not_adjust_audio_reactive_params_when_audio_is_enabled() {
    let mut params = ShaderParams::default();
    params.audio_enabled = true;
    params.frequency = 10.0;
    params.speed = 0.5;
    let mut converter = AsciiConverter::new(AsciiPalette::from(params.palette), true);
    let mut running = true;
    let mut debug_log = test_debug_log();

    invoke_key(
      KeyCode::Up,
      KeyModifiers::NONE,
      &mut params,
      &mut converter,
      &mut running,
      &mut debug_log,
    );
    invoke_key(
      KeyCode::Right,
      KeyModifiers::NONE,
      &mut params,
      &mut converter,
      &mut running,
      &mut debug_log,
    );

    assert_eq!(params.frequency, 10.0);
    assert_eq!(params.speed, 0.5);
  }

  #[test]
  fn test_amplitude_keys_clamp_at_bounds_when_audio_is_disabled() {
    let mut params = ShaderParams::default();
    params.audio_enabled = false;
    params.amplitude = 2.0;
    let mut converter = AsciiConverter::new(AsciiPalette::from(params.palette), true);
    let mut running = true;
    let mut debug_log = test_debug_log();

    invoke_key(
      KeyCode::Char('+'),
      KeyModifiers::NONE,
      &mut params,
      &mut converter,
      &mut running,
      &mut debug_log,
    );
    assert_eq!(params.amplitude, 2.0);

    params.amplitude = 0.0;
    invoke_key(
      KeyCode::Char('-'),
      KeyModifiers::NONE,
      &mut params,
      &mut converter,
      &mut running,
      &mut debug_log,
    );
    assert_eq!(params.amplitude, 0.0);
  }

  #[test]
  fn test_amplitude_alias_keys_adjust_like_primary_keys() {
    let mut params = ShaderParams::default();
    params.audio_enabled = false;
    params.amplitude = 1.0;
    let mut converter = AsciiConverter::new(AsciiPalette::from(params.palette), true);
    let mut running = true;
    let mut debug_log = test_debug_log();

    invoke_key(
      KeyCode::Char('='),
      KeyModifiers::NONE,
      &mut params,
      &mut converter,
      &mut running,
      &mut debug_log,
    );
    assert_eq!(params.amplitude, 1.1);

    invoke_key(
      KeyCode::Char('_'),
      KeyModifiers::NONE,
      &mut params,
      &mut converter,
      &mut running,
      &mut debug_log,
    );
    assert_eq!(params.amplitude, 1.0);
  }

  #[test]
  fn test_scale_adjustment_clamps_at_minimum() {
    let mut params = ShaderParams::default();
    params.scale = 0.1;
    let mut converter = AsciiConverter::new(AsciiPalette::from(params.palette), true);
    let mut running = true;
    let mut debug_log = test_debug_log();

    invoke_key(
      KeyCode::Char('['),
      KeyModifiers::NONE,
      &mut params,
      &mut converter,
      &mut running,
      &mut debug_log,
    );

    assert_eq!(params.scale, 0.1);
  }

  #[test]
  fn test_scale_adjustment_clamps_at_maximum() {
    let mut params = ShaderParams::default();
    params.scale = 5.0;
    let mut converter = AsciiConverter::new(AsciiPalette::from(params.palette), true);
    let mut running = true;
    let mut debug_log = test_debug_log();

    invoke_key(
      KeyCode::Char(']'),
      KeyModifiers::NONE,
      &mut params,
      &mut converter,
      &mut running,
      &mut debug_log,
    );

    assert_eq!(params.scale, 5.0);
  }

  #[test]
  fn test_pattern_and_color_keys_cycle_enums() {
    let mut params = ShaderParams::default();
    params.pattern_type = PatternType::Plasma;
    params.color_mode = ColorMode::Rainbow;
    let mut converter = AsciiConverter::new(AsciiPalette::from(params.palette), true);
    let mut running = true;
    let mut debug_log = test_debug_log();

    invoke_key(
      KeyCode::Char('t'),
      KeyModifiers::NONE,
      &mut params,
      &mut converter,
      &mut running,
      &mut debug_log,
    );
    invoke_key(
      KeyCode::Char('c'),
      KeyModifiers::NONE,
      &mut params,
      &mut converter,
      &mut running,
      &mut debug_log,
    );

    assert_eq!(params.pattern_type, PatternType::Waves);
    assert_eq!(params.color_mode, ColorMode::Monochrome);
  }

  #[test]
  fn test_uppercase_pattern_and_color_keys_cycle_enums() {
    let mut params = ShaderParams::default();
    params.pattern_type = PatternType::Plasma;
    params.color_mode = ColorMode::Rainbow;
    let mut converter = AsciiConverter::new(AsciiPalette::from(params.palette), true);
    let mut running = true;
    let mut debug_log = test_debug_log();

    invoke_key(
      KeyCode::Char('T'),
      KeyModifiers::NONE,
      &mut params,
      &mut converter,
      &mut running,
      &mut debug_log,
    );
    invoke_key(
      KeyCode::Char('C'),
      KeyModifiers::NONE,
      &mut params,
      &mut converter,
      &mut running,
      &mut debug_log,
    );

    assert_eq!(params.pattern_type, PatternType::Waves);
    assert_eq!(params.color_mode, ColorMode::Monochrome);
  }

  #[test]
  fn test_palette_key_updates_params_and_converter_palette() {
    let mut params = ShaderParams::default();
    params.palette = PaletteType::Simple;
    let mut converter = AsciiConverter::new(AsciiPalette::from(params.palette), true);
    let baseline = converter_output(&converter);
    let mut running = true;
    let mut debug_log = test_debug_log();

    invoke_key(
      KeyCode::Char('p'),
      KeyModifiers::NONE,
      &mut params,
      &mut converter,
      &mut running,
      &mut debug_log,
    );

    let expected = AsciiConverter::new(AsciiPalette::from(params.palette), true);

    assert_eq!(params.palette, PaletteType::Standard);
    assert_eq!(converter_output(&converter), converter_output(&expected));
    assert_ne!(baseline, converter_output(&converter));
  }

  #[test]
  fn test_uppercase_palette_key_updates_params_and_converter_palette() {
    let mut params = ShaderParams::default();
    params.palette = PaletteType::Simple;
    let mut converter = AsciiConverter::new(AsciiPalette::from(params.palette), true);
    let baseline = converter_output(&converter);
    let mut running = true;
    let mut debug_log = test_debug_log();

    invoke_key(
      KeyCode::Char('P'),
      KeyModifiers::NONE,
      &mut params,
      &mut converter,
      &mut running,
      &mut debug_log,
    );

    let expected = AsciiConverter::new(AsciiPalette::from(params.palette), true);

    assert_eq!(params.palette, PaletteType::Standard);
    assert_eq!(converter_output(&converter), converter_output(&expected));
    assert_ne!(baseline, converter_output(&converter));
  }

  #[test]
  fn test_effect_key_skips_disabled_effect_slots_and_syncs_effect_time() {
    let mut params = ShaderParams::default();
    params.effect_type = 0;
    params.time = 42.5;
    let mut converter = AsciiConverter::new(AsciiPalette::from(params.palette), true);
    let mut running = true;
    let mut debug_log = test_debug_log();

    invoke_key(
      KeyCode::Char('n'),
      KeyModifiers::NONE,
      &mut params,
      &mut converter,
      &mut running,
      &mut debug_log,
    );

    assert_eq!(params.effect_type, 2);
    assert_eq!(params.effect_time, 42.5);

    params.effect_type = 6;
    params.time = 99.0;

    invoke_key(
      KeyCode::Char('n'),
      KeyModifiers::NONE,
      &mut params,
      &mut converter,
      &mut running,
      &mut debug_log,
    );

    assert_eq!(params.effect_type, 2);
    assert_eq!(params.effect_time, 99.0);
  }

  #[test]
  fn test_uppercase_effect_key_cycles_effects() {
    let mut params = ShaderParams::default();
    params.effect_type = 5;
    params.time = 12.0;
    let mut converter = AsciiConverter::new(AsciiPalette::from(params.palette), true);
    let mut running = true;
    let mut debug_log = test_debug_log();

    invoke_key(
      KeyCode::Char('N'),
      KeyModifiers::NONE,
      &mut params,
      &mut converter,
      &mut running,
      &mut debug_log,
    );

    assert_eq!(params.effect_type, 6);
    assert_eq!(params.effect_time, 12.0);
  }

  #[test]
  fn test_unknown_key_leaves_state_unchanged() {
    let mut params = ShaderParams::default();
    let original = params.clone();
    let mut converter = AsciiConverter::new(AsciiPalette::from(params.palette), true);
    let before = converter_output(&converter);
    let mut running = true;
    let mut debug_log = test_debug_log();

    invoke_key(
      KeyCode::Tab,
      KeyModifiers::NONE,
      &mut params,
      &mut converter,
      &mut running,
      &mut debug_log,
    );

    assert_eq!(params.frequency, original.frequency);
    assert_eq!(params.amplitude, original.amplitude);
    assert_eq!(params.scale, original.scale);
    assert_eq!(params.palette, original.palette);
    assert_eq!(converter_output(&converter), before);
    assert!(running);
  }
}
