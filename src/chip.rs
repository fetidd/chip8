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

impl Chip {
    pub fn run(&mut self) {
        let Self {
            memory,
            stack,
            display,
            index,
            registers,
            pc,
            delay_timer,
            sound_timer,
        } = self;
        loop {
            let opcode = memory.read(pc.value());
            pc.increment();
            match opcode {
                0x00E0 => display.clear(),
                0x00EE => stack.pop(),
                _ => unimplemented!("opcode: {:04X}", opcode),
            }
        }
    }
}
