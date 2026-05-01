use chroma::params::PatternType;

fn scaled_pattern_position(position: [f32; 2], scale: f32, pattern_type: PatternType) -> [f32; 2] {
  if pattern_type == PatternType::Sphere || pattern_type == PatternType::World {
    [
      (position[0] - 0.5) * scale + 0.5,
      (position[1] - 0.5) * scale + 0.5,
    ]
  } else {
    [position[0] * scale, position[1] * scale]
  }
}

#[test]
fn test_globe_patterns_scale_around_center() {
  let center = [0.5, 0.5];

  for pattern_type in [PatternType::Sphere, PatternType::World] {
    assert_eq!(scaled_pattern_position(center, 2.0, pattern_type), center);
    assert_eq!(
      scaled_pattern_position([0.25, 0.75], 2.0, pattern_type),
      [0.0, 1.0]
    );
  }
}

#[test]
fn test_tiled_patterns_keep_origin_based_scaling() {
  assert_eq!(
    scaled_pattern_position([0.5, 0.5], 2.0, PatternType::Plasma),
    [1.0, 1.0]
  );
}

#[test]
fn test_shader_centering_uses_current_globe_pattern_ids() {
  assert_eq!(PatternType::Sphere.to_u32(), 16);
  assert_eq!(PatternType::World.to_u32(), 22);

  let shader_main = include_str!("../src/shader_common/main.wgsl");

  assert!(shader_main.contains("fn pattern_position_for_scale"));
  assert!(shader_main.contains("pattern_type == 16u || pattern_type == 22u"));
  assert!(
    shader_main.contains("return (position - vec2<f32>(0.5, 0.5)) * scale + vec2<f32>(0.5, 0.5);")
  );
}
