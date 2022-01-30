use crate::memory::Memory;

pub struct Cpu {
    vx: [u8; 16],
    i: u16,
    pc: usize,
    stack: Vec<u16>,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            vx: [0; 16],
            i: 0,
            pc: 0x200,
            stack: Vec::new(),
        }
    }

    pub fn execute_instruction(&self, memory: &Memory) {
        let hi = memory.read_byte(self.pc);
        let lo = memory.read_byte(self.pc + 1);
        
        println!("0x{:X?}{:X?}", hi, lo);
    }
}
