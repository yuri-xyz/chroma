# Warped fBM Pattern

## Overview

A new pattern type based on warped fractional Brownian motion (fBM), inspired by Inigo Quilez's domain warping technique. This creates organic, flowing noise patterns with a unique "pink" aesthetic.

## Pattern Details

**Pattern Type:** `WarpedFbm` (Pattern ID: 17)

**Algorithm:**

- Uses domain warping by recursively offsetting noise coordinates
- Implements fBM with multiple octaves of noise at different frequencies
- Applies rotation matrix transformations between octaves for more organic results
- Formula: `fbm(position + fbm(position + fbm(position)))`

**Key Features:**

- 6 octaves of noise with decreasing amplitudes
- Rotation matrix transforms between octaves for variety
- Time-animated for smooth, flowing motion
- Custom colormap available (Color Mode: Warped)

## Custom Colormap

The pattern includes a custom colormap specifically designed for this effect:

**Color Mode:** `Warped` (Mode ID: 8)

This colormap uses piecewise linear functions for each RGB channel, creating a unique gradient that transitions smoothly from dark blues/purples through pinks to bright whites.

## Usage

### Keyboard Controls

- **T** - Next pattern (cycle forward to find WarpedFbm)
- **Y** - Previous pattern (cycle backward)
- **C** - Next color mode (cycle to find Warped colormap)

### Accessing the Pattern

1. Run the application: `cargo run --release`
2. Press **T** repeatedly to cycle through patterns until you reach "Warped"
3. Press **C** to cycle through color modes until you reach "Warped" for the original colormap
4. Adjust parameters as desired:
   - **Frequency** (↑/↓): Controls noise detail/scale
   - **Speed** (→/←): Animation speed
   - **Amplitude** (+/-): Intensity of the effect
   - **Scale** ([/]): Zoom level

### Recommended Settings

**Classic Look (with Warped colormap):**

```
Pattern: WarpedFbm
Color Mode: Warped
Frequency: 4.0-8.0
Speed: 0.3-0.5
Amplitude: 1.0-1.5
Scale: 1.0-2.0
```

**With Other Color Modes:**
The pattern also works beautifully with other color modes:

- **Rainbow**: Psychedelic flowing colors
- **Neon**: Electric, vibrant look
- **Pastel**: Soft, subtle variations
- **Monochrome**: Pure grayscale noise art

## Technical Implementation

### Shader Functions

1. **`warp_rand(n: vec2<f32>) -> f32`**

   - Pseudo-random hash function for 2D coordinates

2. **`warp_noise(p: vec2<f32>) -> f32`**

   - Smooth interpolated noise using bilinear filtering
   - Returns squared result for more contrast

3. **`warp_fbm(p: vec2<f32>, time: f32) -> f32`**

   - Fractional Brownian motion with 6 octaves
   - Uses rotation matrix between octaves
   - Time-based animation for first and last octaves

4. **`warp_pattern_value(p: vec2<f32>, time: f32) -> f32`**

   - Implements triple-nested domain warping
   - Creates the characteristic "organic" look

5. **`warped_fbm_pattern(uv: vec2<f32>, time: f32) -> vec2<f32>`**
   - Main pattern function
   - Scales UV coordinates by frequency
   - Returns value and gradient for color application

### Colormap Functions

- **`warp_colormap_red(x: f32) -> f32`**
- **`warp_colormap_green(x: f32) -> f32`**
- **`warp_colormap_blue(x: f32) -> f32`**
- **`warp_colormap(x: f32) -> vec3<f32>`**

These functions implement piecewise linear RGB curves for the custom gradient.

## Source

Based on techniques from:

- [Inigo Quilez - Domain Warping](https://iquilezles.org/articles/warp)
- Original ShaderToy implementation: "Base warp fBM"

## Performance

The pattern uses:

- 6 noise evaluations per octave
- 3 levels of domain warping
- Rotation matrix multiplications

Performance is similar to other noise-based patterns like Fractal and Noise patterns.
