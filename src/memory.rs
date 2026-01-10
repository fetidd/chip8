const MEMORY_SIZE: usize = 4096;

#[derive(Debug, Clone, Copy)]
pub struct Memory([u8; MEMORY_SIZE]);

impl Memory {
    fn clear(&mut self) {
        self.0 = [0; MEMORY_SIZE];
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self([0; MEMORY_SIZE])
    }
}
