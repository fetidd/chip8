use std::{io::Write, thread, time::Duration};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

use crate::{
    display::Display,
    memory::Memory,
    program_counter::ProgramCounter,
    register::{Register8BitArray, Register16Bit},
    stack::Stack,
    timer::Timer,
    input::Input,
};

pub struct Chip8 {
    memory: Memory,
    stack: Stack,
    display: Display,
    index: Register16Bit,
    registers: Register8BitArray,
    pc: ProgramCounter,
    delay_timer: Timer,
    sound_timer: Timer,
    input: Input,
}

impl Default for Chip8 {
    fn default() -> Self {
        Self {
            memory: Memory::default(),
            stack: Stack::default(),
            display: Display::default(),
            index: Register16Bit::default(),
            registers: Register8BitArray::default(),
            pc: ProgramCounter(Memory::PROGRAM_START),
            delay_timer: Timer::default(),
            sound_timer: Timer::default(),
            input: Input::default(),
        }
    }
}

impl Chip8 {
    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let Self {
            memory,
            stack,
            display,
            index,
            registers,
            pc,
            delay_timer,
            sound_timer,
            input,
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

        'main: loop {
            
            // enable exiting the interpreter with ctrl-c
            Self::check_keyboard_interrupt(&mut exit)?;
            
            input.poll()?;

            // FETCH
            let opcode = memory.read_opcode(pc.get());
            pc.increment();
            
            // DECODE + EXECUTE
            match opcode.code() {
                0x0 => match opcode.inner() {
                    // Clear the display
                    0x00E0 => display.clear(),
                    // Return from subroutine
                    0x00E3 => pc.set(stack.pop()),
                    _ => unknown_opcode(opcode, pc.get(), &mut exit),
                },
                // Jump to address
                0x1 => pc.set(opcode.nnn()),
                // Call subroutine at address
                0x2 => {
                    stack.push(pc.get());
                    pc.set(opcode.nnn());
                }
                // Skip conditionally
                0x3 => {
                    if registers[opcode.x()].get() == opcode.nn() {
                        pc.increment();
                    }
                }
                // Skip conditionally
                0x4 => {
                    if registers[opcode.x()].get() != opcode.nn() {
                        pc.increment();
                    }
                }
                // Skip conditionally
                0x5 => {
                    if registers[opcode.x()].get() == registers[opcode.y()].get() {
                        pc.increment();
                    }
                }
                // Set variable register
                0x6 => registers[opcode.x()].set(opcode.nn()),
                // Add
                0x7 => {
                    let new = registers[opcode.x()].get() + opcode.nn();
                    registers[opcode.x()].set(new);
                }
                // Logical and arithmetic (all set vx unless stated)
                0x8 => match opcode.n() {
                    // Set
                    0x0 => {
                        let vy = registers[opcode.y()].get();
                        registers[opcode.x()].set(vy);
                    }
                    // OR
                    0x1 => {
                        let result = registers[opcode.x()].get() | registers[opcode.y()].get();
                        registers[opcode.x()].set(result);
                    }
                    // AND
                    0x2 => {
                        let result = registers[opcode.x()].get() & registers[opcode.y()].get();
                        registers[opcode.x()].set(result);
                    }
                    // XOR
                    0x3 => {
                        let result = registers[opcode.x()].get() ^ registers[opcode.y()].get();
                        registers[opcode.x()].set(result);
                    }
                    // Add (and set vf=1 (carry flag) if overflow)
                    0x4 => {
                        let (result, overflow) = registers[opcode.x()]
                            .get()
                            .overflowing_add(registers[opcode.y()].get());
                        registers[opcode.x()].set(result);
                        registers[0xF].set(overflow as u8);
                    }
                    // vx - vy
                    0x5 => {
                        let result = registers[opcode.x()].get() - registers[opcode.y()].get();
                        registers[opcode.x()].set(result);
                    }
                    // Shift vx 1bit right TODO weird behavior (see doc)
                    0x6 => {
                        let vx = registers[opcode.x()].get();
                        registers[0xF].set(vx & 0x1);
                        registers[opcode.x()].set(vx >> 1);
                    }
                    // vy - vx
                    0x7 => {
                        let result = registers[opcode.y()].get() - registers[opcode.x()].get();
                        registers[opcode.x()].set(result);
                    }
                    // Shift vx 1bit left TODO weird behavior (see doc)
                    0xE => {
                        let vx = registers[opcode.x()].get();
                        registers[0xF].set((vx >> 7) & 0x1);
                        registers[opcode.x()].set(vx << 1);
                    }
                    _ => unknown_opcode(opcode, pc.get(), &mut exit),
                },
                // Skip conditionally
                0x9 => {
                    if registers[opcode.x()].get() != registers[opcode.y()].get() {
                        pc.increment();
                    }
                }
                // Set index
                0xA => index.set(opcode.nnn()),
                // Jump with offset TODO make configurable (see doc)
                0xB => pc.set(registers[0x0].get() as u16 + opcode.nnn()),
                // Set vx to random number AND nn
                0xC => registers[opcode.x()].set(rand::random::<u8>() & opcode.nn()),
                // Display logic
                0xD => {
                    let x = registers[opcode.x()].get();
                    let y = registers[opcode.y()].get();
                }
                // Skip if key
                0xE => match opcode.nn() {
                    // Skip if key pressed
                    0x9E => if input.is_pressed(opcode.x()) {
                        pc.increment();
                    },
                    // Skip if key not pressed
                    0xA1 => if !input.is_pressed(opcode.x()) {
                        pc.increment();
                    },
                    _ => unknown_opcode(opcode, pc.get(), &mut exit),
                },
                // Timers and memory
                0xF => match opcode.nn() {
                    // Set delay timer
                    0x07 => registers[0x0].set(delay_timer.get()),
                    // Set delay timer to vx
                    0x15 => delay_timer.set(registers[opcode.x()].get()),
                    // Set sound timer to vx
                    0x18 => sound_timer.set(registers[opcode.x()].get()),
                    // Add vx to index
                    0x1E => index.set(index.get() + registers[opcode.x()].get() as u16),
                    // Set index to font location of vx
                    0x29 => {
                        let sprite = memory.read(registers[opcode.x()].get());
                        index.set(sprite as u16);
                    }
                    // Store BCD representation of vx at index
                    0x33 => {
                        let vx = registers[opcode.x()].get();
                        let i = index.get();
                        memory.write(i, vx / 100);
                        memory.write(i + 1, (vx % 100) / 10);
                        memory.write(i + 2, vx % 10);
                    }
                    // Store vx at index, index+1, index+2
                    0x55 => {
                        let i = index.get();
                        for j in 0..=opcode.x() {
                            memory.write(i + j as u16, registers[j].get());
                        }
                    }
                    // Read vx from index, index+1, index+2
                    0x65 => {
                        let i = index.get();
                        for j in 0..=opcode.x() {
                            registers[j].set(memory.read(i + j as u16));
                        }
                    }
                    // Wait for key press and store in vx
                    0x0A => {
                        input.block()?;
                        registers[opcode.x()].set(input.get_pressed().expect("there was no pressed key after blocking for 0xFX0A..."));
                    }
                    _ => unknown_opcode(opcode, pc.get(), &mut exit),
                },
                _ => unknown_opcode(opcode, pc.get(), &mut exit),
            }

            if exit {
                break 'main;
            }
        }
        Ok(())
    }
    
    pub fn load_rom(&mut self, rom: &[u8]) {
        self.memory.write_slice(Memory::PROGRAM_START, rom);
    }
    
    fn check_keyboard_interrupt(should_exit: &mut bool) -> Result<(), std::io::Error> {
        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(KeyEvent { code, modifiers, .. }) = event::read()? {
                match (code, modifiers) {
                    (KeyCode::Char('c'), KeyModifiers::CONTROL) => *should_exit = true,
                    _ => {}
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chip8() {
        let chip8 = Chip8::default();
        assert_eq!(chip8.pc.get(), 0x200);
    }
}
