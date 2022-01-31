use sdl2::Sdl;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct Keyboard {
    event_pump: EventPump,
    held: Option<u8>,
}

impl Keyboard {
    pub fn new(sdl_context: &sdl2::Sdl) -> Keyboard {
        Keyboard {
            event_pump: sdl_context.event_pump().unwrap(),
            held: None,
        }
    }

    pub fn wait_for_press(&mut self) -> Option<u8> {
        if let Some(held) = self.held {
            return Some(held);
        }
        None
    }

    pub fn is_pressed(&mut self, key: u8) -> bool {
        if let Some(held) = self.held {
            if held == key {
                return true;
            }
        }
        false
    }

    pub fn key_handler(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode: Some(key), .. } => {
                    if let Some(byte_key) = Keyboard::map_keycode(key) {
                        self.held = Some(byte_key);
                    }
                },
                _ => {}
            }
        }
    }

    pub fn check_quit(&mut self) -> bool {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    return true;
                },
                _ => {}
            }
        }
        false
    }

    fn map_keycode(key: Keycode) -> Option<u8> {
        let byte_key = match key {
            Keycode::Num1 => 0x1,
            Keycode::Num2 => 0x2,
            Keycode::Num3 => 0x3,
            Keycode::Num4 => 0xC,

            Keycode::Q => 0x4,
            Keycode::W => 0x5,
            Keycode::E => 0x6,
            Keycode::R => 0xD,
            
            Keycode::A => 0x7,
            Keycode::S => 0x8,
            Keycode::D => 0x9,
            Keycode::F => 0xE,
            
            Keycode::Z => 0xA,
            Keycode::X => 0x0,
            Keycode::C => 0xB,
            Keycode::V => 0xF,
            _ => return None,
        };
        Some(byte_key)
    }
}
