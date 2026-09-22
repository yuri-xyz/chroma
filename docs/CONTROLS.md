# Controls

Chroma is driven by the music, so the keyboard is for shaping the look rather than steering the animation: changing the pattern, the colours, and the characters, and saving the result. Keys are not case sensitive.

The keyboard is only active in the interactive UI. [Stream mode](./USAGE.md) ignores input entirely.

### Keyboard shortcuts

| Key | Action |
| --- | --- |
| `Q`, `Esc`, `Ctrl+C` | Quit |
| `R` | Randomise every parameter |
| `S` | Save the current look to `config_<hash>.toml` |
| `T` | Next pattern |
| `C` | Next colour mode |
| `P` | Next palette |
| `N` | Next beat effect |
| `[` | Zoom in (scale `-0.1`) |
| `]` | Zoom out (scale `+0.1`) |

### Exploring looks

`R` is the fastest way to discover something new. It rolls the pattern, colour mode, palette, and numeric parameters within ranges chosen to look good, and occasionally adds a vignette or a background tint. Press it until something catches your eye, refine it with `T`, `C`, `P`, and the zoom keys, then press `S` to keep it.

`T`, `C`, and `P` each step forward through their list and wrap around at the end. To jump straight to a specific entry, start Chroma with `--pattern`, `--color-mode`, or `--palette`. The full lists are printed by `--list-patterns`, `--list-color-modes`, and `--list-palettes`.

Two patterns are easy to confuse. `Vortex` keeps its eye in the middle of the screen. `VortexTL` (`--pattern vortex-corner`) places the eye at screen position `0.5 / scale`, so larger scales pull it towards the top-left corner and let the sweeping outer arms fill the view.

### Zoom

`[` and `]` change `scale` in steps of `0.1` between `0.1` and `5.0`. Lower values zoom in and higher values zoom out, always about the centre of the screen.

### Beat effects

A bass drop triggers a burst effect that spreads across the pattern. `N` chooses its shape, cycling through Diamond, Star, Grid, Octgrams, and Wave, and fires it once so you can preview it. The current effect is the first word in the status bar.

### Saving

`S` writes every parameter to `config_<hash>.toml` in the directory Chroma was started from. The name is a hash of the contents, and because the animation time and audio-driven values change constantly, each press normally writes a new file. Load the file again with `chroma -c <file>`. See [Configuration](./CONFIG_SAVE_LOAD.md).

### What the keyboard does not control

Frequency, speed, and amplitude are rewritten by audio reactivity on every frame, so the arrow keys and `+`/`-` do not adjust them. Set their starting values with `--frequency`, `--speed`, and `--amplitude` or a config file, and tune how strongly sound drives them with `--bass-influence`, `--mid-influence`, and `--treble-influence`. [Audio Setup](./AUDIO_SETUP.md) explains the mapping.

Brightness, contrast, saturation, hue, and the other colour adjustments have no keys either. Set them with flags or a config file; with [live reload](./CONFIG_SAVE_LOAD.md) you can edit the file and watch the change land immediately.

### The status bar

The bottom row of the terminal shows the state of the visualizer and a reminder of the keys:

```
Wave PCS  F:8.4  Q:quit R:random S:save N:effect C:color P:palette
```

Reading from the left: the selected beat effect, then the first letters of the current pattern, colour mode, and palette, then the live `frequency` value. While audio is playing, the bar turns into a moving pastel gradient and music symbols drift across its empty space; when the sound stops it returns to plain black on white.

Hide the bar with `--no-status` to give the shader the whole terminal. It is also hidden automatically when the terminal is too short to spare a row.
