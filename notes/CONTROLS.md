# Keyboard Controls

This document describes all keyboard controls available while running the shader application.

## Basic Controls

### `Q` or `Esc` - Quit

Exit the application.

### `R` - Randomize ⭐

Randomize all shader parameters within sensible ranges for interesting visual effects.

- Creates completely new looks instantly
- Parameters are randomized within safe, visually pleasing ranges
- Vignette has 30% chance of being enabled
- Great for discovering new shader combinations!

### `S` - Save Configuration 💾

Save the current configuration to a file in the working directory.

- Generates a unique filename based on parameter hash: `config_<hash>.toml`
- Won't overwrite if the same configuration already exists
- Config files can be loaded with `--config` flag on startup
- Share configs with others or keep your favorite settings!

## Parameter Adjustments

### Frequency (Wave Density)

- `↑` **Up Arrow** - Increase frequency (+0.1)
- `↓` **Down Arrow** - Decrease frequency (-0.1)
- Range: 3.0 → 18.0
- Effect: More/fewer wave oscillations

### Speed (Animation Speed)

- `→` **Right Arrow** - Increase speed (+0.1)
- `←` **Left Arrow** - Decrease speed (-0.1)
- Range: 0.0 → 1.0
- Effect: Faster/slower animation

### Amplitude (Wave Height)

- `+` or `=` - Increase amplitude (+0.1)
- `-` or `_` - Decrease amplitude (-0.1)
- Range: 0.0 → 2.0
- Effect: More/less extreme color variations

### Scale (Zoom)

- `]` - Increase scale (+0.1) - Zoom out
- `[` - Decrease scale (-0.1) - Zoom in
- Range: 0.1 → 5.0
- Effect: Change pattern size

### Pattern Type

- `T` - Next pattern
- `Y` - Previous pattern
- Effect: Cycle through different visual patterns (Plasma, Waves, Ripples, Vortex, Noise, Geometric, Voronoi, Truchet, Hexagonal, Interference, Fractal, Glitch, Spiral, Rings, Grid, Diamonds, Sphere, WarpedFbm)

### Color Mode

- `C` - Next color mode
- Effect: Cycle through color schemes (Rainbow, Monochrome, Duotone, Warm, Cool, Neon, Pastel, Cyberpunk, Warped, Chromatic)

### Palette

- `P` - Next palette
- `O` - Previous palette
- Effect: Change ASCII character set and color rendering

## Status Bar

The status bar at the bottom shows:

```
Freq: 10.0 | Speed: 0.5 | Amp: 1.0 | Scale: 1.0 | [Q]uit [R]andom [↑↓←→] [+/-] [[]]
```

Real-time display of current parameter values and available controls.

## Tips & Tricks

### Finding Cool Effects

1. Press `R` multiple times to explore different combinations
2. Fine-tune using arrow keys and +/- once you find something you like
3. High frequency + low scale = Dense, detailed patterns
4. Low frequency + high scale = Large, flowing waves

### Performance

- If animation is choppy, try:
  - Decreasing frequency (less computation)
  - Smaller terminal window
  - Reducing speed

### Visual Styles

**Psychedelic**:

- Frequency: High (15-18)
- Speed: Fast (0.7-1.0)
- Amplitude: High (1.5-2.0)

**Calm/Ambient**:

- Frequency: Low (3-6)
- Speed: Slow (0.1-0.3)
- Amplitude: Medium (0.8-1.2)

**Detailed**:

- Frequency: High (12-18)
- Scale: Low (0.3-0.8)
- Speed: Any

**Flowing Waves**:

- Frequency: Medium (6-10)
- Scale: High (2.0-4.0)
- Speed: Medium (0.4-0.6)

## Future Controls (Not Yet Implemented)

Additional controls that could be added:

- `B`/`b` - Brightness up/down
- `C`/`c` - Contrast up/down
- `H`/`h` - Hue shift
- `S`/`s` - Saturation up/down
- `V`/`v` - Toggle vignette
- `P`/`p` - Cycle through palettes
- `Space` - Pause/resume animation
- `0-9` - Save/load presets

## Config File

To save your favorite settings, parameters can be loaded from `config.toml` (if implemented).

Example:

```toml
frequency = 12.0
speed = 0.6
amplitude = 1.2
scale = 1.5
brightness = 1.3
```
