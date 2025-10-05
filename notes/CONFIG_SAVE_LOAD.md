# Configuration Save/Load System

This document describes how to save and load shader configurations.

## Saving Configurations

While the application is running, press the `S` key to save the current configuration. The system will:

1. Hash all configuration values (params, colors, effects, etc.)
2. Generate a filename based on the hash: `config_<hash>.toml`
3. Save the file to the current working directory
4. If a file with the same hash already exists, it won't overwrite it

The hash is a 12-character SHA-256 hash of the configuration, ensuring unique filenames for different configs.

### Example

```bash
# Run the app
cargo run --release --features audio

# Adjust settings using keyboard controls
# Press 'S' to save

# This creates a file like: config_a3f8c2d9e1b5.toml
```

## Loading Configurations

To load a saved configuration, use the `--config` or `-c` command-line argument:

```bash
# Load a specific configuration
cargo run --release --features audio -- --config config_a3f8c2d9e1b5.toml

# Or using the short form
cargo run --release --features audio -- -c config_a3f8c2d9e1b5.toml
```

If the config file fails to load, the application will fall back to the default configuration and print a warning.

## Configuration Format

The saved configuration is in TOML format and includes all shader parameters:

```toml
time = 0.0
resolution_width = 200
resolution_height = 50
frequency = 12.5
amplitude = 1.8
speed = 0.6
color_shift = 2.1
scale = 1.2
octaves = 4
noise_strength = 0.15
distort_amplitude = 0.5
# ... and many more parameters
palette = "Circles"
color_mode = "Chromatic"
pattern_type = "Plasma"
audio_enabled = true
bass_influence = 0.5
mid_influence = 0.3
treble_influence = 0.2
# ... etc
```

## Help

To view all command-line options:

```bash
cargo run --release --features audio -- --help
```

Output:

```
Terminal-based shader visualizer with optional audio reactivity

Usage: term-shaders [OPTIONS]

Options:
  -c, --config <FILE>  Load configuration from a saved config file
  -h, --help           Print help
```

## Workflow Example

1. **Experiment**: Launch the app and tweak settings with keyboard controls
2. **Save**: Press `S` when you find a configuration you like
3. **Reload**: Launch with `--config <filename>` to restore that exact state
4. **Share**: Share config files with others!

## Notes

- The configuration saves **all** parameters, including audio reactivity settings
- Resolution (terminal size) is captured but will be updated to match your current terminal on load
- Time and animation state are reset on load (starts from the beginning)
- Config files are plain TOML and can be manually edited if needed
