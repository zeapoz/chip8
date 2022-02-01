use std::fs::File;
use std::io::Read;
use std::env;

use sdl2;

use chip8::Chip8;

mod chip8;
mod cpu;
mod memory;
mod display;
mod keyboard;
mod audio;

fn main() -> std::io::Result<()> {
    // Collect arguments
    let rom_path: &str;

    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => rom_path = &args[1],
        _ => panic!("arguments should only be 1"),
    }
    // Setup sdl context
    let sdl_context = sdl2::init().unwrap();
    // Create new chip8
    let mut chip8 = Chip8::new(&sdl_context);
    
    let mut data: Vec<u8> = Vec::new();
    let mut file = File::open(rom_path)?;

    file.read_to_end(&mut data)?;
    // Load data into chip8 ram
    chip8.load_rom(&data);
    // Enter loop
    chip8.cycle();

    Ok(())
}
