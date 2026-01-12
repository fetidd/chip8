#[macro_export]
macro_rules! new_register {
    ($name:ident, $size:ty) => {
        #[derive(Debug, Default, Clone, Copy)]
        pub struct $name($size);

        impl $name {
            pub fn get(&self) -> $size {
                self.0
            }

            pub fn set(&mut self, value: $size) {
                self.0 = value;
            }
        }
    };
}

new_register!(Register8Bit, u8);
new_register!(Register16Bit, u16);

#[derive(Debug, Default, Clone, Copy)]
pub struct Register8BitArray([Register8Bit; 16]);

impl Register8BitArray {
    pub fn get(&self, index: u8) -> Result<&Register8Bit, String> {
        self.0
            .get(index as usize)
            .ok_or_else(|| format!("register index out of bounds: {index}"))
    }

    pub fn get_mut(&mut self, index: u8) -> Result<&mut Register8Bit, String> {
        self.0
            .get_mut(index as usize)
            .ok_or_else(|| format!("register index out of bounds: {index}"))
    }
}
