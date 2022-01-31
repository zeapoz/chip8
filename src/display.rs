use sdl2::Sdl;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const SCALE: usize = 10;

pub struct Display {
    canvas: Canvas<Window>,
    screen: [bool; WIDTH * HEIGHT],
}

impl Display {
    pub fn new(sdl_context: &Sdl) -> Display {
        Display {
            canvas: Display::create_window(sdl_context),
            screen: [false; WIDTH * HEIGHT],
        }
    }

    fn create_window(sdl_context: &Sdl) -> Canvas<Window> {
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("Chip8 Emulator", (WIDTH*SCALE) as u32, (HEIGHT*SCALE) as u32)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();
        canvas
    }

    pub fn draw(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        self.canvas.present();
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
