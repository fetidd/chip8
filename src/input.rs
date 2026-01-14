use std::time::Duration;

use crate::error::Result;
use crate::utils::debug_out;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

#[derive(Debug, Default)]
pub struct Keys {
    pressed: [bool; 16],
}

impl Keys {
    pub fn is_pressed(&self, key: u8) -> bool {
        self.pressed[key as usize]
    }

    pub fn first_pressed(&self) -> Option<u8> {
        let first = self
            .pressed
            .iter()
            .enumerate()
            .map(|(i, _)| i as u8)
            .take(1)
            .collect::<Vec<_>>();
        if !first.is_empty() {
            Some(first[0])
        } else {
            None
        }
    }

    pub fn update_pressed(&mut self) -> Result<()> {
        self.update(event::KeyEventKind::Press)
    }

    pub fn update_released(&mut self) -> Result<()> {
        self.update(event::KeyEventKind::Release)
    }

    fn update(&mut self, exp_kind: event::KeyEventKind) -> Result<()> {
        let events = self.get_events()?;
        for (code, _, kind) in events {
            match (code, kind) {
                (c, k) if k == exp_kind => {
                    if let Some(key) = Self::get_key_value(c) {
                        self.pressed[key as usize] = exp_kind == event::KeyEventKind::Press;
                    }
                }
                _ => {}
            }
        }
        // debug_out(&self.pressed);
        Ok(())
    }

    fn get_events(&self) -> Result<Vec<(KeyCode, KeyModifiers, KeyEventKind)>> {
        let mut events = vec![];
        if event::poll(Duration::ZERO)? {
            if let Ok(event) = event::read() {
                if let Event::Key(KeyEvent {
                    code,
                    modifiers,
                    kind,
                    ..
                }) = event
                {
                    events.push((code, modifiers, kind));
                }
            }
        }
        Ok(events)
    }

    fn get_first_event(&self) -> Result<Option<(KeyCode, KeyModifiers, KeyEventKind)>> {
        if event::poll(Duration::ZERO)? {
            if let Ok(event) = event::read() {
                if let Event::Key(KeyEvent {
                    code,
                    modifiers,
                    kind,
                    ..
                }) = event
                {
                    return Ok(Some((code, modifiers, kind)));
                }
            }
        }
        Ok(None)
    }

    pub fn check_for_interrupt(&self) -> Result<()> {
        let event = self.get_first_event()?;
        if let Some((code, modifiers, _)) = event {
            match (code, modifiers) {
                (KeyCode::Char('c'), KeyModifiers::CONTROL) => {
                    return Err(crate::error::Error::UnknownError("quitting...".to_string()));
                }
                _ => {}
            }
        }
        Ok(())
    }

    pub fn iter(&self) -> impl Iterator<Item = (u8, bool)> {
        self.pressed.iter().enumerate().map(|(i, p)| (i as u8, *p))
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
