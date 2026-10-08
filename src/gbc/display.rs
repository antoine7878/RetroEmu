pub const WIDTH: usize = 160;
pub const HEIGHT: usize = 144;

pub struct Display {
    pixels: [u8; WIDTH * HEIGHT],
    dirty: bool,
}

impl Default for Display {
    fn default() -> Self {
        Self {
            pixels: [0; WIDTH * HEIGHT],
            dirty: true,
        }
    }
}

impl Display {
    pub fn clear(&mut self) {
        self.pixels.fill(0);
        self.dirty = true;
    }

    pub fn toggle(&mut self, x: usize, y: usize, value: u8) {
        let pixel = &mut self.pixels[y * WIDTH + x];
        *pixel = value;
        self.dirty = true;
    }

    pub fn pixels(&self) -> &[u8] { &self.pixels }

    pub fn take_dirty(&mut self) -> bool { std::mem::take(&mut self.dirty) }
}
