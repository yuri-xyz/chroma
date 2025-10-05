pub struct AsciiPalette {
  characters: Vec<char>,
}

impl Default for AsciiPalette {
  fn default() -> Self {
    Self::standard()
  }
}

impl AsciiPalette {
  pub fn standard() -> Self {
    Self {
      characters: vec![' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'],
    }
  }

  pub fn extended() -> Self {
    Self {
      characters: vec![
        ' ', '.', '\'', '`', '^', '"', ',', ':', ';', 'I', 'l', '!', 'i', '>', '<', '~', '+', '_',
        '-', '?', ']', '[', '}', '{', '1', ')', '(', '|', '\\', '/', 't', 'f', 'j', 'r', 'x', 'n',
        'u', 'v', 'c', 'z', 'X', 'Y', 'U', 'J', 'C', 'L', 'Q', '0', 'O', 'Z', 'm', 'w', 'q', 'p',
        'd', 'b', 'k', 'h', 'a', 'o', '*', '#', 'M', 'W', '&', '8', '%', 'B', '@', '$',
      ],
    }
  }

  pub fn simple() -> Self {
    Self {
      characters: vec![' ', '.', 'o', 'O', '@'],
    }
  }

  pub fn blocks() -> Self {
    Self {
      characters: vec![' ', '░', '▒', '▓', '█'],
    }
  }

  pub fn smooth() -> Self {
    Self {
      characters: vec![' ', '·', '∘', '○', '◌', '◍', '◎', '◉', '●', '█'],
    }
  }

  pub fn braille() -> Self {
    Self {
      characters: vec![' ', '⠁', '⠃', '⠇', '⠏', '⠟', '⠿', '⡿', '⣿'],
    }
  }

  pub fn geometric() -> Self {
    Self {
      characters: vec![' ', '▪', '▫', '▬', '▭', '▮', '▯', '■', '█'],
    }
  }

  pub fn circles() -> Self {
    Self {
      characters: vec![' ', '·', '∘', '○', '◌', '◍', '◎', '◉', '●', '█'],
    }
  }

  pub fn mixed() -> Self {
    Self {
      characters: vec![' ', '·', '∘', '░', '▒', '▓', '●', '◉', '■', '█'],
    }
  }

  pub fn dots() -> Self {
    Self {
      characters: vec![' ', '⡀', '⡄', '⡆', '⡇', '⣇', '⣧', '⣷', '⣿'],
    }
  }

  pub fn shades() -> Self {
    Self {
      characters: vec![' ', '░', '░', '▒', '▒', '▓', '▓', '█', '█'],
    }
  }

  pub fn lines() -> Self {
    Self {
      characters: vec![' ', '╌', '╍', '┄', '┅', '┈', '┉', '━', '█'],
    }
  }

  pub fn triangles() -> Self {
    Self {
      characters: vec![' ', '▵', '▴', '▿', '▾', '◂', '◃', '▸', '▹'],
    }
  }

  pub fn arrows() -> Self {
    Self {
      characters: vec![' ', '›', '»', '⟩', '→', '⇒', '⟹', '⟾', '▶'],
    }
  }

  pub fn powerline() -> Self {
    Self {
      characters: vec![
        ' ', '\u{e0b0}', '\u{e0b1}', '\u{e0b2}', '\u{e0b3}', '\u{e0b4}', '\u{e0b5}', '\u{e0b6}',
        '█',
      ],
    }
  }

  pub fn boxdraw() -> Self {
    Self {
      characters: vec![' ', '─', '━', '│', '┃', '┼', '╋', '╬', '█'],
    }
  }

  pub fn get_character(&self, brightness: f32) -> char {
    let normalized_brightness = brightness.clamp(0.0, 1.0);
    let index = (normalized_brightness * (self.characters.len() - 1) as f32).round() as usize;

    self.characters[index.min(self.characters.len() - 1)]
  }

  pub fn len(&self) -> usize {
    self.characters.len()
  }

  pub fn is_empty(&self) -> bool {
    self.characters.is_empty()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_standard_palette() {
    let palette = AsciiPalette::standard();

    assert_eq!(palette.get_character(0.0), ' ');
    assert_eq!(palette.get_character(1.0), '@');
  }

  #[test]
  fn test_brightness_mapping() {
    let palette = AsciiPalette::standard();

    assert_eq!(palette.get_character(0.5), '+');
  }

  #[test]
  fn test_blocks_palette() {
    let palette = AsciiPalette::blocks();

    assert_eq!(palette.get_character(0.0), ' ');
    assert_eq!(palette.get_character(1.0), '█');
  }

  #[test]
  fn test_smooth_palette() {
    let palette = AsciiPalette::smooth();

    assert_eq!(palette.get_character(0.0), ' ');
    assert!(palette.characters.len() > 5);
  }

  #[test]
  fn test_braille_palette() {
    let palette = AsciiPalette::braille();

    assert_eq!(palette.get_character(0.0), ' ');
    assert_eq!(palette.get_character(1.0), '⣿');
  }
}
