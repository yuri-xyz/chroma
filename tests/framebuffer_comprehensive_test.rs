use chroma::render::Framebuffer;

#[test]
fn test_framebuffer_size_calculation() {
  let fb = Framebuffer::new(10, 20);

  assert_eq!(fb.size(), 10 * 20 * 4);
}

#[test]
fn test_framebuffer_size_single_pixel() {
  let fb = Framebuffer::new(1, 1);

  assert_eq!(fb.size(), 4);
}

#[test]
fn test_framebuffer_size_large() {
  let fb = Framebuffer::new(1920, 1080);

  assert_eq!(fb.size(), 1920 * 1080 * 4);
}

#[test]
fn test_framebuffer_as_slice() {
  let fb = Framebuffer::new(2, 2);
  let slice = fb.as_slice();

  assert_eq!(slice.len(), 16);

  for &byte in slice {
    assert_eq!(byte, 0);
  }
}

#[test]
fn test_framebuffer_as_mut_slice() {
  let mut fb = Framebuffer::new(1, 1);
  let slice = fb.as_mut_slice();

  assert_eq!(slice.len(), 4);

  slice[0] = 255;
  slice[1] = 128;
  slice[2] = 64;
  slice[3] = 32;

  assert_eq!(fb.pixels[0], 255);
  assert_eq!(fb.pixels[1], 128);
  assert_eq!(fb.pixels[2], 64);
  assert_eq!(fb.pixels[3], 32);
}

#[test]
fn test_framebuffer_resize_larger() {
  let mut fb = Framebuffer::new(5, 5);

  fb.resize(10, 10);

  assert_eq!(fb.width, 10);
  assert_eq!(fb.height, 10);
  assert_eq!(fb.pixels.len(), 400);
}

#[test]
fn test_framebuffer_resize_smaller() {
  let mut fb = Framebuffer::new(10, 10);

  fb.resize(5, 5);

  assert_eq!(fb.width, 5);
  assert_eq!(fb.height, 5);
  assert_eq!(fb.pixels.len(), 100);
}

#[test]
fn test_framebuffer_resize_same_size() {
  let mut fb = Framebuffer::new(10, 10);
  let original_len = fb.pixels.len();
  let original_capacity = fb.pixels.capacity();

  fb.resize(10, 10);

  assert_eq!(fb.width, 10);
  assert_eq!(fb.height, 10);
  assert_eq!(fb.pixels.len(), original_len);
  assert_eq!(fb.pixels.capacity(), original_capacity);
}

#[test]
fn test_framebuffer_resize_preserves_existing_data() {
  let mut fb = Framebuffer::new(2, 2);

  fb.pixels[0] = 100;
  fb.pixels[1] = 200;

  fb.resize(3, 3);

  assert_eq!(fb.pixels[0], 100);
  assert_eq!(fb.pixels[1], 200);
}

#[test]
fn test_framebuffer_resize_aspect_ratio_change() {
  let mut fb = Framebuffer::new(16, 9);

  fb.resize(9, 16);

  assert_eq!(fb.width, 9);
  assert_eq!(fb.height, 16);
  assert_eq!(fb.pixels.len(), 9 * 16 * 4);
}

#[test]
fn test_framebuffer_pixels_initialized_to_zero() {
  let fb = Framebuffer::new(5, 5);

  for &pixel in &fb.pixels {
    assert_eq!(pixel, 0);
  }
}

#[test]
fn test_framebuffer_width_height_match_constructor() {
  let fb = Framebuffer::new(123, 456);

  assert_eq!(fb.width, 123);
  assert_eq!(fb.height, 456);
}

#[test]
fn test_framebuffer_rgba_layout() {
  let fb = Framebuffer::new(1, 1);

  assert_eq!(fb.pixels.len(), 4, "Should have RGBA components");
}
