use std::fs::File;
use std::io::Read;

use sdl2;

use chip8::Chip8;

mod chip8;
mod cpu;
mod memory;
mod display;
mod keyboard;

fn main() -> std::io::Result<()> {
    // Setup sdl context
    let sdl_context = sdl2::init().unwrap();

    // Create new chip8
    let mut chip8 = Chip8::new(&sdl_context);
    // Load and store rom
    let rom_path = "test/space_invaders.ch8";

    let mut file = File::open(rom_path)?;
    let mut data: Vec<u8> = Vec::new();

    file.read_to_end(&mut data)?;
    // Load data into chip8 ram
    chip8.load_rom(&data);

    loop {
        chip8.cycle();
    }

    Ok(())
}
