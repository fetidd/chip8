use std::io::{self, Write};

use crossterm::{cursor, queue, style};

#[derive(Debug)]
pub struct Screen {}

#[derive(Debug)]
pub struct DisplayBuffer {
    pub pixels: [[bool; Self::WIDTH]; Self::HEIGHT],
}

impl Default for DisplayBuffer {
    fn default() -> Self {
        Self {
            pixels: [[false; Self::WIDTH]; Self::HEIGHT],
        }
    }
}

impl DisplayBuffer {
    pub const WIDTH: usize = 64;
    pub const HEIGHT: usize = 32;

    pub fn clear(&mut self) -> Result<(), std::io::Error> {
        self.pixels = [[false; Self::WIDTH]; Self::HEIGHT];
        Ok(())
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
}
