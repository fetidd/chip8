use crate::{
    display::Display,
    memory::Memory,
    program_counter::ProgramCounter,
    register::{Register, Register8Bit, Register16Bit},
    stack::Stack,
    timer::Timer,
};

#[derive(Default)]
pub struct Chip8 {
    memory: Memory,
    stack: Stack,
    display: Display,
    index: Register16Bit,
    registers: [Register8Bit; 16],
    pc: ProgramCounter,
    delay_timer: Timer,
    sound_timer: Timer,
}

impl Chip8 {
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

        let mut exit = false;
        let unknown_opcode = |opcode: crate::memory::OpCode, addr: u16, should_exit: &mut bool| {
            println!(
                "unknown opcode {:04X} at 0x{:04X}",
                opcode.inner(),
                addr - ProgramCounter::INCREMENT
            );
            *should_exit = true;
        };

        pc.set(0x200); // this is where the program starts, memory before this is reserved for the interpreter and the font set

        'main: loop {
            // FETCH
            let opcode = memory.read(pc.value());
            pc.increment();

            // DECODE + EXECUTE
            match opcode.code() {
                // SYSTEM
                0x0 => match opcode.inner() {
                    0x00E0 => display.clear(),
                    _ => unknown_opcode(opcode, pc.value(), &mut exit),
                },
                
                // FLOW CONTROL
                0x1 => pc.set(opcode.nnn()),
                0x2 => {
                    stack.push(pc.value());
                    pc.set(opcode.nnn());
                }
                0xB => pc.set(registers[0x0].get() as u16 + opcode.nnn()),
                
                // MEMORY
                0xA => index.set(opcode.nnn()),
                _ => unknown_opcode(opcode, pc.value(), &mut exit),
            }

            if exit {
                break 'main;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chip8() {
        let mut chip8 = Chip8::new();
        assert_eq!(chip8.pc.value(), 0x200);
    }
}