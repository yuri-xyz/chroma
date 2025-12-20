# Chroma - Project Overview for AI Agents

> ⚠️ **IMPORTANT NOTE FOR AI AGENTS**: This document is a high-level overview and may become stale as the project evolves. **The actual source code is the ultimate source of truth.** When in doubt about implementation details, specific parameter names, function signatures, or behavior:
> - Check the actual source files (especially `src/params/shader_params.rs`, `src/shader_common/*.wgsl`, `src/app/mod.rs`)
> - Read the code comments and type definitions
> - Verify against recent git commits
> - Don't rely solely on this documentation if you find discrepancies

## What is Chroma?

**Chroma** is a GPU-accelerated ASCII art audio visualizer written in Rust. It renders real-time visual effects to the terminal as ASCII characters with ANSI colors, synchronized with system audio input. The visuals are generated using compute shaders (WGSL) running on the GPU via wgpu.

**Key tagline**: 🌈 A Rust-based, ASCII art shader audio visualizer for your terminal!

## Core Features

- **GPU-accelerated shaders** using wgpu compute shaders (WGSL)
- **ASCII art rendering** with ANSI color support for terminal display
- **Highly configurable parameters** via TOML config files
- **Live config reloading** for real-time parameter adjustment without restarting
- **Audio visualization** driven by system audio input (PipeWire/ALSA)
- **FFT-based audio analysis** for bass/mid/treble reactive effects
- **16 different character palettes** for visual variety
- **Save/Load configurations** with automatic deduping via SHA2 hashing
- **Custom shader support** with template examples

## Architecture Overview

### Design Philosophy
- **Simple, Direct, Efficient** - Avoids unnecessary complexity
- No TUI framework - just renders shader output directly
- Config-driven parameters, not interactive menus
- Direct terminal output using crossterm
- GPU does the heavy lifting via compute shaders

### Component Architecture

```
Config/Input → ShaderParams → ShaderUniforms ↓
                                               GPU Shader Pipeline
                                               (Compute Shaders)
                                                     ↓
                                               RGBA Pixel Data
                                                     ↓
                                          ASCII Conversion Layer
                                          (brightness → char mapping)
                                                     ↓
                                            Terminal Rendering
```

### Data Flow - Frame Rendering Cycle (30 FPS)

1. Check for config file changes (file watcher)
2. Load updated parameters if changed
3. Update time uniform
4. Convert params → uniforms (CPU)
5. Upload uniforms to GPU via uniform buffer
6. Dispatch compute shader (GPU processes in parallel)
7. Copy output buffer → staging buffer
8. Map staging buffer (GPU → CPU transfer)
9. Convert float RGBA → u8 RGBA values
10. For each pixel: calculate brightness → map to ASCII character → determine ANSI color
11. Render ASCII frame to terminal
12. Sleep until next frame

## Project Structure

```
src/
├── main.rs                 # Application entry point, main render loop
├── lib.rs                  # Library exports
├── cli.rs                  # Command-line argument parsing (clap)
├── constants.rs            # Global constants
├── app/                    # Core application logic
│   ├── mod.rs
│   ├── rendering.rs        # Frame rendering orchestration
│   ├── audio.rs            # Audio parameter updates
│   ├── input.rs            # Keyboard input handling
│   ├── config_watcher.rs   # File watching for live config reload
│   └── status_bar.rs       # Status bar display
├── ascii/                  # ASCII conversion layer
│   ├── converter.rs        # RGBA → ASCII + color conversion
│   ├── palette.rs          # Character palettes (16 types)
│   └── mod.rs
├── audio/                  # Audio processing (optional feature)
│   ├── capture.rs          # Audio device selection & stream management
│   ├── analyzer.rs         # FFT computation & frequency analysis
│   ├── device_selector.rs  # Interactive audio device selection
│   └── mod.rs
├── params/                 # Parameter management
│   ├── config.rs           # ShaderParams struct, defaults, updates
│   ├── color_mode.rs       # Color mode enums
│   └── config/             # Config serialization types
├── shader/                 # GPU shader pipeline (if exists)
│   └── (GPU rendering pipeline via wgpu)
└── shader_common/          # WGSL shader source files
    ├── main.wgsl           # Main compute shader entry
    ├── uniforms.wgsl       # Uniform buffer definitions
    ├── effects.wgsl        # Effect functions (distortion, etc.)
    ├── color_modes.wgsl    # Color processing
    ├── color_utils.wgsl    # Color utility functions
    └── beat_distortion.wgsl # Beat-reactive distortion

shader_patterns/           # 16+ different shader patterns
├── plasma.wgsl            # Plasma effect
├── noise.wgsl             # Perlin/Simplex noise
├── fractal.wgsl           # Fractal patterns
├── geometric.wgsl         # Geometric shapes
├── waves.wgsl             # Wave interference
├── ripples.wgsl           # Ripple patterns
├── rings.wgsl             # Ring patterns
├── spiral.wgsl            # Spiral patterns
├── interference.wgsl      # Interference patterns
├── voronoi.wgsl           # Voronoi diagram
├── hexagonal.wgsl         # Hexagonal tessellation
├── glitch.wgsl            # Glitch effect
├── grid.wgsl              # Grid pattern
├── sphere.wgsl            # Sphere/3D effect
├── octgrams.wgsl          # Octagram patterns
├── diamonds.wgsl          # Diamond patterns
├── truchet.wgsl           # Truchet tiles
├── vortex.wgsl            # Vortex effect
└── warped_fbm.wgsl        # Fractional Brownian Motion

examples/
├── *.toml                  # Example preset configurations
└── custom_shader.wgsl      # Template for custom shaders

notes/
├── ARCHITECTURE.md         # Detailed architecture documentation
├── CONTROLS.md             # Keyboard controls reference
├── PALETTES.md             # Palette information
├── PARAMETERS.md           # Parameter descriptions
├── USAGE.md                # Usage guide
├── CONFIG_SAVE_LOAD.md     # Config system documentation
└── AUDIO_SETUP.md          # Audio setup guide
```

## How It Works - Technical Details

### GPU Pipeline
- **Compute Shader Model**: Workgroups of 8x8 (64 threads per group)
- **Resolution**: Terminal width × height in characters (typically 80×24)
- **For 80×24**: 10 workgroups × 3 workgroups = 30 groups, ~1,920 pixels
- **Shader Language**: WGSL (WebGPU Shader Language)
- **Graphics API**: Vulkan (via wgpu)

### Compute Shader Execution Model

Each compute shader invocation processes one pixel:

```wgsl
@compute @workgroup_size(8, 8)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    // 1. Boundary check (handles non-divisible dimensions)
    if (global_id.x >= width || global_id.y >= height) { return; }

    // 2. Convert 2D coordinate to 1D buffer index
    let index = global_id.y * width + global_id.x;

    // 3. Convert pixel coordinates to normalized UV [0.0, 1.0]
    let uv = vec2<f32>(f32(global_id.x) / width, f32(global_id.y) / height);

    // 4. Compute color via pattern pipeline (see below)
    let color = plasma_effect(uv, uniforms.time);

    // 5. Write RGBA float to output buffer
    output_buffer[index] = vec4<f32>(color, 1.0);
}
```

### GPU Data Structures (Complete)

The shader uniforms struct contains 27 parameters controlling all visual aspects:

```wgsl
struct Uniforms {
    // Timing
    time: f32,                          // Animation timestamp (seconds)

    // Positioning/Scale
    resolution: vec2<f32>,              // Render resolution (width, height)
    scale: f32,                         // Zoom/scale parameter

    // Wave/Pattern Parameters
    frequency: f32,                     // Wave frequency
    amplitude: f32,                     // Effect amplitude/intensity
    speed: f32,                         // Animation speed multiplier
    octaves: u32,                       // Fractal octaves (noise patterns)
    z_rate: f32,                        // Z-axis rotation (3D effects)

    // Distortion/Noise
    distort_amplitude: f32,             // Distortion effect amplitude
    noise_strength: f32,                // Noise blend strength
    noise_scale: f32,                   // Noise detail level

    // Color Processing
    color_shift: f32,                   // Hue rotation amount
    hue: f32,                           // Hue shift (0-1)
    saturation: f32,                    // Saturation (0=gray, 1=normal)
    brightness: f32,                    // Overall brightness
    contrast: f32,                      // Contrast enhancement
    gamma: f32,                         // Gamma correction exponent

    // Visual Effects
    vignette: f32,                      // Vignette effect radius
    vignette_softness: f32,             // Vignette edge softness
    glyph_sharpness: f32,               // ASCII character sharpness

    // Mode Selectors
    color_mode: u32,                    // Color mapping mode (0-N)
    pattern_type: u32,                  // Pattern selector (0-18, see list below)
    effect_type: u32,                   // Effect type selector

    // Audio-Reactive (beat-sync)
    effect_time: f32,                   // Effect-specific timing
    beat_distortion_time: f32,          // Beat sync distortion timestamp
    beat_distortion_strength: f32,      // Beat distortion intensity (0-1)
    beat_zoom_strength: f32,            // Beat zoom intensity (0-1)
    background_tint: vec3<f32>,         // Background RGB tint color
}
```

**Pattern Type Mapping:**
```
0: plasma      7: truchet     14: grid
1: waves       8: hexagonal   15: diamonds
2: ripples     9: interference 16: sphere
3: vortex      10: fractal    17: octgrams
4: noise       11: glitch     18: warped_fbm
5: geometric   12: spiral
6: voronoi     13: rings
```

### Color Processing Pipeline

The `plasma_effect()` function orchestrates the full rendering chain:

```
1. Position Processing:
   ├─ Apply beat-reactive zoom
   │  └─ Scales from center based on audio energy
   ├─ Apply beat-reactive distortion
   │  └─ Warps position on beat (audio synchronized)

2. Pattern Generation:
   ├─ Apply scale transform
   ├─ Dispatch to pattern function (0-18)
   ├─ Extract: main_value (pattern), gradient (secondary)

3. Color Mapping:
   ├─ Apply color mode (grayscale, HSV, gradient, etc.)
   ├─ Apply color adjustments:
   │  ├─ Brightness adjustment
   │  ├─ Contrast enhancement
   │  ├─ Gamma correction
   │  ├─ Saturation control
   │  └─ Hue rotation

4. Effect Application:
   ├─ Apply effect layer (glitch, bloom, trails, etc.)
   ├─ Apply beat flash (brightness spike)

5. Vignetting:
   └─ Apply optional edge darkening
```

### Buffer Management

**Bindings (from uniforms.wgsl):**
```wgsl
@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@group(0) @binding(1)
var<storage, read_write> output_buffer: array<vec4<f32>>;
```

**Buffer Sizes and Lifecycle:**
- **Uniform Buffer**: 256 bytes (updated every frame with new parameters)
  - Contains all 27 uniforms + padding for alignment
  - Uploaded to GPU before each compute dispatch

- **Output Buffer (GPU)**: W × H × 4 × 4 bytes
  - Storage buffer (read_write in compute shader)
  - Output of compute shader (vec4<f32> RGBA)
  - Example 80×24: 30,720 bytes

- **Staging Buffer**: W × H × 4 × 4 bytes
  - CPU-accessible GPU buffer
  - Used for GPU → CPU async transfer
  - Mapped with MapMode::Read

- **Final RGBA Buffer (CPU)**: W × H × 4 × 1 bytes
  - u8 RGBA converted from float values
  - Ready for ASCII conversion
  - Example 80×24: 7,680 bytes

### Audio Processing Pipeline (Optional Feature)

**Architecture:**
- **Audio Capture** (`cpal` crate): Captures system audio in background thread
- **FFT Analysis** (`rustfft` crate): Analyzes frequency content in real-time
- **Integration**: Audio energy levels fed to shader via uniform parameters

**Frequency Band Extraction:**
```
Sample Rate: 48000 Hz (typical)
FFT Size: 2048 samples
Frequency Resolution: 48000/2048 ≈ 23.4 Hz per bin

Bass (low energy):     20-250 Hz   (bins 1-11)
Mids (mid energy):     250-2000 Hz (bins 11-85)
Treble (high energy):  2000-8000 Hz (bins 85-341)

Energy levels normalized to [0, 1] range
Applied to shader parameters:
- beat_distortion_strength: 0→1 on beat
- beat_zoom_strength: 0→1 on beat
- base_brightness: modulated by overall energy
```

**Threading Model:**
```
Main Thread:
  └─ Frame loop (30 FPS)
     ├─ Check for config changes (file watcher)
     ├─ Update uniforms
     ├─ Dispatch compute shader
     ├─ Read audio energy levels (lock-free via Arc<AtomicF32>)
     ├─ Convert GPU output → ASCII
     └─ Render to terminal

Audio Thread (background):
  └─ Continuous audio capture
     ├─ Read samples from audio device
     ├─ Accumulate in ring buffer
     ├─ Compute FFT periodically
     ├─ Extract frequency bands
     └─ Update atomic energy values
```

### ASCII Conversion Process

**Algorithm (executed per pixel):**
```
For each pixel in output buffer:
  1. Load float RGBA from GPU output
  2. Convert float [0, 1] → u8 [0, 255]

  3. Calculate perceived brightness (Rec. 601 luma):
     brightness = 0.299*R + 0.587*G + 0.114*B

  4. Map brightness to palette character:
     palette = palettes[current_palette_type]
     char_index = (brightness / 255.0) * (palette.len() - 1)
     character = palette[floor(char_index)]

  5. Determine ANSI color:
     - Extract dominant color (max of R, G, B channels)
     - Map to 256-color ANSI codes
     - Or use 24-bit RGB if terminal supports it

  6. Create output string:
     output += format!("{}{}", color_code, character)
```

**Available Palettes (16 types):**
```
1. Blocks (solid shapes)
2. Density (various densities: ░▒▓█)
3. Gradient (smooth gradients)
4. Technical (ASCII symbols)
5. Smooth (cursive characters)
6. ASCII Art (full ASCII range)
... (16 total palettes in palette.rs)
```

### Terminal Rendering

**Rendering Loop:**
```rust
// Per frame:
for y in 0..height {
    for x in 0..width {
        pixel = ascii_buffer[y][x]
        terminal.move_cursor_to(x, y)
        terminal.set_foreground_color(pixel.color)
        terminal.print_char(pixel.character)
    }
    stdout.flush()
}
```

**Performance Characteristics:**
- **Bottleneck**: Terminal I/O via stdout
  - Many small write operations (~1,920 for 80×24)
  - Terminal emulator rendering overhead
  - Typically 60-80% of frame time

**Optimization Opportunities:**
- Batch cursor movements (use cursor positioning sequences)
- Build complete frame string before writing (reduces syscalls)
- Dirty region tracking (only redraw changed cells)
- Double buffering

### Application State Machine

**Main Application Struct (`App`):**
```rust
pub struct App {
    params: ShaderParams,           // Current shader parameters
    pipeline: ShaderPipeline,       // GPU compute pipeline
    converter: AsciiConverter,      // RGBA → ASCII converter
    running: bool,                  // Frame loop control
    show_status_bar: bool,          // UI flag
    last_frame_time: Instant,       // For frame rate limiting
    debug_log: DebugLog,            // Debug output
    last_terminal_size: (u16, u16), // Terminal dimensions
    config_watcher: Option<ConfigWatcher>, // File watching for reload
    custom_shader: Option<String>,  // Optional custom shader code
    audio_capture: Option<AudioCapture>,    // (audio feature)
    audio_analyzer: Option<AudioAnalyzer>,  // (audio feature)
}
```

**Frame Loop:**
```
1. Check terminal size (handle resize)
2. Poll config watcher for file changes
3. Apply audio energy to parameters (if audio enabled)
4. Update shader uniforms (params → GPU)
5. Dispatch compute shader
6. Read GPU output (async staging buffer)
7. Convert RGBA → ASCII
8. Render to terminal
9. Sleep to maintain 30 FPS
10. Poll keyboard input (Q/ESC to quit)
```

### Configuration System

**TOML Config Lifecycle:**
```
1. User edits config.toml
2. File watcher (notify crate) detects change
3. Main thread receives change event (async)
4. Config parser reads TOML file
5. Validation checks parameters
6. ShaderParams struct updated
7. Next frame uses new parameters
```

**Config Hashing & Saving:**
```
When user presses 'S':
  1. Serialize current params to TOML
  2. Compute SHA2 hash of params
  3. Generate filename: config_<hash>.toml
  4. Save to current working directory
  5. Display confirmation to user
```

**Priority Order (CLI args override config):**
```
1. Defaults (lowest)
   ↓
2. Randomized values (if -r/--random flag)
   ↓
3. Config file values (if -c/--config path)
   ↓
4. CLI arguments (highest priority)
```

### Shader Parameter Mapping

**Parameters have three representations:**
```
1. User Level (ShaderParams in src/params/shader_params.rs):
   - Named fields (e.g., "speed", "frequency")
   - User-friendly ranges
   - Serialized to TOML config files

2. GPU Level (Uniforms in src/shader_common/uniforms.wgsl):
   - WGSL struct with exact memory layout
   - Tight packing for efficient GPU transfer
   - #[repr(C)] equivalent for Rust struct

3. CLI Level (CliArgs in src/cli.rs):
   - Command-line arguments
   - Short and long forms
   - Type conversion and validation
```

**Example Parameter Flow:**
```
CLI: --speed 2.5
  ↓
CliArgs struct: { speed: Some(2.5), ... }
  ↓
ShaderParams struct: { speed: 2.5, ... }
  ↓
apply_cli_overrides() merges values
  ↓
ShaderParams → ShaderUniforms conversion
  ↓
uniforms.speed = 2.5 (f32)
  ↓
Uniform buffer uploaded to GPU
  ↓
Compute shader reads uniforms.speed
```

## Build & Setup

### Dependencies
**Runtime**:
- Vulkan ICD Loader
- Vulkan driver (Intel/AMD/NVIDIA)
- Terminal with ANSI color support

**Optional (audio feature)**:
- PipeWire (recommended) or ALSA
- libasound2

**Build**:
- Rust (cargo)
- Git

### Build Commands
```bash
# Visuals only
cargo build --release

# With audio reactivity (recommended)
cargo build --release --features audio

# Development build
cargo build
```

### Installation
```bash
sudo install -Dm755 target/release/chroma /usr/local/bin/chroma
```

## Configuration

### TOML Config Structure
```toml
[shader]
type = "plasma"  # or: noise, fractal, geometric, waves, etc.

[parameters]
speed = 1.0
frequency = 1.0
amplitude = 0.5
scale = 1.0
# More parameters available

[display]
palette = "standard"  # or: blocks, gradient, etc.
use_color = true
target_fps = 30

[audio]
enabled = false
# Audio-specific parameters
```

### Configuration Features
- **Config Files**: Load preset configs via CLI with `-c` flag
- **Live Reloading**: Edit config.toml while running - changes apply instantly
- **CLI Overrides**: Command-line parameters override config file values
- **Config Hashing**: Saved configs include SHA2 hash in filename for deduping
- **Custom Shaders**: Load custom WGSL shader with `--custom-shader shader.wgsl`

## Performance Characteristics

### Frame Time Breakdown (typical)
- **Terminal Rendering**: 60-80% (slowest bottleneck)
- **ASCII Conversion**: 15-20% (per-pixel processing)
- **GPU Compute**: 5-10% (fastest - highly parallel)

### Optimization Status
- ✅ GPU-accelerated for shader computation
- ✅ Efficient buffer management
- ⚠️ Sequential ASCII conversion (opportunity for parallelization with rayon)
- ⚠️ Many small terminal writes (opportunity for batching)

## Key Dependencies

| Crate | Purpose |
|-------|---------|
| `wgpu` | GPU graphics abstraction & compute shaders |
| `crossterm` | Terminal I/O & ANSI control |
| `cpal` | Audio device & stream management (optional) |
| `rustfft` | FFT computation for audio analysis (optional) |
| `serde`/`toml` | Config file serialization |
| `clap` | CLI argument parsing |
| `notify` | File watching for config reload |
| `rand` | Random parameter generation |
| `sha2` | Config file hashing |
| `unicode-width` | Terminal character width calculation |

## Keyboard Controls

| Key | Action |
|-----|--------|
| `Q` / `Esc` | Quit |
| `R` | Randomize parameters |
| `S` | Save configuration |
| `P` / `O` | Cycle palettes |
| `↑` / `↓` | Adjust frequency |
| `←` / `→` | Adjust speed |
| `+` / `-` | Adjust amplitude |
| `[` / `]` | Adjust scale |

See `notes/CONTROLS.md` for full details.

## Development Notes

### For AI Agents/Contributors

1. **Code Style**: Follow existing conventions and file structure
2. **Testing**: Write tests for new features
3. **Shaders**: WGSL compute shaders in `src/shader_patterns/` and `src/shader_common/`
4. **Config Changes**: Update related docs in `notes/` directory
5. **AI-Generated Code**: Welcome, but must be high quality and clean ❤️

### Key Files to Understand First
1. `ARCHITECTURE.md` - Detailed system design
2. `src/main.rs` - Entry point & main loop
3. `src/shader_common/main.wgsl` - GPU compute shader
4. `src/ascii/converter.rs` - ASCII conversion logic
5. `src/params/config.rs` - Parameter management

### Common Tasks
- **Add new shader pattern**: Create `src/shader_patterns/name.wgsl`, add to shader loading
- **Add new parameter**: Add to ShaderParams struct, update uniforms.wgsl, add CLI argument
- **Modify ASCII conversion**: Edit `src/ascii/converter.rs` and palette logic
- **Add audio feature**: Extend `src/audio/` modules

## Future Enhancements

- Hot shader reloading (watch `.wgsl` files)
- Shader transitions & blending
- Recording mode (frame capture, GIF/video export)
- More built-in shader patterns
- Parallel ASCII conversion
- Multi-window support

## Links & Resources

- **Repository**: https://github.com/yuri-xyz/chroma
- **Contributing**: See `CONTRIBUTING.md`
- **Build Details**: See `build.rs`
- **Examples**: Check `examples/` directory
- **Detailed Architecture**: See `notes/ARCHITECTURE.md`

---

## ⚠️ Documentation Staleness Disclaimer

This AGENTS.md file is a snapshot of the Chroma architecture and implementation. Over time, as the project evolves:
- Parameter names may change
- New features may be added
- Implementation details may be refactored
- File locations may shift

**For AI Agents Working on This Codebase:**
Always cross-reference this document with the actual source code. The codebase is the authoritative source. If you encounter:
- Discrepancies between this doc and the code → **trust the code**
- Outdated parameter descriptions → **check `src/params/`**
- Unclear shader behavior → **read `src/shader_common/*.wgsl` and `src/shader_patterns/*.wgsl`**
- Questions about application flow → **trace `src/app/mod.rs` and `src/main.rs`**

When making changes or contributions, update this document to reflect your changes, so the next AI agent has accurate information.
