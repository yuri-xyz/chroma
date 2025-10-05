pub struct Framebuffer {
  pub width: u32,
  pub height: u32,
  pub pixels: Vec<u8>,
}

impl Framebuffer {
  pub fn new(width: u32, height: u32) -> Self {
    let size = (width * height * 4) as usize;

    Self {
      width,
      height,
      pixels: vec![0; size],
    }
  }

  pub fn resize(&mut self, width: u32, height: u32) {
    if self.width == width && self.height == height {
      return;
    }

    self.width = width;
    self.height = height;

    let size = (width * height * 4) as usize;

    self.pixels.resize(size, 0);
  }

  pub fn size(&self) -> u32 {
    self.width * self.height * 4
  }

  pub fn as_slice(&self) -> &[u8] {
    &self.pixels
  }

  pub fn as_mut_slice(&mut self) -> &mut [u8] {
    &mut self.pixels
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_framebuffer_creation() {
    let fb = Framebuffer::new(10, 10);

    assert_eq!(fb.width, 10);
    assert_eq!(fb.height, 10);
    assert_eq!(fb.pixels.len(), 400);
  }

  #[test]
  fn test_framebuffer_resize() {
    let mut fb = Framebuffer::new(10, 10);

    fb.resize(20, 20);

    assert_eq!(fb.width, 20);
    assert_eq!(fb.height, 20);
    assert_eq!(fb.pixels.len(), 1600);
  }
}
