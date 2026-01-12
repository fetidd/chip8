use std::fmt::Debug;

use crossterm::{cursor::MoveTo, execute, style::Print};

use crate::display::Display as Chip8Display;

pub fn debug_out<T: Debug>(msg: T) {
    let mut stdout = std::io::stdout();
    execute!(
        stdout,
        MoveTo(
            (Chip8Display::WIDTH + 5) as u16,
            (Chip8Display::HEIGHT / 2) as u16
        ),
        Print("                       "),
        MoveTo(
            (Chip8Display::WIDTH + 5) as u16,
            (Chip8Display::HEIGHT / 2) as u16
        ),
        Print(format!("{msg:?}"))
    )
    .expect("debug_out failed to execute!");
}
