// Audio-reactive update logic

#[cfg(feature = "audio")]
use super::DebugLog;
#[cfg(feature = "audio")]
use chroma::audio::{AudioAnalyzer, AudioCapture, AudioFeatures};
#[cfg(feature = "audio")]
use chroma::constants::{AUDIO_DECAY_RATE, AUDIO_SILENCE_THRESHOLD, AUDIO_SPEED_DECAY_RATE};
#[cfg(feature = "audio")]
use std::sync::atomic::{AtomicUsize, Ordering};

#[cfg(feature = "audio")]
const SILENT_AMPLITUDE_BASELINE: f32 = 0.4;
#[cfg(feature = "audio")]
const SILENT_FREQUENCY_BASELINE: f32 = 6.0;
#[cfg(feature = "audio")]
const SILENT_BRIGHTNESS_BASELINE: f32 = 0.6;
#[cfg(feature = "audio")]
const SILENT_CONTRAST_BASELINE: f32 = 0.8;
#[cfg(feature = "audio")]
const REGULAR_BEAT_THRESHOLD_BASE: f32 = 0.18;
#[cfg(feature = "audio")]
const DROP_BEAT_DISTORTION_STRENGTH: f32 = 1.2;
#[cfg(feature = "audio")]
const DROP_BEAT_ZOOM_STRENGTH: f32 = 1.0;
#[cfg(feature = "audio")]
const REGULAR_BEAT_DISTORTION_STRENGTH: f32 = 0.85;
#[cfg(feature = "audio")]
const REGULAR_BEAT_ZOOM_STRENGTH: f32 = 0.7;
#[cfg(feature = "audio")]
static EMPTY_SAMPLE_BATCH_COUNT: AtomicUsize = AtomicUsize::new(0);
#[cfg(feature = "audio")]
static FEATURE_LOG_COUNT: AtomicUsize = AtomicUsize::new(0);

#[cfg(feature = "audio")]
fn blend_towards(current: f32, target: f32, retain: f32) -> f32 {
  current * retain + target * (1.0 - retain)
}

#[cfg(feature = "audio")]
fn weighted_energy(features: &AudioFeatures) -> f32 {
  (features.bass * 0.1 + features.mid * 0.3 + features.treble * 0.6).max(0.05)
}

#[cfg(feature = "audio")]
fn regular_beat_threshold(params: &chroma::params::ShaderParams) -> f32 {
  REGULAR_BEAT_THRESHOLD_BASE / params.beat_sensitivity
}

#[cfg(feature = "audio")]
fn trigger_beat_visuals(
  params: &mut chroma::params::ShaderParams,
  distortion_strength: f32,
  zoom_strength: f32,
) {
  params.beat_distortion_time = params.time;
  params.beat_distortion_strength = distortion_strength;
  params.beat_zoom_strength = zoom_strength;
}
/// Update shader parameters based on audio input
#[cfg(feature = "audio")]
pub fn update_audio_reactive(
  params: &mut chroma::params::ShaderParams,
  audio_capture: &Option<AudioCapture>,
  audio_analyzer: &mut Option<AudioAnalyzer>,
  delta_time: f32,
  debug_log: &mut DebugLog,
) -> AudioFeatures {
  if !params.audio_enabled {
    let _ = debug_logln!(debug_log, "AUDIO: audio reactivity disabled");
    return AudioFeatures::default();
  }

  let has_capture = audio_capture.is_some();
  let has_analyzer = audio_analyzer.is_some();

  if let (Some(capture), Some(analyzer)) = (audio_capture, audio_analyzer) {
    let samples = capture.drain_samples();

    if samples.is_empty() {
      let empty_count = EMPTY_SAMPLE_BATCH_COUNT.fetch_add(1, Ordering::Relaxed) + 1;
      if empty_count <= 5 || empty_count % 120 == 0 {
        let _ = debug_logln!(
          debug_log,
          "AUDIO: no samples drained yet (empty_batch_count={})",
          empty_count
        );
      }
      return AudioFeatures::default();
    }

    EMPTY_SAMPLE_BATCH_COUNT.store(0, Ordering::Relaxed);

    let features = analyzer.analyze(&samples, delta_time);
    let is_silent = features.overall < AUDIO_SILENCE_THRESHOLD;
    let feature_log_count = FEATURE_LOG_COUNT.fetch_add(1, Ordering::Relaxed) + 1;

    if feature_log_count <= 5 || feature_log_count % 60 == 0 {
      let peak_sample = samples
        .iter()
        .fold(0.0_f32, |max_value, sample| max_value.max(sample.abs()));
      let _ = debug_logln!(
        debug_log,
        "AUDIO: samples={} peak={:.5} features=bass:{:.4} mid:{:.4} treble:{:.4} overall:{:.4} beat:{:.4} drop:{}",
        samples.len(),
        peak_sample,
        features.bass,
        features.mid,
        features.treble,
        features.overall,
        features.beat_strength,
        features.is_drop
      );
    }

    if is_silent {
      apply_silence_decay(params, &features, debug_log);
    } else {
      apply_audio_reactivity(params, &features, debug_log);
    }

    return features;
  }

  let _ = debug_logln!(
    debug_log,
    "AUDIO: capture/analyzer unavailable (capture={}, analyzer={})",
    has_capture,
    has_analyzer
  );

  AudioFeatures::default()
}

/// Apply decay to parameters when audio is silent
#[cfg(feature = "audio")]
fn apply_silence_decay(
  params: &mut chroma::params::ShaderParams,
  features: &chroma::audio::AudioFeatures,
  debug_log: &mut DebugLog,
) {
  params.amplitude = blend_towards(
    params.amplitude,
    SILENT_AMPLITUDE_BASELINE,
    AUDIO_DECAY_RATE,
  );
  params.distort_amplitude *= AUDIO_DECAY_RATE;
  params.frequency = blend_towards(
    params.frequency,
    SILENT_FREQUENCY_BASELINE,
    AUDIO_DECAY_RATE,
  );
  params.speed *= AUDIO_SPEED_DECAY_RATE;
  params.brightness = blend_towards(
    params.brightness,
    SILENT_BRIGHTNESS_BASELINE,
    AUDIO_DECAY_RATE,
  );
  params.noise_strength *= 0.85;
  params.contrast = blend_towards(params.contrast, SILENT_CONTRAST_BASELINE, AUDIO_DECAY_RATE);

  let _ = debug_logln!(
    debug_log,
    "AUDIO: Silence (vol={:.4}) - slowing to stop (speed={:.3})",
    features.overall,
    params.speed
  );
}

/// Apply audio features to shader parameters
#[cfg(feature = "audio")]
fn apply_audio_reactivity(
  params: &mut chroma::params::ShaderParams,
  features: &chroma::audio::AudioFeatures,
  debug_log: &mut DebugLog,
) {
  // Emphasize treble for melody visibility
  let energy = weighted_energy(features);

  // Bass affects amplitude and distortion - more responsive for pop effect
  let bass_multiplier = 1.0 + features.bass * params.bass_influence * 0.8;
  params.amplitude = blend_towards(params.amplitude, bass_multiplier, 0.50);
  params.distort_amplitude = features.bass * params.bass_influence * 0.6;

  // Mid frequencies
  let mid_boost = 1.0 + features.mid * params.mid_influence * 2.0;
  params.frequency = blend_towards(params.frequency, 8.0 * mid_boost, 0.50);

  // Speed scales with treble - much more responsive
  let treble_boost = 1.0 + features.treble * params.treble_influence * 2.5;
  let base_speed = 0.08 + energy * 0.9;
  let target_speed = base_speed * treble_boost;
  params.speed = blend_towards(params.speed, target_speed, 0.45);

  // Color shift reacts to high notes
  params.color_shift = (params.color_shift + features.treble * 0.25) % std::f32::consts::TAU;

  // Bass drop triggers major effect AND full-strength distortion + zoom (check first for priority)
  if features.is_drop {
    params.effect_time = params.time;

    // Trigger full-strength beat distortion + zoom for maximum impact
    trigger_beat_visuals(
      params,
      DROP_BEAT_DISTORTION_STRENGTH,
      DROP_BEAT_ZOOM_STRENGTH,
    );
    let _ = debug_logln!(
      debug_log,
      "BASS DROP detected! Triggering effect + FULL distortion + ZOOM"
    );
  } else {
    // Use configurable beat sensitivity (higher sensitivity = lower threshold)
    let adjusted_threshold = regular_beat_threshold(params);

    if features.beat_strength > adjusted_threshold {
      // Regular beat triggers subtle distortion + subtle zoom
      params.noise_strength = features.beat_strength * (0.3 + features.treble * 0.7);

      // Trigger beat distortion pop effect (visible but not overwhelming for regular beats)
      trigger_beat_visuals(
        params,
        REGULAR_BEAT_DISTORTION_STRENGTH,
        REGULAR_BEAT_ZOOM_STRENGTH,
      );

      let _ = debug_logln!(
        debug_log,
        "BEAT detected! strength={:.2} - triggering subtle distortion + zoom",
        features.beat_strength
      );
    }
  }

  // Brightness reacts to treble with strong pop effect
  let treble_brightness = features.treble * 1.5;
  let beat_boost = features.beat_strength * 0.4; // Extra boost during beats
  params.brightness = (0.5 + features.overall * 1.0) + treble_brightness + beat_boost;
  params.brightness = params.brightness.min(2.2);

  // Contrast reacts more dynamically
  let treble_contrast = features.treble * 0.8;
  let target_contrast = 0.6 + energy * 0.6 + treble_contrast;
  params.contrast = blend_towards(params.contrast, target_contrast, 0.50);

  // Saturation reacts to bass and beats - colors "pop" on bass hits
  let bass_saturation = features.bass * 0.3; // Bass makes colors more vibrant
  let beat_saturation = features.beat_strength * 0.2; // Extra pop on beats
  params.saturation = params
    .saturation
    .max(0.7 + bass_saturation + beat_saturation)
    .min(1.2);
}

#[cfg(all(test, feature = "audio"))]
mod tests {
  use super::*;
  use chroma::debug::DebugLog;
  use chroma::params::ShaderParams;
  use std::time::{SystemTime, UNIX_EPOCH};

  fn test_debug_log() -> DebugLog {
    let timestamp = SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .unwrap()
      .as_nanos();
    let path = std::env::temp_dir().join(format!("chroma-audio-test-{timestamp}.log"));

    DebugLog::file(path).unwrap_or_else(|_| DebugLog::sink())
  }

  #[test]
  fn test_blend_towards_preserves_expected_weighting() {
    assert!((blend_towards(10.0, 2.0, 0.25) - 4.0).abs() < 0.0001);
  }

  #[test]
  fn test_weighted_energy_has_minimum_floor() {
    let features = AudioFeatures::default();

    assert!((weighted_energy(&features) - 0.05).abs() < 0.0001);
  }

  #[test]
  fn test_regular_beat_threshold_scales_with_sensitivity() {
    let low = ShaderParams {
      beat_sensitivity: 0.5,
      ..ShaderParams::default()
    };
    let high = ShaderParams {
      beat_sensitivity: 2.0,
      ..ShaderParams::default()
    };

    assert!(regular_beat_threshold(&high) < regular_beat_threshold(&low));
    assert!((regular_beat_threshold(&high) - 0.09).abs() < 0.0001);
  }

  #[test]
  fn test_update_audio_reactive_returns_default_when_audio_is_disabled() {
    let mut params = ShaderParams {
      audio_enabled: false,
      amplitude: 1.3,
      ..ShaderParams::default()
    };
    let original = params.clone();
    let mut analyzer = None;
    let mut debug_log = test_debug_log();

    let features = update_audio_reactive(&mut params, &None, &mut analyzer, 1.0 / 30.0, &mut debug_log);

    assert_eq!(features.bass, 0.0);
    assert_eq!(features.mid, 0.0);
    assert_eq!(features.treble, 0.0);
    assert_eq!(features.overall, 0.0);
    assert_eq!(features.beat_strength, 0.0);
    assert!(!features.is_drop);
    assert_eq!(params.amplitude, original.amplitude);
  }

  #[test]
  fn test_update_audio_reactive_returns_default_without_capture_or_analyzer() {
    let mut params = ShaderParams {
      audio_enabled: true,
      amplitude: 1.3,
      ..ShaderParams::default()
    };
    let original = params.clone();
    let mut analyzer = None;
    let mut debug_log = test_debug_log();

    let features = update_audio_reactive(&mut params, &None, &mut analyzer, 1.0 / 30.0, &mut debug_log);

    assert_eq!(features.bass, 0.0);
    assert_eq!(features.overall, 0.0);
    assert_eq!(params.amplitude, original.amplitude);
  }

  #[test]
  fn test_apply_silence_decay_moves_parameters_toward_idle_baselines() {
    let mut params = ShaderParams {
      amplitude: 1.6,
      distort_amplitude: 0.8,
      frequency: 14.0,
      speed: 0.9,
      brightness: 1.8,
      noise_strength: 0.4,
      contrast: 1.4,
      ..ShaderParams::default()
    };
    let features = AudioFeatures {
      overall: 0.0,
      ..AudioFeatures::default()
    };
    let mut debug_log = test_debug_log();

    apply_silence_decay(&mut params, &features, &mut debug_log);

    assert!(params.amplitude < 1.6);
    assert!(params.frequency < 14.0);
    assert!(params.brightness < 1.8);
    assert!(params.speed < 0.9);
    assert!(params.distort_amplitude < 0.8);
    assert!(params.noise_strength < 0.4);
    assert!(params.contrast < 1.4);
    assert!(params.amplitude > SILENT_AMPLITUDE_BASELINE);
    assert!(params.frequency > SILENT_FREQUENCY_BASELINE);
  }

  #[test]
  fn test_apply_audio_reactivity_drop_triggers_full_strength_beat_visuals() {
    let mut params = ShaderParams {
      time: 42.0,
      bass_influence: 0.7,
      mid_influence: 0.5,
      treble_influence: 0.4,
      ..ShaderParams::default()
    };
    let features = AudioFeatures {
      bass: 0.9,
      mid: 0.4,
      treble: 0.3,
      overall: 0.7,
      beat_strength: 0.5,
      is_drop: true,
    };
    let mut debug_log = test_debug_log();

    apply_audio_reactivity(&mut params, &features, &mut debug_log);

    assert_eq!(params.effect_time, 42.0);
    assert_eq!(params.beat_distortion_time, 42.0);
    assert_eq!(
      params.beat_distortion_strength,
      DROP_BEAT_DISTORTION_STRENGTH
    );
    assert_eq!(params.beat_zoom_strength, DROP_BEAT_ZOOM_STRENGTH);
  }

  #[test]
  fn test_apply_audio_reactivity_regular_beat_uses_configured_threshold() {
    let mut params = ShaderParams {
      time: 12.0,
      beat_sensitivity: 2.0,
      bass_influence: 0.5,
      mid_influence: 0.3,
      treble_influence: 0.2,
      ..ShaderParams::default()
    };
    let features = AudioFeatures {
      bass: 0.2,
      mid: 0.3,
      treble: 0.7,
      overall: 0.6,
      beat_strength: 0.10,
      is_drop: false,
    };
    let mut debug_log = test_debug_log();

    apply_audio_reactivity(&mut params, &features, &mut debug_log);

    assert_eq!(params.beat_distortion_time, 12.0);
    assert_eq!(
      params.beat_distortion_strength,
      REGULAR_BEAT_DISTORTION_STRENGTH
    );
    assert_eq!(params.beat_zoom_strength, REGULAR_BEAT_ZOOM_STRENGTH);
    assert!(params.noise_strength > 0.0);
  }
}
