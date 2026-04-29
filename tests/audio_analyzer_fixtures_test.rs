mod support;

use support::audio_fixtures::{
  analyze_fixture, analyze_fixture_with_chunk_schedule, FixtureBuilder, ANALYSIS_HOP,
  ANALYSIS_WINDOW,
};
use support::audio_trace::{
  segment_drop_count, segment_frames, segment_has_drop, segment_max, trailing_segment_average,
  trailing_segment_has_drop,
};

#[test]
fn test_authored_fixture_tracks_band_transitions_across_segments() {
  let fixture = FixtureBuilder::new()
    .silence("warmup", 4)
    .tone("bass_block", 80.0, 0.9, 4)
    .tone("mid_block", 900.0, 0.9, 4)
    .tone("treble_block", 4_200.0, 0.9, 4)
    .build();
  let trace = analyze_fixture(&fixture, ANALYSIS_HOP);

  let bass_segment = fixture.segment("bass_block");
  let mid_segment = fixture.segment("mid_block");
  let treble_segment = fixture.segment("treble_block");

  let bass_peak = segment_max(&trace, bass_segment, |features| features.bass);
  let bass_mid_peak = segment_max(&trace, bass_segment, |features| features.mid);
  let bass_treble_peak = segment_max(&trace, bass_segment, |features| features.treble);

  assert!(
    bass_peak > 0.15,
    "expected meaningful bass block response, got bass={:.4} mid={:.4} treble={:.4}",
    bass_peak,
    bass_mid_peak,
    bass_treble_peak
  );
  assert!(
    bass_peak > bass_mid_peak * 2.4,
    "bass block should favor bass, got bass={:.4} mid={:.4} treble={:.4}",
    bass_peak,
    bass_mid_peak,
    bass_treble_peak
  );
  assert!(
    bass_peak > bass_treble_peak * 4.0,
    "bass block should strongly reject treble, got bass={:.4} mid={:.4} treble={:.4}",
    bass_peak,
    bass_mid_peak,
    bass_treble_peak
  );

  let mid_peak = segment_max(&trace, mid_segment, |features| features.mid);
  let mid_bass_peak = segment_max(&trace, mid_segment, |features| features.bass);
  let mid_treble_peak = segment_max(&trace, mid_segment, |features| features.treble);

  assert!(
    mid_peak > 0.15,
    "expected meaningful mid block response, got bass={:.4} mid={:.4} treble={:.4}",
    mid_bass_peak,
    mid_peak,
    mid_treble_peak
  );
  assert!(
    mid_peak > mid_bass_peak * 2.5,
    "mid block should favor mids, got bass={:.4} mid={:.4} treble={:.4}",
    mid_bass_peak,
    mid_peak,
    mid_treble_peak
  );
  assert!(
    mid_peak > mid_treble_peak * 1.5,
    "mid block should favor mids, got bass={:.4} mid={:.4} treble={:.4}",
    mid_bass_peak,
    mid_peak,
    mid_treble_peak
  );

  let treble_peak = segment_max(&trace, treble_segment, |features| features.treble);
  let treble_bass_peak = segment_max(&trace, treble_segment, |features| features.bass);
  let treble_mid_peak = segment_max(&trace, treble_segment, |features| features.mid);

  assert!(
    treble_peak > 0.15,
    "expected meaningful treble block response, got bass={:.4} mid={:.4} treble={:.4}",
    treble_bass_peak,
    treble_mid_peak,
    treble_peak
  );
  assert!(
    treble_peak > treble_bass_peak * 4.0,
    "treble block should strongly reject bass, got bass={:.4} mid={:.4} treble={:.4}",
    treble_bass_peak,
    treble_mid_peak,
    treble_peak
  );
  assert!(
    treble_peak > treble_mid_peak * 1.5,
    "treble block should favor treble, got bass={:.4} mid={:.4} treble={:.4}",
    treble_bass_peak,
    treble_mid_peak,
    treble_peak
  );
}

#[test]
fn test_authored_fixture_settled_calm_bass_does_not_keep_reporting_drops() {
  let fixture = FixtureBuilder::new()
    .silence("warmup", 4)
    .tone("calm_bass", 80.0, 0.08, 5)
    .silence("release", 3)
    .build();
  let trace = analyze_fixture(&fixture, ANALYSIS_HOP);
  let calm_segment = fixture.segment("calm_bass");

  assert!(
    !trailing_segment_has_drop(&trace, calm_segment),
    "settled calm intro should not keep reporting drops, got bass peak {:.4} and beat peak {:.4}",
    segment_frames(&trace, calm_segment)
      .into_iter()
      .map(|frame| frame.features.bass)
      .fold(0.0_f32, f32::max),
    segment_frames(&trace, calm_segment)
      .into_iter()
      .map(|frame| frame.features.beat_strength)
      .fold(0.0_f32, f32::max)
  );
}

#[test]
fn test_authored_fixture_detects_bass_drop_pulse_segment() {
  let fixture = FixtureBuilder::new()
    .silence("warmup", 4)
    .kick_pulses("drop", 4, ANALYSIS_WINDOW, 224, 1.0, 70.0)
    .silence("release", 3)
    .build();
  let trace = analyze_fixture(&fixture, ANALYSIS_HOP);
  let drop_segment = fixture.segment("drop");
  let drop_bass_peak = segment_frames(&trace, drop_segment)
    .into_iter()
    .map(|frame| frame.features.bass)
    .fold(0.0_f32, f32::max);
  let drop_beat_peak = segment_frames(&trace, drop_segment)
    .into_iter()
    .map(|frame| frame.features.beat_strength)
    .fold(0.0_f32, f32::max);

  assert!(
    segment_has_drop(&trace, drop_segment),
    "expected authored drop segment to trigger drop detection, got bass peak {:.4} and beat peak {:.4}",
    drop_bass_peak,
    drop_beat_peak
  );
}

#[test]
fn test_authored_fixture_beat_energy_decays_after_pulse_section() {
  let fixture = FixtureBuilder::new()
    .silence("warmup", 4)
    .kick_pulses("beats", 4, ANALYSIS_WINDOW, 192, 1.0, 70.0)
    .silence("cooldown", 4)
    .build();
  let trace = analyze_fixture(&fixture, ANALYSIS_HOP / 2);

  let beat_peak = segment_max(&trace, fixture.segment("beats"), |features| {
    features.beat_strength
  });
  let cooldown_peak = segment_max(&trace, fixture.segment("cooldown"), |features| {
    features.beat_strength
  });

  assert!(
    beat_peak > 0.3,
    "expected pulse section to register as beats, got {:.4}",
    beat_peak
  );
  assert!(
    cooldown_peak < 0.2,
    "expected beat energy to decay during cooldown, got {:.4}",
    cooldown_peak
  );
}

#[test]
fn test_authored_fixture_is_stable_under_chunk_jitter_for_events() {
  let fixture = FixtureBuilder::new()
    .silence("warmup", 4)
    .layers("groove", &[(80.0, 0.8), (900.0, 0.35), (4_200.0, 0.2)], 4)
    .kick_pulses("beats", 4, ANALYSIS_WINDOW, 192, 1.0, 70.0)
    .build();

  let aligned_trace = analyze_fixture(&fixture, ANALYSIS_HOP);
  let jittered_trace = analyze_fixture(&fixture, 173);

  let beat_segment = fixture.segment("beats");
  let groove_segment = fixture.segment("groove");

  let aligned_beat_peak = segment_max(&aligned_trace, beat_segment, |features| {
    features.beat_strength
  });
  let jittered_beat_peak = segment_max(&jittered_trace, beat_segment, |features| {
    features.beat_strength
  });
  let aligned_groove_overall =
    segment_max(&aligned_trace, groove_segment, |features| features.overall);
  let jittered_groove_overall =
    segment_max(&jittered_trace, groove_segment, |features| features.overall);

  assert!(
    (aligned_beat_peak - jittered_beat_peak).abs() < 0.12,
    "beat peak should stay stable across chunk jitter"
  );
  assert!(
    (aligned_groove_overall - jittered_groove_overall).abs() < 0.12,
    "overall groove energy should stay stable across chunk jitter"
  );
}

#[test]
fn test_authored_fixture_treble_pulses_do_not_false_trigger_bass_drop_logic() {
  let fixture = FixtureBuilder::new()
    .silence("warmup", 4)
    .pulse_train("hats", 4, ANALYSIS_WINDOW / 2, 160, 0.9, 6_200.0)
    .silence("release", 3)
    .build();
  let trace = analyze_fixture(&fixture, ANALYSIS_HOP / 2);
  let hats = fixture.segment("hats");

  let treble_peak = segment_max(&trace, hats, |features| features.treble);
  let bass_peak = segment_max(&trace, hats, |features| features.bass);
  let beat_peak = segment_max(&trace, hats, |features| features.beat_strength);

  assert!(
    treble_peak > bass_peak * 2.0,
    "treble pulse segment should remain treble-dominant, got bass={:.4} treble={:.4}",
    bass_peak,
    treble_peak
  );
  assert!(
    beat_peak < 0.2,
    "treble pulse segment should not be treated like a kick beat, got {:.4}",
    beat_peak
  );
  assert!(
    !segment_has_drop(&trace, hats),
    "treble pulse segment should not trigger drop detection"
  );
}

#[test]
fn test_authored_fixture_silence_cools_overall_energy_after_dense_section() {
  let fixture = FixtureBuilder::new()
    .silence("warmup", 4)
    .layers("groove", &[(80.0, 0.8), (900.0, 0.45), (4_200.0, 0.25)], 5)
    .silence("cooldown", 6)
    .build();
  let trace = analyze_fixture(&fixture, ANALYSIS_HOP);

  let groove = fixture.segment("groove");
  let cooldown = fixture.segment("cooldown");
  let groove_overall = trailing_segment_average(&trace, groove, |features| features.overall);
  let cooldown_overall = trailing_segment_average(&trace, cooldown, |features| features.overall);

  assert!(
    cooldown_overall < groove_overall * 0.35,
    "silence should cool overall energy, got groove={:.4} cooldown={:.4}",
    groove_overall,
    cooldown_overall
  );
}

#[test]
fn test_authored_fixture_alternating_low_high_pulses_biases_toward_beats_only_on_low_hits() {
  let fixture = FixtureBuilder::new()
    .silence("warmup", 4)
    .alternating_pulses("alternating", 6, ANALYSIS_WINDOW / 2, 160, 70.0, 5_400.0)
    .build();
  let trace = analyze_fixture(&fixture, ANALYSIS_HOP / 2);
  let alternating = fixture.segment("alternating");

  let beat_peak = segment_max(&trace, alternating, |features| features.beat_strength);
  let bass_peak = segment_max(&trace, alternating, |features| features.bass);
  let treble_peak = segment_max(&trace, alternating, |features| features.treble);

  assert!(
    beat_peak > 0.12,
    "alternating low/high pulses should still expose beat energy from low hits, got {:.4}",
    beat_peak
  );
  assert!(
    treble_peak > 0.1,
    "alternating low/high pulses should retain treble response, got {:.4}",
    treble_peak
  );
  assert!(
    bass_peak > 0.1,
    "alternating low/high pulses should retain bass response, got {:.4}",
    bass_peak
  );
}

#[test]
fn test_authored_fixture_snare_pulses_are_mid_dominant_without_drop_detection() {
  let fixture = FixtureBuilder::new()
    .silence("warmup", 4)
    .pulse_train("snares", 4, ANALYSIS_WINDOW / 2, 208, 0.95, 1_800.0)
    .silence("release", 3)
    .build();
  let trace = analyze_fixture(&fixture, ANALYSIS_HOP / 2);
  let snares = fixture.segment("snares");

  let bass_peak = segment_max(&trace, snares, |features| features.bass);
  let mid_peak = segment_max(&trace, snares, |features| features.mid);
  let treble_peak = segment_max(&trace, snares, |features| features.treble);
  let beat_peak = segment_max(&trace, snares, |features| features.beat_strength);

  assert!(
    mid_peak > bass_peak * 1.1,
    "snare pulses should be mid-dominant, got bass={:.4} mid={:.4} treble={:.4}",
    bass_peak,
    mid_peak,
    treble_peak
  );
  assert!(
    mid_peak > treble_peak * 1.0,
    "snare pulses should lean mid over treble, got bass={:.4} mid={:.4} treble={:.4}",
    bass_peak,
    mid_peak,
    treble_peak
  );
  assert!(
    beat_peak < 0.45,
    "snare pulses should stay below kick-like beat levels, got {:.4}",
    beat_peak
  );
  assert!(
    !trailing_segment_has_drop(&trace, snares),
    "snare pulses should not keep reporting drop detection once settled"
  );
}

#[test]
fn test_authored_fixture_section_sequence_tracks_buildup_drop_and_cooldown() {
  let fixture = FixtureBuilder::new()
    .silence("warmup", 4)
    .layers(
      "buildup_low",
      &[(80.0, 0.25), (900.0, 0.18), (4_200.0, 0.08)],
      4,
    )
    .layers(
      "buildup_high",
      &[(80.0, 0.55), (900.0, 0.32), (4_200.0, 0.18)],
      4,
    )
    .kick_pulses("drop", 4, ANALYSIS_WINDOW, 224, 1.0, 70.0)
    .silence("cooldown", 6)
    .build();
  let trace = analyze_fixture(&fixture, ANALYSIS_HOP);

  let buildup_low = fixture.segment("buildup_low");
  let buildup_high = fixture.segment("buildup_high");
  let drop = fixture.segment("drop");
  let cooldown = fixture.segment("cooldown");

  let buildup_low_overall =
    trailing_segment_average(&trace, buildup_low, |features| features.overall);
  let buildup_high_overall =
    trailing_segment_average(&trace, buildup_high, |features| features.overall);
  let buildup_high_bass = segment_max(&trace, buildup_high, |features| features.bass);
  let drop_beat_peak = segment_max(&trace, drop, |features| features.beat_strength);
  let drop_bass_peak = segment_max(&trace, drop, |features| features.bass);
  let cooldown_overall = trailing_segment_average(&trace, cooldown, |features| features.overall);

  assert!(
    buildup_high_overall > buildup_low_overall * 1.2,
    "later buildup section should carry more overall energy, got low={:.4} high={:.4}",
    buildup_low_overall,
    buildup_high_overall
  );
  assert!(
    drop_bass_peak > buildup_high_bass * 1.05,
    "drop segment should raise bass energy above buildup, got buildup={:.4} drop={:.4} beat={:.4}",
    buildup_high_bass,
    drop_bass_peak,
    drop_beat_peak
  );
  assert!(
    cooldown_overall < buildup_high_overall * 0.35,
    "cooldown should shed most buildup energy, got buildup={:.4} cooldown={:.4}",
    buildup_high_overall,
    cooldown_overall
  );
}

#[test]
fn test_authored_fixture_percussion_sections_keep_kick_snare_and_hat_roles_distinct() {
  let fixture = FixtureBuilder::new()
    .silence("warmup", 4)
    .kick_pulses("kicks", 4, ANALYSIS_WINDOW, 192, 1.0, 70.0)
    .pulse_train("snares", 4, ANALYSIS_WINDOW / 2, 208, 0.95, 1_800.0)
    .pulse_train("hats", 4, ANALYSIS_WINDOW / 3, 96, 0.55, 6_200.0)
    .build();
  let trace = analyze_fixture(&fixture, ANALYSIS_HOP / 2);

  let kicks = fixture.segment("kicks");
  let snares = fixture.segment("snares");
  let hats = fixture.segment("hats");

  let kick_bass = segment_max(&trace, kicks, |features| features.bass);
  let kick_mid = segment_max(&trace, kicks, |features| features.mid);
  let kick_beat = segment_max(&trace, kicks, |features| features.beat_strength);
  let snare_bass = segment_max(&trace, snares, |features| features.bass);
  let snare_mid = segment_max(&trace, snares, |features| features.mid);
  let snare_beat = segment_max(&trace, snares, |features| features.beat_strength);
  let hat_treble = segment_max(&trace, hats, |features| features.treble);
  let hat_beat = segment_max(&trace, hats, |features| features.beat_strength);

  assert!(
    kick_bass > kick_mid,
    "kick section should stay bass-led, got bass={:.4} mid={:.4}",
    kick_bass,
    kick_mid
  );
  assert!(
    snare_mid > snare_bass * 1.05,
    "snare section should stay mid-led, got bass={:.4} mid={:.4}",
    snare_bass,
    snare_mid
  );
  assert!(
    kick_beat > snare_beat * 1.2,
    "kick section should drive stronger beat energy than snares, got kick={:.4} snare={:.4} hat={:.4}",
    kick_beat,
    snare_beat,
    hat_beat
  );
  assert!(
    kick_beat > hat_beat * 2.0,
    "kick section should beat-react more strongly than hats, got kick={:.4} hat={:.4}",
    kick_beat,
    hat_beat
  );
  assert!(
    hat_treble > 0.2,
    "hat section should still produce a meaningful treble response, got treble={:.4} beat={:.4}",
    hat_treble,
    hat_beat
  );
}

#[test]
fn test_authored_fixture_fakeout_buildup_stays_below_drop_threshold() {
  let fixture = FixtureBuilder::new()
    .silence("warmup", 4)
    .layers("setup", &[(900.0, 0.25), (4_200.0, 0.18)], 2)
    .layered_pulse_train(
      "fakeout",
      4,
      &[(900.0, 0.28), (4_800.0, 0.18)],
      ANALYSIS_WINDOW / 2,
      128,
      0.72,
      80.0,
    )
    .silence("release", 4)
    .build();
  let trace = analyze_fixture(&fixture, ANALYSIS_HOP / 2);
  let fakeout = fixture.segment("fakeout");

  let overall_peak = segment_max(&trace, fakeout, |features| features.overall);
  let beat_peak = segment_max(&trace, fakeout, |features| features.beat_strength);
  let bass_peak = segment_max(&trace, fakeout, |features| features.bass);
  let mid_peak = segment_max(&trace, fakeout, |features| features.mid);
  let treble_peak = segment_max(&trace, fakeout, |features| features.treble);

  assert!(
    overall_peak > 0.35,
    "fakeout should still build noticeable overall energy, got overall={:.4} beat={:.4} bass={:.4} mid={:.4} treble={:.4}",
    overall_peak,
    beat_peak,
    bass_peak,
    mid_peak,
    treble_peak
  );
  assert!(
    beat_peak < 0.12,
    "fakeout should stay below committed beat thresholds, got overall={:.4} beat={:.4}",
    overall_peak,
    beat_peak
  );
  assert!(
    !segment_has_drop(&trace, fakeout),
    "fakeout should not trigger drop detection, got overall={:.4} beat={:.4} bass={:.4} mid={:.4} treble={:.4}",
    overall_peak,
    beat_peak,
    bass_peak,
    mid_peak,
    treble_peak
  );
}

#[test]
fn test_authored_fixture_keeps_event_shape_under_variable_chunk_schedule() {
  let fixture = FixtureBuilder::new()
    .silence("warmup", 4)
    .layers("groove", &[(80.0, 0.7), (900.0, 0.4), (4_200.0, 0.2)], 4)
    .kick_pulses("drop", 4, ANALYSIS_WINDOW / 2, 192, 1.0, 70.0)
    .silence("cooldown", 5)
    .build();

  let aligned_trace = analyze_fixture(&fixture, ANALYSIS_HOP);
  let scheduled_trace =
    analyze_fixture_with_chunk_schedule(&fixture, &[97, 509, 1_537, 61, 887, 32, 703]);

  let groove = fixture.segment("groove");
  let drop = fixture.segment("drop");
  let cooldown = fixture.segment("cooldown");

  let aligned_groove = segment_max(&aligned_trace, groove, |features| features.overall);
  let scheduled_groove = segment_max(&scheduled_trace, groove, |features| features.overall);
  let aligned_drop = segment_max(&aligned_trace, drop, |features| features.beat_strength);
  let scheduled_drop = segment_max(&scheduled_trace, drop, |features| features.beat_strength);
  let aligned_cooldown =
    trailing_segment_average(&aligned_trace, cooldown, |features| features.overall);
  let scheduled_cooldown =
    trailing_segment_average(&scheduled_trace, cooldown, |features| features.overall);

  assert!(
    (aligned_groove - scheduled_groove).abs() < 0.12,
    "groove energy should stay stable across variable chunk schedules, got aligned={:.4} scheduled={:.4}",
    aligned_groove,
    scheduled_groove
  );
  assert!(
    (aligned_drop - scheduled_drop).abs() < 0.15,
    "drop beat energy should stay stable across variable chunk schedules, got aligned={:.4} scheduled={:.4}",
    aligned_drop,
    scheduled_drop
  );
  assert!(
    (aligned_cooldown - scheduled_cooldown).abs() < 0.10,
    "cooldown energy should stay stable across variable chunk schedules, got aligned={:.4} scheduled={:.4}",
    aligned_cooldown,
    scheduled_cooldown
  );
  assert_eq!(
    segment_has_drop(&aligned_trace, drop),
    segment_has_drop(&scheduled_trace, drop),
    "variable chunk schedule should preserve drop detection for the same fixture"
  );
}

#[test]
fn test_authored_fixture_drop_cooldown_blocks_immediate_retrigger_but_allows_later_return() {
  let fixture = FixtureBuilder::new()
    .silence("warmup", 4)
    .kick_pulses("first_drop", 4, ANALYSIS_WINDOW, 224, 1.0, 70.0)
    .silence("short_gap", 5)
    .kick_pulses("blocked_drop", 4, ANALYSIS_WINDOW, 224, 1.0, 70.0)
    .silence("rearm_gap", 24)
    .kick_pulses("late_drop", 4, ANALYSIS_WINDOW, 224, 1.0, 70.0)
    .build();
  let trace = analyze_fixture(&fixture, ANALYSIS_HOP);

  let first_drop = fixture.segment("first_drop");
  let blocked_drop = fixture.segment("blocked_drop");
  let late_drop = fixture.segment("late_drop");

  let first_drop_count = segment_drop_count(&trace, first_drop);
  let blocked_drop_count = segment_drop_count(&trace, blocked_drop);
  let late_drop_count = segment_drop_count(&trace, late_drop);

  assert!(
    first_drop_count >= 1,
    "first drop section should trigger at least one drop event, got {}",
    first_drop_count
  );
  assert_eq!(
    blocked_drop_count, 0,
    "drop cooldown should block immediate retriggers, got {}",
    blocked_drop_count
  );
  assert!(
    late_drop_count >= 1,
    "drop detection should re-arm after a long enough gap, got {}",
    late_drop_count
  );
}

#[test]
fn test_authored_fixture_fakeout_then_real_drop_distinguishes_buildup_from_commitment() {
  let fixture = FixtureBuilder::new()
    .silence("warmup", 4)
    .layers("setup", &[(900.0, 0.25), (4_200.0, 0.18)], 2)
    .layered_pulse_train(
      "fakeout",
      4,
      &[(900.0, 0.28), (4_800.0, 0.18)],
      ANALYSIS_WINDOW / 2,
      128,
      0.72,
      80.0,
    )
    .silence("breath", 2)
    .kick_pulses("real_drop", 4, ANALYSIS_WINDOW, 224, 1.0, 70.0)
    .silence("cooldown", 5)
    .build();
  let trace = analyze_fixture(&fixture, ANALYSIS_HOP / 2);

  let fakeout = fixture.segment("fakeout");
  let real_drop = fixture.segment("real_drop");
  let cooldown = fixture.segment("cooldown");

  let fakeout_overall = segment_max(&trace, fakeout, |features| features.overall);
  let fakeout_beat = segment_max(&trace, fakeout, |features| features.beat_strength);
  let fakeout_drops = segment_drop_count(&trace, fakeout);
  let real_drop_beat = segment_max(&trace, real_drop, |features| features.beat_strength);
  let real_drop_bass = segment_max(&trace, real_drop, |features| features.bass);
  let real_drop_drops = segment_drop_count(&trace, real_drop);
  let cooldown_overall = trailing_segment_average(&trace, cooldown, |features| features.overall);

  assert!(
    fakeout_overall > 0.35,
    "fakeout should still register as an active buildup, got overall={:.4}",
    fakeout_overall
  );
  assert_eq!(
    fakeout_drops, 0,
    "fakeout should not trigger committed drop events, got {}",
    fakeout_drops
  );
  assert!(
    real_drop_drops >= 1,
    "real drop should trigger at least one drop event, got {}",
    real_drop_drops
  );
  assert!(
    real_drop_beat > fakeout_beat * 3.0,
    "real drop should beat-react more strongly than the fakeout, got fakeout={:.4} drop={:.4} bass={:.4}",
    fakeout_beat,
    real_drop_beat,
    real_drop_bass
  );
  assert!(
    cooldown_overall < fakeout_overall * 0.5,
    "cooldown should release energy after the real drop, got fakeout={:.4} cooldown={:.4}",
    fakeout_overall,
    cooldown_overall
  );
}
