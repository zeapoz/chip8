use std::collections::HashMap;

use sdl2::Sdl;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct Keyboard {
    event_pump: EventPump,
}

impl Keyboard {
    pub fn new(sdl_context: &sdl2::Sdl) -> Keyboard {
        Keyboard {
            event_pump: sdl_context.event_pump().unwrap(),
        }
    }

    pub fn is_pressed(&mut self, key: u8) -> bool {
        let key = self.map_keycode(key);
        for event in self.event_pump.poll_iter() {
            match event {
                Event::KeyDown {keycode: Some(key), ..} => {
                    println!("{:?}", key);
                    return true;
                },
                _ => {}
            }
        }
        return false;
    }

    pub fn check_quit(&mut self) -> bool {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    return true
                },
                _ => return false
            }
        }
        false
    }

    fn map_keycode(&self, key: u8) -> Keycode {
        let keys = HashMap::from([
            (0x1, Keycode::Num1),
            (0x2, Keycode::Num2),
            (0x3, Keycode::Num3),
            (0xC, Keycode::Num4),

            (0x4, Keycode::Q),
            (0x5, Keycode::W),
            (0x6, Keycode::E),
            (0xD, Keycode::R),
            
            (0x7, Keycode::A),
            (0x8, Keycode::S),
            (0x9, Keycode::D),
            (0xE, Keycode::F),
            
            (0xA, Keycode::Z),
            (0x0, Keycode::X),
            (0xB, Keycode::C),
            (0xF, Keycode::V),
        ]);
        keys[&key]
    }
}
