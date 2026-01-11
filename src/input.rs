use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEvent};

#[derive(Debug, Default)]
pub struct Input {
    keys: [bool; 16],
}

impl Input {
    pub fn poll(&mut self) -> Result<(), std::io::Error> {
        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                if let Some(value) = Self::get_key_value(code) {
                    self.keys[value as usize] = true;
                }
            }
        }
        Ok(())
    }

    pub fn is_pressed(&self, key: u8) -> bool {
        self.keys[key as usize]
    }

    pub fn get_pressed(&self) -> Option<u8> {
        for (i, &key) in self.keys.iter().enumerate() {
            if key {
                return Some(i as u8);
            }
        }
        None
    }

    pub fn get_key_value(key: KeyCode) -> Option<u8> {
        match key {
            // TODO use scancodes?
            KeyCode::Char('1') => Some(0x0),
            KeyCode::Char('2') => Some(0x1),
            KeyCode::Char('3') => Some(0x2),
            KeyCode::Char('4') => Some(0x3),
            KeyCode::Char('q') => Some(0x4),
            KeyCode::Char('w') => Some(0x5),
            KeyCode::Char('e') => Some(0x6),
            KeyCode::Char('r') => Some(0x7),
            KeyCode::Char('a') => Some(0x8),
            KeyCode::Char('s') => Some(0x9),
            KeyCode::Char('d') => Some(0xA),
            KeyCode::Char('f') => Some(0xB),
            KeyCode::Char('z') => Some(0xC),
            KeyCode::Char('x') => Some(0xD),
            KeyCode::Char('c') => Some(0xE),
            KeyCode::Char('v') => Some(0xF),
            _ => None,
        }
    }
}
