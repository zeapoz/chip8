use crate::memory::Memory;
use crate::display::Display;
use crate::keyboard::Keyboard;

pub struct Cpu {
    v: [u8; 16],
    i: u16,
    pc: usize,
    stack: Vec<u16>,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            v: [0; 16],
            i: 0,
            pc: 0x200,
            stack: Vec::new(),
        }
    }

    pub fn execute_instruction(&mut self, memory: &Memory, display: &mut Display, keyboard: &Keyboard) {
        let hi = memory.read_byte(self.pc);
        let lo = memory.read_byte(self.pc + 1);
        let instruction: u16 = (hi as u16) << 8 | (lo as u16);
        
        println!("\nRead instruction: 0x{:X} from hi: 0x{:X} and lo: 0x{:X}", instruction, hi, lo);

        let nnn = instruction & 0xFFF;
        let n = instruction & 0xF;
        let x = hi & 0xF;
        let y = (lo & 0xF0) >> 4;
        let kk = lo;
        println!("nnn: 0x{:X}, n: 0x{:X}, x: 0x{:X}, y: 0x{:X}", nnn, n, x, y);

        // TODO more match instructions
        match (instruction & 0xF000) >> 12 {
            0x0 => {
                match lo {
                    0xE0 => {
                        // Clear the display.
                        display.clear();
                        self.pc += 2;
                    },
                    0xEE => {
                        // Return from a subroutine.
                        self.pc = self.stack.pop().unwrap() as usize;
                    },
                    _ => panic!("instruction not found in 0x0"),
                }
            },
            0x1 => {
                // Jump to location nnn.
                self.pc = nnn as usize;
            },
            0x2 => {
                // Call subroutine at nnn.
                self.stack.push(self.pc as u16);
                self.pc = nnn as usize;
            },
            0x3 => {
                // 3xkk - SE Vx, byte
                if self.v[x as usize] == kk {
                    self.pc += 2;
                } else {
                    self.pc += 4;
                }
            },
            0x6 => {
                // Set Vx = kk.
                self.write_vx(x, kk);
                self.pc += 2; 
            },
            0x7 => {
                // Set Vx = Vx + kk.
                let value = self.read_vx(x);
                self.write_vx(x, value + kk);
                self.pc += 2;
            },
            0x8 => {
                match n {
                    0x0 => {
                        // Set Vx = Vy.
                        let value = self.read_vx(y);
                        self.write_vx(x, value);
                        self.pc += 2;
                    },
                    _ => panic!("instruction not found in 0x8"),
                }
            },
            0xA => {
                // Set I = nnn.
                self.i = nnn;
                self.pc += 2;
            },
            0xD => {
                // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
                let vx = self.read_vx(x) as usize;
                let vy = self.read_vx(y) as usize;
                for y in 0..n {
                    let s = memory.read_byte((self.i + y) as usize);
                    if display.toggle_bytes(s, vx, vy + y as usize) {
                        self.write_vx(0xF, 1);
                    } else {
                        self.write_vx(0xF, 0);
                    }
                } 
                self.pc += 2;
            },
            0xE => {
                match lo {
                    0xA1 => {
                        // Skip next instruction if key with the value of Vx is not pressed.
                        let key = self.read_vx(x);
                        if keyboard.is_pressed(key) {
                            self.pc += 2;
                        } else {
                            self.pc += 4;
                        }
                    },
                    _ => panic!("instruction not found in 0xF"),
                }
            },
            0xF => {
                match lo {
                    0x1E => {
                        // Set I = I + Vx.
                        self.i += x as u16;
                        self.pc += 2;
                    },
                    0x65 => {
                        // Read registers V0 through Vx from memory starting at location I.
                        for i in 0..x + 1 {
                            let value = memory.read_byte((self.i + i as u16) as usize);
                            self.write_vx(i, value);
                        }
                        self.pc += 2;
                    },
                    _ => panic!("instruction not found in 0xF"),
                }
            },
            _ => panic!("instruction not found for 0x{:X}", instruction),
        }
    }

    fn read_vx(&self, x: u8) -> u8 {
        self.v[x as usize]
    }

    fn write_vx(&mut self, x: u8, value: u8) {
        self.v[x as usize] = value;
    }
}
