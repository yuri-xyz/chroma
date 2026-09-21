// GPU-dependent checks that `frequency` and `scale` zoom patterns about the
// screen centre. Audio reactivity changes frequency every frame, so a pattern that
// scales raw coordinates instead looks like it zooms into the top-left corner.
//
// Like tests/render_test.rs, these skip gracefully without GPU access.

use chroma::{
  params::{PatternType, ShaderParams},
  shader::{ShaderPipeline, ShaderUniforms},
};

// Even, so the screen centre (0.5, 0.5) falls exactly on a pixel.
const FRAME_SIZE: u32 = 240;
const CENTER_PIXEL: i64 = (FRAME_SIZE / 2) as i64;
const ORIGIN_PIXEL: i64 = 0;
// The two renders differ by this zoom factor, so every second pixel of the
// zoomed frame maps onto a whole pixel of the reference frame.
const ZOOM_NUMERATOR: i64 = 3;
const ZOOM_DENOMINATOR: i64 = 2;
const LOW_FREQUENCY: f32 = 8.0;
const HIGH_FREQUENCY: f32 = 12.0;

async fn gpu_pipeline_or_skip() -> Option<ShaderPipeline> {
  let mut debug_sink = std::io::sink();

  match ShaderPipeline::new(FRAME_SIZE, FRAME_SIZE, None, &mut debug_sink).await {
    Ok(pipeline) => Some(pipeline),
    Err(e) => {
      eprintln!("Skipping GPU test: {}", e);
      None
    }
  }
}

fn render(pipeline: &ShaderPipeline, pattern: PatternType, frequency: f32, scale: f32) -> Vec<u8> {
  let mut params = ShaderParams {
    pattern_type: pattern,
    frequency,
    scale,
    time: 3.0,
    // Bass drives this, so the anchor has to hold while it is active.
    distort_amplitude: 0.6,
    // Per-pixel noise and the vignette are tied to the screen, not the pattern.
    noise_strength: 0.0,
    vignette: 0.0,
    ..ShaderParams::default()
  };
  params.set_resolution(FRAME_SIZE, FRAME_SIZE);

  pipeline
    .render(&ShaderUniforms::from_params(&params))
    .expect("Failed to render")
}

/// Mean absolute RGB difference between `zoomed` and `reference` resampled as a
/// zoom about `anchor_pixel`. Zero means `zoomed` is exactly that zoom.
fn zoom_mismatch(reference: &[u8], zoomed: &[u8], anchor_pixel: i64) -> f64 {
  let size = FRAME_SIZE as i64;
  let mut total = 0.0;
  let mut samples = 0u32;

  for y in 0..size {
    for x in 0..size {
      let (dx, dy) = (x - anchor_pixel, y - anchor_pixel);

      if dx % ZOOM_DENOMINATOR != 0 || dy % ZOOM_DENOMINATOR != 0 {
        continue;
      }

      let source_x = anchor_pixel + dx * ZOOM_NUMERATOR / ZOOM_DENOMINATOR;
      let source_y = anchor_pixel + dy * ZOOM_NUMERATOR / ZOOM_DENOMINATOR;

      if !(0..size).contains(&source_x) || !(0..size).contains(&source_y) {
        continue;
      }

      let zoomed_index = ((y * size + x) * 4) as usize;
      let reference_index = ((source_y * size + source_x) * 4) as usize;

      for channel in 0..3 {
        total += (zoomed[zoomed_index + channel] as f64
          - reference[reference_index + channel] as f64)
          .abs();
        samples += 1;
      }
    }
  }

  total / samples as f64
}

#[ignore = "requires GPU hardware and driver availability"]
#[pollster::test]
async fn test_frequency_zooms_tiled_patterns_exactly_about_screen_center() {
  let Some(pipeline) = gpu_pipeline_or_skip().await else {
    return;
  };

  // These depend on position only through `position * frequency`, so a frequency
  // change has to be an exact zoom, and the anchor decides where it appears to go.
  for pattern in [
    PatternType::Voronoi,
    PatternType::Truchet,
    PatternType::Hexagonal,
    PatternType::WarpedFbm,
    PatternType::Grid,
    PatternType::Kaleidoscope,
    PatternType::Rings,
    PatternType::Spiral,
  ] {
    for scale in [1.0, 2.0] {
      let reference = render(&pipeline, pattern, LOW_FREQUENCY, scale);
      let zoomed = render(&pipeline, pattern, HIGH_FREQUENCY, scale);
      let about_center = zoom_mismatch(&reference, &zoomed, CENTER_PIXEL);
      let about_origin = zoom_mismatch(&reference, &zoomed, ORIGIN_PIXEL);

      assert!(
        about_center < 1.0,
        "{} at scale {scale}: frequency is not a zoom about the centre (mismatch {about_center:.2})",
        pattern.name()
      );
      // A flat frame would match any anchor and prove nothing.
      assert!(
        about_origin > 5.0,
        "{} at scale {scale}: frame has too little detail to locate the anchor",
        pattern.name()
      );
    }
  }
}

#[ignore = "requires GPU hardware and driver availability"]
#[pollster::test]
async fn test_frequency_anchors_layered_patterns_near_screen_center() {
  let Some(pipeline) = gpu_pipeline_or_skip().await else {
    return;
  };

  // These mix frequency terms with terms that do not scale (angles, fixed warps),
  // so they never match a zoom exactly; the centre still has to be the better fit.
  for pattern in [
    PatternType::Plasma,
    PatternType::Waves,
    PatternType::Noise,
    PatternType::Geometric,
    PatternType::Glitch,
  ] {
    let reference = render(&pipeline, pattern, LOW_FREQUENCY, 1.0);
    let zoomed = render(&pipeline, pattern, HIGH_FREQUENCY, 1.0);
    let about_center = zoom_mismatch(&reference, &zoomed, CENTER_PIXEL);
    let about_origin = zoom_mismatch(&reference, &zoomed, ORIGIN_PIXEL);

    assert!(
      about_center * 3.0 < about_origin,
      "{}: frequency anchors nearer the top-left than the centre (centre {about_center:.2}, origin {about_origin:.2})",
      pattern.name()
    );
  }
}

#[ignore = "requires GPU hardware and driver availability"]
#[pollster::test]
async fn test_scale_zooms_every_pattern_about_screen_center() {
  let Some(pipeline) = gpu_pipeline_or_skip().await else {
    return;
  };

  for &pattern in PatternType::all() {
    // Its eye follows the top-left corner by design; covered by the test below.
    if pattern == PatternType::VortexCorner {
      continue;
    }

    let reference = render(&pipeline, pattern, LOW_FREQUENCY, 1.0);
    let zoomed = render(&pipeline, pattern, LOW_FREQUENCY, 1.5);
    let about_center = zoom_mismatch(&reference, &zoomed, CENTER_PIXEL);

    // Cell-based patterns flip a few pixels on cell edges through float rounding.
    assert!(
      about_center < 3.0,
      "{}: scale is not a zoom about the centre (mismatch {about_center:.2})",
      pattern.name()
    );
  }
}

#[ignore = "requires GPU hardware and driver availability"]
#[pollster::test]
async fn test_vortex_corner_keeps_its_eye_anchored_to_the_top_left() {
  let Some(pipeline) = gpu_pipeline_or_skip().await else {
    return;
  };

  let reference = render(&pipeline, PatternType::VortexCorner, LOW_FREQUENCY, 1.0);
  let zoomed = render(&pipeline, PatternType::VortexCorner, LOW_FREQUENCY, 1.5);
  let about_origin = zoom_mismatch(&reference, &zoomed, ORIGIN_PIXEL);
  let about_center = zoom_mismatch(&reference, &zoomed, CENTER_PIXEL);

  assert!(
    about_origin < 1.0,
    "scale moved the eye off its top-left anchor (mismatch {about_origin:.2})"
  );
  assert!(about_center > 5.0);

  // At scale 1 the eye is in the middle, so both variants draw the same frame.
  assert_eq!(
    reference,
    render(&pipeline, PatternType::Vortex, LOW_FREQUENCY, 1.0)
  );
}
