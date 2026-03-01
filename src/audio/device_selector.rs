use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{Device, Host};

#[cfg(target_os = "linux")]
const PACTL_BIN: &str = "pactl";
#[cfg(target_os = "linux")]
const PACTL_DEFAULT_SINK_KEY: &str = "Default Sink";
#[cfg(target_os = "linux")]
const PACTL_RUNNING_STATE: &str = "RUNNING";

/// Helper to get device name using the new cpal 0.17+ API
fn get_device_name(device: &Device) -> Option<String> {
  device
    .description()
    .ok()
    .map(|desc| desc.name().to_string())
}

/// Keywords that identify system audio monitor/loopback sources
const MONITOR_KEYWORDS: &[&str] = &[
  // Linux PulseAudio/PipeWire monitor sources
  ".monitor",
  "Monitor of",
  "monitor",
  // macOS loopback devices (virtual audio drivers)
  "BlackHole",
  "Soundflower",
  "Loopback",
  "Multi-Output",
  // macOS ScreenCaptureKit
  "ScreenCapture",
  // macOS CoreAudio loopback (cpal 0.17+ on macOS 14.2+)
  "default-output",
  // Windows
  "Stereo Mix",
  "What U Hear",
  "Wave Out",
];

/// Keywords that identify microphones (to exclude)
const MICROPHONE_KEYWORDS: &[&str] = &[
  "Microphone",
  "microphone",
  "Mic ",
  " Mic",
  "Internal Mic",
  "Built-in Mic",
  "Webcam",
  "Camera",
  "MacBook Pro Microphone",
  "MacBook Air Microphone",
  "iPhone Microphone",
  "Headset",
];

/// Check if a device name indicates a system audio monitor/loopback source
fn is_monitor_source(name: &str) -> bool {
  let name_lower = name.to_lowercase();
  MONITOR_KEYWORDS
    .iter()
    .any(|keyword| name_lower.contains(&keyword.to_lowercase()))
}

/// Check if a device name indicates a microphone
fn is_microphone(name: &str) -> bool {
  if is_monitor_source(name) {
    return false;
  }
  let name_lower = name.to_lowercase();
  MICROPHONE_KEYWORDS
    .iter()
    .any(|keyword| name_lower.contains(&keyword.to_lowercase()))
}

/// Check if a device can actually be configured for input
/// On macOS 14.2+, output devices can also be used as loopback inputs
fn is_device_usable(device: &Device) -> bool {
  // First try input config (normal input devices)
  if device.default_input_config().is_ok() {
    return true;
  }
  // On macOS, output devices can be used for loopback (cpal 0.17+ on macOS 14.2+)
  #[cfg(target_os = "macos")]
  if device.default_output_config().is_ok() {
    return true;
  }
  false
}

// Check if a device is a dummy (null) device
fn is_dummy_device(name: &str) -> bool {
  let name = name.to_lowercase();
  name.contains("discard")
    || name.contains("dummy")
    || name.contains("null")
    || name.contains("zero samples")
}

#[cfg(target_os = "linux")]
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

#[cfg(target_os = "linux")]
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

#[cfg(target_os = "linux")]
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

#[cfg(target_os = "linux")]
fn get_linux_sink_candidates() -> Vec<String> {
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

#[cfg(target_os = "linux")]
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

#[cfg(target_os = "linux")]
fn find_linux_default_monitor_device_for_sinks(
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

#[cfg(target_os = "linux")]
fn find_linux_default_monitor_device(devices: &[(Device, String)]) -> Option<Device> {
  let sink_candidates = get_linux_sink_candidates();
  if sink_candidates.is_empty() {
    return None;
  }

  find_linux_default_monitor_device_for_sinks(devices, &sink_candidates)
}

/// Try to get the best host for system audio capture
fn get_best_host() -> Host {
  // Try to find a host that supports loopback/screen capture
  #[cfg(target_os = "macos")]
  {
    // On macOS, try ScreenCaptureKit host first if available
    for host_id in cpal::available_hosts() {
      let host_name = format!("{:?}", host_id);
      if host_name.contains("ScreenCapture") {
        if let Ok(host) = cpal::host_from_id(host_id) {
          return host;
        }
      }
    }
  }

  // Fall back to default host
  cpal::default_host()
}

/// Find a specific audio device by name (partial match)
pub fn find_device_by_name(host: &Host, device_name: &str) -> anyhow::Result<Device> {
  let search_term = device_name.to_lowercase();

  // Check input devices first
  if let Ok(devices) = host.input_devices() {
    for device in devices {
      if let Some(name) = get_device_name(&device) {
        if name.to_lowercase().contains(&search_term) || name.contains(device_name) {
          return Ok(device);
        }
      }
    }
  }

  // Check output devices as fallback
  if let Ok(devices) = host.output_devices() {
    for device in devices {
      if let Some(name) = get_device_name(&device) {
        if name.to_lowercase().contains(&search_term) || name.contains(device_name) {
          return Ok(device);
        }
      }
    }
  }

  Err(anyhow::anyhow!(
    "Audio device '{}' not found. Run with --list-audio-devices to see available devices.",
    device_name
  ))
}

/// Automatically find the best system audio device
/// Priority: monitor sources > loopback devices > non-microphone inputs
pub fn find_system_audio_device(host: &Host) -> anyhow::Result<Device> {
  let devices: Vec<(Device, String)> = host
    .input_devices()
    .map(|devs| {
      devs
        .filter_map(|d| get_device_name(&d).map(|name| (d, name)))
        .collect()
    })
    .unwrap_or_default();

  #[cfg(target_os = "linux")]
  if let Some(device) = find_linux_default_monitor_device(&devices) {
    return Ok(device);
  }

  // Priority 1: Find explicit monitor/loopback source that is actually usable
  for (device, name) in &devices {
    if !is_dummy_device(name) && is_monitor_source(name) && is_device_usable(device) {
      return Ok(device.clone());
    }
  }

  // Priority 2: On macOS 14.2+, try CoreAudio loopback
  // cpal 0.17+ supports loopback by treating output devices as input sources
  // We don't check is_device_usable() here because the loopback capability
  // is verified when building the stream, not when getting the device
  #[cfg(target_os = "macos")]
  {
    if let Some(output_device) = host.default_output_device() {
      // Return the output device for loopback - cpal will handle the rest
      return Ok(output_device);
    }

    // No output device available
    Err(anyhow::anyhow!(
      "No audio output device found for system audio loopback."
    ))
  }

  // Priority 2 (non-macOS): Find any usable device that's NOT a microphone
  #[cfg(not(target_os = "macos"))]
  for (device, name) in &devices {
    if !is_dummy_device(name) && !is_microphone(name) && is_device_usable(device) {
      return Ok(device.clone());
    }
  }

  // On Linux/other platforms, fall back to default device if usable
  #[cfg(not(target_os = "macos"))]
  {
    if let Some(device) = host.default_input_device() {
      let is_default_device_dummy =
        get_device_name(&device).is_some_and(|name| is_dummy_device(&name));
      if !is_default_device_dummy && is_device_usable(&device) {
        return Ok(device);
      }
    }
    Err(anyhow::anyhow!("No usable audio input device available"))
  }
}

/// Try to automatically find system audio across all available hosts
pub fn find_system_audio_auto() -> anyhow::Result<(Host, Device)> {
  #[cfg(target_os = "linux")]
  {
    let sink_candidates = get_linux_sink_candidates();
    if !sink_candidates.is_empty() {
      for host_id in cpal::available_hosts() {
        if let Ok(host) = cpal::host_from_id(host_id) {
          if let Ok(devices) = host.input_devices().map(|devs| {
            devs
              .filter_map(|device| get_device_name(&device).map(|name| (device, name)))
              .collect::<Vec<_>>()
          }) {
            if let Some(device) =
              find_linux_default_monitor_device_for_sinks(&devices, &sink_candidates)
            {
              return Ok((host, device));
            }
          }
        }
      }
    }
  }

  // First, try all available hosts to find a dedicated monitor source
  // (e.g., PipeWire monitor on Linux, BlackHole on macOS)
  for host_id in cpal::available_hosts() {
    if let Ok(host) = cpal::host_from_id(host_id) {
      if let Ok(devices) = host.input_devices() {
        for device in devices {
          if let Some(name) = get_device_name(&device) {
            if !is_dummy_device(&name) && is_monitor_source(&name) && is_device_usable(&device) {
              return Ok((host, device));
            }
          }
        }
      }
    }
  }

  // No dedicated monitor source found - use platform-specific fallback
  // On macOS: uses output device for loopback (cpal 0.17+ on macOS 14.2+)
  // On Linux: uses default input device
  let host = get_best_host();
  let device = find_system_audio_device(&host)?;
  Ok((host, device))
}

/// List all available audio devices across all hosts
pub fn list_devices(host: &Host) -> anyhow::Result<()> {
  println!("\n=== Available Audio Hosts ===");
  for host_id in cpal::available_hosts() {
    let marker = if host_id == host.id() {
      " ← ACTIVE"
    } else {
      ""
    };
    println!("  {:?}{}", host_id, marker);
  }

  println!("\n=== Input Devices (Host: {:?}) ===", host.id());

  if let Ok(devices) = host.input_devices() {
    let devices: Vec<_> = devices.collect();

    if devices.is_empty() {
      println!("  (none)");
    } else {
      let default_name = host
        .default_input_device()
        .and_then(|d| get_device_name(&d));

      for device in &devices {
        if let Some(name) = get_device_name(device) {
          let is_default = default_name.as_ref() == Some(&name);
          let device_type = if is_monitor_source(&name) {
            " [SYSTEM AUDIO]"
          } else if is_microphone(&name) {
            " [MICROPHONE]"
          } else {
            ""
          };
          let marker = if is_default { " ← DEFAULT" } else { "" };
          println!("  • {}{}{}", name, device_type, marker);
        }
      }
    }
  }

  println!("\n=== Output Devices ===");

  let default_output_name = host
    .default_output_device()
    .and_then(|d| get_device_name(&d));

  if let Ok(devices) = host.output_devices() {
    let devices: Vec<_> = devices.collect();

    if devices.is_empty() {
      println!("  (none)");
    } else {
      for device in &devices {
        if let Some(name) = get_device_name(device) {
          let is_default = default_output_name.as_ref() == Some(&name);
          #[cfg(target_os = "macos")]
          let loopback_marker = if is_default { " [LOOPBACK SOURCE]" } else { "" };
          #[cfg(not(target_os = "macos"))]
          let loopback_marker = "";
          let default_marker = if is_default { " ← DEFAULT" } else { "" };
          println!("  • {}{}{}", name, loopback_marker, default_marker);
        }
      }
    }
  }

  // Check if we found any system audio sources in input devices
  let has_monitor = host.input_devices().is_ok_and(|devs| {
    devs
      .filter_map(|d| get_device_name(&d))
      .any(|n| is_monitor_source(&n))
  });

  // On macOS, output device loopback is available even without monitor sources
  #[cfg(target_os = "macos")]
  let has_loopback = default_output_name.is_some();
  #[cfg(not(target_os = "macos"))]
  let has_loopback = false;

  if !has_monitor && !has_loopback {
    println!("\n⚠️  No system audio source detected!");
    println!("\nTo capture system audio:");

    #[cfg(target_os = "linux")]
    {
      println!("  Linux: Monitor sources should appear automatically with PipeWire/PulseAudio");
      println!("         Check: pactl list sources | grep -i monitor");
    }
  } else if !has_monitor && has_loopback {
    #[cfg(target_os = "macos")]
    {
      println!("\n✓ System audio will use output device loopback (macOS 14.2+)");
      println!("  For better compatibility, install BlackHole: https://github.com/ExistentialAudio/BlackHole");
    }
  }

  println!("\nTo manually specify a device:");
  println!("  chroma --audio-device \"DEVICE_NAME\"");

  Ok(())
}

#[cfg(all(test, target_os = "linux"))]
mod linux_tests {
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
