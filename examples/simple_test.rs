use crossterm::{cursor, execute, style::Print, terminal};
use std::io::{Write, stdout};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = stdout();

    terminal::enable_raw_mode()?;
    execute!(
        stdout,
        terminal::EnterAlternateScreen,
        cursor::Hide,
        terminal::Clear(terminal::ClearType::All)
    )?;

    // Test 1: Simple text
    execute!(
        stdout,
        cursor::MoveTo(0, 0),
        Print("Test 1: Can you see this text?")
    )?;
    stdout.flush()?;
    std::thread::sleep(std::time::Duration::from_secs(2));

    // Test 2: ANSI colors in buffer
    let mut buffer = String::new();
    buffer.push_str("\x1b[31mRed\x1b[0m ");
    buffer.push_str("\x1b[32mGreen\x1b[0m ");
    buffer.push_str("\x1b[34mBlue\x1b[0m\n");
    buffer.push_str("Test 2: Can you see colors above?\n");

    execute!(stdout, cursor::MoveTo(0, 1))?;
    write!(stdout, "{}", buffer)?;
    stdout.flush()?;
    std::thread::sleep(std::time::Duration::from_secs(2));

    // Test 3: Fill screen with characters
    execute!(stdout, cursor::MoveTo(0, 4))?;
    let test_line = "X".repeat(80) + "\n";
    write!(stdout, "{}", test_line.repeat(10))?;
    stdout.flush()?;
    std::thread::sleep(std::time::Duration::from_secs(2));

    // Test 4: RGB colors
    execute!(stdout, cursor::MoveTo(0, 15))?;
    let rgb_test = format!(
        "\x1b[38;2;255;0;0mRGB RED\x1b[0m \x1b[38;2;0;255;0mRGB GREEN\x1b[0m \x1b[38;2;0;0;255mRGB BLUE\x1b[0m\n"
    );
    write!(stdout, "{}", rgb_test)?;
    write!(stdout, "Test 4: Can you see RGB colors above?\n")?;
    stdout.flush()?;

    std::thread::sleep(std::time::Duration::from_secs(3));

    execute!(stdout, cursor::Show, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    println!("Test complete. Did you see:");
    println!("1. Simple text at the top?");
    println!("2. Red, Green, Blue colored text?");
    println!("3. Lines of X's?");
    println!("4. RGB colored text?");

    Ok(())
}
