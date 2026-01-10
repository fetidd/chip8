use crate::font;

const MEMORY_SIZE: usize = 4096;

#[derive(Debug, Clone, Copy)]
pub struct Memory([u8; MEMORY_SIZE]);

impl Memory {
    fn clear(&mut self) {
        self.0 = [0; MEMORY_SIZE];
        self.0[0x50..0x90].copy_from_slice(&font::FONT);
    }

    pub(crate) fn read(&self, value: _) -> i32 {
        todo!()
    }
}

impl Default for Memory {
    fn default() -> Self {
        let mut memory = Self([0; MEMORY_SIZE]);
        memory.clear();
        memory
    }
}
