const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub struct Display {
    screen: [bool; WIDTH * HEIGHT],
}

impl Display {
    pub fn new() -> Display {
        Display {
            screen: [false; WIDTH * HEIGHT],
        }
    }

    pub fn clear(&mut self) {
        self.screen = [false; WIDTH * HEIGHT];
    }

    pub fn toggle_bytes(&mut self, byte: u8, x: usize, y: usize) -> bool {
        let mut erased = false;
        let i = Display::get_index(x, y);
        let mut byte = byte;
        for j in 0..8 {
            let bit = (byte & 0b1000_0000) >> 7;
            let prev = self.screen[i + j];
            self.screen[i + j] = bit != 0;

            if prev == !self.screen[i + j] {
                erased = true;
            }

            byte <<= 1;
        }
        erased
    }

    fn get_index(x: usize, y: usize) -> usize {
        x + y*WIDTH
    }

    pub fn debug_draw(&self) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let i = Display::get_index(x, y);
                if self.screen[i] {
                    print!("*");
                } else {
                    print!(" ");
                }
            }
            print!("\n");
        }
    }
}
