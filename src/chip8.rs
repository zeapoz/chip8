use std::time::Duration;

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
        loop {
            if self.keyboard.check_quit() {
                break;
            }
            self.display.draw();
            self.keyboard.key_handler();
            // self.display.debug_draw();
            self.cpu.cycle(&mut self.memory, &mut self.display, &mut self.keyboard);
            // Sleep for a duration equivalent to a refresh rate of 300hz
            std::thread::sleep(Duration::new(0, 1_000_000_000 / 300));
        }
    }
}
