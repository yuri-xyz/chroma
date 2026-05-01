mod support;

use support::{
  audio_fixtures::{
    analyze_fixture, analyze_fixture_with_chunk_schedule, FixtureBuilder, ANALYSIS_HOP,
    ANALYSIS_WINDOW,
  },
  audio_trace::{
    segment_drop_onset_count, segment_has_drop, segment_max, trailing_segment_average,
  },
};

#[test]
fn test_drop_cooldown_behavior_survives_variable_chunk_schedule() {
  let fixture = FixtureBuilder::new()
    .silence("warmup", 4)
    .kick_pulses("first_drop", 4, ANALYSIS_WINDOW, 224, 1.0, 70.0)
    .silence("short_gap", 5)
    .kick_pulses("blocked_drop", 4, ANALYSIS_WINDOW, 224, 1.0, 70.0)
    .silence("rearm_gap", 24)
    .kick_pulses("late_drop", 4, ANALYSIS_WINDOW, 224, 1.0, 70.0)
    .build();

  let aligned_trace = analyze_fixture(&fixture, ANALYSIS_HOP);
  let scheduled_trace =
    analyze_fixture_with_chunk_schedule(&fixture, &[61, 733, 97, 1_011, 257, 509, 43]);

  for segment_name in ["first_drop", "blocked_drop", "late_drop"] {
    let segment = fixture.segment(segment_name);

    assert_eq!(
      segment_drop_onset_count(&aligned_trace, segment),
      segment_drop_onset_count(&scheduled_trace, segment),
      "drop onset count should stay stable across chunk schedules for {segment_name}"
    );
  }

  assert!(
    segment_has_drop(&scheduled_trace, fixture.segment("first_drop")),
    "first drop should still be detected under a variable chunk schedule"
  );
  assert!(
    !segment_has_drop(&scheduled_trace, fixture.segment("blocked_drop")),
    "cooldown should still suppress the immediate retrigger under a variable chunk schedule"
  );
  assert!(
    segment_has_drop(&scheduled_trace, fixture.segment("late_drop")),
    "late drop should still re-arm under a variable chunk schedule"
  );
}

#[test]
fn test_non_bass_reentry_after_rearm_does_not_false_trigger_drop_detection() {
  let fixture = FixtureBuilder::new()
    .silence("warmup", 4)
    .kick_pulses("first_drop", 4, ANALYSIS_WINDOW, 224, 1.0, 70.0)
    .silence("rearm_gap", 24)
    .layers("bright_bed", &[(1_100.0, 0.18), (4_800.0, 0.20)], 3)
    .pulse_train("reentry", 4, ANALYSIS_WINDOW / 2, 128, 0.55, 6_200.0)
    .silence("release", 4)
    .build();
  let trace = analyze_fixture(&fixture, ANALYSIS_HOP / 2);

  let reentry = fixture.segment("reentry");
  let reentry_mid = segment_max(&trace, reentry, |features| features.mid);
  let reentry_bass = segment_max(&trace, reentry, |features| features.bass);
  let reentry_treble = segment_max(&trace, reentry, |features| features.treble);
  let reentry_beat = segment_max(&trace, reentry, |features| features.beat_strength);

  assert!(
    reentry_treble > reentry_bass * 2.0,
    "non-bass reentry should remain high-frequency-led, got bass={:.4} mid={:.4} treble={:.4}",
    reentry_bass,
    reentry_mid,
    reentry_treble
  );
  assert!(
    reentry_beat < 0.20,
    "non-bass reentry should stay below committed kick-like beat levels, got {:.4}",
    reentry_beat
  );
  assert!(
    !segment_has_drop(&trace, reentry),
    "non-bass reentry should not trigger drop detection after re-arming"
  );
}

#[test]
fn test_post_drop_silence_clears_residual_energy_and_drop_state() {
  let fixture = FixtureBuilder::new()
    .silence("warmup", 4)
    .kick_pulses("drop", 4, ANALYSIS_WINDOW, 224, 1.0, 70.0)
    .silence("cooldown", 18)
    .build();
  let trace = analyze_fixture(&fixture, ANALYSIS_HOP);

  let drop = fixture.segment("drop");
  let cooldown = fixture.segment("cooldown");

  let drop_beat = segment_max(&trace, drop, |features| features.beat_strength);
  let drop_overall = segment_max(&trace, drop, |features| features.overall);
  let cooldown_beat = trailing_segment_average(&trace, cooldown, |features| features.beat_strength);
  let cooldown_overall = trailing_segment_average(&trace, cooldown, |features| features.overall);

  assert!(
    drop_beat > 0.15,
    "drop section should register meaningful beat energy before decay, got {:.4}",
    drop_beat
  );
  assert!(
    cooldown_beat < drop_beat * 0.2,
    "long silence should clear residual beat energy after a drop, got drop={:.4} cooldown={:.4}",
    drop_beat,
    cooldown_beat
  );
  assert!(
    cooldown_overall < drop_overall * 0.1,
    "long silence should clear overall energy after a drop, got drop={:.4} cooldown={:.4}",
    drop_overall,
    cooldown_overall
  );
  assert!(
    !segment_has_drop(&trace, cooldown),
    "cooldown segment should not keep reporting drop detection"
  );
}
