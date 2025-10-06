use cpal::traits::{DeviceTrait, HostTrait};
use cpal::Device;
use std::fs::OpenOptions;
use std::io::Write;

/// Logger for audio device detection
struct AudioLogger {
  file: Option<std::fs::File>,
}

impl AudioLogger {
  fn new() -> Self {
    let file = OpenOptions::new()
      .create(true)
      .append(true)
      .open("audio_debug.log")
      .ok();

    Self { file }
  }

  fn log(&mut self, message: &str) {
    if let Some(ref mut f) = self.file {
      writeln!(f, "{}", message).ok();
    }
  }
}

/// Find a specific audio device by name (partial match)
pub fn find_device_by_name(host: &cpal::Host, device_name: &str) -> anyhow::Result<Device> {
  let mut logger = AudioLogger::new();
  logger.log(&format!("Looking for device: {}", device_name));

  // Check input devices first (where monitor sources appear)
  if let Ok(devices) = host.input_devices() {
    for device in devices {
      if let Ok(name) = device.name() {
        if name == device_name || name.contains(device_name) {
          logger.log(&format!("✓ Found device: {}", name));

          return Ok(device);
        }
      }
    }
  }

  // Check output devices as fallback
  if let Ok(devices) = host.output_devices() {
    for device in devices {
      if let Ok(name) = device.name() {
        if name == device_name || name.contains(device_name) {
          logger.log(&format!("⚠️  Found output device: {}", name));
          logger.log("Note: This is an output device. You need its monitor source instead.");

          return Ok(device);
        }
      }
    }
  }

  Err(anyhow::anyhow!(
    "Audio device '{}' not found.\nRun with --list-audio-devices to see available devices.",
    device_name
  ))
}

/// Find the default input device (simple approach)
pub fn find_system_audio_device(host: &cpal::Host) -> anyhow::Result<Device> {
  let mut logger = AudioLogger::new();
  logger.log("\n=== Audio Device Detection ===");

  // Get default input device
  let device = host
    .default_input_device()
    .ok_or_else(|| anyhow::anyhow!("No input device available. Check system audio settings."))?;

  if let Ok(name) = device.name() {
    logger.log(&format!("Using default input device: {}", name));
  }

  Ok(device)
}

/// List all available audio devices
pub fn list_devices(host: &cpal::Host) -> anyhow::Result<()> {
  println!("\n=== Audio Devices (Host: {:?}) ===\n", host.id());
  println!("Input Devices:");

  if let Ok(devices) = host.input_devices() {
    let devices: Vec<_> = devices.collect();

    if devices.is_empty() {
      println!("  (none)");
    } else {
      for (index, device) in devices.iter().enumerate() {
        if let Ok(name) = device.name() {
          let is_default = host
            .default_input_device()
            .and_then(|d| d.name().ok())
            .as_ref()
            == Some(&name);

          let marker = if is_default { " ← DEFAULT" } else { "" };

          println!("  [{}] {}{}", index, name, marker);
        }
      }
    }
  } else {
    println!("  (error reading devices)");
  }

  println!("\nOutput Devices:");

  if let Ok(devices) = host.output_devices() {
    let devices: Vec<_> = devices.collect();

    if devices.is_empty() {
      println!("  (none)");
    } else {
      for (index, device) in devices.iter().enumerate() {
        if let Ok(name) = device.name() {
          let is_default = host
            .default_output_device()
            .and_then(|d| d.name().ok())
            .as_ref()
            == Some(&name);

          let marker = if is_default { " ← DEFAULT" } else { "" };
          println!("  [{}] {}{}", index, name, marker);
        }
      }
    }
  } else {
    println!("  (error reading devices)");
  }

  if let Some(default_input) = host.default_input_device() {
    if let Ok(name) = default_input.name() {
      println!("\nDefault Input: {}", name);
    }
  }

  if let Some(default_output) = host.default_output_device() {
    if let Ok(name) = default_output.name() {
      println!("Default Output: {}", name);
    }
  }

  println!("\nTo use a specific device, run:");
  println!("  cargo run --features audio -- --audio-device \"DEVICE_NAME\"");

  Ok(())
}
