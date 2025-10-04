# Term Shaders 🎨🎵

A Rust-based terminal shader visualizer and audio visualizer that renders GPU-computed shaders as ASCII art in your terminal.

## Features

- 🎨 **GPU-accelerated shaders** using wgpu (compute shaders)
- 🖼️ **ASCII art rendering** with ANSI color support
- ⚙️ **Highly configurable parameters** via config file
- 🔄 **Live config reloading** for real-time parameter adjustment
- 🎵 **Audio visualization** driven by system audio input
- 📊 **FFT-based audio analysis** for reactive visual effects

## Architecture Overview

```
┌─────────────────────────────────────────────────┐
│              Terminal Display (TUI)             │
│         (crossterm + ratatui)                   │
└────────────────┬────────────────────────────────┘
                 │
┌────────────────▼────────────────────────────────┐
│          ASCII Converter                        │
│   (Pixel brightness → ASCII palette)            │
└────────────────┬────────────────────────────────┘
                 │
┌────────────────▼────────────────────────────────┐
│          Frame Buffer (RGBA)                    │
│        (Output from GPU)                        │
└────────────────┬────────────────────────────────┘
                 │
┌────────────────▼────────────────────────────────┐
│         Shader Pipeline (wgpu)                  │
│    Compute/Render shaders with uniforms         │
└────────────────┬────────────────────────────────┘
                 │
        ┌────────┴────────┐
        │                 │
┌───────▼─────┐  ┌────────▼────────┐
│  Parameters │  │  Audio Input    │
│   System    │  │  + FFT          │
│             │  │  (cpal)         │
└─────────────┘  └─────────────────┘
```

## Implementation Roadmap

### Phase 1: Foundation (MVP) ✅

1. **Project Setup**

   - Set up Cargo.toml with core dependencies
   - Create modular project structure
   - Write basic documentation

2. **Shader Pipeline**

   - Initialize wgpu (headless mode)
   - Create compute shader for procedural generation
   - Set up uniform buffers for parameters
   - Render to texture/buffer

3. **ASCII Conversion**

   - Implement pixel-to-ASCII mapping
   - Add ANSI color support
   - Optimize for terminal rendering

4. **Basic Parameters**

   - Time uniform
   - Resolution
   - Color palette selection
   - Basic shader-specific parameters (frequency, amplitude, etc.)

5. **First Shader Example**
   - Implement plasma effect or procedural gradient
   - Validate end-to-end pipeline

### Phase 2: Configuration & Interactivity

6. **Config File System**

   - TOML-based configuration
   - Live config file reloading
   - Watch for file changes
   - Apply parameters in real-time

7. **Additional Shader Effects**
   - Mandelbrot/Julia sets
   - Perlin/Simplex noise patterns
   - Ray marching effects
   - Cellular automata

### Phase 3: Audio Visualization

8. **Audio Input**

   - Integrate cpal for system audio capture
   - Implement audio buffer management
   - Add audio source selection

9. **FFT Processing**

   - Implement FFT analysis (using rustfft)
   - Extract frequency bands
   - Map audio features to shader uniforms
   - Add smoothing/interpolation

10. **Audio-Reactive Parameters**
    - Bass → amplitude
    - Mids → color shifts
    - Treble → detail/frequency
    - Volume → overall intensity

### Phase 4: Polish

11. **Testing**

    - Unit tests for ASCII conversion
    - Integration tests for shader pipeline
    - Parameter validation tests

12. **Performance Optimization**

    - Adaptive resolution
    - Frame rate control
    - GPU/CPU profiling

13. **Documentation**
    - API documentation
    - Shader creation guide
    - Usage examples

## Technical Stack

### Core Dependencies

- **wgpu** - Modern GPU API for shader computation
- **pollster** - Async executor for wgpu initialization
- **bytemuck** - Safe casting for shader data

### Terminal

- **crossterm** - Cross-platform terminal manipulation
- **notify** - File system event watching (for config reload)

### Audio Processing

- **cpal** - Cross-platform audio I/O
- **rustfft** - Fast Fourier Transform implementation

### Utilities

- **glam** - Mathematics library for 3D graphics
- **anyhow** - Error handling
- **serde** - Serialization for config/presets

## Project Structure

```
term-shaders/
├── src/
│   ├── main.rs              # Entry point, main loop
│   ├── shader/              # Shader management
│   │   ├── mod.rs
│   │   ├── pipeline.rs      # wgpu pipeline setup
│   │   ├── uniforms.rs      # Uniform buffer management
│   │   └── shaders/         # WGSL shader files
│   │       ├── plasma.wgsl
│   │       ├── mandelbrot.wgsl
│   │       └── noise.wgsl
│   ├── render/              # Rendering to framebuffer
│   │   ├── mod.rs
│   │   └── framebuffer.rs
│   ├── ascii/               # ASCII conversion
│   │   ├── mod.rs
│   │   ├── converter.rs
│   │   └── palette.rs
│   ├── params/              # Parameter system
│   │   ├── mod.rs
│   │   ├── config.rs
│   │   └── loader.rs        # Config file loading
│   ├── audio/               # Audio input & FFT
│   │   ├── mod.rs
│   │   ├── input.rs
│   │   └── fft.rs
│   └── lib.rs               # Library exports
├── tests/                   # Integration tests
├── examples/                # Usage examples
├── Cargo.toml
└── README.md
```

## Usage

```bash
# Run with default settings (uses config.toml)
cargo run --release

# Run with custom config file
cargo run --release -- --config my-config.toml
```

### Controls

- `Q` or `Esc` - Quit application
- `R` - **Randomize parameters** ⭐ (Discover new effects!)
- `P`/`O` - **Cycle palettes** 🎨 (10 different character sets!)
- `↑`/`↓` - Adjust frequency
- `→`/`←` - Adjust speed
- `+`/`-` - Adjust amplitude
- `[`/`]` - Adjust scale

See [CONTROLS.md](CONTROLS.md) and [PALETTES.md](PALETTES.md) for more details.

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

## Requirements

- Rust 2024 edition (1.82+)
- GPU with wgpu support (Vulkan, Metal, DX12, or WebGPU)
- Terminal with ANSI color support

## Performance Considerations

- Shaders run on GPU (compute shaders)
- ASCII conversion is CPU-bound but optimized
- Target: 30-60 FPS on modern hardware
- Adaptive resolution based on terminal size

## License

MIT

## Inspiration

This project is inspired by various ASCII shader visualizers and audio visualizers that combine GPU computing with terminal rendering.

## Contributing

Contributions welcome! Please open an issue or PR.
