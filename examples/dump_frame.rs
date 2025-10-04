use std::fs::File;
use std::io::Write;

use anyhow::Result;
use term_shaders::{
    ascii::{AsciiConverter, AsciiPalette},
    params::ShaderParams,
    shader::{ShaderPipeline, ShaderUniforms},
};

#[pollster::main]
async fn main() -> Result<()> {
    let width = 80;
    let height = 24;

    let mut params = ShaderParams::default();
    params.set_resolution(width, height);
    params.time = 1.0;

    println!("Creating pipeline...");
    let pipeline = ShaderPipeline::new(width, height).await?;

    println!("Rendering frame...");
    let uniforms = ShaderUniforms::from_params(&params);
    let pixel_data = pipeline.render(&uniforms)?;

    println!("Pixel data length: {}", pixel_data.len());
    println!("Expected length: {}", width * height * 4);

    println!("\nFirst 10 pixels RGB values:");
    for i in 0..10.min(pixel_data.len() / 4) {
        let idx = i * 4;
        println!(
            "  Pixel {}: R={:3}, G={:3}, B={:3}, A={:3}",
            i,
            pixel_data[idx],
            pixel_data[idx + 1],
            pixel_data[idx + 2],
            pixel_data[idx + 3]
        );
    }

    let mut min_brightness = 255u8;
    let mut max_brightness = 0u8;
    let mut total_brightness = 0u64;

    for i in 0..(width * height) as usize {
        let idx = i * 4;
        let avg = ((pixel_data[idx] as u32
            + pixel_data[idx + 1] as u32
            + pixel_data[idx + 2] as u32)
            / 3) as u8;
        min_brightness = min_brightness.min(avg);
        max_brightness = max_brightness.max(avg);
        total_brightness += avg as u64;
    }

    let avg_brightness = total_brightness / (width * height) as u64;

    println!("\nBrightness statistics:");
    println!("  Min: {}", min_brightness);
    println!("  Max: {}", max_brightness);
    println!("  Avg: {}", avg_brightness);

    println!("\nConverting to ASCII...");
    let converter = AsciiConverter::new(AsciiPalette::standard(), true);
    let ascii_frame = converter.convert_frame(&pixel_data, width, height);

    println!("ASCII frame rows: {}", ascii_frame.len());
    println!("ASCII frame first row length: {}", ascii_frame[0].len());

    let mut unique_chars = std::collections::HashSet::new();
    for row in &ascii_frame {
        for (ch, _) in row {
            unique_chars.insert(*ch);
        }
    }
    println!("Unique characters used: {}", unique_chars.len());
    println!("Characters: {:?}", unique_chars);

    println!("\nWriting frame to output.txt...");
    let mut file = File::create("output.txt")?;

    writeln!(file, "=== RAW FRAME OUTPUT (no colors) ===")?;
    for row in &ascii_frame {
        for (ch, _) in row {
            write!(file, "{}", ch)?;
        }
        writeln!(file)?;
    }

    writeln!(file, "\n=== FRAME WITH COLOR INFO ===")?;
    for (y, row) in ascii_frame.iter().enumerate() {
        for (x, (ch, color)) in row.iter().enumerate() {
            if x < 10 && y < 3 {
                writeln!(file, "Pos ({}, {}): char='{}' color={:?}", x, y, ch, color)?;
            }
        }
    }

    println!("Done! Check output.txt for the rendered frame.");

    Ok(())
}
