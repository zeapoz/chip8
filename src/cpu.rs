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
        let instruction: u16 = (hi as u16) << 8 | (lo as u16);
        
        println!("\nRead instruction: 0x{:X} from hi: 0x{:X} and lo: 0x{:X}", instruction, hi, lo);

        let nnn = instruction & 0xFFF;
        let n = instruction & 0xF;
        let x = hi & 0xF;
        let y = (lo & 0xF0) >> 4;
        println!("nnn: 0x{:X}, n: 0x{:X}, x: 0x{:X}, y: 0x{:X}", nnn, n, x, y);

        // TODO match instructions
        match (instruction & 0xF000) >> 12 {
            _ => panic!("Instruction not found for 0x{:X}", instruction),
        }
    }
}
