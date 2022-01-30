use crate::memory::Memory;

pub struct Chip8 {
    memory: Memory,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            memory: Memory::new(),
        }
    }

    pub fn load_rom(&mut self, rom: &Vec<u8>) {
        let offset = 0x200 as usize;
        let end = rom.len() as usize;
        for i in 0..end {
            self.memory.write_byte(offset + i, rom[i]);
        }
    }
}
