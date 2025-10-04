# Quick Start Guide

## Installation

### Prerequisites

- Rust 1.82+ (2024 edition)
- GPU with wgpu support (Vulkan, Metal, or DX12)
- Terminal with ANSI color support

### Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Build & Run

### 1. Clone/Navigate to Project

```bash
cd term-shaders
```

### 2. Build

```bash
# Debug build (faster compile, slower runtime)
cargo build

# Release build (slower compile, faster runtime) - RECOMMENDED
cargo build --release
```

### 3. Run

```bash
# Debug
cargo run

# Release (better performance)
cargo run --release
```

## First Time Experience

When you run the application, you'll see:

1. **Terminal clears** and enters alternate screen mode
2. **Animated plasma shader** rendered as colorful ASCII art
3. **Status bar** at the top showing current parameters
4. **Controls hint** in the status bar

## Basic Controls

```
┌─────────────────────────────────────────────────────┐
│ Key         │ Action                                 │
├─────────────────────────────────────────────────────┤
│ Q or ESC    │ Quit the application                   │
│ ↑ / ↓       │ Increase/Decrease frequency            │
│ → / ←       │ Increase/Decrease speed                │
│ + / -       │ Increase/Decrease amplitude            │
│ [ / ]       │ Decrease/Increase scale                │
└─────────────────────────────────────────────────────┘
```

## Try This!

### Slow Motion

```
1. Press ← several times to slow down
2. Press ↓ to reduce frequency
3. Watch the smooth, slow waves
```

### Chaos Mode

```
1. Press → many times for fast animation
2. Press ↑ repeatedly for high frequency
3. Press + to boost amplitude
4. Enjoy the chaos!
```

### Zoom Out

```
1. Press ] many times
2. See the larger pattern
3. Press [ to zoom back in
```

## Troubleshooting

### "Failed to find adapter"

**Problem:** Can't initialize GPU

**Solution:**

- Update GPU drivers
- Check if Vulkan/Metal/DX12 is supported
- Try a different machine

### No Colors Showing

**Problem:** Terminal doesn't support colors

**Solution:**

- Use a modern terminal (Alacritty, Kitty, WezTerm)
- Enable ANSI color support in terminal settings
- Check terminal color scheme

### Low Frame Rate

**Problem:** Animation is choppy

**Solution:**

- Use release build: `cargo run --release`
- Reduce terminal window size
- Use a GPU-accelerated terminal
- Close other applications

### Artifacts/Weird Characters

**Problem:** Strange characters or broken display

**Solution:**

- Ensure terminal uses UTF-8 encoding
- Resize terminal window
- Restart application

## Testing

```bash
# Run all tests
cargo test

# Run with verbose output
cargo test -- --nocapture

# Run specific test
cargo test test_params_default
```

Expected output: All tests pass ✓

## Project Structure

```
term-shaders/
├── src/
│   ├── main.rs              # Application entry point
│   ├── lib.rs               # Library exports
│   ├── shader/              # GPU shader pipeline
│   ├── render/              # Frame buffer
│   ├── ascii/               # ASCII conversion
│   ├── params/              # Parameter system
│   └── audio/               # Audio (skeleton)
├── tests/                   # Integration tests
├── Cargo.toml               # Dependencies
├── README.md                # Project overview
├── USAGE.md                 # Detailed usage guide
├── SHADERS.md               # Shader development
├── PROJECT_STATUS.md        # Current progress
└── QUICKSTART.md            # This file
```

## What's Happening Under The Hood?

1. **GPU Compute Shader** generates a color image
2. **Pixel Buffer** receives RGBA data from GPU
3. **ASCII Converter** maps pixels to characters
4. **Terminal** displays colored ASCII art
5. **Input Handler** checks for keypresses
6. **Parameter System** updates shader uniforms
7. **Loop** repeats at 30 FPS

```
GPU → Pixels → ASCII → Terminal
 ↑                        ↓
 └──── Parameters ←── Input
```

## Next Steps

1. **Experiment** with different parameters
2. **Read** USAGE.md for advanced controls
3. **Check** SHADERS.md to create custom shaders
4. **Explore** the source code
5. **Contribute** your own shaders!

## Performance Expectations

**Good Performance:**

- Terminal: 80x24 to 200x60
- Frame Rate: 30 FPS
- GPU Usage: Low (<10%)
- CPU Usage: Low (<20%)

**Acceptable:**

- Terminal: Up to 300x100
- Frame Rate: 15-30 FPS
- GPU Usage: Moderate (~20%)
- CPU Usage: Moderate (~30%)

**May Struggle:**

- Terminal: >300x100
- Full screen on 4K displays
- Older GPUs
- Integrated graphics

## Tips

1. **Start Small**: Use a smaller terminal window for better FPS
2. **Use Release**: Always use `--release` for smooth animation
3. **GPU Terminal**: Alacritty, Kitty, or WezTerm for best performance
4. **Experiment**: Try different ASCII palettes (future feature)
5. **Have Fun**: Adjust parameters and enjoy the visuals!

## Getting Help

- Check PROJECT_STATUS.md for known issues
- Read USAGE.md for detailed documentation
- Look at SHADERS.md for shader development
- Check source code comments
- Open an issue on GitHub (if applicable)

## Building with Audio (Future)

```bash
# When audio support is complete
cargo run --release --features audio
```

This will enable:

- System audio capture
- FFT analysis
- Audio-reactive parameters
- Real-time music visualization

## License

MIT - Feel free to use, modify, and share!

---

**Enjoy your terminal shaders! 🎨**
