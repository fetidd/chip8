mod chip8;
mod display;
mod font;
mod memory;
mod program_counter;
mod register;
mod stack;
mod timer;

fn main() {
    let mut chip = chip::Chip8::default();
    chip.run();
}
