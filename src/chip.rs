use crate::{
    display::Display,
    memory::Memory,
    program_counter::ProgramCounter,
    register::{Register8Bit, Register16Bit},
    stack::Stack,
    timer::Timer,
};

#[derive(Default)]
pub struct Chip {
    memory: Memory,
    stack: Stack,
    display: Display,
    index: Register16Bit,
    registers: [Register8Bit; 16],
    pc: ProgramCounter,
    delay_timer: Timer,
    sound_timer: Timer,
}

impl Chip {}
