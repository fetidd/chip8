use std::{error::Error, time::Duration};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

use crate::utils::debug_out;

pub struct Keys {
    pressed: [u8; 16],
}

impl Keys {
    pub fn update_pressed(&mut self) -> Result<(), Box<dyn Error>> {
        if event::poll(Duration::from_micros(5))? {
            if let Event::Key(KeyEvent {
                code,
                modifiers,
                kind,
                ..
            }) = event::read()?
            {
                match (code, modifiers, kind) {
                    (KeyCode::Char('c'), KeyModifiers::CONTROL, event::KeyEventKind::Press) => {
                        return Err("quitting...".into());
                    }
                    (c, _, event::KeyEventKind::Press) => {
                        if let Some(value) = Self::get_key_value(c) {
                            debug_out(&value);
                            return Ok(Some(value));
                        } else {
                            return Ok(None);
                        }
                    }
                    _ => {
                        return Ok(None);
                    }
                }
            } else {
                return Ok(None);
            }
        }
        Ok(None)
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
