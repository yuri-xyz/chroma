// Application initialization and diagnostics

#[cfg(feature = "audio")]
use crate::constants::AUDIO_TEST_DURATION_MS;
#[cfg(feature = "audio")]
use std::time::Duration;
#[cfg(feature = "audio")]
use term_shaders::audio::AudioCapture;

/// Run audio diagnostics and print results
#[cfg(feature = "audio")]
pub fn run_audio_diagnostics() -> anyhow::Result<()> {
  println!("🎵 Audio Reactivity Diagnostics:");
  println!("   Checking audio system...");

  match AudioCapture::new() {
    Ok(capture) => {
      println!("   ✓ Audio device found: {} Hz", capture.sample_rate);

      // Test if we're receiving audio data
      std::thread::sleep(Duration::from_millis(AUDIO_TEST_DURATION_MS));
      let test_samples = capture.get_samples();

      if test_samples.is_empty() {
        print_audio_warning();
        println!("   Continuing anyway (audio will work once configured)...");
      } else {
        let max_sample = test_samples.iter().map(|s| s.abs()).fold(0.0f32, f32::max);
        if max_sample > 0.02 {
          println!("   ✓ Audio data flowing! (peak: {:.3})", max_sample);
          println!("   Audio reactivity ready. Press 'A' to toggle.");
        } else {
          println!("   ✓ Audio system working (currently silent)");
          println!("   Play some audio to test reactivity. Press 'A' to toggle.");
        }
      }

      Ok(())
    }
    Err(e) => {
      print_audio_error(&e);
      Err(e)
    }
  }
}

#[cfg(feature = "audio")]
fn print_audio_warning() {
  println!("   ⚠ WARNING: No audio data received!");
  println!("   Audio device opened but no samples captured.");
  println!("   This usually means:");
  println!("     • Using microphone input (need loopback for system audio)");
  println!("     • No audio currently playing");
  println!("     • Need to configure PulseAudio/PipeWire monitor");
  println!();
  println!("   Quick fix: pavucontrol → Recording → Select 'Monitor of...'");
  println!("   See AUDIO_DIAGNOSTICS.md for detailed troubleshooting.");
  println!();
}

#[cfg(feature = "audio")]
fn print_audio_error(e: &anyhow::Error) {
  let error_str = e.to_string();

  eprintln!("   ✗ Failed to initialize audio");
  eprintln!("   Error: {}", e);
  eprintln!();

  // Check for common error patterns
  if error_str.contains("No such file") || error_str.contains("cannot find card") {
    eprintln!("   This looks like: No audio hardware found");
    eprintln!("     • Running in VM/container without audio passthrough?");
    eprintln!("     • No sound card available?");
    eprintln!("     • ALSA not configured?");
  } else if error_str.contains("no longer available") || error_str.contains("unplugged") {
    eprintln!("   This looks like: Audio device not available");
    eprintln!("     • Check: arecord -l");
    eprintln!("     • Ensure audio hardware is connected");
  }

  eprintln!();
  eprintln!("   You have two options:");
  eprintln!("     1. Fix audio setup (see AUDIO_SETUP.md)");
  eprintln!("     2. Build without audio: cargo build --release");
  eprintln!();
  eprintln!("   If you just want to test visuals without audio, use option 2.");
  eprintln!();
}

/// Print initialization message for non-audio builds
#[cfg(not(feature = "audio"))]
pub fn print_no_audio_message() {
  println!("   Audio reactivity: Not enabled");
  println!("   To enable: cargo build --release --features audio");
  println!();
}
