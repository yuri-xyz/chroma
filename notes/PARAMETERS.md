# Shader Parameters Reference

This document describes all available shader parameters, their ranges, and default values.

## Core Animation Parameters

### frequency

- **Range**: 3.0 → 18.0
- **Default**: 10.0
- **Description**: Controls the spatial frequency of the plasma waves. Higher values create more oscillations.

### speed

- **Range**: 0.0 → 1.0
- **Default**: 0.5
- **Description**: Animation speed multiplier. Controls how fast the shader animates over time.

### amplitude

- **Range**: 0.0 → 2.0
- **Default**: 1.0
- **Description**: Wave amplitude. Higher values create more extreme variations.

### scale

- **Range**: 0.1 → 5.0
- **Default**: 1.0
- **Description**: UV coordinate scale. Lower values zoom in, higher values zoom out.

### color_shift

- **Range**: 0.0 → 6.28 (2π)
- **Default**: 0.0
- **Description**: Phase shift for color generation.

## Distortion & Noise Parameters

### noise_strength

- **Range**: 0.0 → 0.5
- **Default**: 0.15
- **Description**: Strength of noise applied to the pattern.

### distort_amplitude

- **Range**: 0.0 → 2.0
- **Default**: 0.5
- **Description**: Amount of spatial distortion applied to circular patterns.

### noise_scale

- **Range**: 0.000 → 0.010
- **Default**: 0.005
- **Description**: Scale of noise patterns. Smaller values create finer noise.

### z_rate

- **Range**: 0.000 → 0.100
- **Default**: 0.02
- **Description**: Rate of change in the Z dimension (for 3D noise evolution).

## Color Adjustment Parameters

### brightness

- **Range**: 0.0 → 2.0
- **Default**: 1.2
- **Description**: Overall brightness multiplier. 1.0 is neutral, <1.0 darkens, >1.0 brightens.

### contrast

- **Range**: 0.2 → 2.0
- **Default**: 1.0
- **Description**: Contrast adjustment. 1.0 is neutral, <1.0 reduces contrast, >1.0 increases contrast.

### hue

- **Range**: 0 → 360 (degrees)
- **Default**: 0
- **Description**: Hue rotation in degrees. Shifts all colors around the color wheel.

### saturation

- **Range**: 0.0 → 2.0
- **Default**: 1.0
- **Description**: Color saturation. 0.0 is grayscale, 1.0 is neutral, >1.0 is hyper-saturated.

### gamma

- **Range**: 0.5 → 2.0
- **Default**: 1.0
- **Description**: Gamma correction. <1.0 brightens midtones, >1.0 darkens midtones.

## Visual Effects Parameters

### vignette

- **Range**: 0.0 → 1.0
- **Default**: 0.0
- **Description**: Vignette intensity. 0.0 is disabled, higher values darken edges more.

### vignette_softness

- **Range**: 0.0 → 1.0
- **Default**: 0.5
- **Description**: Softness of vignette transition. Higher values create smoother transitions.

### glyph_sharpness

- **Range**: 0.5 → 2.0
- **Default**: 1.0
- **Description**: Sharpness applied to ASCII characters. (Currently reserved for future use)

## Background Parameters

### background_tint_r

- **Range**: 0.0 → 1.0
- **Default**: 0.0
- **Description**: Red component of background tint (used with vignette).

### background_tint_g

- **Range**: 0.0 → 1.0
- **Default**: 0.0
- **Description**: Green component of background tint (used with vignette).

### background_tint_b

- **Range**: 0.0 → 1.0
- **Default**: 0.0
- **Description**: Blue component of background tint (used with vignette).

## Legacy Parameters

### octaves

- **Type**: u32
- **Default**: 4
- **Description**: Number of octaves for multi-octave noise. (Reserved for future use)

## Audio Parameters

### audio_enabled

- **Type**: bool
- **Default**: false
- **Description**: Enable audio reactivity (requires `audio` feature).

### bass_influence

- **Range**: 0.0 → 1.0
- **Default**: 0.5
- **Description**: How much bass frequencies affect amplitude.

### mid_influence

- **Range**: 0.0 → 1.0
- **Default**: 0.3
- **Description**: How much mid frequencies affect color shift.

### treble_influence

- **Range**: 0.0 → 1.0
- **Default**: 0.2
- **Description**: How much treble frequencies affect frequency.

## Usage Examples

### High Contrast Psychedelic

```rust
params.frequency = 15.0;
params.speed = 0.8;
params.brightness = 1.5;
params.contrast = 1.8;
params.saturation = 1.5;
```

### Subtle Ambient

```rust
params.frequency = 5.0;
params.speed = 0.2;
params.brightness = 0.8;
params.contrast = 0.7;
params.saturation = 0.6;
```

### Monochrome Dream

```rust
params.saturation = 0.0;
params.contrast = 1.5;
params.vignette = 0.3;
params.vignette_softness = 0.7;
```
