<div align="center">
  <img src="readme/logo.png" width="300" />

🌈 A Rust-based, ASCII art shader audio visualizer for your terminal!

  <img src="readme/preview.gif" width="550" />
</div>

## ⭐ Features

- 🎨 **GPU-accelerated shaders** using wgpu (compute shaders)
- 🖼️ **ASCII art rendering** with ANSI color support
- ⚙️ **Highly configurable parameters** via config file
- 💾 **Save/Load configurations** with automatic deduping via hashing
- 🔄 **Live config reloading** for real-time parameter adjustment
- 🎵 **Audio visualization** driven by system audio input
- 📊 **FFT-based audio analysis** for reactive visual effects

## ✨ Demos & screenshots

🔊 Make sure you turn on sound on the videos!

<img width="2474" height="1248" alt="chroma" src="https://github.com/user-attachments/assets/b6caaef4-f861-4a96-b06d-d087a3ad15fa" />

[chroma.webm](https://github.com/user-attachments/assets/9e821a20-8394-445c-9542-91e294225e63)

[chroma-demo-long.webm](https://github.com/user-attachments/assets/3ae02009-b9a5-4003-93b3-8120db869447)

## 🔗 Install

### Arch Linux

> [!WARNING]
> Since this project is so new I haven't had a chance to publish it to the AUR yet,
> but you can go ahead and install it using makepkg like so:

```bash
git clone https://github.com/yuri-xyz/chroma
cd chroma/packaging/arch
makepkg -si
```

> [!WARNING]
> Once it's available on the AUR:

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

### Other distros

...More packaging coming soon!...

Meanwhile you can build from source below:

### From source (manual)

```bash
# Clone the git repo and enter it:
git clone https://github.com/yuri-xyz/chroma.git
cd chroma

# Make sure you have the `alsa-lib` & `pipewire` packages installed,
# the exact package names may vary depending on your distro.

# Pick one:
cargo build --release                    # visuals only
cargo build --release --features audio   # with audio reactivity (recommended)

# Install the built bin so that you can run it with `chroma`:
sudo install -Dm755 target/release/chroma /usr/local/bin/chroma
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

### 🕹️ Controls

- `Q` or `Esc` - Quit application
- `R` - **Randomize parameters** ⭐ (Discover new effects!)
- `S` - **Save configuration** 💾 (Creates `config_<hash>.toml` in current directory)
- `P`/`O` - **Cycle palettes** 🎨 (16 different character sets!)
- `↑`/`↓` - Adjust frequency
- `→`/`←` - Adjust speed
- `+`/`-` - Adjust amplitude
- `[`/`]` - Adjust scale

See [CONTROLS.md](./notes/CONTROLS.md) and [PALETTES.md](./notes/PALETTES.md) for more details.

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

## 📦 Dependencies

- Runtime
  - vulkan-icd-loader
  - A Vulkan driver: one of `vulkan-intel`, `vulkan-radeon`, or `nvidia-utils`
  - Terminal with ANSI color support
- Optional (audio feature)
  - pipewire (recommended) or alsa-lib
- Build
  - rust, cargo, git

## 🏗️ Contributing

All contributions welcome! If you have some cool ideas, found some bugs, or would like to improve the project anyhow, feel free to create an issue and then a corresponding PR :)

Here's some tips to making good contributions:

- ✅ Create a corresponding issue with a reasonable explanation.
- ✅ Keep PRs small and focused. This makes it much easier & faster to review!
- ✅ Avoid making changes that are exclusive to certain situations, we want to keep the app focused and minimalist.
- ✅ Follow existing code conventions, style, and file structure.
- ✅ Always write tests for new features!
- ✅ Heavy AI-driven, vibe-coded contributions are welcome! (so long as they are good & clean ❤️)

And here's how it works for my nerdy friends:

1.  Shader files are compiled by `wgpu` and rendered as ASCII output with ANSI colors in the terminal.
2.  Hook up to system audio with the `cpal` crate.
3.  Use [fast Fourier transforms](https://en.wikipedia.org/wiki/Fast_Fourier_transform) from the `rustfft` crate to make the visuals react to what's playing. For example, by extracting bass (20-250 Hz), mid (250-2000 Hz), and treble (2000-8000 Hz) bands.

See [CONTRIBUTING](./CONTRIBUTING.md) for a more in-depth guide. If using AI (or you ARE an AI model reading this), make sure your model reads the `CONTRIBUTING.md` file!
