# Architecture Overview

## Design Philosophy

**Simple, Direct, Efficient**

This project avoids unnecessary complexity:

- No TUI framework - just render shader output
- Config-driven parameters, not interactive menus
- Direct terminal output using crossterm
- GPU does the heavy lifting

## Component Architecture

```
┌──────────────────────────────────────────────────────────┐
│                   Main Application                       │
│                                                          │
│  ┌────────────┐  ┌──────────────┐  ┌─────────────┐     │
│  │   Config   │  │  File Watch  │  │   Keyboard  │     │
│  │   Loader   │  │   (notify)   │  │   Input     │     │
│  └──────┬─────┘  └──────┬───────┘  └──────┬──────┘     │
│         │                │                  │            │
│         └────────────────┴──────────────────┘            │
│                          ↓                               │
│                  ┌───────────────┐                       │
│                  │  ShaderParams │                       │
│                  └───────┬───────┘                       │
│                          ↓                               │
│                  ┌───────────────┐                       │
│                  │ShaderUniforms │                       │
│                  └───────┬───────┘                       │
└──────────────────────────┼──────────────────────────────┘
                           ↓
┌──────────────────────────────────────────────────────────┐
│                  GPU Shader Pipeline                     │
│                                                          │
│  ┌──────────────┐       ┌──────────────┐               │
│  │   Uniforms   │  →    │   Compute    │               │
│  │   Buffer     │       │   Shader     │               │
│  └──────────────┘       └──────┬───────┘               │
│                                 ↓                        │
│                         ┌──────────────┐                │
│                         │   Output     │                │
│                         │   Buffer     │                │
│                         └──────┬───────┘                │
│                                ↓                         │
│                         ┌──────────────┐                │
│                         │   Staging    │                │
│                         │   Buffer     │                │
│                         └──────┬───────┘                │
└────────────────────────────────┼────────────────────────┘
                                 ↓
                         [RGBA Pixel Data]
                                 ↓
┌──────────────────────────────────────────────────────────┐
│                  ASCII Conversion                        │
│                                                          │
│  ┌──────────────┐       ┌──────────────┐               │
│  │   Brightness │  →    │   Palette    │               │
│  │  Calculation │       │   Mapping    │               │
│  └──────────────┘       └──────┬───────┘               │
│                                 ↓                        │
│                         ┌──────────────┐                │
│                         │   ANSI Color │                │
│                         │   Selection  │                │
│                         └──────┬───────┘                │
└────────────────────────────────┼────────────────────────┘
                                 ↓
                        [ASCII Frame Data]
                                 ↓
┌──────────────────────────────────────────────────────────┐
│                  Terminal Rendering                      │
│                                                          │
│  • Move cursor to (0,0)                                  │
│  • For each character:                                   │
│    - Set foreground color                                │
│    - Print character                                     │
│  • Flush stdout                                          │
└──────────────────────────────────────────────────────────┘
```

## Data Flow

### Frame Rendering Cycle (30 FPS)

```
1. Check for config file changes
   ↓
2. Load updated parameters (if changed)
   ↓
3. Update time uniform
   ↓
4. Convert params → uniforms (CPU)
   ↓
5. Upload uniforms to GPU
   ↓
6. Dispatch compute shader (GPU)
   ↓
7. Copy output buffer → staging buffer
   ↓
8. Map staging buffer (GPU → CPU)
   ↓
9. Convert float RGBA → u8 RGBA
   ↓
10. For each pixel:
    - Calculate brightness
    - Map to ASCII character
    - Determine ANSI color
   ↓
11. Render to terminal
   ↓
12. Sleep until next frame
```

### Config Reload Flow

```
User edits config.toml
   ↓
File system notifies app (notify crate)
   ↓
Parse TOML file
   ↓
Validate parameters
   ↓
Update ShaderParams
   ↓
Next frame uses new parameters
```

## Module Responsibilities

### `main.rs`

- Application entry point
- Main render loop
- Terminal setup/cleanup
- Input handling (quit only)
- Frame timing

### `shader/`

**pipeline.rs**

- wgpu device/queue initialization
- Shader module loading
- Buffer management
- Compute pipeline setup
- Frame rendering

**uniforms.rs**

- GPU data structures
- Params → Uniforms conversion
- Memory layout (#[repr(C)])

### `params/`

**config.rs**

- ShaderParams struct
- Default values
- Parameter update logic
- Audio parameter application

**loader.rs** (TODO)

- TOML file parsing
- Parameter validation
- Config watching
- Error handling

### `ascii/`

**converter.rs**

- RGBA → brightness calculation
- Character + color generation
- Frame conversion

**palette.rs**

- ASCII character sets
- Brightness → character mapping
- Multiple palette types

### `render/`

**framebuffer.rs**

- Frame buffer abstraction
- Buffer management
- Resize handling

### `audio/` (Future)

**input.rs**

- Audio device selection
- Audio stream management
- Sample buffering

**fft.rs**

- FFT computation
- Frequency band extraction
- Bass/mid/treble levels

## GPU Pipeline Details

### Compute Shader Workgroups

```
Workgroup Size: 8x8 (64 threads per group)

For resolution 80x24:
- workgroup_count_x = ceil(80/8) = 10
- workgroup_count_y = ceil(24/8) = 3
- Total workgroups: 30
- Total threads: 1,920 (for 1,920 pixels)
```

### Buffer Sizes

```
Resolution: W × H pixels

Uniform Buffer:
  size = sizeof(ShaderUniforms)
  size = 48 bytes (with padding)

Output Buffer (GPU):
  size = W × H × 4 floats × 4 bytes
  Example (80×24): 30,720 bytes

Staging Buffer (CPU-accessible):
  size = same as output buffer
  Used for GPU → CPU transfer

Final RGBA Buffer:
  size = W × H × 4 bytes
  Example (80×24): 7,680 bytes
```

## Performance Characteristics

### Bottlenecks

1. **Terminal Rendering** (slowest)

   - Many `execute!` calls
   - Terminal emulator speed
   - ~60-80% of frame time

2. **ASCII Conversion** (medium)

   - Per-pixel brightness calculation
   - Character/color selection
   - ~15-20% of frame time

3. **GPU Compute** (fastest)
   - Highly parallel
   - Simple shader
   - ~5-10% of frame time

### Optimization Opportunities

**Current:**

- Sequential ASCII conversion (single-threaded)
- Many small terminal writes

**Future:**

- Parallel ASCII conversion (rayon)
- Batch terminal output
- Double buffering
- Dirty region tracking

## Error Handling

### Initialization Errors

- GPU not found → Clear error message
- Shader compilation → Show WGSL errors
- Config parsing → Show line/column

### Runtime Errors

- Frame render failure → Skip frame, log error
- Config reload error → Keep old config, warn user
- Buffer map failure → Retry or panic

## Configuration Format

### TOML Structure

```toml
[shader]
type = "plasma"

[parameters]
speed = 1.0
frequency = 1.0
# ... more params

[display]
palette = "standard"
use_color = true
target_fps = 30

[audio]
enabled = false
# ... audio params
```

## Future Enhancements

### Hot Shader Reloading

- Watch `.wgsl` files
- Recompile on change
- Swap pipelines atomically

### Multiple Shaders

- Enum for shader types
- Match on shader type for loading
- Separate parameter sets per shader

### Shader Transitions

- Blend between two shaders
- Configurable transition duration
- Various blend modes

### Recording Mode

- Capture frames to buffer
- Export as image sequence
- Generate animated GIFs
- Video encoding
