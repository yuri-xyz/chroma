#![allow(dead_code)]
#![cfg(feature = "audio")]

use chroma::audio::AudioFeatures;

use crate::support::audio_fixtures::{AnalyzerFrame, FixtureSegment, ANALYSIS_WINDOW};

pub fn segment_frames<'a>(
  trace: &'a [AnalyzerFrame],
  segment: &FixtureSegment,
) -> Vec<&'a AnalyzerFrame> {
  trace
    .iter()
    .filter(|frame| {
      frame.sample_end > segment.start_sample && frame.sample_end <= segment.end_sample
    })
    .collect()
}

pub fn trailing_segment_frames<'a>(
  trace: &'a [AnalyzerFrame],
  segment: &FixtureSegment,
) -> Vec<&'a AnalyzerFrame> {
  let focus_start = (segment.start_sample + segment.len() / 2)
    .max(segment.end_sample.saturating_sub(ANALYSIS_WINDOW));

  trace
    .iter()
    .filter(|frame| frame.sample_end > focus_start && frame.sample_end <= segment.end_sample)
    .collect()
}

pub fn segment_max(
  trace: &[AnalyzerFrame],
  segment: &FixtureSegment,
  selector: impl Fn(&AudioFeatures) -> f32,
) -> f32 {
  trailing_segment_frames(trace, segment)
    .into_iter()
    .map(|frame| selector(&frame.features))
    .fold(0.0_f32, f32::max)
}

pub fn segment_has_drop(trace: &[AnalyzerFrame], segment: &FixtureSegment) -> bool {
  segment_frames(trace, segment)
    .into_iter()
    .any(|frame| frame.features.is_drop)
}

pub fn trailing_segment_has_drop(trace: &[AnalyzerFrame], segment: &FixtureSegment) -> bool {
  trailing_segment_frames(trace, segment)
    .into_iter()
    .any(|frame| frame.features.is_drop)
}

#[allow(dead_code)]
pub fn segment_drop_count(trace: &[AnalyzerFrame], segment: &FixtureSegment) -> usize {
  segment_frames(trace, segment)
    .into_iter()
    .filter(|frame| frame.features.is_drop)
    .count()
}

pub fn segment_drop_onset_count(trace: &[AnalyzerFrame], segment: &FixtureSegment) -> usize {
  let mut previous_is_drop = false;
  let mut drop_onsets = 0;

  for frame in trace {
    let in_segment =
      frame.sample_end > segment.start_sample && frame.sample_end <= segment.end_sample;

    if in_segment && frame.features.is_drop && !previous_is_drop {
      drop_onsets += 1;
    }

    previous_is_drop = frame.features.is_drop;
  }

  drop_onsets
}

pub fn trailing_segment_average(
  trace: &[AnalyzerFrame],
  segment: &FixtureSegment,
  selector: impl Fn(&AudioFeatures) -> f32,
) -> f32 {
  let frames = trailing_segment_frames(trace, segment);
  let frame_count = frames.len();

  assert!(
    frame_count > 0,
    "expected fixture segment to produce analyzer frames"
  );

  frames
    .into_iter()
    .map(|frame| selector(&frame.features))
    .sum::<f32>()
    / frame_count as f32
}
