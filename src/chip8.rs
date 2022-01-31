use std::time::{Duration, Instant};

use sdl2::Sdl;

use crate::cpu::Cpu;
use crate::memory::Memory;
use crate::display::Display;
use crate::keyboard::Keyboard;

pub struct Chip8 {
    cpu: Cpu,
    memory: Memory,
    display: Display,
    keyboard: Keyboard,
}

impl Chip8 {
    pub fn new(sdl_context: &Sdl) -> Chip8 {
        Chip8 {
            cpu: Cpu::new(),
            memory: Memory::new(),
            display: Display::new(sdl_context),
            keyboard: Keyboard::new(sdl_context),
        }
    }

    pub fn load_rom(&mut self, rom: &Vec<u8>) {
        let offset = 0x200 as usize;
        let end = rom.len() as usize;
        for i in 0..end {
            self.memory.write_byte(offset + i, rom[i]);
        }
    }

    pub fn cycle(&mut self) {
        let cpu_clock = Duration::from_millis(2);
        let mut last_cpu_time = Instant::now();
        
        loop {
            // Poll for keypresses, break if returned false
            if !self.keyboard.key_handler() {
                break;
            }
            // self.display.debug_draw();
            self.display.draw();

            // Keep cpu instructions seperate from input and draw clock
            if Instant::now() - last_cpu_time > cpu_clock {
                self.cpu.cycle(&mut self.memory, &mut self.display, &mut self.keyboard);
                last_cpu_time = Instant::now();
            }
        }
    }
}
