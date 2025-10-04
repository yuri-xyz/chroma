use std::fs::File;
use std::io::{BufWriter, Write, stdout};
use std::time::{Duration, Instant};

use anyhow::Result;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    terminal::{self, ClearType},
};

use term_shaders::{
    ascii::{AsciiConverter, AsciiPalette},
    params::{PaletteType, ShaderParams},
    shader::{ShaderPipeline, ShaderUniforms},
};

const TARGET_FPS: u32 = 30;
const FRAME_DURATION: Duration = Duration::from_millis(1000 / TARGET_FPS as u64);

struct App {
    params: ShaderParams,
    pipeline: ShaderPipeline,
    converter: AsciiConverter,
    running: bool,
    last_frame_time: Instant,
    debug_log: BufWriter<File>,
}

impl App {
    async fn new() -> Result<Self> {
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

        let mut params = ShaderParams::default();
        params.set_resolution(shader_width, shader_height);

        let pipeline = ShaderPipeline::new(shader_width, shader_height).await?;
        let palette = Self::palette_from_type(params.palette);
        let converter = AsciiConverter::new(palette, true);

        Ok(Self {
            params,
            pipeline,
            converter,
            running: true,
            last_frame_time: Instant::now(),
            debug_log,
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
            PaletteType::Extended => AsciiPalette::extended(),
            PaletteType::Simple => AsciiPalette::simple(),
        }
    }

    fn update(&mut self) {
        let current_time = Instant::now();
        let delta_time = current_time
            .duration_since(self.last_frame_time)
            .as_secs_f32();

        self.params.update_time(delta_time);
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

        let mut buffer = String::with_capacity(ascii_frame.len() * ascii_frame[0].len() * 25);

        buffer.push_str("\x1b[?25l\x1b[H");

        let terminal_width = ascii_frame[0].len();

        for (row_idx, row) in ascii_frame.iter().enumerate() {
            for (character, color) in row.iter() {
                if *character == ' ' {
                    buffer.push(' ');
                    continue;
                }

                let brightness = if let crossterm::style::Color::Rgb { r, g, b } = color {
                    ((*r as u32 + *g as u32 + *b as u32) / 3) as u8
                } else {
                    128
                };

                if brightness < 30 {
                    buffer.push(' ');
                    continue;
                }

                if let crossterm::style::Color::Rgb { r, g, b } = color {
                    buffer.push_str(&format!("\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, character));
                } else {
                    buffer.push(*character);
                }
            }

            if row_idx < ascii_frame.len() - 1 {
                buffer.push_str("\x1b[0m\r\n");
            }
        }

        buffer.push_str("\x1b[0m\r\n");

        let status = format!(
            "\x1b[37m {} | Freq:{:.1} Spd:{:.1} Amp:{:.1} Scl:{:.1} | [Q]uit [R]nd [P]al [↑↓←→][+/-][[]] \x1b[0m",
            self.params.palette.name(),
            self.params.frequency,
            self.params.speed,
            self.params.amplitude,
            self.params.scale
        );
        buffer.push_str(&status);

        writeln!(
            self.debug_log,
            "DEBUG: frame rendered {} rows x {} cols, buffer size: {}",
            ascii_frame.len(),
            terminal_width,
            buffer.len()
        )?;

        let mut stdout = stdout();
        write!(stdout, "{}", buffer)?;
        stdout.flush()?;

        Ok(())
    }

    fn handle_input(&mut self) -> Result<()> {
        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(KeyEvent {
                code,
                kind: KeyEventKind::Press,
                ..
            }) = event::read()?
            {
                match code {
                    KeyCode::Char('q') | KeyCode::Esc => {
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
                    _ => {}
                }
            }
        }

        Ok(())
    }

    fn run(&mut self) -> Result<()> {
        while self.running {
            let frame_start = Instant::now();

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
    terminal::enable_raw_mode()?;

    execute!(
        stdout(),
        terminal::EnterAlternateScreen,
        cursor::Hide,
        terminal::Clear(ClearType::All)
    )?;

    let result = pollster::block_on(async {
        let mut app = App::new().await?;
        app.run()
    });

    execute!(stdout(), cursor::Show, terminal::LeaveAlternateScreen)?;

    terminal::disable_raw_mode()?;

    result
}
