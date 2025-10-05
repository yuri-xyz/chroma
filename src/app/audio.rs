// Audio-reactive update logic

#[cfg(feature = "audio")]
use crate::constants::{AUDIO_DECAY_RATE, AUDIO_SILENCE_THRESHOLD, AUDIO_SPEED_DECAY_RATE};
#[cfg(feature = "audio")]
use chroma::audio::{AudioAnalyzer, AudioCapture};
#[cfg(feature = "audio")]
use std::fs::File;
#[cfg(feature = "audio")]
use std::io::{BufWriter, Write};

/// Update shader parameters based on audio input
#[cfg(feature = "audio")]
pub fn update_audio_reactive(
  params: &mut chroma::params::ShaderParams,
  audio_capture: &Option<AudioCapture>,
  audio_analyzer: &mut Option<AudioAnalyzer>,
  delta_time: f32,
  debug_log: &mut BufWriter<File>,
) {
  if !params.audio_enabled {
    return;
  }

  if let (Some(capture), Some(analyzer)) = (audio_capture, audio_analyzer) {
    let samples = capture.get_samples();

    if samples.is_empty() {
      return;
    }

    let features = analyzer.analyze(&samples, delta_time);
    let is_silent = features.overall < AUDIO_SILENCE_THRESHOLD;

    if is_silent {
      apply_silence_decay(params, &features, debug_log);
    } else {
      apply_audio_reactivity(params, &features, debug_log);
    }
  }
}

/// Apply decay to parameters when audio is silent
#[cfg(feature = "audio")]
fn apply_silence_decay(
  params: &mut chroma::params::ShaderParams,
  features: &chroma::audio::AudioFeatures,
  debug_log: &mut BufWriter<File>,
) {
  params.amplitude = params.amplitude * AUDIO_DECAY_RATE + 0.4 * (1.0 - AUDIO_DECAY_RATE);
  params.distort_amplitude *= AUDIO_DECAY_RATE;
  params.frequency = params.frequency * AUDIO_DECAY_RATE + 6.0 * (1.0 - AUDIO_DECAY_RATE);
  params.speed *= AUDIO_SPEED_DECAY_RATE;
  params.brightness = params.brightness * AUDIO_DECAY_RATE + 0.6 * (1.0 - AUDIO_DECAY_RATE);
  params.noise_strength *= 0.85;
  params.contrast = params.contrast * AUDIO_DECAY_RATE + 0.8 * (1.0 - AUDIO_DECAY_RATE);

  writeln!(
    debug_log,
    "AUDIO: Silence (vol={:.4}) - slowing to stop (speed={:.3})",
    features.overall, params.speed
  )
  .ok();
}

/// Apply audio features to shader parameters
#[cfg(feature = "audio")]
fn apply_audio_reactivity(
  params: &mut chroma::params::ShaderParams,
  features: &chroma::audio::AudioFeatures,
  debug_log: &mut BufWriter<File>,
) {
  // Emphasize treble for melody visibility
  let energy = (features.bass * 0.1 + features.mid * 0.3 + features.treble * 0.6).max(0.05);

  // Bass affects amplitude and distortion (subtle)
  let bass_multiplier = 1.0 + features.bass * params.bass_influence * 0.6;
  params.amplitude = (params.amplitude * 0.95) + (bass_multiplier * 0.05);
  params.distort_amplitude = features.bass * params.bass_influence * 0.4;

  // Mid frequencies
  let mid_boost = 1.0 + features.mid * params.mid_influence * 1.8;
  params.frequency = (params.frequency * 0.90) + (8.0 * mid_boost * 0.10);

  // Speed scales with treble for high notes
  let treble_boost = 1.0 + features.treble * params.treble_influence * 2.0;
  let base_speed = 0.08 + energy * 0.7;
  let target_speed = base_speed * treble_boost;
  params.speed = (params.speed * 0.88) + (target_speed * 0.12);

  // Color shift reacts to high notes
  params.color_shift = (params.color_shift + features.treble * 0.25) % std::f32::consts::TAU;

  // Beat triggers effects
  if features.beat_strength > 0.35 {
    params.noise_strength = features.beat_strength * (0.2 + features.treble * 0.5);
  }

  // Bass drop triggers major effect
  if features.is_drop {
    params.effect_time = params.time;
    writeln!(debug_log, "BASS DROP detected! Triggering effect").ok();
  }

  // Brightness reacts to treble
  let treble_brightness = features.treble * 0.8;
  params.brightness = (0.5 + features.overall * 0.6) + treble_brightness;
  params.brightness = params.brightness.min(1.8);

  // Contrast reacts to treble
  let treble_contrast = features.treble * 0.6;
  let target_contrast = 0.6 + energy * 0.4 + treble_contrast;
  params.contrast = (params.contrast * 0.90) + (target_contrast * 0.10);
}
