#[derive(Debug)]
pub struct Display {
    pub pixels: [[bool; 64]; 32],
}

impl Default for Display {
    fn default() -> Self {
        Self {
            pixels: [[false; 64]; 32],
        }
    }
}

impl Display {
    pub fn clear(&mut self) {
        self.pixels = [[false; 64]; 32];
    }
}
