use crate::font::FONT;

const _MEMORY_SIZE: usize = 4096;

#[derive(Debug, Clone, Copy)]
pub struct Memory([u8; _MEMORY_SIZE]);

impl Memory {
    const MEMORY_SIZE: usize = _MEMORY_SIZE;
    pub const FONT_START: u16 = 0x050;
    pub const PROGRAM_START: u16 = 0x200;

    fn clear(&mut self) {
        self.write_slice(Self::PROGRAM_START, &[0; Self::MEMORY_SIZE - Self::PROGRAM_START as usize]);
    }

    pub fn read_opcode<A: Into<usize>>(&self, addr: A) -> OpCode {
        let addr = addr.into();
        OpCode(u16::from_be_bytes([self.0[addr], self.0[addr + 1]]))
    }

    pub fn read<A: Into<usize>>(&self, addr: A) -> u8 {
        self.0[addr.into()]
    }

    pub fn write<A: Into<usize>>(&mut self, addr: A, value: u8) {
        self.0[addr.into()] = value;
    }
    
    pub fn write_slice<A: Into<usize>>(&mut self, addr: A, data: &[u8]) {
        let addr = addr.into();
        self.0[addr..addr + data.len()].copy_from_slice(data);
    }
}

impl Default for Memory {
    fn default() -> Self {
        let mut memory = Self([0; Self::MEMORY_SIZE]);
        memory.0[Self::FONT_START as usize..Self::FONT_START as usize + FONT.len()].copy_from_slice(&FONT);
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

    pub fn x(&self) -> u8 {
        ((self.0 & 0x0F00) >> 8) as u8
    }

    pub fn y(&self) -> u8 {
        ((self.0 & 0x00F0) >> 4) as u8
    }
}
