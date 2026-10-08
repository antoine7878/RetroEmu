#[derive(Default)]
pub struct Keypad {
    pressed: u16,
}

impl Keypad {
    pub fn set(&mut self, key: u8, pressed: bool) {
        let mask = 1 << (key & 0xF);
        if pressed {
            self.pressed |= mask;
        } else {
            self.pressed &= !mask;
        }
    }

    pub fn is_pressed(&self, key: u8) -> bool { self.pressed & (1 << (key & 0xF)) != 0 }

    pub fn first_pressed(&self) -> Option<u8> { (self.pressed != 0).then(|| self.pressed.trailing_zeros() as u8) }
}
