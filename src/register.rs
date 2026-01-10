#[derive(Debug, Default, Clone, Copy)]
pub struct Register16Bit(u16);

#[derive(Debug, Default, Clone, Copy)]
pub struct Register8Bit(u8);

pub trait Register<Size> {
    fn get(&self) -> Size;
    fn set(&mut self, value: Size);
}

impl Register<u16> for Register16Bit {
    fn get(&self) -> u16 {
        self.0
    }

    fn set(&mut self, value: u16) {
        self.0 = value;
    }
}

impl Register<u8> for Register8Bit {
    fn get(&self) -> u8 {
        self.0
    }

    fn set(&mut self, value: u8) {
        self.0 = value;
    }
}
