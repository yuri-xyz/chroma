# Parameters

Every look in Chroma is a set of parameters. Each one can be written in a [config file](./CONFIG_SAVE_LOAD.md) under the name shown here, and the common ones also have a command-line flag. [Usage](./USAGE.md) explains how presets, files, and flags are layered.

Values outside a parameter's range are clamped when they are loaded, and a value that is not a number falls back to the default. The defaults listed are the values Chroma starts with when no preset, file, or flag sets them. They are deliberately calm, because audio lifts them as soon as music plays.

### Audio-driven parameters

These set the character of the motion. While audio is playing, Chroma rewrites them on every frame, so the values you give act as the starting point and the resting state rather than as fixed settings. [Audio Setup](./AUDIO_SETUP.md) describes what drives each one.

| Name | Flag | Range | Default | Description |
| --- | --- | --- | --- | --- |
| `frequency` | `-f, --frequency` | 3.0 to 18.0 | 6.0 | Density of the pattern. Higher values pack in more detail. |
| `amplitude` | `-A, --amplitude` | 0.0 to 2.0 | 0.4 | Intensity of the pattern's variation. |
| `speed` | `-s, --speed` | 0.0 to 1.0 | 0.05 | Animation speed. `0` is frozen. |
| `color_shift` | | 0.0 to 6.28 | 0.0 | Phase offset of the colour cycle, in radians. |
| `brightness` | `-b, --brightness` | 0.0 to 2.0 | 0.6 | Overall brightness. `1.0` is neutral. |
| `contrast` | `-C, --contrast` | 0.2 to 2.0 | 0.8 | Lower is softer, higher is sharper. `1.0` is neutral. |

Audio may push `speed`, `frequency`, and `brightness` beyond these ranges for a moment. That is intentional; the ranges apply to the values you set.

### Shape

| Name | Flag | Range | Default | Description |
| --- | --- | --- | --- | --- |
| `pattern_type` | `-p, --pattern` | see `--list-patterns` | `plasma` | The shader pattern. |
| `scale` | `-S, --scale` | 0.1 to 5.0 | 1.0 | Zoom. Lower values zoom in, higher values zoom out, always about the centre of the screen. |
| `distort_amplitude` | `-x, --distort-amplitude` | 0.0 to 2.0 | 0.5 | Spatial warping of the pattern. Bass drives this while audio plays. |
| `noise_strength` | `-n, --noise-strength` | 0.0 to 0.5 | 0.15 | Grain laid over the pattern. Beats drive this while audio plays. |
| `octaves` | | 1 to 8 | 4 | Number of layers in the Noise pattern. At least three are always used. |
| `noise_scale` | | 0.0 to 0.01 | 0.005 | Variation seed for the World and Infinity patterns. |
| `z_rate` | | 0.0 to 0.1 | 0.02 | Variation seed for the World and Infinity patterns. |
| `glyph_sharpness` | | 0.5 to 2.0 | 1.0 | Variation seed for the World and Infinity patterns. |

### Colour

| Name | Flag | Range | Default | Description |
| --- | --- | --- | --- | --- |
| `color_mode` | `-m, --color-mode` | see `--list-color-modes` | `chromatic` | The colour scheme applied to the pattern. |
| `hue` | `-H, --hue` | 0 to 360 | 0 | Rotates every colour around the colour wheel, in degrees. Values outside the range wrap around. |
| `saturation` | `-t, --saturation` | 0.0 to 2.0 | 1.0 | `0` is greyscale and `2` is very vivid. While audio plays it is held between `0.7` and `1.2`, rising with bass and beats. |
| `gamma` | | 0.5 to 2.0 | 1.0 | Below `1.0` brightens midtones; above darkens them. |

### Framing

| Name | Flag | Range | Default | Description |
| --- | --- | --- | --- | --- |
| `vignette` | `-v, --vignette` | 0.0 to 1.0 | 0.3 | Darkens the edges of the screen. `0` turns it off and lets the pattern fill the terminal. |
| `vignette_softness` | | 0.0 to 1.0 | 0.5 | Width of the vignette's fade. Higher is more gradual. |
| `background_tint_r`, `_g`, `_b` | | 0.0 to 1.0 | 0.0 | The colour the vignette fades towards. |
| `terminal_bg_r`, `_g`, `_b` | `--background-color` | 0.0 to 1.0 | 0.0 | Background colour of the terminal cells. The flag takes a hex colour such as `#101020` or `ABC`. |
| `palette` | `-P, --palette` | see `--list-palettes` | `simple` | The character set used to draw the frame. See [Palettes](./PALETTES.md). |

A strong vignette makes a pattern look as if it sits inside an oval. Presets `18` and `25` show the difference: they are identical except that `25` sets `vignette` to `0`.

### Audio response

| Name | Flag | Range | Default | Description |
| --- | --- | --- | --- | --- |
| `bass_influence` | `-B, --bass-influence` | 0.0 to 1.0 | 0.5 | How strongly bass swells the amplitude and warps the pattern. |
| `mid_influence` | `-M, --mid-influence` | 0.0 to 1.0 | 0.3 | How strongly mids raise the pattern frequency. |
| `treble_influence` | `-T, --treble-influence` | 0.0 to 1.0 | 0.2 | How strongly treble boosts the animation speed. |
| `beat_sensitivity` | `--beat-sensitivity` | 0.1 to 3.0 | 1.0 | Higher values register subtler beats. |
| `effect_type` | | 0 to 6 | 0 | The effect fired by a bass drop: Circle, Cross, Diamond, Star, Grid, Octgrams, or Wave. Change it with the `N` key. |
| `beat_distortion_strength` | `-D, --beat-distortion` | 0.0 to 2.0 | 0.8 | Strength of the distortion "pop" on a beat. |
| `beat_zoom_strength` | `-z, --beat-zoom` | 0.0 to 2.0 | 0.0 | Strength of the zoom pulse on a beat. |

The last two only set an initial value. Chroma assigns both on every detected beat, using a moderate strength for ordinary beats and a stronger one for bass drops.

### Runtime fields

A saved config also contains `time`, `resolution_width`, `resolution_height`, `audio_enabled`, `effect_time`, and `beat_distortion_time`. These record the state of the running program rather than the look. The resolution is always replaced by the current terminal size and `audio_enabled` is always on, so there is no reason to edit them by hand.

### Example looks

A slow, ambient backdrop:

```toml
pattern_type = "fluid"
color_mode = "ocean"
treble_influence = 0.05
vignette = 0.4
vignette_softness = 0.8
```

A sharp, high-energy look:

```toml
pattern_type = "kaleidoscope"
color_mode = "neon"
bass_influence = 1.0
treble_influence = 0.6
beat_sensitivity = 1.8
vignette = 0.0
```

A monochrome study:

```toml
color_mode = "monochrome"
palette = "braille"
gamma = 1.3
vignette = 0.3
```

Save any of these as a `.toml` file and run `chroma -c <file>`. Because they are partial files, you can also lay them over a preset, for example `chroma --preset 12 -c <file>`.
