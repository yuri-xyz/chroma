# Chroma Documentation

Chroma renders GPU shaders as coloured ASCII in your terminal and makes them react to whatever your system is playing. These pages cover how to use it, how to tune it, and how it works inside.

If you have just installed Chroma, read [Usage](./USAGE.md) first and then [Controls](./CONTROLS.md). Everything else is reference material you can dip into when you need it.

### Using Chroma

- [Usage](./USAGE.md) covers the command line, built-in presets, stream mode, custom shaders, and troubleshooting.
- [Controls](./CONTROLS.md) lists the keyboard shortcuts and explains the status bar.
- [Configuration](./CONFIG_SAVE_LOAD.md) explains saving a look you like, loading it again, and editing it live.
- [Audio Setup](./AUDIO_SETUP.md) covers the system libraries, how audio is captured on each platform, and how sound drives the visuals.

### Reference

- [Parameters](./PARAMETERS.md) lists every parameter with its range, default, and command-line flag.
- [Palettes](./PALETTES.md) shows the character sets used to draw each frame.

### Working on Chroma

- [Architecture](./ARCHITECTURE.md) follows a frame from the GPU to the terminal and maps out the source tree.
- [Nix](./NIX.md) describes the flake outputs, the pinned toolchains, and the validation checks.
- [Contributing](../CONTRIBUTING.md) holds the contributor rules, validation commands, and the versioning scheme.
