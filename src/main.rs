mod chip8;
mod display;
mod error;
mod font;
mod input;
mod memory;
mod program_counter;
mod register;
mod stack;
mod timer;
mod utils;

use std::{
    io::{self, Write},
    time::{Duration, Instant},
};

use crossterm::{
    cursor::{self, Hide, Show},
    execute, queue, style,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};

use crate::{display::DisplayBuffer, input::Keypad};

const FRAME_TIMEOUT: f32 = 1.0 / 60.0;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rom_path = std::env::args().nth(1).ok_or("usage: chip8 <rom_path>")?;
    let rom = std::fs::read(&rom_path).map_err(|e| format!("failed to read rom: {e}"))?;
    let mut chip = chip8::Chip8::default();
    chip.load_rom(&rom)?;

    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, Hide)?;
    enable_raw_mode()?;

    let mut quit = false;
    let mut keypad = Keypad::default();
    let mut last_display_buffer_refresh = Instant::now();
    while !quit {
        let loop_time = Instant::now();
        if loop_time - last_display_buffer_refresh > Duration::from_secs_f32(FRAME_TIMEOUT) {
            render_to_screen(&chip.display_buffer)?;
            last_display_buffer_refresh = loop_time;
        }
        quit = keypad.poll()?;
        chip.cycle(&keypad)?;
    }

    disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen, Show)?;
    stdout.flush()?;

    Ok(())
}

pub fn render_to_screen(display_buffer: &DisplayBuffer) -> Result<(), io::Error> {
    let mut stdout = io::stdout();
    for y in 0..display_buffer.pixels.len() {
        for x in 0..display_buffer.pixels[y].len() {
            let char = match display_buffer.pixels[y][x] {
                true => '█',
                false => ' ',
            };
            queue!(
                stdout,
                cursor::MoveTo(x as u16, y as u16),
                style::Print(char)
            )?;
        }
    }
    stdout.flush()?;
    Ok(())
}
