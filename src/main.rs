mod chip8;
mod display;
mod font;
mod input;
mod memory;
mod program_counter;
mod register;
mod stack;
mod timer;

use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    enable_raw_mode()?;

    let mut chip = chip8::Chip8::default();
    let rom = std::fs::read("roms/IBM Logo.ch8").unwrap();
    chip.load_rom(&rom);
    let err = chip.run();

    disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen)?;
    if let Err(e) = err {
        println!("{e}");
        // dbg!(chip);
    }
    Ok(())
}
