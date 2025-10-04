# Shader Development Guide

## Current Shaders

### Plasma (plasma.wgsl)

A classic plasma effect using sine waves.

**Parameters:**

- `frequency`: Controls wave density
- `amplitude`: Controls color intensity
- `speed`: Animation speed
- `scale`: Zoom level
- `color_shift`: Hue rotation

**Algorithm:**
Combines three sine wave patterns with different phase offsets to create organic, flowing patterns.

## Future Shader Ideas

### 1. Mandelbrot Set

Classic fractal visualization with zoom and pan controls.

**New Parameters Needed:**

- `zoom_level: f32`
- `center_x: f32`
- `center_y: f32`
- `max_iterations: u32`

**Implementation Notes:**

```wgsl
fn mandelbrot(c: vec2<f32>, max_iter: u32) -> f32 {
    var z = vec2<f32>(0.0, 0.0);
    var iter = 0u;

    while (iter < max_iter && length(z) < 2.0) {
        z = vec2<f32>(
            z.x * z.x - z.y * z.y + c.x,
            2.0 * z.x * z.y + c.y
        );
        iter += 1u;
    }

    return f32(iter) / f32(max_iter);
}
```

### 2. Perlin/Simplex Noise

Smooth, natural-looking noise patterns.

**New Parameters Needed:**

- `noise_scale: f32`
- `noise_octaves: u32`
- `persistence: f32`
- `lacunarity: f32`

**Use Cases:**

- Terrain generation
- Cloud patterns
- Organic textures
- Marble effects

### 3. Ray Marching

3D scene rendering using distance fields.

**New Parameters Needed:**

- `camera_position: vec3<f32>`
- `camera_rotation: vec3<f32>`
- `fov: f32`
- `max_steps: u32`
- `max_distance: f32`

**Possible Scenes:**

- Rotating cube
- Sphere with lighting
- Tunnel effect
- Abstract 3D shapes

### 4. Cellular Automata

Conway's Game of Life and variants.

**New Parameters Needed:**

- `cell_size: f32`
- `birth_rule: u32` (bit mask)
- `survival_rule: u32` (bit mask)
- `random_seed: u32`

**Implementation:**

- Use storage buffer for cell state
- Update rules in compute shader
- Visualize cell states

### 5. Voronoi Diagram

Cell-based patterns with customizable metrics.

**New Parameters Needed:**

- `num_points: u32`
- `distance_metric: u32` (0=euclidean, 1=manhattan, 2=chebyshev)
- `animation_type: u32`

**Visual Effects:**

- Crystal patterns
- Cell division
- Organic boundaries

### 6. Tunnel Effect

Classic demoscene effect.

**New Parameters Needed:**

- `tunnel_depth: f32`
- `twist_amount: f32`
- `texture_scroll: f32`

### 7. Shader Toy Ports

Many shaders from shadertoy.com can be adapted.

**Conversion Notes:**

- Replace `iTime` with `uniforms.time`
- Replace `iResolution` with `uniforms.resolution`
- Replace `fragCoord` with computed UV coordinates
- Convert `mainImage` to compute shader output

### 8. Audio-Reactive Visualizers

**Frequency Domain:**

```wgsl
// Bass (20-250 Hz) - affects amplitude
// Mids (250-4000 Hz) - affects color
// Treble (4000-20000 Hz) - affects detail
```

**Waveform Domain:**

- Circular waveform
- Oscilloscope-style display
- Particle system driven by amplitude

## Shader Best Practices

### Performance

1. **Minimize texture lookups** - Use procedural generation when possible
2. **Avoid branches** - Use step() and smoothstep() instead of if/else
3. **Optimize loops** - Keep iteration counts low or use early exits
4. **Workgroup size** - 8x8 is a good default for 2D compute shaders

### Code Style

1. **Use descriptive names** for functions and variables
2. **Comment complex math** - Explain the algorithm
3. **Modularize** - Break complex effects into helper functions
4. **Test incrementally** - Build up complex shaders from simple parts

### Color Management

```wgsl
// HSV to RGB conversion
fn hsv_to_rgb(h: f32, s: f32, v: f32) -> vec3<f32> {
    let c = v * s;
    let x = c * (1.0 - abs((h / 60.0) % 2.0 - 1.0));
    let m = v - c;

    var rgb: vec3<f32>;
    if (h < 60.0) {
        rgb = vec3<f32>(c, x, 0.0);
    } else if (h < 120.0) {
        rgb = vec3<f32>(x, c, 0.0);
    } else if (h < 180.0) {
        rgb = vec3<f32>(0.0, c, x);
    } else if (h < 240.0) {
        rgb = vec3<f32>(0.0, x, c);
    } else if (h < 300.0) {
        rgb = vec3<f32>(x, 0.0, c);
    } else {
        rgb = vec3<f32>(c, 0.0, x);
    }

    return rgb + vec3<f32>(m, m, m);
}
```

### Noise Functions

```wgsl
// Pseudo-random 2D noise
fn random(uv: vec2<f32>) -> f32 {
    return fract(sin(dot(uv, vec2<f32>(12.9898, 78.233))) * 43758.5453);
}

// Smooth noise
fn noise(uv: vec2<f32>) -> f32 {
    let i = floor(uv);
    let f = fract(uv);

    let a = random(i);
    let b = random(i + vec2<f32>(1.0, 0.0));
    let c = random(i + vec2<f32>(0.0, 1.0));
    let d = random(i + vec2<f32>(1.0, 1.0));

    let u = f * f * (3.0 - 2.0 * f);

    return mix(a, b, u.x) + (c - a) * u.y * (1.0 - u.x) + (d - b) * u.x * u.y;
}
```

## Adding a Custom Shader

### Step 1: Create Shader File

Create `src/shader/shaders/my_shader.wgsl`

### Step 2: Update ShaderPipeline

```rust
// In shader/pipeline.rs
pub enum ShaderType {
    Plasma,
    MyShader,
}

impl ShaderPipeline {
    pub fn new_with_shader(width: u32, height: u32, shader_type: ShaderType) -> Result<Self> {
        // Load appropriate shader based on type
        let shader_source = match shader_type {
            ShaderType::Plasma => include_str!("shaders/plasma.wgsl"),
            ShaderType::MyShader => include_str!("shaders/my_shader.wgsl"),
        };
        // ...
    }
}
```

### Step 3: Add Parameters

```rust
// In params/config.rs
pub struct ShaderParams {
    // ... existing fields
    pub my_custom_param: f32,
}

// In shader/uniforms.rs
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct ShaderUniforms {
    // ... existing fields
    pub my_custom_param: f32,
}
```

### Step 4: Add Controls

```rust
// In main.rs, handle_input()
KeyCode::Char('c') => {
    self.params.my_custom_param += 0.1;
}
```

### Step 5: Test

```bash
cargo run --release
```

## Resources

- [WebGPU Shading Language Spec](https://www.w3.org/TR/WGSL/)
- [Shadertoy](https://www.shadertoy.com/) - Shader examples (needs conversion)
- [The Book of Shaders](https://thebookofshaders.com/) - Learning resource
- [Inigo Quilez Articles](https://iquilezles.org/articles/) - Advanced techniques
