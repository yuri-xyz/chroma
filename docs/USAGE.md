# Usage

Chroma is a single command. Run it with no arguments and it fills the terminal with an audio-reactive shader; add flags to pick a look, load a saved configuration, or embed the output in another program.

See also [Controls](./CONTROLS.md) for the keyboard, [Parameters](./PARAMETERS.md) for every tunable value, and [Configuration](./CONFIG_SAVE_LOAD.md) for saved looks.

### Quick start

```bash
# Run with the default look
chroma

# Start from a built-in preset, or from random parameters
chroma --preset 7
chroma --random

# Load a saved configuration
chroma -c config_a3f8c2d9e1b5.toml

# See every flag
chroma --help
```

From a source checkout, replace `chroma` with `cargo run --release --`.

Press `Q` or `Esc` to quit. Chroma starts calm and dim, and comes to life as soon as it hears audio. If nothing reacts, see [Audio Setup](./AUDIO_SETUP.md).

### How parameters are layered

A look is built from four layers, each applied on top of the previous one:

1. The starting point: the audio-reactive defaults, randomised if you pass `--random`, or a built-in preset if you pass `--preset` (a preset takes priority over `--random`).
2. The config file given with `-c/--config`. It only needs to contain the fields you want to change.
3. Parameter flags such as `--scale` or `--color-mode`.
4. Changes you make with the keyboard while Chroma runs.

This makes combinations predictable. `chroma --preset 4 -c mine.toml --vignette 0` means "preset 4, adjusted by my file, with the vignette forced off". The same layering is repeated whenever the config file is reloaded or the preset changes, so the flags you passed stay in effect for the whole session.

### Built-in presets

`--preset NUM` starts from one of the presets embedded in the binary. They are numbered `0` to `25`, and larger numbers wrap around, so `--preset 27` is preset `1`. `--preset random` picks one for you. The same looks are available as editable files in the `examples/` directory.

`--preset-interval SECONDS` keeps switching presets while Chroma runs, without a restart. It takes a whole number of at least `1` and requires `--preset`, which also decides the order: a number steps through the presets in order starting from that preset, and `random` keeps picking a random preset that differs from the current one.

```bash
chroma --preset random --preset-interval 30
chroma --stream 80x24 --preset 0 --preset-interval 60 --bass-influence 0.8
```

Every switch re-applies the config file and your parameter flags on top of the new preset, exactly as at startup, so a flag like `--bass-influence` survives each change. Keyboard changes do not; the next preset replaces them. Be aware that a config saved with `S` contains every field, so loading one alongside `--preset-interval` hides the preset changes entirely. Use a partial config holding only the fields you want to pin. Preset cycling works in both the interactive UI and stream mode.

### Choosing a look by hand

Three flags select the main ingredients, and each has a matching list command that prints the accepted names and exits:

| Flag | Chooses | List command |
| --- | --- | --- |
| `-p, --pattern` | The shader pattern, such as `plasma` or `tunnel` | `--list-patterns` |
| `-m, --color-mode` | The colour scheme, such as `neon` or `aurora` | `--list-color-modes` |
| `-P, --palette` | The character set, such as `blocks` or `braille` | `--list-palettes` |

The remaining flags set numeric parameters and are described in [Parameters](./PARAMETERS.md). `--no-status` hides the status bar so the shader fills the whole terminal, `--fps` changes the target frame rate (the default is `60`), and `--background-color` takes a hex colour such as `#101020` for the terminal cells behind the glyphs.

### Stream mode

`--stream WIDTHxHEIGHT` emits fixed-size frames to stdout so that another terminal application can embed Chroma:

```bash
chroma --stream 80x24
```

Stream mode skips terminal setup, the status bar, and keyboard input. Chroma exits cleanly when the consumer closes the pipe.

By default the stream uses the legacy format, which existing embedders rely on: each frame is a block of ANSI-coloured rows followed by a blank line.

`--stream-format ansi` and `--stream-format cells` opt into a framed protocol instead. Every frame then starts with a header line:

```text
CHROMA_FRAME v=1 frame=<index> width=<w> height=<h> format=<format> encoding=utf-8 bytes=<payload_bytes>
```

The header is followed by exactly `bytes` bytes of UTF-8 payload. With `ansi` the payload is the ANSI-coloured rows. With `cells` it is one tab-separated record per cell: `x`, `y`, display width, Unicode code point, foreground RGB hex or `-`, and background RGB hex or `-`.

### Custom shaders

`--custom-shader FILE` replaces the built-in patterns with your own WGSL compute shader. It overrides `--pattern` and any pattern set in a config file.

```bash
chroma --custom-shader examples/custom_shader.wgsl
```

Start from [`examples/custom_shader.wgsl`](../examples/custom_shader.wgsl), which is written as a beginner template. A custom shader must keep three things from it:

- The `Uniforms` struct, which has to match `src/shader_common/uniforms.wgsl` field for field.
- The two bindings: the uniforms at `@group(0) @binding(0)` and the `output_buffer` storage array at `@group(0) @binding(1)`.
- A `@compute @workgroup_size(8, 8)` entry point named `main` that writes one RGBA colour per pixel into `output_buffer`.

Because the uniforms carry the audio-driven values (`frequency`, `amplitude`, `speed`, `brightness`, and the beat timers), a custom shader reacts to music as soon as it uses them. `time` already advances at `speed`, so animate with `time` directly rather than multiplying it by `speed`. The beat timers `effect_time` and `beat_distortion_time` are wall-clock stamps: measure them against `real_time`, not `time`. If the file fails to compile, Chroma exits with the WGSL error instead of starting.

### Troubleshooting

**No GPU found.** Chroma needs a GPU backend that wgpu supports: Vulkan, Metal, or DX12. On Linux, install the Vulkan loader and the Vulkan driver for your card, and confirm that `vulkaninfo` works.

**The visuals do not react to sound.** Run `chroma --list-audio-devices` and work through [Audio Setup](./AUDIO_SETUP.md). With no audio the animation deliberately slows to a stop and dims.

**Low frame rate.** The shader resolution equals the terminal size, so a smaller window is the most effective fix. A lower `--fps` also helps, as does a GPU-accelerated terminal such as Alacritty, Kitty, WezTerm, or Ghostty.

**Glyphs show as boxes or question marks.** The selected palette uses characters your font lacks. Switch to `--palette standard` or install a font with broad Unicode coverage; see [Palettes](./PALETTES.md).

**Colours are missing or wrong.** Chroma emits 24-bit ANSI colour. Use a terminal with true-colour support.

**A config edit is ignored.** Invalid edits are skipped on purpose and the current look stays active. Fix the file and save again; see [Configuration](./CONFIG_SAVE_LOAD.md).

**Finding out more.** Chroma never prints diagnostics while it renders, because that would corrupt the display. Debug builds (`cargo run` without `--release`) write details to `debug.log` in the working directory instead. Release builds do not write a log.
