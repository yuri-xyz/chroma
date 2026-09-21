// GPU-dependent render tests
//
// These tests create actual GPU shader pipelines and render frames.
// In CI environments without GPU access, they gracefully skip with a message.
// This allows the test suite to pass in headless CI while still providing
// valuable integration tests when run locally with GPU access.

use chroma::{
  ascii::{AsciiConverter, AsciiPalette},
  params::ShaderParams,
  shader::{ShaderPipeline, ShaderUniforms},
};

#[ignore = "requires GPU hardware and driver availability"]
#[pollster::test]
async fn test_shader_produces_non_zero_output() {
  let width = 10;
  let height = 10;
  let mut params = ShaderParams::default();

  params.set_resolution(width, height);
  params.time = 1.0;

  let mut debug_sink = std::io::sink();
  let pipeline = match ShaderPipeline::new(width, height, None, &mut debug_sink).await {
    Ok(p) => p,
    Err(e) => {
      eprintln!("Skipping GPU test: {}", e);
      eprintln!("This is expected in CI environments without GPU access");
      return;
    }
  };

  let uniforms = ShaderUniforms::from_params(&params);
  let pixel_data = pipeline.render(&uniforms).expect("Failed to render");

  assert_eq!(pixel_data.len(), (width * height * 4) as usize);

  let mut has_non_zero = false;

  for &byte in &pixel_data {
    if byte != 0 {
      has_non_zero = true;
      break;
    }
  }

  assert!(has_non_zero, "Shader produced all zero pixels");
}

#[ignore = "requires GPU hardware and driver availability"]
#[pollster::test]
async fn test_shader_produces_varied_colors() {
  let width = 20;
  let height = 20;

  let mut params = ShaderParams::default();
  params.set_resolution(width, height);
  params.time = 1.0;

  let mut debug_sink = std::io::sink();
  let pipeline = match ShaderPipeline::new(width, height, None, &mut debug_sink).await {
    Ok(p) => p,
    Err(e) => {
      eprintln!("Skipping GPU test: {}", e);
      eprintln!("This is expected in CI environments without GPU access");
      return;
    }
  };

  let uniforms = ShaderUniforms::from_params(&params);

  let pixel_data = pipeline.render(&uniforms).expect("Failed to render");

  let mut min_r = 255u8;
  let mut max_r = 0u8;
  let mut min_g = 255u8;
  let mut max_g = 0u8;
  let mut min_b = 255u8;
  let mut max_b = 0u8;

  for i in 0..(width * height) as usize {
    let idx = i * 4;
    min_r = min_r.min(pixel_data[idx]);
    max_r = max_r.max(pixel_data[idx]);
    min_g = min_g.min(pixel_data[idx + 1]);
    max_g = max_g.max(pixel_data[idx + 1]);
    min_b = min_b.min(pixel_data[idx + 2]);
    max_b = max_b.max(pixel_data[idx + 2]);
  }

  println!("R range: {} to {}", min_r, max_r);
  println!("G range: {} to {}", min_g, max_g);
  println!("B range: {} to {}", min_b, max_b);

  assert!(max_r > min_r + 10, "Red channel has insufficient variation");
  assert!(
    max_g > min_g + 10,
    "Green channel has insufficient variation"
  );
  assert!(
    max_b > min_b + 10,
    "Blue channel has insufficient variation"
  );
}

#[ignore = "requires GPU hardware and driver availability"]
#[pollster::test]
async fn test_ascii_conversion_produces_varied_characters() {
  let width = 20;
  let height = 20;
  let mut params = ShaderParams::default();

  params.set_resolution(width, height);
  params.time = 1.0;

  let mut debug_sink = std::io::sink();
  let pipeline = match ShaderPipeline::new(width, height, None, &mut debug_sink).await {
    Ok(p) => p,
    Err(e) => {
      eprintln!("Skipping GPU test: {}", e);
      eprintln!("This is expected in CI environments without GPU access");

      return;
    }
  };

  let uniforms = ShaderUniforms::from_params(&params);
  let pixel_data = pipeline.render(&uniforms).expect("Failed to render");
  let converter = AsciiConverter::new(AsciiPalette::standard(), true);
  let ascii_frame = converter.convert_frame(&pixel_data, width, height);

  assert_eq!(ascii_frame.len(), height as usize);
  assert_eq!(ascii_frame[0].len(), width as usize);

  let mut unique_chars = std::collections::HashSet::new();

  for row in &ascii_frame {
    for (ch, _) in row {
      unique_chars.insert(*ch);
    }
  }

  println!("Unique characters: {:?}", unique_chars);

  assert!(
    unique_chars.len() > 1,
    "ASCII conversion produced only one character type"
  );
}

#[test]
fn test_ascii_palette_has_range() {
  let palette = AsciiPalette::standard();
  let dark_char = palette.get_character(0.0);
  let mid_char = palette.get_character(0.5);
  let bright_char = palette.get_character(1.0);

  println!(
    "Dark: '{}', Mid: '{}', Bright: '{}'",
    dark_char, mid_char, bright_char
  );

  assert_ne!(
    dark_char, bright_char,
    "Palette doesn't vary between dark and bright"
  );
}

async fn gpu_pipeline_or_skip(
  width: u32,
  height: u32,
  custom_shader: Option<String>,
) -> Option<ShaderPipeline> {
  let mut debug_sink = std::io::sink();

  match ShaderPipeline::new(width, height, custom_shader, &mut debug_sink).await {
    Ok(pipeline) => Some(pipeline),
    Err(e) => {
      eprintln!("Skipping GPU test: {}", e);
      None
    }
  }
}

#[ignore = "requires GPU hardware and driver availability"]
#[pollster::test]
async fn test_invalid_custom_shader_returns_error_instead_of_panicking() {
  if gpu_pipeline_or_skip(4, 4, None).await.is_none() {
    return;
  }

  let mut debug_sink = std::io::sink();
  let result =
    ShaderPipeline::new(4, 4, Some("this is not wgsl".to_string()), &mut debug_sink).await;

  assert!(result.is_err());
}

#[ignore = "requires GPU hardware and driver availability"]
#[pollster::test]
async fn test_pipeline_resize_reuses_device_and_rejects_zero_dimensions() {
  let Some(mut pipeline) = gpu_pipeline_or_skip(4, 4, None).await else {
    return;
  };

  assert!(pipeline.resize(0, 4).is_err());
  assert!(pipeline.resize(4, 0).is_err());

  pipeline.resize(12, 3).expect("resize should succeed");

  let mut params = ShaderParams::default();
  params.set_resolution(12, 3);
  let pixel_data = pipeline
    .render(&ShaderUniforms::from_params(&params))
    .expect("Failed to render after resize");

  assert_eq!(pixel_data.len(), 12 * 3 * 4);
}

#[ignore = "requires GPU hardware and driver availability"]
#[pollster::test]
async fn test_color_adjustments_preserve_magenta_hues() {
  let shader = format!(
    "{}\n{}\n{}",
    include_str!("../src/shader_common/uniforms.wgsl"),
    include_str!("../src/shader_common/color_utils.wgsl"),
    r#"
@compute @workgroup_size(1, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    output_buffer[0] = vec4<f32>(apply_color_adjustments(vec3<f32>(1.0, 0.2, 0.6)), 1.0);
}
"#
  );
  let Some(pipeline) = gpu_pipeline_or_skip(1, 1, Some(shader)).await else {
    return;
  };
  let params = ShaderParams {
    brightness: 1.0,
    contrast: 1.0,
    saturation: 1.0,
    gamma: 1.0,
    hue: 0.0,
    ..ShaderParams::default()
  };

  let pixel = pipeline
    .render(&ShaderUniforms::from_params(&params))
    .expect("Failed to render");

  // Neutral adjustments must leave the color unchanged (255, 51, 153).
  assert!(pixel[0] >= 253, "red was {}", pixel[0]);
  assert!((49..=53).contains(&pixel[1]), "green was {}", pixel[1]);
  assert!((151..=155).contains(&pixel[2]), "blue was {}", pixel[2]);
}
