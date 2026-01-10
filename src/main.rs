#[macro_use]
mod register;

mod chip8;
mod display;
mod font;
mod memory;
mod program_counter;
mod stack;
mod timer;

fn main() {
    let mut chip = chip8::Chip8::default();
    chip.run();
}
