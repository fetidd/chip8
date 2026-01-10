mod chip8;
mod display;
mod font;
mod memory;
mod program_counter;
mod register;
mod stack;
mod timer;
mod input;

use crossterm::{terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, execute};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    enable_raw_mode()?;
    
    
    let mut chip = chip8::Chip8::default();
    let rom = std::fs::read("roms/IBM Logo.ch8").unwrap();
    chip.load_rom(&rom);
    chip.run()?;
    
    
    disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen)?;
    Ok(())
}
