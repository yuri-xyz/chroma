# Architecture

Chroma does one thing sixty times a second: it asks the GPU to paint a small image, turns that image into characters, and writes the characters to the terminal. Around that loop sit three sources of change, namely audio, the config file, and the keyboard, all of which feed a single struct of parameters.

This page follows a frame through the program and then maps the source tree. For contributor rules and conventions, see [Contributing](../CONTRIBUTING.md).

### Design principles

Chroma stays deliberately direct. There is no TUI framework; frames are written straight to stdout as ANSI text. There are no interactive menus; a look is described by plain parameters that can come from a preset, a file, a flag, or a key press. The GPU does the expensive per-pixel work, and the CPU code in the frame loop avoids per-cell allocation, blocking I/O, and logging.

Stdout belongs to the picture. Nothing in a runtime path may print to stdout or stderr, because a stray line corrupts the display. Diagnostics go to a file-backed debug log instead, which is only written by debug builds.

### The shape of the program

```
  audio capture ──► FFT analysis ──┐
  config file ──► watcher thread ──┼──► ShaderParams ──► ShaderUniforms
  keyboard ────────────────────────┤                          │
  preset cycle ────────────────────┘                          ▼
                                                     GPU compute shader
                                                              │
                                                      RGBA pixel buffer
                                                              │
                                                     brightness ► glyph
                                                              │
                                                       RenderedFrame
                                                              │
                                        terminal (ANSI)  or  --stream output
```

`ShaderParams` is the hub. Everything that can change the picture writes to it, and the renderer only ever reads from it.

### One frame

Each pass through `App::run` in `src/app/mod.rs` does the following.

1. In the interactive UI, check whether the terminal was resized and handle any pending key press. Stream mode skips both.
2. Advance the animation clock by the real frame time multiplied by `speed`.
3. Drain the captured audio samples, analyse them, and let the result drive the parameters. With no signal, the parameters decay towards a calm resting state instead.
4. Apply a reloaded config if the watcher thread has delivered one, then switch presets if a `--preset-interval` has elapsed.
5. Convert `ShaderParams` into the `ShaderUniforms` struct and upload it to the GPU.
6. Dispatch the compute shader with one invocation per character cell, in workgroups of 8×8.
7. Copy the output buffer into a staging buffer, map it, and convert the float RGBA values to bytes.
8. Map each pixel's brightness to a palette character and keep its colour.
9. Build a `RenderedFrame`, add the status bar, serialise it to one string, and write it to stdout in a single write. The pixel bytes and the output string live in buffers that are reused from frame to frame.
10. Sleep for whatever remains of the frame budget set by `--fps`.

The shader resolution is the terminal size in cells, less one row when the status bar is visible. An 80×24 terminal therefore renders just 1,840 pixels per frame, which is why the GPU work is rarely the bottleneck; terminal output usually is.

### The shader

The built-in shader is assembled at build time. `build.rs` concatenates the WGSL modules in `src/shader_common/` and `src/shader_patterns/` into one `compiled_shader.wgsl`, in a fixed order: uniforms and colour utilities first, then every pattern, then the effects and the `main` entry point. A new WGSL module has to be added to that list.

`main` in `src/shader_common/main.wgsl` computes a pattern-space coordinate for its cell, applies the beat zoom and distortion, dispatches on `pattern_type` to one pattern function, then applies the colour mode, the colour adjustments, the beat effect and flash, and finally the vignette.

Pattern space keeps the centre of the screen at `(0.5, 0.5)` for every pattern and every `scale`. Audio changes `frequency` on every frame, so tiled patterns multiply centred coordinates, never raw ones, by the frequency. Without that rule each beat would look like a zoom into the top-left corner. `VortexCorner` is the one deliberate exception, and `tests/pattern_zoom_test.rs` measures the zoom anchor on a real GPU.

The numeric IDs in the WGSL dispatch must match the order of the `PatternType` enum, and the `Uniforms` struct in `src/shader_common/uniforms.wgsl` must stay layout-compatible with `src/shader/uniforms.rs`. A shader passed with `--custom-shader` replaces the concatenated source entirely but uses the same uniforms and bindings.

### Parameters and their layers

A look is assembled in `src/main.rs` from layers: the audio-reactive defaults (optionally randomised) or a built-in preset, then the config file, then the command-line flags. The base layer is shared behind a mutex because preset cycling swaps it while the program runs, and a config reload that happens afterwards has to layer over the preset now showing rather than the one Chroma started with.

Loading merges a file over its base field by field, then clamps every value into range. `prepare_reloaded_params` carries the animation clock and resolution across a reload so the picture does not jump.

### Config reload

`src/app/config_watcher.rs` watches the config file's parent directory rather than the file itself, which is what makes editors that save by rename work. When the file changes, a background thread loads and validates it and places the result in a channel, replacing any older result that has not been picked up. The render loop takes it on the next frame. An invalid file produces no message, so the current look simply stays.

Preset cycling and reloading share a gate. While the render loop switches presets it holds the gate, discards any queued reload, and builds the new look itself. This ordering guarantees that a reload computed against the previous preset can never arrive late and undo the switch.

### Audio

Capture runs on its own thread and pushes samples into a bounded ring buffer; the render loop drains it once per frame. On Linux, `src/audio/pulse_capture.rs` records the default output's monitor through libpulse and reconnects if the sound server restarts. Elsewhere, and as the Linux fallback, `src/audio/capture.rs` uses CPAL, with `src/audio/device_selector.rs` choosing a monitor or loopback device.

`src/audio/analyzer.rs` runs a Hann-windowed FFT over 2048-sample windows (scaled up for capture rates above 48 kHz) and produces bass, mid, treble, overall level, beat strength, and bass-drop detection. `src/app/audio.rs` maps those features onto parameters. Its smoothing factors are scaled by the real frame time, so the response is the same at any `--fps`. [Audio Setup](./AUDIO_SETUP.md) describes the mapping from the user's point of view.

### Output

`src/ascii/` converts RGBA bytes into `(char, Color)` pairs using the active palette. `src/render/` wraps those in a `RenderedFrame`, which knows how to serialise itself for the terminal and for each stream format in `src/render/stream.rs`. Pixels darker than a small threshold become blank cells, and colour escape codes are only emitted when the colour changes from one cell to the next, which keeps frames compact.

Stream mode uses the same pipeline and only changes the last step. A closed pipe is treated as the consumer going away, and Chroma exits quietly.

### Source map

| Path | Responsibility |
| --- | --- |
| `src/main.rs` | Startup, parameter layering, preset cycling setup |
| `src/cli.rs`, `src/list_commands.rs` | Flags and the `--list-*` commands |
| `src/app/` | The render loop, input, audio reactivity, config watcher, preset cycling, status bar, frame output |
| `src/params/` | `ShaderParams`, the pattern, colour-mode, and palette enums, clamping, save and load, randomisation |
| `src/presets/` | The built-in presets `p0` to `p25` and the cycle timer |
| `src/shader/` | The wgpu pipeline and the uniform layout |
| `src/shader_common/`, `src/shader_patterns/` | WGSL sources |
| `src/audio/` | Capture, device selection, FFT analysis |
| `src/ascii/`, `src/render/` | Pixel-to-glyph conversion and frame serialisation |
| `src/terminal.rs`, `src/debug.rs`, `src/constants.rs` | Terminal setup, teardown, and the panic hook; the debug log; shared constants |
| `tests/` | Integration tests; the GPU tests are ignored by default |
| `benches/` | Criterion benchmarks for frame serialisation, GPU render and readback, and audio analysis |
| `examples/` | Preset configs as TOML files and the custom shader template |

### Failure handling

Problems found at startup stop the program with a clear message: no usable GPU, a shader that fails to compile, or a config file that cannot be parsed. Problems at runtime do not. A bad config edit is skipped, a preset that fails to load is logged and the current look kept, and a lost audio connection is retried in the background. A panic hook restores the terminal before the message is printed, so a crash never leaves the terminal in raw mode.
