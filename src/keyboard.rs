pub struct Keyboard {

}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {}
    }

    pub fn is_pressed(&self, key: u8) -> bool {
        true
    }
}
