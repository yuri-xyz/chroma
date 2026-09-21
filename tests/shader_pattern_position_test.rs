/// Mirrors `pattern_position_for_scale` in `src/shader_common/main.wgsl`.
fn scaled_pattern_position(position: [f32; 2], scale: f32) -> [f32; 2] {
  [
    (position[0] - 0.5) * scale + 0.5,
    (position[1] - 0.5) * scale + 0.5,
  ]
}

#[test]
fn test_patterns_scale_around_screen_center() {
  let center = [0.5, 0.5];

  for scale in [0.5, 2.0, 3.0] {
    assert_eq!(scaled_pattern_position(center, scale), center);
  }

  assert_eq!(scaled_pattern_position([0.25, 0.75], 2.0), [0.0, 1.0]);
}

#[test]
fn test_shader_scales_every_pattern_around_screen_center() {
  let shader_main = include_str!("../src/shader_common/main.wgsl");

  assert!(shader_main.contains("const PATTERN_CENTER: vec2<f32> = vec2<f32>(0.5, 0.5);"));
  assert!(shader_main.contains("return (position - PATTERN_CENTER) * scale + PATTERN_CENTER;"));
  // Origin-based scaling put hard-coded 0.5 pattern centres off-screen-centre.
  assert!(!shader_main.contains("return position * scale;"));
}

/// Patterns that tile the plane have to apply frequency to centre-relative
/// coordinates. Audio reactivity changes frequency every frame, and scaling raw
/// `uv` makes that look like a zoom into the top-left corner.
#[test]
fn test_tiled_patterns_apply_frequency_to_centered_coordinates() {
  let tiled_patterns = [
    ("plasma", include_str!("../src/shader_patterns/plasma.wgsl")),
    ("waves", include_str!("../src/shader_patterns/waves.wgsl")),
    ("noise", include_str!("../src/shader_patterns/noise.wgsl")),
    (
      "geometric",
      include_str!("../src/shader_patterns/geometric.wgsl"),
    ),
    (
      "voronoi",
      include_str!("../src/shader_patterns/voronoi.wgsl"),
    ),
    (
      "truchet",
      include_str!("../src/shader_patterns/truchet.wgsl"),
    ),
    (
      "hexagonal",
      include_str!("../src/shader_patterns/hexagonal.wgsl"),
    ),
    ("glitch", include_str!("../src/shader_patterns/glitch.wgsl")),
    ("grid", include_str!("../src/shader_patterns/grid.wgsl")),
    (
      "diamonds",
      include_str!("../src/shader_patterns/diamonds.wgsl"),
    ),
    (
      "warped_fbm",
      include_str!("../src/shader_patterns/warped_fbm.wgsl"),
    ),
    ("fluid", include_str!("../src/shader_patterns/fluid.wgsl")),
  ];

  for (name, source) in tiled_patterns {
    assert!(
      source.contains("centered_uv(uv)"),
      "{name} scales raw uv by frequency"
    );
  }
}

#[test]
fn test_pyramid_rotation_uses_continuous_drift() {
  let pyramid_shader = include_str!("../src/shader_patterns/pyramid.wgsl");

  assert!(pyramid_shader.contains("fn pyramid_drift"));
  assert!(pyramid_shader.contains("fn pyramid_rotation_angles"));
  assert!(!pyramid_shader.contains("floor(time"));
}

#[test]
fn test_pyramid_rotates_around_model_center() {
  let pyramid_shader = include_str!("../src/shader_patterns/pyramid.wgsl");

  assert!(pyramid_shader.contains("fn pyramid_center_for_rotation"));
  assert!(pyramid_shader.contains("let rotation_center = pyramid_center_for_rotation();"));
  assert!(pyramid_shader.contains("vec3<f32>(0.0, 0.58, 0.0) - rotation_center"));
  assert!(pyramid_shader.contains("vec3<f32>(-0.52, -0.36, -0.52) - rotation_center"));
  assert!(pyramid_shader.contains("vec3<f32>(0.52, -0.36, 0.52) - rotation_center"));
}

#[test]
fn test_infinity_pattern_uses_centered_continuous_3d_motion() {
  let infinity_shader = include_str!("../src/shader_patterns/infinity.wgsl");

  assert!(infinity_shader.contains("fn infinity_drift"));
  assert!(infinity_shader.contains("fn infinity_size_variation"));
  assert!(infinity_shader.contains("fn infinity_beat_glow"));
  assert!(infinity_shader.contains("fn infinity_motion_time"));
  assert!(infinity_shader.contains("fn infinity_curve"));
  assert!(infinity_shader.contains("fn infinity_rotate"));
  assert!(infinity_shader.contains("return time;"));
  assert!(!infinity_shader.contains("return time * uniforms.speed"));
  assert!(infinity_shader.contains("uniforms.gamma"));
  assert!(infinity_shader.contains("uniforms.vignette_softness"));
  assert!(infinity_shader.contains("uniforms.glyph_sharpness"));
  assert!(!infinity_shader.contains("uniforms.color_shift * 17.13"));
  assert!(!infinity_shader.contains("uniforms.amplitude * 11.19"));
  assert!(!infinity_shader.contains("let beat_acceleration = 1.0 + beat_activity *"));
  assert!(infinity_shader.contains("uniforms.beat_distortion_strength"));
  assert!(infinity_shader.contains("uniforms.beat_zoom_strength"));
  assert!(infinity_shader.contains("let motion_time = infinity_motion_time(time);"));
  assert!(infinity_shader.contains("let velocity = vec2<f32>"));
  assert!(infinity_shader.contains("let shell_radius = tube_radius *"));
  assert!(infinity_shader.contains("for (var i = 0u; i < 72u; i = i + 1u)"));
  assert!(!infinity_shader.contains("floor(time"));
}

#[test]
fn test_world_pattern_has_optional_seeded_rings() {
  let world_shader = include_str!("../src/shader_patterns/world.wgsl");

  assert!(world_shader.contains("fn world_ring_seed"));
  assert!(world_shader.contains("fn world_ring"));
  assert!(world_shader.contains("if seed.x < 0.5"));
  assert!(world_shader.contains("uniforms.gamma"));
  assert!(world_shader.contains("uniforms.vignette_softness"));
  assert!(world_shader.contains("uniforms.glyph_sharpness"));
  assert!(!world_shader.contains("uniforms.color_shift * 13.17"));
  assert!(!world_shader.contains("uniforms.frequency * 2.31"));
  assert!(!world_shader.contains("uniforms.amplitude * 5.73"));
  assert!(world_shader.contains("let spin_direction = select(-1.0, 1.0, seed.y > 0.5);"));
  assert!(world_shader.contains("if radius > globe_radius"));
  assert!(world_shader.contains("if ring_result.x > 0.01"));
}
