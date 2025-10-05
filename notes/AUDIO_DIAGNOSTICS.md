# Audio Diagnostics Guide

## Status Bar Indicators

When audio reactivity is enabled (`A` key), the status bar shows:

| Indicator | Meaning                     | Action                            |
| --------- | --------------------------- | --------------------------------- |
| **🎵●**   | Audio flowing (active)      | Working! Visuals respond to audio |
| **🎵○**   | Listening but silent        | Device OK, no audio detected      |
| **🎵?**   | No data received            | Check audio routing (see below)   |
| **🎵✗**   | Audio initialization failed | Check dependencies                |
| (blank)   | Audio reactivity OFF        | Press `A` to enable               |

## Startup Messages

### ✓ Success Messages

```
🎵 Audio Reactivity Setup:
   Attempting to capture system audio...
   ✓ Audio device found: 44100 Hz
   ✓ Audio data flowing! (peak: 0.234)
   Audio reactivity ready. Press 'A' to toggle.
```

**Meaning**: Everything working! Audio is being captured.

### ⚠ Warning Messages

```
🎵 Audio Reactivity Setup:
   Attempting to capture system audio...
   ✓ Audio device found: 44100 Hz
   ⚠ WARNING: No audio data received!
   Audio device opened but no samples captured.
```

**Meaning**: Device found but not receiving audio. Common causes:

1. **Using microphone instead of system audio**

   - Solution: Configure audio loopback (see below)

2. **No audio currently playing**

   - Solution: Play some music and check if 🎵○ → 🎵●

3. **Wrong audio source selected**
   - Solution: Run `pavucontrol` and configure (see below)

### ✗ Error Messages

```
🎵 Audio Reactivity Setup:
   Attempting to capture system audio...
   ✗ Failed to initialize audio: No input device available
   Audio reactivity disabled.
```

**Meaning**: Cannot find any audio device. Solutions:

1. **Missing system dependencies**

   ```bash
   sudo pacman -S pkg-config alsa-lib
   cargo build --release --features audio
   ```

2. **No input devices configured**
   - Check: `arecord -l` (should list devices)
   - May need to configure PulseAudio/PipeWire

## Setting Up System Audio Capture

### For PulseAudio/PipeWire (Most Linux Systems)

The app captures from the **default input device**, which is usually a microphone. To capture **system audio** (music/videos), you need to route it through a "monitor" (loopback):

#### Method 1: pavucontrol (GUI - Recommended)

```bash
# Install if not present
sudo pacman -S pavucontrol

# Run
pavucontrol
```

1. Start playing some music
2. Go to **"Recording"** tab in pavucontrol
3. Find "term-shaders" or "ALSA plug-in"
4. Click dropdown → Select **"Monitor of [Your Audio Device]"**
   - Example: "Monitor of Built-in Audio Analog Stereo"

#### Method 2: pactl (Command Line)

```bash
# List available sources
pactl list sources short

# Look for one ending in ".monitor"
# Example: alsa_output.pci-0000_00_1f.3.analog-stereo.monitor

# Set as default
pactl set-default-source alsa_output.pci-0000_00_1f.3.analog-stereo.monitor
```

#### Method 3: Create Loopback Module

```bash
# Load loopback module
pactl load-module module-loopback latency_msec=1

# To make permanent, add to /etc/pulse/default.pa:
# load-module module-loopback latency_msec=1
```

## Testing Audio Flow

### 1. Check if device exists

```bash
arecord -l
# Should list at least one device
```

### 2. Test raw capture

```bash
# Record 5 seconds and check if file has data
arecord -d 5 -f cd test.wav
ls -lh test.wav  # Should be ~900KB if audio captured
```

### 3. Monitor in real-time

```bash
# Install and run audio visualizer
sudo pacman -S cava
cava
```

If `cava` shows audio bars moving, your audio routing is correct!

## Common Issues

### Issue: 🎵? (No data) but audio playing

**Cause**: Wrong input source selected

**Fix**:

1. Open `pavucontrol`
2. Go to "Recording" tab
3. Change source to "Monitor of..." while app is running

### Issue: Audio works but no visual response

**Cause**: Audio reactivity may be toggled off or influence parameters too low

**Fix**:

1. Press `A` to ensure it's enabled (check for 🎵 icon)
2. Check `debug.log` for audio levels
3. Default influence values should work - if not, may need to adjust in code

### Issue: Choppy/laggy audio visualization

**Cause**: High CPU usage or audio buffer issues

**Fix**:

1. Lower terminal size (smaller window = less rendering)
2. Reduce FPS target in code if needed
3. Check if other processes using CPU

### Issue: Audio works briefly then stops

**Cause**: Device was unplugged/changed or PulseAudio stream disconnected

**Fix**: Restart the application

## Debug Logging

Check `debug.log` for detailed audio information:

```bash
tail -f debug.log | grep -i audio
```

Look for:

- "Audio capture initialized successfully"
- "AUDIO: Silence" (when no audio)
- "BASS DROP detected!" (when bass drops)
- "AUDIO: Silence (vol=X.XXXX) - slowing to stop (speed=X.XXX)"

## Still Not Working?

1. **Verify build with audio feature**:

   ```bash
   cargo build --release --features audio
   # Check for audio-related compilation
   ```

2. **Check system audio server**:

   ```bash
   # Check if PulseAudio/PipeWire running
   pactl info
   ```

3. **Try simplest test first**:

   - Play YouTube video at high volume
   - Open pavucontrol
   - Set recording source to monitor
   - Watch for 🎵○ → 🎵● transition

4. **Post issue with**:
   - Startup messages
   - Contents of `debug.log`
   - Output of `pactl list sources short`
   - Your audio setup (PulseAudio/PipeWire/ALSA)
