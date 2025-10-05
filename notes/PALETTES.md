# ASCII Palettes

This document describes all available ASCII palettes for rendering shaders. Each palette uses different characters to represent brightness levels from dark to bright.

## Available Palettes

### `standard()` - Classic ASCII

**Characters**: ` . : - = + * # % @`

Traditional ASCII art characters. Good compatibility, works everywhere.

- Best for: Classic look, maximum compatibility
- Character count: 10
- Visual style: Traditional text art

### `blocks()` - Block Elements

**Characters**: ` ░ ▒ ▓ █`

Unicode block elements with increasing density. Very smooth gradients.

- Best for: Smooth gradients, clean look
- Character count: 5
- Visual style: Smooth, modern
- Requires: Unicode support

### `circles()` - Circle Glyphs ⭐ **Recommended**

**Characters**: ` · ∘ ○ ◌ ◍ ◎ ◉ ● ⬤`

Progressive circle characters from empty to filled. Beautiful for plasma effects!

- Best for: Plasma/wave shaders, organic patterns
- Character count: 10
- Visual style: Smooth, flowing, organic
- Requires: Nerd Font or good Unicode support

### `smooth()` - Smooth Shapes

**Characters**: ` · ∘ ○ ◌ ◍ ◎ ● ⬤ ⬛`

Mix of circles and squares for ultra-smooth gradients.

- Best for: High-quality rendering, detailed patterns
- Character count: 10
- Visual style: Professional, high-fidelity
- Requires: Nerd Font recommended

### `braille()` - Braille Patterns

**Characters**: ` ⠁ ⠃ ⠇ ⠏ ⠟ ⠿ ⡿ ⣿`

Progressive braille characters. Dense, unique texture.

- Best for: Fine detail, technical aesthetic
- Character count: 9
- Visual style: Dense, textured, unique
- Requires: Good Unicode/Nerd Font

### `geometric()` - Geometric Shapes

**Characters**: ` ▪ ▫ ◽ ◾ ◼ ◻ ■ ⬛`

Squares of varying sizes and fills. Sharp, geometric look.

- Best for: Angular patterns, geometric shaders
- Character count: 9
- Visual style: Sharp, geometric, modern
- Requires: Unicode support

### `mixed()` - Mixed Styles

**Characters**: ` · ∘ ░ ▒ ▓ ● ◉ █ ⬛`

Combination of circles, blocks, and squares. Very versatile.

- Best for: Complex patterns, varied textures
- Character count: 10
- Visual style: Rich, varied, interesting
- Requires: Nerd Font recommended

### `dots()` - Dot Progression

**Characters**: ` ⡀ ⡄ ⡆ ⡇ ⣇ ⣧ ⣷ ⣿`

Braille-based dot patterns building from bottom-right.

- Best for: Subtle transitions, fine grain
- Character count: 9
- Visual style: Subtle, fine-grained
- Requires: Braille support

### `extended()` - Full ASCII Set

**Characters**: 70+ ASCII and special characters

Massive range of ASCII characters for extremely detailed rendering.

- Best for: High detail requirements, ASCII art
- Character count: 70+
- Visual style: Highly detailed, traditional ASCII art
- Requires: Standard ASCII (maximum compatibility)

### `simple()` - Minimal

**Characters**: ` . o O @`

Just 5 characters for low-detail or retro look.

- Best for: Retro aesthetic, low bandwidth, debugging
- Character count: 5
- Visual style: Minimalist, retro

## Usage

### In Code

```rust
use chroma::ascii::{AsciiConverter, AsciiPalette};

// Use the recommended circles palette (default in main.rs)
let converter = AsciiConverter::new(AsciiPalette::circles(), true);

// Or try other palettes
let converter = AsciiConverter::new(AsciiPalette::blocks(), true);
let converter = AsciiConverter::new(AsciiPalette::braille(), true);
let converter = AsciiConverter::new(AsciiPalette::geometric(), true);
```

### Changing Default Palette

Edit `src/main.rs` line 57:

```rust
let converter = AsciiConverter::new(AsciiPalette::circles(), true);
//                                    ^^^^^^^^^^^^^^^^^^^^^^
//                                    Change this!
```

## Visual Comparison

Here's how different brightness levels map to characters:

```
Brightness:  0%   11%  22%  33%  44%  55%  66%  77%  88%  100%
standard:    ' '  '.'  ':'  '-'  '='  '+'  '*'  '#'  '%'  '@'
blocks:      ' '  '░'  '▒'  '▓'  '█'
circles:     ' '  '·'  '∘'  '○'  '◌'  '◍'  '◎'  '◉'  '●'  '⬤'
braille:     ' '  '⠁'  '⠃'  '⠇'  '⠏'  '⠟'  '⠿'  '⡿'  '⣿'
geometric:   ' '  '▪'  '▫'  '◽'  '◾'  '◼'  '◻'  '■'  '⬛'
```

## Recommendations by Shader Type

- **Plasma/Waves**: `circles()` or `smooth()` - Organic, flowing shapes
- **Noise/Static**: `braille()` or `dots()` - Fine, textured appearance
- **Geometric**: `geometric()` or `blocks()` - Sharp edges
- **Classic**: `standard()` - Traditional ASCII art look
- **High Performance**: `simple()` or `blocks()` - Fewer characters = faster

## Font Requirements

**Best Experience**: Install a [Nerd Font](https://www.nerdfonts.com/) for full glyph support.

Popular choices:

- JetBrains Mono Nerd Font
- Fira Code Nerd Font
- Hack Nerd Font
- Cascadia Code Nerd Font

Most palettes will work with standard Unicode fonts, but Nerd Fonts provide the best rendering quality and symbol coverage.
