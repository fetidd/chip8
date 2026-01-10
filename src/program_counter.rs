#[derive(Debug, Default, Clone, Copy)]
pub struct ProgramCounter(u16);

impl ProgramCounter {
    pub const INCREMENT: u16 = 2;

    pub fn value(&self) -> u16 {
        self.0
    }

    pub fn increment(&mut self) {
        self.0 += Self::INCREMENT;
    }

    pub fn set(&mut self, addr: u16) {
        self.0 = addr;
    }
}
