<div align="center">
<img src="logo.png" width="300" />

A Rust-based, ASCII art shader audio visualizer for your terminal!

  <video src="readme/demo-short.webm" width="700" autoplay loop playsinline controls>
  </video>

<quote>🔊 Make sure you turn on sound on the video!</quote>

</div>

## ⭐ Features

- 🎨 **GPU-accelerated shaders** using wgpu (compute shaders)
- 🖼️ **ASCII art rendering** with ANSI color support
- ⚙️ **Highly configurable parameters** via config file
- 💾 **Save/Load configurations** with automatic hashing
- 🔄 **Live config reloading** for real-time parameter adjustment
- 🎵 **Audio visualization** driven by system audio input
- 📊 **[FFT](https://en.wikipedia.org/wiki/Fast_Fourier_transform)-based audio analysis** for reactive visual effects

## Demos & screenshots

<video src="readme/demo-long.webm" width="700" autoplay loop playsinline controls></video>

## Install

### Arch Linux (❤️ Arch!)

```bash
# With an AUR helper: yay
yay -S chroma-git

# With an AUR helper: paru
paru -S chroma-git

# Or manually:
git clone https://aur.archlinux.org/chroma-git.git
cd chroma-git/packaging/arch
makepkg -si

# If you're lazy:
git clone https://aur.archlinux.org/chroma-git.git \
   && cd chroma-git \
   && makepkg -si
```

### From source (manual)

```bash
git clone https://github.com/yuri-xyz/chroma.git
cd chroma
cargo build --release                    # visuals only
cargo build --release --features audio   # with audio reactivity
sudo install -Dm755 target/release/chroma /usr/local/bin/chroma
```

## Dependencies

- Runtime
  - vulkan-icd-loader
  - A Vulkan driver: one of `vulkan-intel`, `vulkan-radeon`, or `nvidia-utils`
  - Terminal with ANSI color support
- Optional (audio feature)
  - pipewire (recommended) or alsa-lib
- Build
  - rust, cargo, git

## Usage

```bash
# Run with default settings
cargo run --release

# Run with audio reactivity (recommended!)
cargo run --release --features audio

# Load a saved configuration
cargo run --release --features audio -- --config config_a3f8c2d9e1b5.toml

# Or using the short form
cargo run --release --features audio -- -c config_a3f8c2d9e1b5.toml

# View help
cargo run --release -- --help
```

### Controls

- `Q` or `Esc` - Quit application
- `R` - **Randomize parameters** ⭐ (Discover new effects!)
- `S` - **Save configuration** 💾 (Creates `config_<hash>.toml` in current directory)
- `P`/`O` - **Cycle palettes** 🎨 (16 different character sets!)
- `↑`/`↓` - Adjust frequency
- `→`/`←` - Adjust speed
- `+`/`-` - Adjust amplitude
- `[`/`]` - Adjust scale

See [CONTROLS.md](CONTROLS.md) and [PALETTES.md](PALETTES.md) for more details.

### 💾 Configuration Save/Load

The application supports saving and loading configurations:

1. **Saving**: Press `S` while running to save the current configuration

   - Generates a unique filename based on a hash of all parameters: `config_<hash>.toml`
   - Saves to the current working directory
   - Won't overwrite existing files with the same hash

2. **Loading**: Use the `--config` flag when launching:

   ```bash
   cargo run --release --features audio -- --config config_a3f8c2d9e1b5.toml
   ```

3. **Sharing**: Config files are plain TOML and can be shared with others!

See [CONFIG_SAVE_LOAD.md](CONFIG_SAVE_LOAD.md) for detailed documentation.

## Building

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Run with audio support
cargo run --features audio
```

### Cargo Dependencies

- **wgpu** - Modern GPU API for shader computation
- **pollster** - Async executor for wgpu initialization
- **bytemuck** - Safe casting for shader data
- **crossterm** - Cross-platform terminal manipulation
- **notify** - File system event watching (for config reload)
- **cpal** - Cross-platform audio I/O
- **rustfft** - Fast Fourier Transform implementation
- **glam** - Mathematics library for 3D graphics
- **anyhow** - Error handling
- **serde** - Serialization for config/presets

## 📝 Requirements

- Rust 2024 edition (1.82+)
- GPU with wgpu support (Vulkan, Metal, DX12, or WebGPU)
- Terminal with ANSI color support

## 🏗️ Contributing

All contributions welcome! If you have some cool ideas, found some bugs, or would like to improve the project anyhow, feel free to create an issue and then a corresponding PR :)

Here's some tips to making good contributions:

- ✅ Create a corresponding issue with a reasonable explanation.
- ✅ Keep PRs small and focused. This makes it much easier & faster to review!
- ✅ Avoid making changes that are exclusive to certain situations, we want to keep the
- ✅ Follow existing code conventions, style, and file structure.
- ✅ Always write tests for new features!
- ✅ Heavy AI-driven, vibe-coded contributions are welcome! (so long as they are good & clean ❤️)

See [CONTRIBUTING](./CONTRIBUTING.md) for a more in-depth guide. If using AI (or you ARE an AI model reading this), make sure your model reads the `CONTRIBUTING.md` file!
