mod chip8;
mod display;
mod font;
mod input;
mod memory;
mod program_counter;
mod register;
mod stack;
mod timer;
mod utils;

use std::io::Write;

use crossterm::{
    cursor::{Hide, Show},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rom_path = std::env::args().nth(1).ok_or("usage: chip8 <rom_path>")?;
    let rom = std::fs::read(&rom_path).map_err(|e| format!("failed to read rom: {e}"))?;
    let mut chip = chip8::Chip8::default();
    chip.load_rom(&rom)?;

    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, Hide)?;
    enable_raw_mode()?;

    let err = chip.run();

    disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen, Show)?;
    stdout.flush()?;

    if let Err(e) = err {
        println!("{e}");
        // dbg!(chip);
    }
    Ok(())
}
