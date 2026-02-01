use anyhow::Result;

/// List all available pattern types
pub fn list_patterns() -> Result<()> {
  use chroma::params::PatternType;

  println!("Available Pattern Types:");
  println!();

  for pattern in PatternType::all() {
    println!(
      "  {:<15} (display: {})",
      pattern.full_name(),
      pattern.name()
    );
  }

  println!();
  println!("Use with: --pattern <PATTERN>");
  println!("In-app: Press 'T' to cycle through patterns");

  Ok(())
}

/// List all available color modes
pub fn list_color_modes() -> Result<()> {
  use chroma::params::ColorMode;

  println!("Available Color Modes:");
  println!();

  for mode in ColorMode::all() {
    println!("  {:<15} (display: {})", mode.full_name(), mode.name());
  }

  println!();
  println!("Use with: --color-mode <MODE>");
  println!("In-app: Press 'C' to cycle through color modes");

  Ok(())
}

/// List all available palette types
pub fn list_palettes() -> Result<()> {
  use chroma::params::PaletteType;

  println!("Available ASCII Palettes:");
  println!();

  for palette in PaletteType::all() {
    println!(
      "  {:<15} (display: {})",
      palette.full_name(),
      palette.name()
    );
  }

  println!();
  println!("Use with: --palette <PALETTE>");
  println!("In-app: Press 'P' to cycle through palettes");

  Ok(())
}
