use crate::font::FONT;

#[derive(Debug, Clone, Copy)]
pub struct Memory([u8; Self::MEMORY_SIZE]);

impl Memory {
    const MEMORY_SIZE: usize = 4096;
    pub const FONT_START: u16 = 0x050;
    pub const PROGRAM_START: u16 = 0x200;

    fn _clear(&mut self) -> Result<(), String> {
        self.write_slice(
            Self::PROGRAM_START,
            &[0; Self::MEMORY_SIZE - Self::PROGRAM_START as usize],
        )
    }

    pub fn read_opcode<A: Into<usize>>(&self, addr: A) -> Result<OpCode, String> {
        let addr = addr.into();
        let hi = *self
            .0
            .get(addr)
            .ok_or_else(|| format!("memory read out of bounds: {addr}"))?;
        let lo = *self
            .0
            .get(addr + 1)
            .ok_or_else(|| format!("memory read out of bounds: {}", addr + 1))?;
        Ok(OpCode(u16::from_be_bytes([hi, lo])))
    }

    pub fn read<A: Into<usize>>(&self, addr: A) -> Result<u8, String> {
        let addr = addr.into();
        self.0
            .get(addr)
            .copied()
            .ok_or_else(|| format!("memory read out of bounds: {addr}"))
    }

    pub fn write<A: Into<usize>>(&mut self, addr: A, value: u8) -> Result<(), String> {
        let addr = addr.into();
        let cell = self
            .0
            .get_mut(addr)
            .ok_or_else(|| format!("memory write out of bounds: {addr}"))?;
        *cell = value;
        Ok(())
    }

    pub fn write_slice<A: Into<usize>>(&mut self, addr: A, data: &[u8]) -> Result<(), String> {
        let addr = addr.into();
        let end = addr + data.len();
        if end > Self::MEMORY_SIZE {
            return Err(format!("memory write_slice out of bounds: {addr}..{end}"));
        }
        self.0[addr..end].copy_from_slice(data);
        Ok(())
    }
}

impl Default for Memory {
    fn default() -> Self {
        let mut memory = Self([0; Self::MEMORY_SIZE]);
        memory.0[Self::FONT_START as usize..Self::FONT_START as usize + FONT.len()]
            .copy_from_slice(&FONT);
        memory
    }
}

#[derive(Debug, Clone, Copy)]
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
