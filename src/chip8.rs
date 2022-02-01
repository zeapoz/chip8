use std::time::{Duration, Instant};

use sdl2::Sdl;

use crate::cpu::Cpu;
use crate::memory::Memory;
use crate::display::Display;
use crate::keyboard::Keyboard;
use crate::audio::Device;

pub struct Chip8 {
    cpu: Cpu,
    memory: Memory,
    display: Display,
    keyboard: Keyboard,
    audio_device: Device,
}

impl Chip8 {
    pub fn new(sdl_context: &Sdl) -> Chip8 {
        Chip8 {
            cpu: Cpu::new(),
            memory: Memory::new(),
            display: Display::new(sdl_context),
            keyboard: Keyboard::new(sdl_context),
            audio_device: Device::new(sdl_context),
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
            // Poll for keypresses, break if returned false
            if !self.keyboard.key_handler() {
                break;
            }
            // Store time to sync variable cpu execution time
            let now = Instant::now();
            // Execute 8 cpu instructions per 60hz cycle to match a 480hz clock
            for _ in 0..8 {
                // Run cpu instructions
                self.cpu.execute_instruction(&mut self.memory, &mut self.display, &mut self.keyboard, &mut self.audio_device);
            }
            // Decrement timers
            self.cpu.decrement_timers(&self.audio_device);
            // self.display.debug_draw();
            self.display.draw();
            // Subtract elapsed time from 60hz and sleep for that duration
            let sleep_time = Duration::from_micros(16_667) - now.elapsed();
            std::thread::sleep(sleep_time);
        }
    }
}
