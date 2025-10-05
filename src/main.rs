use std::fs::File;
use std::io::{BufWriter, Write, stdout};
use std::time::{Duration, Instant};

use anyhow::Result;
use clap::Parser;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    terminal::{self, ClearType},
};
use unicode_width::UnicodeWidthChar;

use term_shaders::{
    ascii::{AsciiConverter, AsciiPalette},
    params::{PaletteType, ShaderParams},
    shader::{ShaderPipeline, ShaderUniforms},
};

#[cfg(feature = "audio")]
use term_shaders::audio::{AudioAnalyzer, AudioCapture};

#[derive(Parser, Debug)]
#[command(name = "term-shaders")]
#[command(about = "Terminal-based shader visualizer with optional audio reactivity", long_about = None)]
struct CliArgs {
    #[arg(
        short,
        long,
        value_name = "FILE",
        help = "Load configuration from a saved config file"
    )]
    config: Option<String>,
}

const TARGET_FPS: u32 = 30;
const FRAME_DURATION: Duration = Duration::from_millis(1000 / TARGET_FPS as u64);

struct App {
    params: ShaderParams,
    pipeline: ShaderPipeline,
    converter: AsciiConverter,
    running: bool,
    last_frame_time: Instant,
    debug_log: BufWriter<File>,
    last_terminal_size: (u16, u16),
    #[cfg(feature = "audio")]
    audio_capture: Option<AudioCapture>,
    #[cfg(feature = "audio")]
    audio_analyzer: Option<AudioAnalyzer>,
}

impl App {
    async fn new(loaded_config: Option<ShaderParams>) -> Result<Self> {
        let debug_file = File::create("debug.log")?;
        let mut debug_log = BufWriter::new(debug_file);

        let (terminal_width, terminal_height) = terminal::size()?;

        writeln!(
            debug_log,
            "DEBUG: Terminal size: {}x{}",
            terminal_width, terminal_height
        )?;

        let shader_width = terminal_width as u32;
        let shader_height = (terminal_height - 1) as u32;

        writeln!(
            debug_log,
            "DEBUG: Shader size: {}x{}",
            shader_width, shader_height
        )?;

        let mut params = if let Some(config) = loaded_config {
            writeln!(debug_log, "Using loaded configuration")?;
            config
        } else {
            #[cfg(feature = "audio")]
            {
                ShaderParams::with_audio_reactive_defaults()
            }
            #[cfg(not(feature = "audio"))]
            {
                ShaderParams::default()
            }
        };

        params.set_resolution(shader_width, shader_height);

        let pipeline = ShaderPipeline::new(shader_width, shader_height).await?;
        let palette = Self::palette_from_type(params.palette);
        let converter = AsciiConverter::new(palette, true);

        #[cfg(feature = "audio")]
        let (audio_capture, audio_analyzer) = {
            match AudioCapture::new() {
                Ok(capture) => {
                    writeln!(
                        debug_log,
                        "Audio capture initialized successfully at {} Hz",
                        capture.sample_rate
                    )?;
                    let analyzer = AudioAnalyzer::new(capture.sample_rate);
                    (Some(capture), Some(analyzer))
                }
                Err(e) => {
                    writeln!(debug_log, "Failed to initialize audio: {}", e)?;
                    (None, None)
                }
            }
        };

        Ok(Self {
            params,
            pipeline,
            converter,
            running: true,
            last_frame_time: Instant::now(),
            debug_log,
            last_terminal_size: (terminal_width, terminal_height),
            #[cfg(feature = "audio")]
            audio_capture,
            #[cfg(feature = "audio")]
            audio_analyzer,
        })
    }

    fn palette_from_type(palette_type: PaletteType) -> AsciiPalette {
        match palette_type {
            PaletteType::Standard => AsciiPalette::standard(),
            PaletteType::Blocks => AsciiPalette::blocks(),
            PaletteType::Circles => AsciiPalette::circles(),
            PaletteType::Smooth => AsciiPalette::smooth(),
            PaletteType::Braille => AsciiPalette::braille(),
            PaletteType::Geometric => AsciiPalette::geometric(),
            PaletteType::Mixed => AsciiPalette::mixed(),
            PaletteType::Dots => AsciiPalette::dots(),
            PaletteType::Shades => AsciiPalette::shades(),
            PaletteType::Lines => AsciiPalette::lines(),
            PaletteType::Triangles => AsciiPalette::triangles(),
            PaletteType::Arrows => AsciiPalette::arrows(),
            PaletteType::Powerline => AsciiPalette::powerline(),
            PaletteType::BoxDraw => AsciiPalette::boxdraw(),
            PaletteType::Extended => AsciiPalette::extended(),
            PaletteType::Simple => AsciiPalette::simple(),
        }
    }

    fn hue_to_pastel_rgb(hue: f32) -> (u8, u8, u8) {
        let hue_normalized = (hue / 6.28) % 1.0;
        let h = hue_normalized * 6.0;
        let c = 1.0;
        let x = 1.0 - ((h % 2.0) - 1.0).abs();

        let (r, g, b) = if h < 1.0 {
            (c, x, 0.0)
        } else if h < 2.0 {
            (x, c, 0.0)
        } else if h < 3.0 {
            (0.0, c, x)
        } else if h < 4.0 {
            (0.0, x, c)
        } else if h < 5.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };

        let lightness = 0.35;
        let pastel_r = ((r * 0.85 + lightness) * 255.0).min(255.0) as u8;
        let pastel_g = ((g * 0.85 + lightness) * 255.0).min(255.0) as u8;
        let pastel_b = ((b * 0.85 + lightness) * 255.0).min(255.0) as u8;

        (pastel_r, pastel_g, pastel_b)
    }

    fn update(&mut self) {
        let current_time = Instant::now();
        let delta_time = current_time
            .duration_since(self.last_frame_time)
            .as_secs_f32();

        self.params.update_time(delta_time);

        // Audio-reactive updates
        #[cfg(feature = "audio")]
        if self.params.audio_enabled {
            if let (Some(capture), Some(analyzer)) = (&self.audio_capture, &mut self.audio_analyzer)
            {
                let samples = capture.get_samples();
                if !samples.is_empty() {
                    let features = analyzer.analyze(&samples, delta_time);

                    // Detect silence (very low overall volume)
                    let is_silent = features.overall < 0.02;

                    if is_silent {
                        // Gradually slow down and fade out when silent
                        let decay_rate = 0.92; // Smooth decay
                        let speed_decay_rate = 0.88; // Slower decay for dramatic stop

                        self.params.amplitude =
                            self.params.amplitude * decay_rate + 0.4 * (1.0 - decay_rate);
                        self.params.distort_amplitude = self.params.distort_amplitude * decay_rate;
                        self.params.frequency =
                            self.params.frequency * decay_rate + 6.0 * (1.0 - decay_rate);

                        // Speed gradually approaches zero for complete stop
                        self.params.speed = self.params.speed * speed_decay_rate;

                        // Fade out brightness and effects
                        self.params.brightness =
                            self.params.brightness * decay_rate + 0.6 * (1.0 - decay_rate);
                        self.params.noise_strength = self.params.noise_strength * 0.85;
                        self.params.contrast =
                            self.params.contrast * decay_rate + 0.8 * (1.0 - decay_rate);

                        writeln!(
                            self.debug_log,
                            "AUDIO: Silence (vol={:.4}) - slowing to stop (speed={:.3})",
                            features.overall, self.params.speed
                        )
                        .ok();
                    } else {
                        // Heavily emphasize treble for TOP NOTES visibility (melody, high piano keys)
                        // De-emphasize bass/chords - they should be subtle background
                        let energy =
                            (features.bass * 0.1 + features.mid * 0.3 + features.treble * 0.6)
                                .max(0.05); // Treble is 60% of energy!

                        // Map audio features to shader parameters
                        // Bass affects amplitude and distortion (minimal influence - chords are muted)
                        let bass_multiplier =
                            1.0 + features.bass * self.params.bass_influence * 0.6; // Very subtle
                        self.params.amplitude =
                            (self.params.amplitude * 0.95) + (bass_multiplier * 0.05); // Very slow
                        self.params.distort_amplitude =
                            features.bass * self.params.bass_influence * 0.4; // Minimal distortion

                        // Mid frequencies (piano middle range, vocals)
                        let mid_boost = 1.0 + features.mid * self.params.mid_influence * 1.8;
                        self.params.frequency =
                            (self.params.frequency * 0.90) + (8.0 * mid_boost * 0.10); // More reactive

                        // Speed scales with treble for HIGH NOTES to really pop
                        // High notes should speed up visibly while chords stay calm
                        let treble_boost =
                            1.0 + features.treble * self.params.treble_influence * 2.0; // STRONG treble boost!
                        let base_speed = 0.08 + energy * 0.7; // Range: 0.08 to 0.78 (calmer base)
                        let target_speed = base_speed * treble_boost; // High notes can push to 1.5x+
                        self.params.speed = (self.params.speed * 0.88) + (target_speed * 0.12); // Faster response

                        // Color shift reacts STRONGLY to high notes - visible color change on melody
                        self.params.color_shift =
                            (self.params.color_shift + features.treble * 0.25) % 6.28; // Much faster

                        // Beat triggers effects (catches note attacks, especially high notes)
                        if features.beat_strength > 0.35 {
                            // Lower threshold
                            self.params.noise_strength =
                                features.beat_strength * (0.2 + features.treble * 0.5);
                        }

                        // Bass drop triggers major effect
                        if features.is_drop {
                            self.params.effect_time = self.params.time;
                            writeln!(self.debug_log, "BASS DROP detected! Triggering effect").ok();
                        }

                        // Brightness reacts STRONGLY to treble - high notes should flash/pop
                        let treble_brightness = features.treble * 0.8; // Extra brightness on high notes
                        self.params.brightness = (0.5 + features.overall * 0.6) + treble_brightness;
                        self.params.brightness = self.params.brightness.min(1.8); // Cap at 1.8

                        // Contrast also reacts to treble for sharper visuals on melody
                        let treble_contrast = features.treble * 0.6;
                        let target_contrast = 0.6 + energy * 0.4 + treble_contrast;
                        self.params.contrast =
                            (self.params.contrast * 0.90) + (target_contrast * 0.10);
                    }
                }
            }
        }

        self.last_frame_time = current_time;
    }

    fn render(&mut self) -> Result<()> {
        let uniforms = ShaderUniforms::from_params(&self.params);

        writeln!(
            self.debug_log,
            "DEBUG: Uniforms - time: {}, freq: {}, amp: {}, scale: {}",
            self.params.time, self.params.frequency, self.params.amplitude, self.params.scale
        )?;
        writeln!(
            self.debug_log,
            "DEBUG: Resolution in uniforms: {}x{}",
            self.params.resolution_width, self.params.resolution_height
        )?;

        let pixel_data = self.pipeline.render(&uniforms)?;

        writeln!(
            self.debug_log,
            "DEBUG: pixel_data length: {}",
            pixel_data.len()
        )?;
        writeln!(
            self.debug_log,
            "DEBUG: Expected size: {}",
            self.pipeline.width() * self.pipeline.height() * 4
        )?;

        let ascii_frame = self.converter.convert_frame(
            &pixel_data,
            self.pipeline.width(),
            self.pipeline.height(),
        );

        writeln!(
            self.debug_log,
            "DEBUG: ascii_frame rows: {}",
            ascii_frame.len()
        )?;
        if !ascii_frame.is_empty() {
            writeln!(
                self.debug_log,
                "DEBUG: first row length: {}",
                ascii_frame[0].len()
            )?;
            if !ascii_frame[0].is_empty() {
                let (ch, col) = &ascii_frame[0][0];
                writeln!(
                    self.debug_log,
                    "DEBUG: first character: '{}' color: {:?}",
                    ch, col
                )?;
            }
        }

        writeln!(self.debug_log, "DEBUG: First few pixels RGB values:")?;
        for i in 0..4.min(pixel_data.len() / 4) {
            let idx = i * 4;
            writeln!(
                self.debug_log,
                "  Pixel {}: R={}, G={}, B={}, A={}",
                i,
                pixel_data[idx],
                pixel_data[idx + 1],
                pixel_data[idx + 2],
                pixel_data[idx + 3]
            )?;
        }

        let mut min_brightness = 255u8;
        let mut max_brightness = 0u8;
        for i in 0..(pixel_data.len() / 4).min(100) {
            let idx = i * 4;
            let avg =
                ((pixel_data[idx] as u32 + pixel_data[idx + 1] as u32 + pixel_data[idx + 2] as u32)
                    / 3) as u8;
            min_brightness = min_brightness.min(avg);
            max_brightness = max_brightness.max(avg);
        }
        writeln!(
            self.debug_log,
            "DEBUG: Brightness range in first 100 pixels: {} to {}",
            min_brightness, max_brightness
        )?;

        self.debug_log.flush()?;

        let (current_width, current_height) = terminal::size()?;

        writeln!(
            self.debug_log,
            "DEBUG: Current terminal size: {}x{}, shader size: {}x{}",
            current_width,
            current_height,
            self.pipeline.width(),
            self.pipeline.height()
        )?;

        let expected_rows = (current_height - 1) as usize;
        let expected_cols = current_width as usize;

        if ascii_frame.len() > expected_rows
            || (!ascii_frame.is_empty() && ascii_frame[0].len() > expected_cols)
        {
            writeln!(
                self.debug_log,
                "WARNING: Frame size mismatch! Frame: {}x{}, Expected: {}x{}",
                ascii_frame.len(),
                if ascii_frame.is_empty() {
                    0
                } else {
                    ascii_frame[0].len()
                },
                expected_rows,
                expected_cols
            )?;
        }

        let mut buffer = String::with_capacity(expected_rows * expected_cols * 25);

        buffer.push_str("\x1b[?25l\x1b[H\x1b[0m\x1b[49m");

        let rows_to_render = ascii_frame.len().min(expected_rows);

        for row_idx in 0..rows_to_render {
            let row = &ascii_frame[row_idx];
            let mut current_col = 0;
            let mut col_idx = 0;

            while col_idx < row.len() && current_col < expected_cols {
                let (character, color) = &row[col_idx];

                let char_width = character.width().unwrap_or(1);

                if current_col + char_width > expected_cols {
                    writeln!(
                        self.debug_log,
                        "WARNING: Character '{}' (width={}) at col {} would overflow (limit={}), skipping rest of row",
                        character, char_width, current_col, expected_cols
                    )?;
                    break;
                }

                if *character == ' ' {
                    buffer.push(' ');
                    current_col += 1;
                    col_idx += 1;
                    continue;
                }

                let brightness = if let crossterm::style::Color::Rgb { r, g, b } = color {
                    ((*r as u32 + *g as u32 + *b as u32) / 3) as u8
                } else {
                    128
                };

                if brightness < 30 {
                    buffer.push(' ');
                    current_col += 1;
                    col_idx += 1;
                    continue;
                }

                if let crossterm::style::Color::Rgb { r, g, b } = color {
                    buffer.push_str(&format!(
                        "\x1b[38;2;{};{};{}m{}\x1b[39m\x1b[49m",
                        r, g, b, character
                    ));
                } else {
                    buffer.push(*character);
                }

                current_col += char_width;
                col_idx += 1;
            }

            if row_idx < rows_to_render - 1 {
                buffer.push_str("\x1b[0m\r\n");
            }
        }

        buffer.push_str("\x1b[0m\x1b[49m\r\n");

        let effect_names = ["Circle", "Cross", "Diamond", "Star", "Grid", "Wave"];
        let effect_name = effect_names[self.params.effect_type as usize % 6];

        #[cfg(feature = "audio")]
        let has_sound = if self.params.audio_enabled {
            if let (Some(capture), Some(_)) = (&self.audio_capture, &self.audio_analyzer) {
                let samples = capture.get_samples();
                !samples.is_empty() && samples.iter().any(|s| s.abs() > 0.02)
            } else {
                false
            }
        } else {
            false
        };
        #[cfg(not(feature = "audio"))]
        let has_sound = false;

        let status = format!(
            "{} {} {} {} | F:{:.1} | [Q] [R]nd [S]ave [A]udio [E]fx [N]xt [C]lr [P]al",
            self.params.palette.name(),
            self.params.pattern_type.name(),
            self.params.color_mode.name(),
            effect_name,
            self.params.frequency
        );

        let status_visual_len: usize = status
            .chars()
            .map(|c| c.width().unwrap_or(1))
            .sum::<usize>();

        let available_cols = expected_cols;

        let truncated_status = if status_visual_len > available_cols {
            writeln!(
                self.debug_log,
                "WARNING: Status bar too long ({} visual cols) for available space ({}), truncating",
                status_visual_len, available_cols
            )?;

            let target_len = available_cols.saturating_sub(3);
            let mut current_width = 0;
            let mut truncated = String::new();

            for ch in status.chars() {
                let char_width = ch.width().unwrap_or(1);
                if current_width + char_width > target_len {
                    break;
                }
                truncated.push(ch);
                current_width += char_width;
            }

            format!("{}...", truncated)
        } else {
            let padding = " ".repeat(available_cols - status_visual_len);
            format!("{}{}", status, padding)
        };

        if has_sound {
            let gradient_offset = (self.params.time * 2.0) % 6.28;

            let mut formatted_status = String::new();
            let mut char_pos = 0;

            for ch in truncated_status.chars() {
                let hue = (gradient_offset + (char_pos as f32 * 0.1)) % 6.28;
                let (r, g, b) = Self::hue_to_pastel_rgb(hue);

                formatted_status.push_str(&format!(
                    "\x1b[48;2;{};{};{}m\x1b[30m{}\x1b[49m\x1b[39m",
                    r, g, b, ch
                ));
                char_pos += 1;
            }

            buffer.push_str(&formatted_status);
        } else {
            buffer.push_str(&format!("\x1b[47m\x1b[30m{}\x1b[0m", truncated_status));
        }

        writeln!(
            self.debug_log,
            "DEBUG: frame rendered {} rows x {} cols (expected {}x{}), buffer size: {}",
            rows_to_render,
            if ascii_frame.is_empty() {
                0
            } else {
                ascii_frame[0].len().min(expected_cols)
            },
            expected_rows,
            expected_cols,
            buffer.len()
        )?;

        let mut stdout = stdout();
        write!(stdout, "{}", buffer)?;
        stdout.flush()?;

        Ok(())
    }

    async fn handle_resize(&mut self, new_width: u16, new_height: u16) -> Result<()> {
        writeln!(
            self.debug_log,
            "RESIZE: Terminal resized to {}x{} (was {}x{})",
            new_width, new_height, self.last_terminal_size.0, self.last_terminal_size.1
        )?;

        let shader_width = new_width as u32;
        let shader_height = (new_height - 1) as u32;

        self.params.set_resolution(shader_width, shader_height);
        self.pipeline = ShaderPipeline::new(shader_width, shader_height).await?;
        self.last_terminal_size = (new_width, new_height);

        writeln!(
            self.debug_log,
            "RESIZE: Pipeline recreated with dimensions {}x{}",
            shader_width, shader_height
        )?;

        Ok(())
    }

    fn handle_input(&mut self) -> Result<()> {
        if event::poll(Duration::from_millis(0))? {
            match event::read()? {
                Event::Resize(width, height) => {
                    if (width, height) != self.last_terminal_size {
                        pollster::block_on(async { self.handle_resize(width, height).await })?;
                    }
                }
                Event::Key(KeyEvent {
                    code,
                    kind: KeyEventKind::Press,
                    ..
                }) => match code {
                    KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                        self.running = false;
                    }
                    KeyCode::Up => {
                        self.params.frequency += 0.1;
                    }
                    KeyCode::Down => {
                        self.params.frequency = (self.params.frequency - 0.1).max(0.1);
                    }
                    KeyCode::Right => {
                        self.params.speed += 0.1;
                    }
                    KeyCode::Left => {
                        self.params.speed = (self.params.speed - 0.1).max(0.1);
                    }
                    KeyCode::Char('+') | KeyCode::Char('=') => {
                        self.params.amplitude += 0.1;
                    }
                    KeyCode::Char('-') | KeyCode::Char('_') => {
                        self.params.amplitude = (self.params.amplitude - 0.1).max(0.1);
                    }
                    KeyCode::Char('[') => {
                        self.params.scale = (self.params.scale - 0.1).max(0.1);
                    }
                    KeyCode::Char(']') => {
                        self.params.scale += 0.1;
                    }
                    KeyCode::Char('r') | KeyCode::Char('R') => {
                        self.params.randomize();
                        let new_palette = Self::palette_from_type(self.params.palette);
                        self.converter.set_palette(new_palette);
                    }
                    KeyCode::Char('p') | KeyCode::Char('P') => {
                        self.params.palette = self.params.palette.next();
                        let new_palette = Self::palette_from_type(self.params.palette);
                        self.converter.set_palette(new_palette);
                    }
                    KeyCode::Char('o') | KeyCode::Char('O') => {
                        self.params.palette = self.params.palette.previous();
                        let new_palette = Self::palette_from_type(self.params.palette);
                        self.converter.set_palette(new_palette);
                    }
                    KeyCode::Char('c') | KeyCode::Char('C') => {
                        self.params.color_mode = self.params.color_mode.next();
                    }
                    KeyCode::Char('e') | KeyCode::Char('E') | KeyCode::Char(' ') => {
                        self.params.effect_time = self.params.time;
                        writeln!(
                            self.debug_log,
                            "EFFECT: Triggered effect type {} at time {:.2}",
                            self.params.effect_type, self.params.effect_time
                        )?;
                    }
                    KeyCode::Char('n') | KeyCode::Char('N') => {
                        self.params.effect_type = (self.params.effect_type + 1) % 6;
                        writeln!(
                            self.debug_log,
                            "EFFECT: Switched to effect type {}",
                            self.params.effect_type
                        )?;
                    }
                    KeyCode::Char('a') | KeyCode::Char('A') => {
                        #[cfg(feature = "audio")]
                        {
                            self.params.audio_enabled = !self.params.audio_enabled;
                            writeln!(
                                self.debug_log,
                                "AUDIO: Audio reactivity {}",
                                if self.params.audio_enabled {
                                    "enabled"
                                } else {
                                    "disabled"
                                }
                            )?;
                        }
                    }
                    KeyCode::Char('s') | KeyCode::Char('S') => match self.params.save_to_file() {
                        Ok(filename) => {
                            writeln!(
                                self.debug_log,
                                "CONFIG: Saved configuration to {}",
                                filename
                            )?;
                        }
                        Err(error) => {
                            writeln!(
                                self.debug_log,
                                "CONFIG: Failed to save configuration: {}",
                                error
                            )?;
                        }
                    },
                    _ => {}
                },
                _ => {}
            }
        }

        Ok(())
    }

    fn run(&mut self) -> Result<()> {
        while self.running {
            let frame_start = Instant::now();

            let (current_width, current_height) = terminal::size()?;
            if (current_width, current_height) != self.last_terminal_size {
                pollster::block_on(async {
                    self.handle_resize(current_width, current_height).await
                })?;
            }

            self.handle_input()?;
            self.update();
            self.render()?;

            let frame_time = frame_start.elapsed();

            if frame_time < FRAME_DURATION {
                std::thread::sleep(FRAME_DURATION - frame_time);
            }
        }

        Ok(())
    }
}

fn main() -> Result<()> {
    let cli_args = CliArgs::parse();

    let loaded_config = if let Some(config_path) = &cli_args.config {
        match ShaderParams::load_from_file(config_path) {
            Ok(config) => {
                println!("✓ Loaded configuration from: {}", config_path);
                Some(config)
            }
            Err(error) => {
                eprintln!("✗ Failed to load config from {}: {}", config_path, error);
                eprintln!("  Falling back to default configuration.\n");
                None
            }
        }
    } else {
        None
    };

    println!("🎨 term-shaders initializing...\n");

    #[cfg(feature = "audio")]
    {
        use term_shaders::audio::AudioCapture;

        println!("🎵 Audio Reactivity Diagnostics:");
        println!("   Checking audio system...");

        match AudioCapture::new() {
            Ok(capture) => {
                println!("   ✓ Audio device found: {} Hz", capture.sample_rate);

                // Test if we're actually receiving audio data
                std::thread::sleep(std::time::Duration::from_millis(200));
                let test_samples = capture.get_samples();

                if test_samples.is_empty() {
                    println!("   ⚠ WARNING: No audio data received!");
                    println!("   Audio device opened but no samples captured.");
                    println!("   This usually means:");
                    println!("     • Using microphone input (need loopback for system audio)");
                    println!("     • No audio currently playing");
                    println!("     • Need to configure PulseAudio/PipeWire monitor");
                    println!();
                    println!("   Quick fix: pavucontrol → Recording → Select 'Monitor of...'");
                    println!("   See AUDIO_DIAGNOSTICS.md for detailed troubleshooting.");
                    println!();
                    println!("   Continuing anyway (audio will work once configured)...");
                } else {
                    let max_sample = test_samples.iter().map(|s| s.abs()).fold(0.0f32, f32::max);
                    if max_sample > 0.02 {
                        println!("   ✓ Audio data flowing! (peak: {:.3})", max_sample);
                        println!("   Audio reactivity ready. Press 'A' to toggle.");
                    } else {
                        println!("   ✓ Audio system working (currently silent)");
                        println!("   Play some audio to test reactivity. Press 'A' to toggle.");
                    }
                }
            }
            Err(e) => {
                let error_str = e.to_string();
                eprintln!("   ✗ Failed to initialize audio");
                eprintln!("   Error: {}", e);
                eprintln!();

                // Check for common error patterns
                if error_str.contains("No such file") || error_str.contains("cannot find card") {
                    eprintln!("   This looks like: No audio hardware found");
                    eprintln!("     • Running in VM/container without audio passthrough?");
                    eprintln!("     • No sound card available?");
                    eprintln!("     • ALSA not configured?");
                } else if error_str.contains("no longer available")
                    || error_str.contains("unplugged")
                {
                    eprintln!("   This looks like: Audio device not available");
                    eprintln!("     • Check: arecord -l");
                    eprintln!("     • Ensure audio hardware is connected");
                }

                eprintln!();
                eprintln!("   You have two options:");
                eprintln!("     1. Fix audio setup (see AUDIO_SETUP.md)");
                eprintln!("     2. Build without audio: cargo build --release");
                eprintln!();
                eprintln!("   If you just want to test visuals without audio, use option 2.");
                eprintln!();
                std::process::exit(1);
            }
        }

        println!();
    }

    #[cfg(not(feature = "audio"))]
    {
        println!("   Audio reactivity: Not enabled");
        println!("   To enable: cargo build --release --features audio");
        println!();
    }

    println!("Starting shader rendering in 1 second...");
    std::thread::sleep(std::time::Duration::from_millis(1000));

    terminal::enable_raw_mode()?;

    execute!(
        stdout(),
        terminal::EnterAlternateScreen,
        cursor::Hide,
        terminal::Clear(ClearType::All)
    )?;

    let result = pollster::block_on(async {
        let mut app = App::new(loaded_config).await?;
        app.run()
    });

    execute!(stdout(), cursor::Show, terminal::LeaveAlternateScreen)?;

    terminal::disable_raw_mode()?;

    result
}
