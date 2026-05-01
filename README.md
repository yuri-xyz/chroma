<div align="center">
  <img width="300" alt="Chroma's logo in ASCII rainbow" src="https://github.com/user-attachments/assets/871f6c7b-8b7c-486d-8cae-41ec13ed2d02" />

🌈 A Rust-based, ASCII art shader audio visualizer for your terminal!

  <img src="https://github.com/user-attachments/assets/b71074f2-3e77-4fb9-a8ef-30288a3690c4" width="550" />

</div>

## ⭐ Features

- 🎨 **GPU-accelerated shaders** using wgpu (compute shaders)
- 🖼️ **ASCII art rendering** with ANSI color support
- ⚙️ **Highly configurable parameters** via config file
- 💾 **Save and load configurations** with automatic deduplication via hashing
- 🔄 **Live config reloading** for real-time parameter adjustment
- 🎵 **Audio visualization** driven by system audio input
- 📊 **FFT-based audio analysis** for reactive visual effects

## ✨ Demos & screenshots

🔊 Make sure sound is enabled for the videos.

<img width="2474" height="1248" alt="chroma-themes" src="https://github.com/user-attachments/assets/0f43781d-4276-4d5f-8247-a932df43372e" />

<img width="1958" height="1103" alt="chroma-config" src="https://github.com/user-attachments/assets/96dae99e-2e93-470a-b44f-40c0a09f098a" />

[chroma.webm](https://github.com/user-attachments/assets/9e821a20-8394-445c-9542-91e294225e63)

[chroma-demo-long.webm](https://github.com/user-attachments/assets/3ae02009-b9a5-4003-93b3-8120db869447)

## 🔗 Install

### Nix

```bash
# Run Chroma
nix run github:yuri-xyz/chroma

# Install the CLI into your profile
nix profile install github:yuri-xyz/chroma

# Develop locally with Rust, Vulkan, and audio dependencies
nix develop
```

Chroma always builds with audio support. Vulkan still requires a working host GPU driver/ICD.

### Cargo

```bash
git clone https://github.com/yuri-xyz/chroma.git
cd chroma

# Make sure you have ALSA, libpulse, and PipeWire/PulseAudio packages installed,
# the exact package names may vary depending on your distro.

cargo install --path .
```

## ℹ️ Usage

```bash
# Run with default settings
chroma

# Load a saved configuration
chroma --config config_a3f8c2d9e1b5.toml

# Or using the short form
chroma -c config_a3f8c2d9e1b5.toml

# View help for all arguments and settings
chroma --help
```

## 🕹️ Controls

- `Q` or `Esc` - Quit application
- `R` - **Randomize parameters** ⭐ (Discover new effects!)
- `S` - **Save configuration** 💾 (Creates `config_<hash>.toml` in current directory)
- `P`/`O` - **Cycle palettes** 🎨 (16 different character sets!)
- `↑`/`↓` - Adjust frequency
- `→`/`←` - Adjust speed
- `+`/`-` - Adjust amplitude
- `[`/`]` - Adjust scale

See [CONTROLS.md](./notes/CONTROLS.md) and [PALETTES.md](./notes/PALETTES.md) for more details.

## 🎨 Configuration & Ricing

Chroma is designed to be highly configurable and CLI-friendly, so it feels natural alongside your other terminal tools. There are multiple ways to configure the effects and visuals:

**Config files**: Load preset configurations from TOML files. You can find [example preset configs in the `examples` directory](./examples):

```
chroma -c examples/0.toml
```

**Live reloading**: Edit your config file while Chroma is running and see changes applied instantly. This makes it easy to tweak parameters and visualize your adjustments in real time.

**CLI parameters**: Most parameters can be set via command-line arguments. Run `chroma --help` to see all available options.

> [!TIP]
> You can combine config files with CLI parameters—CLI args take precedence. This is perfect when you have a favorite base config but want to tweak specific values on the fly or in a script.

> [!TIP]
> Use `--random` or `-r` to randomize any parameters that haven't been explicitly set by your config file or CLI args. Great for adding variety to each run!

You can also create custom shader patterns and load them with `chroma --custom-shader my_shader.wgsl`. See [`examples/custom_shader.wgsl`](./examples/custom_shader.wgsl) for a beginner template.

## 📦 Dependencies

- Runtime
  - vulkan-icd-loader
  - A Vulkan driver: one of `vulkan-intel`, `vulkan-radeon`, or `nvidia-utils`
  - Terminal with ANSI color support
  - PipeWire/PulseAudio with libpulse for Linux system-audio capture
  - alsa-lib for CPAL fallback capture
- Build
  - rust, cargo, git

## 🏗️ Contributing

Contributions are welcome. If you have ideas, find bugs, or want to improve the project, feel free to create an issue and a corresponding PR.

How it works:

1. Shader files are compiled by `wgpu` and rendered as ASCII output with ANSI colors in the terminal.
2. Chroma connects to system audio with PulseAudio/PipeWire on Linux, falling back to `cpal` where needed.
3. It uses [fast Fourier transforms](https://en.wikipedia.org/wiki/Fast_Fourier_transform) from the `rustfft` crate to make the visuals react to what is playing, extracting bass (20-250 Hz), mid (250-2000 Hz), and treble (2000-8000 Hz) bands.

See [CONTRIBUTING](./CONTRIBUTING.md) for a more in-depth guide.
