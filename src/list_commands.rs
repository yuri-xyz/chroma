use anyhow::Result;

fn render_named_list<'a>(
  title: &str,
  entries: impl IntoIterator<Item = (&'a str, &'a str)>,
  usage: &str,
  shortcut: &str,
) -> String {
  let mut output = format!("{title}\n\n");

  for (full_name, display_name) in entries {
    output.push_str(&format!("  {full_name:<15} (display: {display_name})\n"));
  }

  output.push_str(&format!("\nUse with: {usage}\n"));
  output.push_str(&format!("In-app: {shortcut}\n"));

  output
}

fn render_pattern_list() -> String {
  use chroma::params::PatternType;

  render_named_list(
    "Available Pattern Types:",
    PatternType::all()
      .iter()
      .map(|pattern| (pattern.full_name(), pattern.name())),
    "--pattern <PATTERN>",
    "Press 'T' to cycle through patterns",
  )
}

/// List all available pattern types
pub fn list_patterns() -> Result<()> {
  print!("{}", render_pattern_list());

  Ok(())
}

fn render_color_mode_list() -> String {
  use chroma::params::ColorMode;

  render_named_list(
    "Available Color Modes:",
    ColorMode::all()
      .iter()
      .map(|mode| (mode.full_name(), mode.name())),
    "--color-mode <MODE>",
    "Press 'C' to cycle through color modes",
  )
}

/// List all available color modes
pub fn list_color_modes() -> Result<()> {
  print!("{}", render_color_mode_list());

  Ok(())
}

fn render_palette_list() -> String {
  use chroma::params::PaletteType;

  render_named_list(
    "Available ASCII Palettes:",
    PaletteType::all()
      .iter()
      .map(|palette| (palette.full_name(), palette.name())),
    "--palette <PALETTE>",
    "Press 'P' to cycle through palettes",
  )
}

/// List all available palette types
pub fn list_palettes() -> Result<()> {
  print!("{}", render_palette_list());

  Ok(())
}

#[cfg(test)]
mod tests {
  use chroma::params::{ColorMode, PaletteType, PatternType};

  use super::*;

  fn entry_lines(output: &str) -> Vec<&str> {
    output
      .lines()
      .filter(|line| line.starts_with("  "))
      .collect()
  }

  #[test]
  fn test_render_pattern_list_contains_headings_and_examples() {
    let output = render_pattern_list();
    let lines = entry_lines(&output);
    let first_pattern = PatternType::all()[0];
    let fluid = PatternType::Fluid;

    assert!(output.starts_with("Available Pattern Types:\n\n"));
    assert_eq!(lines.len(), PatternType::all().len());
    assert_eq!(
      lines[0],
      format!(
        "  {:<15} (display: {})",
        first_pattern.full_name(),
        first_pattern.name()
      )
    );
    assert!(lines
      .iter()
      .any(|line| { *line == format!("  {:<15} (display: {})", fluid.full_name(), fluid.name()) }));
    assert!(output
      .ends_with("\nUse with: --pattern <PATTERN>\nIn-app: Press 'T' to cycle through patterns\n"));
  }

  #[test]
  fn test_render_color_mode_list_contains_all_guidance() {
    let output = render_color_mode_list();
    let lines = entry_lines(&output);
    let first_mode = ColorMode::all()[0];
    let chromatic = ColorMode::Chromatic;

    assert!(output.starts_with("Available Color Modes:\n\n"));
    assert_eq!(lines.len(), ColorMode::all().len());
    assert_eq!(
      lines[0],
      format!(
        "  {:<15} (display: {})",
        first_mode.full_name(),
        first_mode.name()
      )
    );
    assert!(lines.iter().any(|line| {
      *line
        == format!(
          "  {:<15} (display: {})",
          chromatic.full_name(),
          chromatic.name()
        )
    }));
    assert!(output.ends_with(
      "\nUse with: --color-mode <MODE>\nIn-app: Press 'C' to cycle through color modes\n"
    ));
  }

  #[test]
  fn test_render_palette_list_contains_all_guidance() {
    let output = render_palette_list();
    let lines = entry_lines(&output);
    let first_palette = PaletteType::all()[0];
    let boxdraw = PaletteType::BoxDraw;

    assert!(output.starts_with("Available ASCII Palettes:\n\n"));
    assert_eq!(lines.len(), PaletteType::all().len());
    assert_eq!(
      lines[0],
      format!(
        "  {:<15} (display: {})",
        first_palette.full_name(),
        first_palette.name()
      )
    );
    assert!(lines.iter().any(|line| {
      *line
        == format!(
          "  {:<15} (display: {})",
          boxdraw.full_name(),
          boxdraw.name()
        )
    }));
    assert!(output
      .ends_with("\nUse with: --palette <PALETTE>\nIn-app: Press 'P' to cycle through palettes\n"));
  }
}
