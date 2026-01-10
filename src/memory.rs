use crate::font;

const _MEMORY_SIZE: usize = 4096;

#[derive(Debug, Clone, Copy)]
pub struct Memory([u8; _MEMORY_SIZE]);

impl Memory {
    const MEMORY_SIZE: usize = _MEMORY_SIZE;

    fn clear(&mut self) {
        self.0 = [0; Self::MEMORY_SIZE];
        self.0[0x00..font::FONT.len()].copy_from_slice(&font::FONT);
    }

    pub(crate) fn read(&self, addr: u16) -> OpCode {
        let addr = addr as usize;
        OpCode(u16::from_be_bytes([self.0[addr], self.0[addr + 1]]))
    }
}

impl Default for Memory {
    fn default() -> Self {
        let mut memory = Self([0; Self::MEMORY_SIZE]);
        memory.clear();
        memory
    }
}

pub struct OpCode(u16);

impl OpCode {
    pub fn inner(&self) -> u16 {
        self.0
        }

    pub fn code(&self) -> u8 {
        ((self.0 & 0xF000) >> 12) as u8
        }

    pub fn n(&self) -> u8 {
        (self.0 & 0x000F) as u8
        }

    pub fn nn(&self) -> u8 {
        (self.0 & 0x00FF) as u8
        }

    pub fn nnn(&self) -> u16 {
        self.0 & 0x0FFF
        }

    pub fn x(&self) -> usize {
        ((self.0 & 0x0F00) >> 8) as usize
        }

    pub fn y(&self) -> usize {
        ((self.0 & 0x00F0) >> 4) as usize
        }

    pub fn kk(&self) -> u8 {
        self.nn()
    }
}
