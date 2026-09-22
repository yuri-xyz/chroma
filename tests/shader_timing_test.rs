use std::{fs, path::Path};

fn shader_sources() -> Vec<(String, String)> {
  let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
  let mut sources = Vec::new();

  for directory in ["shader_common", "shader_patterns"] {
    for entry in fs::read_dir(root.join(directory)).unwrap() {
      let path = entry.unwrap().path();

      if path
        .extension()
        .is_some_and(|extension| extension == "wgsl")
      {
        sources.push((
          path.display().to_string(),
          fs::read_to_string(&path).unwrap(),
        ));
      }
    }
  }

  assert!(!sources.is_empty(), "expected WGSL sources");
  sources
}

/// `time` already advances at `speed`, and audio changes `speed` every frame.
/// Multiplying the accumulated time by the current speed again makes the
/// animation phase jump on every speed change, more the longer chroma runs.
#[test]
fn test_shaders_do_not_rescale_accumulated_time_by_speed() {
  for (path, source) in shader_sources() {
    assert!(
      !source.contains("uniforms.speed"),
      "{path} reads uniforms.speed; use `time`, which already follows speed"
    );
  }
}

/// Effects and beats are stamped on the wall clock so they keep their real
/// duration when audio slows `time` down, or freezes it in silence.
#[test]
fn test_effect_and_beat_timers_use_wall_clock() {
  for (path, source) in shader_sources() {
    for stamp in ["uniforms.effect_time", "uniforms.beat_distortion_time"] {
      let elapsed_measurements = source.matches(&format!("- {stamp}")).count();
      let wall_clock_measurements = source
        .matches(&format!("uniforms.real_time - {stamp}"))
        .count();

      assert_eq!(
        elapsed_measurements, wall_clock_measurements,
        "{path} must measure {stamp} against uniforms.real_time"
      );
    }
  }
}
