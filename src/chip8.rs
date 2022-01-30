use std::time::Duration;

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
    pub fn new() -> Chip8 {
        Chip8 {
            cpu: Cpu::new(),
            memory: Memory::new(),
            display: Display::new(),
            keyboard: Keyboard::new(),
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
        self.display.debug_draw();
        self.cpu.execute_instruction(&self.memory, &mut self.display, &self.keyboard);
        std::thread::sleep(Duration::new(0, 1_000_000_000 / 60));
    }
}
