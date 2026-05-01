# Audio Reactivity Setup

## System Dependencies

To enable audio reactivity, you need to install system audio libraries:

### Arch Linux

```bash
sudo pacman -S pkg-config alsa-lib libpulse pipewire-pulse
```

### Ubuntu/Debian

```bash
sudo apt install pkg-config libasound2-dev libpulse-dev
```

### Fedora

```bash
sudo dnf install pkg-config alsa-lib-devel pulseaudio-libs-devel
```

## Building

After installing dependencies:

```bash
cargo build --release
```

Chroma always builds with audio support.

## How Audio Reactivity Works

### Audio Features Detected:

- **Bass** (20-250 Hz): Deep low frequencies
- **Mid** (250-2000 Hz): Vocals, most instruments
- **Treble** (2000-8000 Hz): High frequencies, cymbals
- **Overall Energy**: Weighted mix (40% bass + 30% mid + 30% treble)
- **Overall Volume**: Total amplitude level
- **Beat Detection**: Sudden bass increases
- **Bass Drop**: Major bass spike detection

### Dynamic Energy Response:

The shader continuously adapts to the song's energy level:

- **🎹 Calm/Quiet sections** (low energy): Slower speed (0.2-0.5x), softer contrast, dimmer
- **🎸 Moderate sections** (medium energy): Normal speed (0.5-1.0x), balanced visuals
- **🔥 Intense/Loud sections** (high energy): Fast speed (1.0-2.0x), sharp contrast, bright

This means the animation naturally breathes with the music - slowing during quiet verses and exploding during choruses!

### Mappings to Shader Parameters:

| Audio Feature      | Affects        | Effect                                           |
| ------------------ | -------------- | ------------------------------------------------ |
| **Bass**           | Amplitude      | Makes patterns bigger/more intense               |
| **Bass**           | Distortion     | Warps the pattern                                |
| **Mid**            | Frequency      | Changes pattern detail/density                   |
| **Treble**         | Speed Boost    | Extra speed on high frequencies                  |
| **Overall Energy** | Speed          | 🎵 **Calm → slow (0.2x), Intense → fast (2.0x)** |
| **Overall Energy** | Contrast       | Softer on calm parts, sharp on intense           |
| **Treble**         | Color Shift    | Colors cycle faster with energy                  |
| **Beat**           | Noise Strength | Adds texture on beats (scaled by energy)         |
| **Bass Drop**      | Visual Effect  | Triggers Circle/Cross/etc explosion              |
| **Overall Volume** | Brightness     | Dimmer on calm, brighter on intense              |
| **Silence**        | All params     | Gradually stops and fades out                    |

### Silence Detection:

When no audio is playing (volume < 2%), the shader automatically:

- **Fades amplitude** back to 0.4 (minimal baseline)
- **Reduces distortion** to near-zero
- **Normalizes frequency** to 6.0
- **Slows down speed** to complete stop (→ 0.0) 🎬
- **Dims brightness** to 0.6
- **Reduces contrast** to 0.8
- **Decays noise** smoothly

The animation uses **exponential decay** (88-92% per frame) to create a dramatic "wind down" effect. When music stops:

1. **First 2 seconds**: Visuals noticeably slow and dim
2. **After 3-4 seconds**: Nearly frozen in place
3. **When music returns**: Instant response! 🎵

This creates a powerful visual effect where the shader appears to "die down" during silence and **springs back to life** when audio returns.

## Controls

- **`A`** - Toggle audio reactivity on/off
- 🎵 indicator in status bar when audio is active

## Notes

- On Linux, Chroma first records directly from the default PulseAudio/PipeWire sink monitor using libpulse.
- If PulseAudio/PipeWire is not available, Chroma falls back to CPAL device selection.
- To inspect capture devices, run `chroma --list-audio-devices`.
- If automatic monitor capture fails, use `pavucontrol` and set Chroma's recording source to "Monitor of ...".
