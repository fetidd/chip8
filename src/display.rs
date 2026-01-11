use std::io::{self, Write};

use crossterm::{cursor, queue, style};

#[derive(Debug)]
pub struct Display {
    pub pixels: [[bool; Self::WIDTH]; Self::HEIGHT],
}

impl Default for Display {
    fn default() -> Self {
        Self {
            pixels: [[false; Self::WIDTH]; Self::HEIGHT],
        }
    }
}

impl Display {
    pub const WIDTH: usize = 64;
    pub const HEIGHT: usize = 32;

    pub fn clear(&mut self) {
        self.pixels = [[false; Self::WIDTH]; Self::HEIGHT];
    }

    pub fn is_on<A: Into<usize>>(&self, x: A, y: A) -> Result<bool, String> {
        let x = x.into();
        let y = y.into();
        let row = self.pixels.get(y).ok_or("y overflow".to_string())?;
        let pixel = row.get(x).ok_or("x overflow".to_string())?;
        Ok(*pixel)
    }

    pub fn set<A: Into<usize>>(&mut self, x: A, y: A, state: bool) -> Result<(), String> {
        let x = x.into();
        let y = y.into();
        let row = self.pixels.get_mut(y).ok_or("y overflow".to_string())?;
        let pixel = row.get_mut(x).ok_or("x overflow".to_string())?;
        *pixel = state;
        Ok(())
    }

    pub fn render(&self) -> Result<(), io::Error> {
        let mut stdout = io::stdout();
        for y in 0..self.pixels.len() - 1 {
            for x in 0..self.pixels[y].len() - 1 {
                if self.pixels[y][x] {
                    queue!(
                        stdout,
                        cursor::MoveTo(x as u16, y as u16),
                        style::Print("█")
                    )?;
                }
            }
        }
        stdout.flush()?;
        Ok(())
    }
}
