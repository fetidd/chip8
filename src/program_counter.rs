#[derive(Debug, Default, Clone, Copy)]
pub struct ProgramCounter(pub u16);

impl ProgramCounter {
    pub const INCREMENT: u16 = 2;

    pub fn get(&self) -> u16 {
        self.0
    }

    pub fn increment(&mut self) {
        self.0 += Self::INCREMENT;
    }

    pub fn decrement(&mut self) {
        self.0 -= Self::INCREMENT;
    }

    pub fn set(&mut self, arg: u16) {
        self.0 = arg;
    }
}
