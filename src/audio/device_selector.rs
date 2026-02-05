use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{Device, Host};

/// Helper to get device name using the new cpal 0.17+ API
fn get_device_name(device: &Device) -> Option<String> {
  device.description().ok().map(|desc| desc.name().to_string())
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
    return Err(anyhow::anyhow!(
      "No audio output device found for system audio loopback."
    ));
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
      if is_device_usable(&device) {
        return Ok(device);
      }
    }
    Err(anyhow::anyhow!("No usable audio input device available"))
  }
}

/// Try to automatically find system audio across all available hosts
pub fn find_system_audio_auto() -> anyhow::Result<(Host, Device)> {
  // First, try all available hosts to find a dedicated monitor source
  // (e.g., PipeWire monitor on Linux, BlackHole on macOS)
  for host_id in cpal::available_hosts() {
    if let Ok(host) = cpal::host_from_id(host_id) {
      if let Ok(devices) = host.input_devices() {
        for device in devices {
          if let Some(name) = get_device_name(&device) {
            if is_monitor_source(&name) && is_device_usable(&device) {
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
      let default_name = host.default_input_device().and_then(|d| get_device_name(&d));

      for device in &devices {
        if let Some(name) = get_device_name(&device) {
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

  let default_output_name = host.default_output_device().and_then(|d| get_device_name(&d));

  if let Ok(devices) = host.output_devices() {
    let devices: Vec<_> = devices.collect();

    if devices.is_empty() {
      println!("  (none)");
    } else {
      for device in &devices {
        if let Some(name) = get_device_name(&device) {
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
  let has_monitor = host.input_devices().map_or(false, |devs| {
    devs.filter_map(|d| get_device_name(&d)).any(|n| is_monitor_source(&n))
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
