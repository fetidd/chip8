use std::{
    error::Error,
    io::Write,
    time::{Duration, Instant},
};

use crate::{
    display::Display,
    input,
    memory::{Memory, OpCode},
    program_counter::ProgramCounter,
    register::{Register8BitArray, Register16Bit},
    stack::Stack,
    timer::Timer,
    utils::debug_out,
};

#[derive(Debug)]
pub struct Chip8 {
    memory: Memory,
    stack: Stack,
    display: Display,
    index: Register16Bit,
    registers: Register8BitArray,
    pc: ProgramCounter,
    delay_timer: Timer,
    sound_timer: Timer,
    // input: Input,
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
            // input: Input::default(),
        }
    }
}

const FRAME_TIMEOUT: f32 = 1.0 / 60.0;

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
            // input,
        } = self;

        let unknown_opcode = |opcode: crate::memory::OpCode, addr: u16| {
            let addr = addr - ProgramCounter::INCREMENT;
            let e = format!(
                "unknown opcode {:04X} at {addr} [0x{addr:04X}]",
                opcode.inner(),
            );
            Err(e)
        };

        let mut last_display_refresh = Instant::now();
        let mut op_codes = std::fs::File::create("./codes.log")?;
        loop {
            let loop_time = Instant::now();

            let pressed = input::get_pressed_keys()?;

            if loop_time - last_display_refresh > Duration::from_secs_f32(FRAME_TIMEOUT) {
                display.render()?;
                last_display_refresh = loop_time;
            }

            // FETCH
            let opcode = memory.read_opcode(pc.get())?;
            pc.increment();

            let opcode_str = format!("{:04X}", opcode.inner()); // TODO remove this writing out
            write!(op_codes, "{opcode_str}\n")?;

            // DECODE + EXECUTE
            match opcode.code() {
                0x0 => match opcode.inner() {
                    // Clear the display
                    0x00E0 => display.clear()?,
                    // Return from subroutine
                    0x00EE => pc.set(stack.pop()),
                    _ => return unknown_opcode(opcode, pc.get())?,
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
                    if registers.get(opcode.x())?.get() == opcode.nn() {
                        pc.increment();
                    }
                }
                // Skip conditionally
                0x4 => {
                    if registers.get(opcode.x())?.get() != opcode.nn() {
                        pc.increment();
                    }
                }
                // Skip conditionally
                0x5 => {
                    if registers.get(opcode.x())?.get() == registers.get(opcode.y())?.get() {
                        pc.increment();
                    }
                }
                // Set variable register
                0x6 => registers.get_mut(opcode.x())?.set(opcode.nn()),
                // Add
                0x7 => {
                    let (new, _overflow) = registers
                        .get(opcode.x())?
                        .get()
                        .overflowing_add(opcode.nn());
                    registers.get_mut(opcode.x())?.set(new);
                }
                // Logical and arithmetic (all set vx unless stated)
                0x8 => match opcode.n() {
                    // Set
                    0x0 => {
                        let vy = registers.get(opcode.y())?.get();
                        registers.get_mut(opcode.x())?.set(vy);
                    }
                    // OR
                    0x1 => {
                        let result =
                            registers.get(opcode.x())?.get() | registers.get(opcode.y())?.get();
                        registers.get_mut(opcode.x())?.set(result);
                    }
                    // AND
                    0x2 => {
                        let result =
                            registers.get(opcode.x())?.get() & registers.get(opcode.y())?.get();
                        registers.get_mut(opcode.x())?.set(result);
                    }
                    // XOR
                    0x3 => {
                        let result =
                            registers.get(opcode.x())?.get() ^ registers.get(opcode.y())?.get();
                        registers.get_mut(opcode.x())?.set(result);
                    }
                    // Add (and set vf=1 (carry flag) if overflow)
                    0x4 => {
                        let (result, overflow) = registers
                            .get(opcode.x())?
                            .get()
                            .overflowing_add(registers.get(opcode.y())?.get());
                        registers.get_mut(opcode.x())?.set(result);
                        registers.get_mut(0xF)?.set(overflow as u8);
                    }
                    0x5 => Self::subtract_x_y(opcode.x(), opcode.y(), registers)?,
                    0x6 => Self::shift(opcode, registers, Dir::Right)?,
                    0x7 => Self::subtract_y_x(opcode.x(), opcode.y(), registers)?,
                    0xE => Self::shift(opcode, registers, Dir::Left)?,
                    _ => return unknown_opcode(opcode, pc.get())?,
                },
                // Skip conditionally
                0x9 => {
                    if registers.get(opcode.x())?.get() != registers.get(opcode.y())?.get() {
                        pc.increment();
                    }
                }
                // Set index
                0xA => index.set(opcode.nnn()),
                // Jump with offset TODO make configurable (see doc)
                0xB => pc.set(registers.get(0x0)?.get() as u16 + opcode.nnn()),
                // Set vx to random number AND nn
                0xC => registers
                    .get_mut(opcode.x())?
                    .set(rand::random::<u8>() & opcode.nn()),
                // Display logic
                0xD => Self::update_display(opcode, index, registers, memory, display)?,
                // Skip if key
                0xE => {
                    // let pressed = input.get_pressed_keys()?;
                    let register_value = registers.get(opcode.x())?.get();
                    match opcode.nn() {
                        // Skip if key pressed
                        0x9E => {
                            if pressed.contains(&register_value) {
                                pc.increment();
                            }
                        }
                        // Skip if key not pressed
                        0xA1 => {
                            if !pressed.contains(&register_value) {
                                pc.increment();
                            }
                        }
                        _ => return unknown_opcode(opcode, pc.get())?,
                    }
                }
                // Timers and memory
                0xF => match opcode.nn() {
                    // Set delay timer
                    0x07 => registers.get_mut(opcode.x())?.set(delay_timer.get()),
                    // Set delay timer to vx
                    0x15 => delay_timer.set(registers.get(opcode.x())?.get()),
                    // Set sound timer to vx
                    0x18 => sound_timer.set(registers.get(opcode.x())?.get()),
                    // Add vx to index
                    0x1E => index.set(index.get() + registers.get(opcode.x())?.get() as u16),
                    // Set index to font location of vx
                    0x29 => {
                        let char = registers.get(opcode.x())?.get() & 0x0F;
                        let char_addr = (char * 5) + 5;
                        index.set(char_addr as u16);
                    }
                    // Store BCD representation of vx at index
                    0x33 => {
                        let vx = registers.get(opcode.x())?.get();
                        let i = index.get();
                        memory.write(i, vx / 100)?;
                        memory.write(i + 1, (vx % 100) / 10)?;
                        memory.write(i + 2, vx % 10)?;
                    }
                    // Store vx at index, index+1, index+2
                    0x55 => {
                        let i = index.get();
                        for j in 0..=opcode.x() {
                            memory.write(i + j as u16, registers.get(j)?.get())?;
                        }
                    }
                    // Read vx from index, index+1, index+2
                    0x65 => {
                        let i = index.get();
                        for j in 0..=opcode.x() {
                            registers.get_mut(j)?.set(memory.read(i + j as u16)?);
                        }
                    }
                    // Wait for key press and store in vx
                    0x0A => {
                        if !pressed.is_empty() {
                            registers.get_mut(opcode.x())?.set(pressed[0]);
                        } else {
                            pc.decrement();
                        }
                    }
                    _ => return unknown_opcode(opcode, pc.get())?,
                },
                _ => return unknown_opcode(opcode, pc.get())?,
            }
        }
    }

    pub fn load_rom(&mut self, rom: &[u8]) -> Result<(), String> {
        self.memory.write_slice(Memory::PROGRAM_START, rom)
    }

    fn subtract(
        left: u8,
        right: u8,
        registers: &mut Register8BitArray,
    ) -> Result<(u8, bool), Box<dyn Error>> {
        let vl = registers.get(left)?;
        let vr = registers.get(right)?;
        let l = vl.get();
        let r = vr.get();
        Ok(l.overflowing_sub(r))
    }

    fn subtract_x_y(x: u8, y: u8, registers: &mut Register8BitArray) -> Result<(), Box<dyn Error>> {
        let (result, overflow) = Self::subtract(x, y, registers)?;
        registers.get_mut(x)?.set(result);
        registers.get_mut(0xF)?.set(if overflow { 0 } else { 1 });
        Ok(())
    }

    fn subtract_y_x(x: u8, y: u8, registers: &mut Register8BitArray) -> Result<(), Box<dyn Error>> {
        let (result, overflow) = Self::subtract(y, x, registers)?;
        registers.get_mut(x)?.set(result);
        registers.get_mut(0xF)?.set(if overflow { 0 } else { 1 });
        Ok(())
    }

    fn update_display(
        opcode: OpCode,
        index: &Register16Bit,
        registers: &mut Register8BitArray,
        memory: &Memory,
        display: &mut Display,
    ) -> Result<(), Box<dyn Error>> {
        let start_x = registers.get(opcode.x())?.get() % Display::WIDTH as u8;
        let mut y = registers.get(opcode.y())?.get() % Display::HEIGHT as u8;
        let n = opcode.n();
        registers.get_mut(0xF)?.set(0); // Reset collision flag
        for i in 0..n {
            let sprite = memory.read(index.get() + i as u16)?;
            let mut x = start_x; // Reset x for each row
            for bit_mask in [128, 64, 32, 16, 8, 4, 2, 1] {
                if x as usize >= Display::WIDTH {
                    break; // Clip at screen edge
                }
                if sprite & bit_mask != 0 {
                    if display.is_on(x, y)? {
                        display.set(x, y, false)?;
                        registers.get_mut(0xF)?.set(1); // Collision detected
                    } else {
                        display.set(x, y, true)?;
                    }
                }
                x += 1;
            }
            y += 1;
            if y as usize >= Display::HEIGHT {
                break;
            }
        }
        Ok(())
    }

    fn shift(
        opcode: OpCode,
        registers: &mut Register8BitArray,
        dir: Dir,
    ) -> Result<(), Box<dyn Error>> {
        let vx = registers.get(opcode.x())?.get();
        registers.get_mut(opcode.x())?.set(match dir {
            Dir::Left => vx << 1,
            Dir::Right => vx >> 1,
        });
        registers.get_mut(0xF)?.set(
            match dir {
                Dir::Left => vx >> 7,
                Dir::Right => vx,
            } & 0x1,
        );
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Dir {
    Left,
    Right,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chip8() {
        let chip8 = Chip8::default();
        assert_eq!(chip8.pc.get(), 0x200);
    }

    #[test]
    fn test_subtract_x_y() {}

    #[test]
    fn test_subtract_y_x() {}

    #[test]
    fn test_update_display() {}

    #[test]
    fn test_shift() {}
}
