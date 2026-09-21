# Audio Setup

Chroma listens to what your system is playing, not to a microphone. Audio support is always built in, so there is nothing to enable: if Chroma can capture your system audio, the visuals react to it.

This page covers the libraries you need, how capture works on each platform, and how sound is turned into motion.

### System libraries on Linux

Building Chroma on Linux needs the ALSA and PulseAudio development packages. PipeWire users need its PulseAudio compatibility layer, which most distributions install by default.

| Distribution | Command |
| --- | --- |
| Arch Linux | `sudo pacman -S pkg-config alsa-lib libpulse pipewire-pulse` |
| Ubuntu, Debian | `sudo apt install pkg-config libasound2-dev libpulse-dev` |
| Fedora | `sudo dnf install pkg-config alsa-lib-devel pulseaudio-libs-devel` |

Then build as usual with `cargo build --release`. The [Nix flake](./NIX.md) provides all of these libraries for you.

Older instructions used `cargo build --release --features audio`. That command still works: the `audio` feature is kept as an empty placeholder so existing build scripts do not break, and it changes nothing.

### How audio is captured

| Platform | Capture method |
| --- | --- |
| Linux | Records the monitor of the default PulseAudio or PipeWire output through libpulse. If no sound server is reachable, falls back to CPAL device selection. If the sound server restarts, Chroma reconnects on its own. |
| Windows | WASAPI loopback of the default output device. No "Stereo Mix" input has to be enabled. |
| macOS | CPAL loopback capture of the output device on recent macOS releases. Older systems can use a virtual loopback driver such as BlackHole, which Chroma recognises by name. |

To capture a different device, list what Chroma can see and pass a name:

```bash
chroma --list-audio-devices
chroma --audio-device "Monitor of Built-in Audio"
```

On Windows, `--audio-device` takes the name of an output device to loop back.

### When nothing reacts

Start with `chroma --list-audio-devices` to confirm that a monitor or loopback device is visible. On Linux, if automatic capture picks the wrong source, open `pavucontrol` while Chroma is running, find Chroma on the Recording tab, and set its source to "Monitor of" your output device.

Reactivity follows the level of the captured signal. Very quiet playback produces calm, slow visuals; turning the player up makes them livelier.

### What Chroma hears

Each frame, the captured samples are run through a Hann-windowed FFT (2048 samples, advancing 512 at a time) and reduced to a handful of features:

| Feature | Meaning |
| --- | --- |
| Bass | Energy from 20 to 250 Hz: kick drums and basslines |
| Mid | Energy from 250 to 2000 Hz: vocals and most instruments |
| Treble | Energy from 2000 to 8000 Hz: cymbals, hi-hats, and melodic sparkle |
| Overall | The general loudness of the signal |
| Beat strength | A pulse that fires on sudden rises in bass |
| Bass drop | A rare, large bass spike, limited to one per second |

Chroma also derives a weighted energy of 10% bass, 30% mid, and 60% treble. It leans on treble deliberately so that melodies stay visible instead of being buried under the bassline.

### How sound drives the picture

| Sound | Parameter | Effect |
| --- | --- | --- |
| Bass | Amplitude | Patterns swell on low notes |
| Bass | Distortion | The pattern warps with the bassline |
| Bass and beats | Saturation | Colours become more vivid on hits |
| Mid | Frequency | Pattern detail and density increase |
| Weighted energy | Speed | Calm passages drift, intense passages race |
| Treble | Speed | High notes add an extra push |
| Treble | Colour shift | Colours cycle faster |
| Treble and overall | Brightness | Loud, bright passages glow, capped to avoid washing out |
| Weighted energy and treble | Contrast | Soft in calm passages, sharp in intense ones |
| Beat | Noise, distortion, zoom | A short textured "pop" and a zoom pulse about the screen centre |
| Bass drop | Beat effect | The selected effect fires with a stronger distortion and zoom |

The result is an animation that breathes with the music: it slows and softens through a quiet verse and surges through a chorus.

Three parameters set how hard sound pushes. `--bass-influence` scales the amplitude and distortion response, `--mid-influence` scales the frequency response, and `--treble-influence` scales the speed boost. `--beat-sensitivity` lowers or raises the threshold for what counts as a beat. All of them are described in [Parameters](./PARAMETERS.md).

Most responses are smoothed, and the smoothing is scaled by the real frame time, so the visuals react at the same pace whatever `--fps` you choose.

### Silence

When the signal falls below 2% of full scale, Chroma winds the animation down instead of freezing it abruptly. Amplitude eases to `0.4`, frequency to `6.0`, brightness to `0.6`, and contrast to `0.8`, while distortion, noise, and speed decay towards zero.

In practice the picture visibly slows and dims within about two seconds and is almost still after three or four. The moment audio returns it springs back, because the reactive values are recomputed from the very next frame.

A single stray click does not count as sound: both the peak and the average level of a batch of samples must rise above the threshold.

### Knowing that audio is detected

While sound is playing, the status bar changes from plain black on white to a moving pastel gradient, and music symbols drift across it. See [Controls](./CONTROLS.md).
