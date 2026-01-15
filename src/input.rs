use crate::error::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::time::Duration;

#[derive(Debug, Default)]
pub struct Keypad {
    pressed: [bool; 16],
}

impl Keypad {
    pub fn poll(&mut self) -> Result<bool> {
        if event::poll(Duration::from_millis(1))? {
            if let Event::Key(KeyEvent {
                code, modifiers, ..
            }) = event::read()?
            {
                match (code, modifiers) {
                    (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                        return Ok(true);
                    }
                    (code, _) => {
                        if let Some(key) = Keypad::get_key_value(code) {
                            self.pressed[key as usize] = true;
                        }
                    }
                }
            }
        }
        Ok(false)
    }

    pub fn pressed(&self) -> Option<u8> {
        self.pressed
            .iter()
            .enumerate()
            .filter(|(_i, k)| **k)
            .map(|(i, _)| i as u8)
            .collect::<Vec<_>>()
            .first()
            .copied()
    }

    pub fn is_pressed(&self, key: u8) -> bool {
        self.pressed
            .iter()
            .enumerate()
            .any(|(i, x)| *x && i == key as usize)
    }

    fn get_key_value(key: KeyCode) -> Option<u8> {
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
            KeyCode::Char('v') => Some(0xF),
            _ => None,
        }
    }
}
