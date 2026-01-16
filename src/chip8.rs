use crate::{
    display::DisplayBuffer,
    error::Result,
    input::Keypad,
    memory::{Memory, OpCode},
    program_counter::ProgramCounter,
    register::{Register8BitArray, Register16Bit},
    stack::Stack,
    timer::Timer,
};

#[derive(Debug)]
pub struct Chip8 {
    memory: Memory,
    stack: Stack,
    pub(crate) display_buffer: DisplayBuffer,
    index: Register16Bit,
    registers: Register8BitArray,
    pc: ProgramCounter,
    delay_timer: Timer,
    sound_timer: Timer,
}

impl Default for Chip8 {
    fn default() -> Self {
        Self {
            memory: Memory::default(),
            stack: Stack::default(),
            display_buffer: DisplayBuffer::default(),
            index: Register16Bit::default(),
            registers: Register8BitArray::default(),
            pc: ProgramCounter(Memory::PROGRAM_START),
            delay_timer: Timer::default(),
            sound_timer: Timer::default(),
        }
    }
}

impl Chip8 {
    pub fn cycle(&mut self, keypad: &Keypad) -> Result<()> {
        // FETCH
        let opcode = self.memory.read_opcode(self.pc.get())?;
        self.pc.increment();

        // DECODE + EXECUTE
        self.execute_instruction(opcode, keypad)?;
        Ok(())
    }

    fn execute_instruction(&mut self, opcode: OpCode, keypad: &Keypad) -> Result<()> {
        match opcode.code() {
            0x0 => match opcode.inner() {
                0x00E0 => self.clear_display_buffer(opcode)?,
                0x00EE => self.return_from_subroutine(opcode)?,
                _ => self.unknown_opcode(opcode)?,
            },
            0x1 => self.jump_to_address(opcode)?,
            0x2 => self.call_subroutine(opcode)?,
            0x3 => self.skip_conditonally_equal_nn(opcode)?,
            0x4 => self.skip_conditonally_not_equal_nn(opcode)?,
            0x5 => self.skip_conditonally_equal_xy(opcode)?,
            0x6 => self.set_variable_register(opcode)?,
            0x7 => self.add(opcode)?,
            0x8 => match opcode.n() {
                0x0 => self.set_vx_from_vy(opcode)?,
                0x1 => self.or(opcode)?,
                0x2 => self.and(opcode)?,
                0x3 => self.xor(opcode)?,
                0x4 => self.add_carry(opcode)?,
                0x5 => self.subtract_x_y(opcode)?,
                0x6 => self.shift_right(opcode)?,
                0x7 => self.subtract_y_x(opcode)?,
                0xE => self.shift_left(opcode)?,
                _ => self.unknown_opcode(opcode)?,
            },
            0x9 => self.skip_conditonally_not_equal_xy(opcode)?,
            0xA => self.set_index(opcode)?,
            0xB => self.jump_with_offset(opcode)?,
            0xC => self.random_and(opcode)?,
            0xD => self.update_display_buffer(opcode)?,
            0xE => match opcode.nn() {
                0x9E => self.skip_if_key(opcode, keypad)?,
                0xA1 => self.skip_if_not_key(opcode, keypad)?,
                _ => self.unknown_opcode(opcode)?,
            },
            0xF => match opcode.nn() {
                0x07 => self.set_x_to_delay(opcode)?,
                0x15 => self.set_delay(opcode)?,
                0x18 => self.set_sound(opcode)?,
                0x1E => self.add_x_to_index(opcode)?,
                0x29 => self.set_index_to_font(opcode)?,
                0x33 => self.bcd_x_in_index(opcode)?,
                0x55 => self.set_x_in_index_spread(opcode)?,
                0x65 => self.read_x_from_index_spread(opcode)?,
                0x0A => self.wait_for_key(opcode, keypad)?,
                _ => self.unknown_opcode(opcode)?,
            },
            _ => self.unknown_opcode(opcode)?,
        }
        Ok(())
    }

    fn unknown_opcode(&mut self, opcode: OpCode) -> Result<()> {
        let addr = self.pc.get() - ProgramCounter::INCREMENT;
        let e = format!(
            "unknown opcode {:04X} at {addr} [0x{addr:04X}]",
            opcode.inner(),
        );
        Err(crate::error::Error::Fatal(e))
    }

    fn call_subroutine(&mut self, opcode: OpCode) -> Result<()> {
        self.stack.push(self.pc.get());
        self.pc.set(opcode.nnn());
        Ok(())
    }

    fn return_from_subroutine(&mut self, _opcode: OpCode) -> Result<()> {
        self.pc.set(self.stack.pop());
        Ok(())
    }

    fn jump_to_address(&mut self, opcode: OpCode) -> Result<()> {
        self.pc.set(opcode.nnn());
        Ok(())
    }

    pub fn load_rom(&mut self, rom: &[u8]) -> Result<()> {
        self.memory.write_slice(Memory::PROGRAM_START, rom)
    }

    fn subtract(&mut self, left: u8, right: u8) -> Result<(u8, bool)> {
        let vl = self.registers.get(left)?;
        let vr = self.registers.get(right)?;
        let l = vl.get();
        let r = vr.get();
        Ok(l.overflowing_sub(r))
    }

    fn subtract_x_y(&mut self, opcode: OpCode) -> Result<()> {
        let (x, y) = (opcode.x(), opcode.y());
        let (result, overflow) = self.subtract(x, y)?;
        self.registers.get_mut(x)?.set(result);
        self.registers
            .get_mut(0xF)?
            .set(if overflow { 0 } else { 1 });
        Ok(())
    }

    fn subtract_y_x(&mut self, opcode: OpCode) -> Result<()> {
        let (x, y) = (opcode.x(), opcode.y());
        let (result, overflow) = self.subtract(y, x)?;
        self.registers.get_mut(x)?.set(result);
        self.registers
            .get_mut(0xF)?
            .set(if overflow { 0 } else { 1 });
        Ok(())
    }

    fn clear_display_buffer(&mut self, _opcode: OpCode) -> Result<()> {
        self.display_buffer.clear()?;
        Ok(())
    }

    fn update_display_buffer(&mut self, opcode: OpCode) -> Result<()> {
        let start_x = self.registers.get(opcode.x())?.get() % DisplayBuffer::WIDTH as u8;
        let mut y = self.registers.get(opcode.y())?.get() % DisplayBuffer::HEIGHT as u8;
        let n = opcode.n();
        self.registers.get_mut(0xF)?.set(0); // Reset collision flag
        for i in 0..n {
            let sprite = self.memory.read(self.index.get() + i as u16)?;
            let mut x = start_x; // Reset x for each row
            for bit_mask in [128, 64, 32, 16, 8, 4, 2, 1] {
                if x as usize >= DisplayBuffer::WIDTH {
                    break; // Clip at screen edge
                }
                if sprite & bit_mask != 0 {
                    if self.display_buffer.is_on(x, y)? {
                        self.display_buffer.set(x, y, false)?;
                        self.registers.get_mut(0xF)?.set(1); // Collision detected
                    } else {
                        self.display_buffer.set(x, y, true)?;
                    }
                }
                x += 1;
            }
            y += 1;
            if y as usize >= DisplayBuffer::HEIGHT {
                break;
            }
        }
        Ok(())
    }

    fn shift(&mut self, opcode: OpCode, dir: Dir) -> Result<()> {
        let vx = self.registers.get(opcode.x())?.get();
        self.registers.get_mut(opcode.x())?.set(match dir {
            Dir::Left => vx << 1,
            Dir::Right => vx >> 1,
        });
        self.registers.get_mut(0xF)?.set(
            match dir {
                Dir::Left => vx >> 7,
                Dir::Right => vx,
            } & 0x1,
        );
        Ok(())
    }

    pub fn shift_left(&mut self, opcode: OpCode) -> Result<()> {
        self.shift(opcode, Dir::Left)
    }

    pub fn shift_right(&mut self, opcode: OpCode) -> Result<()> {
        self.shift(opcode, Dir::Right)
    }

    fn set_vx_from_vy(&mut self, opcode: OpCode) -> Result<()> {
        let vy = self.registers.get(opcode.y())?.get();
        self.registers.get_mut(opcode.x())?.set(vy);
        Ok(())
    }

    fn set_variable_register(&mut self, opcode: OpCode) -> Result<()> {
        self.registers.get_mut(opcode.x())?.set(opcode.nn());
        Ok(())
    }

    fn set_index(&mut self, opcode: OpCode) -> Result<()> {
        self.index.set(opcode.nnn());
        Ok(())
    }

    fn or(&mut self, opcode: OpCode) -> Result<()> {
        let result = self.registers.get(opcode.x())?.get() | self.registers.get(opcode.y())?.get();
        self.registers.get_mut(opcode.x())?.set(result);
        Ok(())
    }

    fn and(&mut self, opcode: OpCode) -> Result<()> {
        let result = self.registers.get(opcode.x())?.get() & self.registers.get(opcode.y())?.get();
        self.registers.get_mut(opcode.x())?.set(result);
        Ok(())
    }

    fn xor(&mut self, opcode: OpCode) -> Result<()> {
        let result = self.registers.get(opcode.x())?.get() ^ self.registers.get(opcode.y())?.get();
        self.registers.get_mut(opcode.x())?.set(result);
        Ok(())
    }

    fn add(&mut self, opcode: OpCode) -> Result<()> {
        let (new, _overflow) = self
            .registers
            .get(opcode.x())?
            .get()
            .overflowing_add(opcode.nn());
        self.registers.get_mut(opcode.x())?.set(new);
        Ok(())
    }

    fn add_carry(&mut self, opcode: OpCode) -> Result<()> {
        let (result, overflow) = self
            .registers
            .get(opcode.x())?
            .get()
            .overflowing_add(self.registers.get(opcode.y())?.get());
        self.registers.get_mut(opcode.x())?.set(result);
        self.registers.get_mut(0xF)?.set(overflow as u8);
        Ok(())
    }

    fn skip_if_key(&mut self, opcode: OpCode, keypad: &Keypad) -> Result<()> {
        let register_value = self.registers.get(opcode.x())?.get();
        if keypad.is_pressed(register_value) {
            self.pc.increment();
        }
        Ok(())
    }

    fn skip_if_not_key(&mut self, opcode: OpCode, keypad: &Keypad) -> Result<()> {
        let register_value = self.registers.get(opcode.x())?.get();
        if !keypad.is_pressed(register_value) {
            self.pc.increment();
        }
        Ok(())
    }

    fn jump_with_offset(&mut self, opcode: OpCode) -> Result<()> {
        self.pc
            .set(self.registers.get(0x0)?.get() as u16 + opcode.nnn());
        Ok(())
    }

    fn random_and(&mut self, opcode: OpCode) -> Result<()> {
        self.registers
            .get_mut(opcode.x())?
            .set(rand::random::<u8>() & opcode.nn());
        Ok(())
    }

    fn skip_conditonally_not_equal_nn(&mut self, opcode: OpCode) -> Result<()> {
        if self.registers.get(opcode.x())?.get() != opcode.nn() {
            self.pc.increment();
        }
        Ok(())
    }

    fn skip_conditonally_equal_nn(&mut self, opcode: OpCode) -> Result<()> {
        if self.registers.get(opcode.x())?.get() == opcode.nn() {
            self.pc.increment();
        }
        Ok(())
    }

    fn skip_conditonally_equal_xy(&mut self, opcode: OpCode) -> Result<()> {
        if self.registers.get(opcode.x())?.get() == self.registers.get(opcode.y())?.get() {
            self.pc.increment();
        }
        Ok(())
    }

    fn skip_conditonally_not_equal_xy(&mut self, opcode: OpCode) -> Result<()> {
        if self.registers.get(opcode.x())?.get() != self.registers.get(opcode.y())?.get() {
            self.pc.increment();
        }
        Ok(())
    }

    fn set_x_to_delay(&mut self, opcode: OpCode) -> Result<()> {
        self.registers
            .get_mut(opcode.x())?
            .set(self.delay_timer.get());
        Ok(())
    }

    fn set_delay(&mut self, opcode: OpCode) -> Result<()> {
        self.delay_timer.set(self.registers.get(opcode.x())?.get());
        Ok(())
    }

    fn set_sound(&mut self, opcode: OpCode) -> Result<()> {
        self.sound_timer.set(self.registers.get(opcode.x())?.get());
        Ok(())
    }

    fn add_x_to_index(&mut self, opcode: OpCode) -> Result<()> {
        self.index
            .set(self.index.get() + self.registers.get(opcode.x())?.get() as u16);
        Ok(())
    }

    fn set_index_to_font(&mut self, opcode: OpCode) -> Result<()> {
        let char = self.registers.get(opcode.x())?.get() & 0x0F;
        let char_addr = (char * 5) + 5;
        self.index.set(char_addr as u16);
        Ok(())
    }

    fn bcd_x_in_index(&mut self, opcode: OpCode) -> Result<()> {
        let vx = self.registers.get(opcode.x())?.get();
        let i = self.index.get();
        self.memory.write(i, vx / 100)?;
        self.memory.write(i + 1, (vx % 100) / 10)?;
        self.memory.write(i + 2, vx % 10)?;
        Ok(())
    }

    fn set_x_in_index_spread(&mut self, opcode: OpCode) -> Result<()> {
        let i = self.index.get();
        for j in 0..=opcode.x() {
            self.memory
                .write(i + j as u16, self.registers.get(j)?.get())?;
        }
        Ok(())
    }

    fn read_x_from_index_spread(&mut self, opcode: OpCode) -> Result<()> {
        let i = self.index.get();
        for j in 0..=opcode.x() {
            self.registers
                .get_mut(j)?
                .set(self.memory.read(i + j as u16)?);
        }
        Ok(())
    }

    fn wait_for_key(&mut self, opcode: OpCode, keypad: &Keypad) -> Result<()> {
        let pressed = keypad.pressed();
        if !pressed.is_empty() {
            self.registers.get_mut(opcode.x())?.set(pressed[0]);
        } else {
            self.pc.decrement();
        }
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
    fn test_update_display_buffer() {}

    #[test]
    fn test_shift() {}
}
