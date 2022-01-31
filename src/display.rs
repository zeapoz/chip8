use sdl2::Sdl;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const SCALE: u32 = 10;

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

        let window = video_subsystem.window("Chip8 Emulator", (WIDTH as u32)*SCALE, (HEIGHT as u32)*SCALE)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();
        canvas
    }

    pub fn draw(&mut self) {
        // Clear screen
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        // Draw toggled pixels
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let i = Display::get_index(x, y);
                if self.screen[i] {
                    self.canvas.set_draw_color(Color::RGB(255, 255, 255));
                    // Draw rect at position multiplied by scale
                    self.canvas.fill_rect(Rect::new(
                        (x as i32)*SCALE as i32,
                        (y as i32)*SCALE as i32,
                        SCALE, SCALE
                    )).unwrap();
                } else {
                }
            }
        }
        self.canvas.present();
    }

    pub fn clear(&mut self) {
        self.screen = [false; WIDTH * HEIGHT];
    }

    pub fn toggle_bytes(&mut self, byte: u8, x: usize, y: usize) -> bool {
        let mut erased = false;
        let mut byte = byte;

        for j in 0..8 {
            let coord_x = (x + j) % WIDTH;
            let coord_y = y % HEIGHT;
            let i = Display::get_index(coord_x, coord_y);

            let bit = (byte & 0b1000_0000) >> 7;
            let prev = self.screen[i];
            self.screen[i] = bit != 0;

            if prev && !self.screen[i] {
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
