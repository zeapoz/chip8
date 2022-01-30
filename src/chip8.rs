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
}
