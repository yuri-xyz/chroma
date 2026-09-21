# Palettes

A palette is the set of characters Chroma draws with. Every pixel the shader produces has a brightness, and the palette maps that brightness to a character: the darkest pixels become a space, the brightest become the densest glyph, and everything else lands in between. Colour is applied separately, so the palette only decides texture.

Choose one with `-P, --palette`, set `palette` in a [config file](./CONFIG_SAVE_LOAD.md), or press `P` while Chroma runs to step through them. `chroma --list-palettes` prints the names.

### Available palettes

Characters are listed from darkest to brightest. Every palette starts with a space.

| Name | Characters | Look |
| --- | --- | --- |
| `standard` | `. : - = + * # % @` | The classic ASCII-art ramp. Works in every terminal and font. |
| `simple` | `. o O @` | Four glyphs for a bold, retro look. This is the default. |
| `extended` | 70 ASCII characters, from `.` to `$` | The finest brightness steps available in pure ASCII. Detailed but busy. |
| `blocks` | `░ ▒ ▓ █` | Shaded blocks. Smooth and solid, with no visible glyph shapes. |
| `shades` | `░ ░ ▒ ▒ ▓ ▓ █ █` | The same blocks with each step doubled, which gives broader, flatter bands. |
| `circles` | `· ∘ ○ ◌ ◍ ◎ ◉ ● █` | Rings that fill in as they brighten. Organic, and well suited to plasma and waves. |
| `smooth` | `· ∘ ○ ◌ ◍ ◎ ◉ ● █` | Currently identical to `circles`. |
| `mixed` | `· ∘ ░ ▒ ▓ ● ◉ ■ █` | Dots, shades, and solids together for a rich, varied texture. |
| `braille` | `⠁ ⠃ ⠇ ⠏ ⠟ ⠿ ⡿ ⣿` | Braille cells that fill dot by dot. Fine-grained and technical. |
| `dots` | `⡀ ⡄ ⡆ ⡇ ⣇ ⣧ ⣷ ⣿` | Braille cells that fill from the bottom up, like a level meter. |
| `geometric` | `▪ ▫ ▬ ▭ ▮ ▯ ■ █` | Small squares and bars. Sharp and angular. |
| `lines` | `╌ ╍ ┄ ┅ ┈ ┉ ━ █` | Dashed horizontal strokes of growing weight. Gives a scanline feel. |
| `boxdraw` | `─ ━ │ ┃ ┼ ╋ ╬ █` | Box-drawing strokes and crosses. Looks like circuitry. |
| `triangles` | `▵ ▴ ▿ ▾ ◂ ◃ ▸ ▹` | Small triangles pointing in every direction. |
| `arrows` | `› » ⟩ → ⇒ ⟹ ⟾ ▶` | Arrows of growing weight, all pointing right, for a sense of flow. |
| `powerline` | Powerline separators `U+E0B0` to `U+E0B6`, then `█` | Large wedges and curves. Requires a Powerline or Nerd Font. |

Several names have short aliases that work anywhere a palette name is accepted: `std`, `block`, `circle`, `geo`, `shade`, `tri`, `arrow`, `power`, `box`, and `extend`.

### Choosing a palette

For flowing patterns such as Plasma, Waves, and Fluid, the round glyphs of `circles` or the solid fill of `blocks` keep the motion smooth. For noisy or detailed patterns, `braille` and `dots` hold fine texture without turning to clutter. Angular patterns such as Geometric, Grid, and Truchet pair naturally with `geometric` and `boxdraw`. When in doubt, `standard` and `simple` read well everywhere.

Very dark pixels are left blank whatever the palette, so the terminal background shows through the quiet parts of a pattern.

### Fonts

`standard`, `simple`, and `extended` are pure ASCII and work with any font. The others need a font that covers the Unicode blocks they use; most modern monospace fonts include block elements, box drawing, and geometric shapes, while Braille coverage is less universal. `powerline` uses private-use code points that only exist in Powerline-patched fonts and [Nerd Fonts](https://www.nerdfonts.com/).

If a palette shows hollow boxes or question marks, the font lacks those glyphs. If the glyphs appear but the picture looks stretched or misaligned, the font is drawing them wider than one cell; pick a different palette or a font with strictly monospaced symbols.

### Using palettes from code

The library exposes the same palettes to Rust callers:

```rust
use chroma::ascii::{AsciiConverter, AsciiPalette};

let converter = AsciiConverter::new(AsciiPalette::braille(), true);
let frame = converter.convert_frame(&rgba_pixels, width, height);
```

`convert_frame` takes tightly packed RGBA bytes and returns one `(char, Color)` pair per pixel. The second argument to `AsciiConverter::new` enables colour; with `false`, every character is white.
