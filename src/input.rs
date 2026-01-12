use std::{error::Error, time::Duration};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

use crate::utils::debug_out;

#[derive(Debug, Default)]
pub struct Input {
    keys: [bool; 16],
}

impl Input {
    pub fn poll(&mut self) -> Result<(), Box<dyn Error>> {
        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(KeyEvent {
                code, modifiers, ..
            }) = event::read()?
            {
                match (code, modifiers) {
                    (KeyCode::Char('c'), KeyModifiers::CONTROL) => return Err("quitting...".into()),
                    (c, _) => {
                        if let Some(value) = Self::get_key_value(c) {
                            debug_out(value);
                            self.keys[value as usize] = true;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub fn clear(&mut self) {
        self.keys = [false; 16];
    }

    pub fn is_pressed(&self, key: u8) -> Result<bool, String> {
        let _key = self.keys.get(key as usize);
        if let Some(k) = _key {
            debug_out(k);
            return Ok(k.clone());
        }
        Err(format!("key index out of bounds: {key}"))
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
            KeyCode::Char('1') => Some(0x1),
            KeyCode::Char('2') => Some(0x2),
            KeyCode::Char('3') => Some(0x3),
            KeyCode::Char('4') => Some(0xC),
            KeyCode::Char('q') => Some(0x4),
            KeyCode::Char('w') => Some(0x5),
            KeyCode::Char('e') => Some(0x6),
            KeyCode::Char('r') => Some(0xD),
            KeyCode::Char('a') => Some(0x7),
            KeyCode::Char('s') => Some(0x8),
            KeyCode::Char('d') => Some(0x9),
            KeyCode::Char('f') => Some(0xE),
            KeyCode::Char('z') => Some(0xA),
            KeyCode::Char('x') => Some(0x0),
            KeyCode::Char('c') => Some(0xB),
            KeyCode::Char('v') => Some(0xE),
            _ => None,
        }
    }
}
