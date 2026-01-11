mod chip8;
mod display;
mod font;
mod input;
mod memory;
mod program_counter;
mod register;
mod stack;
mod timer;

use std::io::Write;

use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rom = std::fs::read("roms/br8kout.ch8").unwrap();
    let mut chip = chip8::Chip8::default();
    chip.load_rom(&rom);

    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    enable_raw_mode()?;

    let err = chip.run();

    disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen)?;
    stdout.flush()?;

    if let Err(e) = err {
        println!("{e}");
        // dbg!(chip);
    }
    Ok(())
}
