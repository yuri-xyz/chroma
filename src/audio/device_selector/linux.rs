use cpal::Device;

use super::{is_device_usable, is_dummy_device, is_monitor_source};

const PACTL_BIN: &str = "pactl";
const PACTL_DEFAULT_SINK_KEY: &str = "Default Sink";
const PACTL_RUNNING_STATE: &str = "RUNNING";

fn run_command_capture(command: &str, args: &[&str]) -> Option<String> {
  let output = std::process::Command::new(command)
    .args(args)
    .output()
    .ok()?;
  if !output.status.success() {
    return None;
  }

  let stdout = String::from_utf8(output.stdout).ok()?;
  let trimmed = stdout.trim();
  if trimmed.is_empty() {
    return None;
  }

  Some(trimmed.to_string())
}

fn parse_default_sink_from_pactl_info(info_output: &str) -> Option<String> {
  info_output.lines().find_map(|line| {
    let (key, value) = line.split_once(':')?;
    if key.trim().eq_ignore_ascii_case(PACTL_DEFAULT_SINK_KEY) {
      let sink_name = value.trim();
      if sink_name.is_empty() {
        None
      } else {
        Some(sink_name.to_string())
      }
    } else {
      None
    }
  })
}

fn parse_short_list_name_and_state(line: &str) -> Option<(String, Option<String>)> {
  // `pactl list short <type>` is tab-delimited; split_whitespace handles both tabs and spaces.
  let fields: Vec<&str> = line.split_whitespace().collect();
  if fields.len() < 2 {
    return None;
  }

  let name = fields.get(1)?.trim();
  if name.is_empty() {
    return None;
  }

  let state = fields.last().map(|field| field.trim().to_string());
  Some((name.to_string(), state))
}

pub(super) fn get_linux_sink_candidates() -> Vec<String> {
  use std::collections::HashSet;

  let mut ordered = Vec::new();
  let mut seen = HashSet::new();

  let mut push_unique = |candidate: String| {
    let normalized = candidate.trim();
    if normalized.is_empty() {
      return;
    }

    let normalized = normalized.to_string();
    if seen.insert(normalized.to_lowercase()) {
      ordered.push(normalized);
    }
  };

  let default_sink = run_command_capture(PACTL_BIN, &["get-default-sink"]).or_else(|| {
    run_command_capture(PACTL_BIN, &["info"])
      .and_then(|output| parse_default_sink_from_pactl_info(&output))
  });

  if let Some(default_sink) = default_sink {
    push_unique(default_sink);
  }

  if let Some(short_sinks) = run_command_capture(PACTL_BIN, &["list", "short", "sinks"]) {
    let mut running_sinks = Vec::new();
    let mut other_sinks = Vec::new();

    for line in short_sinks.lines() {
      if let Some((sink_name, state)) = parse_short_list_name_and_state(line) {
        if state.as_deref() == Some(PACTL_RUNNING_STATE) {
          running_sinks.push(sink_name);
        } else {
          other_sinks.push(sink_name);
        }
      }
    }

    for sink_name in running_sinks.into_iter().chain(other_sinks) {
      push_unique(sink_name);
    }
  }

  ordered
}

fn linux_monitor_match_index(device_name: &str, sink_candidates: &[String]) -> Option<usize> {
  let device_name_lower = device_name.to_lowercase();

  sink_candidates
    .iter()
    .enumerate()
    .find_map(|(index, sink_name)| {
      let sink_name_lower = sink_name.to_lowercase();
      let monitor_name = format!("{}.monitor", sink_name_lower);

      let matches_sink =
        device_name_lower == monitor_name || device_name_lower.contains(&monitor_name);

      if matches_sink {
        Some(index)
      } else {
        None
      }
    })
}

pub(super) fn find_linux_default_monitor_device_for_sinks(
  devices: &[(Device, String)],
  sink_candidates: &[String],
) -> Option<Device> {
  devices
    .iter()
    .filter(|(device, name)| {
      !is_dummy_device(name) && is_monitor_source(name) && is_device_usable(device)
    })
    .filter_map(|(device, name)| {
      linux_monitor_match_index(name, sink_candidates).map(|priority| (priority, device))
    })
    .min_by_key(|(priority, _)| *priority)
    .map(|(_, device)| device.clone())
}

pub(super) fn find_linux_default_monitor_device(devices: &[(Device, String)]) -> Option<Device> {
  let sink_candidates = get_linux_sink_candidates();
  if sink_candidates.is_empty() {
    return None;
  }

  find_linux_default_monitor_device_for_sinks(devices, &sink_candidates)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse_default_sink_from_pactl_info_reads_default_sink() {
    let info = r#"
Server String: /tmp/pulse
Default Sink: alsa_output.usb-Device.analog-stereo
Default Source: alsa_input.usb-Device.analog-stereo
"#;

    let parsed = parse_default_sink_from_pactl_info(info);

    assert_eq!(
      parsed.as_deref(),
      Some("alsa_output.usb-Device.analog-stereo")
    );
  }

  #[test]
  fn parse_short_list_name_and_state_reads_sink_fields() {
    let line = "54\talsa_output.usb-Device.analog-stereo\tPipeWire\ts16le 2ch 48000Hz\tRUNNING";

    let parsed = parse_short_list_name_and_state(line);

    assert_eq!(
      parsed,
      Some((
        "alsa_output.usb-Device.analog-stereo".to_string(),
        Some("RUNNING".to_string())
      ))
    );
  }

  #[test]
  fn linux_monitor_match_index_matches_exact_monitor_name() {
    let sink_candidates = vec![
      "alsa_output.pci-0000_00_1f.3.analog-stereo".to_string(),
      "alsa_output.usb-Device.analog-stereo".to_string(),
    ];

    let matched = linux_monitor_match_index(
      "alsa_output.usb-Device.analog-stereo.monitor",
      &sink_candidates,
    );

    assert_eq!(matched, Some(1));
  }

  #[test]
  fn linux_monitor_match_index_avoids_partial_sink_name_collisions() {
    let sink_candidates = vec![
      "alsa_output.usb-Device.analog".to_string(),
      "alsa_output.usb-Device.analog-pro".to_string(),
    ];

    let matched = linux_monitor_match_index(
      "alsa_output.usb-Device.analog-pro.monitor",
      &sink_candidates,
    );

    assert_eq!(matched, Some(1));
  }
}
