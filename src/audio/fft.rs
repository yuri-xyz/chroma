use anyhow::Result;

pub struct AudioAnalyzer {
  bass_level: f32,
  mid_level: f32,
  treble_level: f32,
}

impl Default for AudioAnalyzer {
  fn default() -> Self {
    Self::new()
  }
}

impl AudioAnalyzer {
  pub fn new() -> Self {
    Self {
      bass_level: 0.0,
      mid_level: 0.0,
      treble_level: 0.0,
    }
  }

  #[cfg(feature = "audio")]
  pub fn analyze(&mut self, _samples: &[f32]) -> Result<()> {
    Ok(())
  }

  #[cfg(not(feature = "audio"))]
  pub fn analyze(&mut self, _samples: &[f32]) -> Result<()> {
    Err(anyhow::anyhow!("Audio feature not enabled"))
  }

  pub fn bass(&self) -> f32 {
    self.bass_level
  }

  pub fn mid(&self) -> f32 {
    self.mid_level
  }

  pub fn treble(&self) -> f32 {
    self.treble_level
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_analyzer_creation() {
    let analyzer = AudioAnalyzer::new();

    assert_eq!(analyzer.bass(), 0.0);
    assert_eq!(analyzer.mid(), 0.0);
    assert_eq!(analyzer.treble(), 0.0);
  }
}
