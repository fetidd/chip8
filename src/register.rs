use std::ops::{Index, IndexMut};

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

impl Index<u8> for Register8BitArray {
    type Output = Register8Bit;

    fn index(&self, index: u8) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl IndexMut<u8> for Register8BitArray {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}
