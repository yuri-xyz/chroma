# Usage Guide

## Quick Start

```bash
# Build and run
cargo run --release
```

## Controls

### Basic Controls

- **Q** or **ESC**: Quit the application

### Parameter Adjustment

Edit `config.toml` in another editor while the app runs:

- Changes are detected automatically
- Parameters update in real-time
- No need to restart the application

## Parameters Explained

### Frequency

Controls the density/repetition of patterns in the shader.

- Range: 0.1 to ∞
- Default: 1.0
- Effect: Higher values create more detailed, tighter patterns

### Speed

Controls how fast the shader animation progresses.

- Range: 0.1 to ∞
- Default: 1.0
- Effect: Higher values make animations move faster

### Amplitude

Controls the intensity/contrast of the shader effect.

- Range: 0.1 to ∞
- Default: 1.0
- Effect: Higher values create more pronounced color variations

### Scale

Controls the zoom level of the shader pattern.

- Range: 0.1 to ∞
- Default: 1.0
- Effect: Lower values zoom in, higher values zoom out

### Gravity

Controls downward pattern scroll. Default is off (`0.0`).

- Range: 0.0 to 2.0
- CLI: `--gravity FLOAT`, `--mouse-fight FLOAT`
- Keys: `g` / `G`
- Mouse movement and click fight the fall but cannot cancel it (capped below full gravity). Interactive sessions enable mouse capture so live gravity adjustments can receive pointer input.

## ASCII Palettes

The application supports multiple ASCII palettes:

### Standard (Default)

` .:-=+*#%@`

- Good balance between detail and clarity
- Works well with most shaders

### Extended

Full range of ASCII characters for maximum detail

- Best for high-resolution terminals
- May be harder to read

### Simple

` .oO@`

- Minimalist approach
- Good for low-resolution or slower terminals

### Blocks

` ░▒▓█`

- Uses Unicode block characters
- Creates smooth gradients
- Requires Unicode support

## Performance Tips

1. **Resolution**: The shader resolution matches your terminal size. Smaller terminals = better performance.

2. **Frame Rate**: Default target is 60 FPS. Use `--fps <value>` for different performance profiles.

3. **GPU Support**: The application requires a GPU with wgpu support (Vulkan, Metal, DX12, or WebGPU).

4. **Terminal**: Use a GPU-accelerated terminal emulator for best performance:
   - Alacritty
   - Kitty
   - WezTerm

## Stream Mode

Use `--stream WIDTHxHEIGHT` to emit fixed-size frames to stdout for embedding in another terminal application:

```bash
chroma --stream 80x24
```

Stream mode skips terminal setup, the status bar, and interactive input. By default, `--stream` uses the legacy ANSI stream format: each frame contains ANSI-colored rows followed by a blank line delimiter.

Use `--stream-format ansi` or `--stream-format cells` to opt into the framed protocol. In framed modes, each frame starts with a header:

```text
CHROMA_FRAME v=1 frame=<index> width=<w> height=<h> format=<format> encoding=utf-8 bytes=<payload_bytes>
```

The header is followed by exactly `bytes` UTF-8 payload bytes. `--stream-format ansi` preserves ANSI-colored rows. `--stream-format cells` emits one tab-separated record per cell: `x`, `y`, display width, Unicode code point, foreground RGB hex or `-`, and background RGB hex or `-`.

## Creating Custom Shaders

### WGSL Shader Format

Shaders are written in WGSL (WebGPU Shading Language). See `src/shader/shaders/plasma.wgsl` for an example.

Basic structure:

```wgsl
struct Uniforms {
    time: f32,
    resolution: vec2<f32>,
    // ... your custom parameters
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@group(0) @binding(1)
var<storage, read_write> output_buffer: array<vec4<f32>>;

@compute @workgroup_size(8, 8)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    // Your shader logic here
    // Output RGB color to output_buffer
}
```

### Adding a New Shader

1. Create a new `.wgsl` file in `src/shader/shaders/`
2. Update `ShaderPipeline` to load your shader
3. Add any new uniforms to `ShaderParams` and `ShaderUniforms`
4. Rebuild and run!

## Audio Visualization

The application will:

1. Capture system audio using PulseAudio/PipeWire on Linux, with CPAL fallback
2. Perform FFT analysis
3. Map frequency bands to shader parameters:
   - Bass → Amplitude
   - Mids → Color Shift
   - Treble → Frequency

## Troubleshooting

### GPU Not Found

- Ensure your GPU drivers are up to date
- Check wgpu backend support for your platform
- Try different wgpu backends (Vulkan, Metal, DX12)

### Low Frame Rate

- Reduce terminal size
- Run with a lower `--fps` value
- Use a simpler ASCII palette
- Use a GPU-accelerated terminal

### Colors Not Showing

- Ensure your terminal supports ANSI color codes
- Try a different terminal emulator
- Check terminal color scheme settings

### Shader Not Updating

- Check that parameters are being modified
- Verify GPU is processing compute shaders
- Look for errors in terminal output

## Examples

### Creating Slow, Subtle Animation

```
Speed: 0.3
Frequency: 0.5
Amplitude: 0.8
Scale: 2.0
```

### Creating Fast, Chaotic Patterns

```
Speed: 3.0
Frequency: 5.0
Amplitude: 2.0
Scale: 0.5
```

### Creating Large, Smooth Waves

```
Speed: 1.0
Frequency: 0.2
Amplitude: 1.5
Scale: 5.0
```
