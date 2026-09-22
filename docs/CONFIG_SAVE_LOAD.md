# Configuration

A configuration is a plain TOML file of [parameters](./PARAMETERS.md). Chroma can write one for you when you find a look you like, load it at startup, and re-apply it live while you edit it.

### Saving a look

Press `S` while Chroma is running. It writes every current parameter to a file named `config_<hash>.toml` in the directory Chroma was started from.

The hash is the first 12 hex characters of a SHA-256 digest of the parameter values, so a file is never overwritten by a different look. The saved values include the animation time and the audio-driven `frequency`, `speed`, and `brightness`, which change on every frame, so each press normally writes a new file.

```bash
chroma
# Press R until something looks good, adjust it, then press S.
# A file such as config_a3f8c2d9e1b5.toml now exists in the current directory.
```

### Loading a look

Pass the file with `-c` or `--config`:

```bash
chroma -c config_a3f8c2d9e1b5.toml
```

If the file cannot be read or parsed at startup, Chroma exits with an error rather than silently falling back to defaults. The ready-made files in the `examples/` directory are loaded the same way, for example `chroma -c examples/0.toml`.

### Partial files and layering

A config file only needs the fields you care about. Anything it omits keeps the value from the layer beneath it: the defaults, the `--random` roll, or the `--preset` you chose. Parameter flags on the command line are applied on top of the file.

```toml
# pinned.toml - keep whatever look is underneath, but force these
color_mode = "neon"
vignette = 0.0
bass_influence = 0.8
```

```bash
chroma --preset random --preset-interval 30 -c pinned.toml
```

[Usage](./USAGE.md) describes the full layering order. Note that a file saved with `S` always contains every field, so it completely replaces the layer beneath it.

### Live reload

While Chroma runs with `-c`, it watches that file and applies changes as soon as you save them, with no restart. Open the file in an editor beside the terminal and tune values by eye.

A reload rebuilds the look exactly as startup does, with the current preset underneath and your command-line flags on top. The animation clock and the terminal resolution carry over, so the picture does not jump. Keyboard changes made since the last load are replaced by the file's values.

If an edit leaves the file invalid, for example halfway through typing a value, the reload is skipped and the current look stays on screen. Editors that save by writing a new file and renaming it over the old one are supported.

### File format

The file is a flat list of `key = value` pairs:

```toml
frequency = 12.5
amplitude = 1.8
speed = 0.6
scale = 1.2
brightness = 1.3
vignette = 0.2
palette = "Circles"
color_mode = "Chromatic"
pattern_type = "Plasma"
bass_influence = 0.5
mid_influence = 0.3
treble_influence = 0.2
```

Enum fields (`palette`, `color_mode`, `pattern_type`) accept either the names Chroma writes when saving, such as `"WarpedFbm"`, or the command-line names and aliases, such as `"warped"`. An unknown name is an error.

Numeric values are clamped into their valid ranges when the file is loaded, so an out-of-range number is corrected rather than rejected.

A saved file also records a few runtime values. `time` is the animation clock, so a loaded look resumes from the moment it was saved. `resolution_width` and `resolution_height` are written for completeness but are always replaced by the current terminal or stream size. `audio_enabled` is always on.
