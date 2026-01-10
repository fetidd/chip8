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
                0x0 => match opcode.inner() {
                    0x00E0 => display.clear(),     // clear the display
                    0x00E3 => pc.set(stack.pop()), // return from subroutine
                    _ => unknown_opcode(opcode, pc.value(), &mut exit),
                },
                0x1 => pc.set(opcode.nnn()), // jump to address
                0x2 => {
                    // call subroutine at address
                    stack.push(pc.value());
                    pc.set(opcode.nnn());
                }
                0x3 => {} // skip conditionally
                0x4 => {} // skip conditionally
                0x5 => {} // skip conditionally
                0x6 => {} // set variable register
                0x7 => {} // add
                0x8 => match opcode.n() {
                    // logical and arithmetic (all set vx unless stated)
                    0x0 => {} // set
                    0x1 => {} // OR
                    0x2 => {} // AND
                    0x3 => {} // XOR
                    0x4 => {} // add (and set vf=1 (carry flag))
                    0x5 => {} // vx - vy
                    0x6 => {} // shift vx 1bit right TODO weird behavior (see doc)
                    0x7 => {} // vy - vx
                    0xE => {} // shift vx 1bit left TODO weird behavior (see doc)
                    _ => unknown_opcode(opcode, pc.value(), &mut exit),
                },
                0x9 => {}                       // skip conditionally
                0xA => index.set(opcode.nnn()), // set index
                0xB => pc.set(registers[0x0].get() as u16 + opcode.nnn()), // jump with offset TODO make configurable (see doc)
                0xC => {} // set vx to random number AND nn
                0xD => {} // display logic
                0xE => match opcode.nn() {
                    // skip if key
                    0x9E => {}
                    0xA1 => {}
                    _ => unknown_opcode(opcode, pc.value(), &mut exit),
                },
                0xF => {} // many things
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
        let chip8 = Chip8::default();
        assert_eq!(chip8.pc.value(), 0x200);
    }
}
