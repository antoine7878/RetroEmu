use crate::gbc::io::IORegister;

pub const WIDTH: usize = 160;
pub const HEIGHT: usize = 144;

#[derive(Default)]
pub struct Display {
    // pixels: [u8; WIDTH * HEIGHT],
    // dirty: bool,
    pub lcdc: IORegister<0xFF>,
    pub stat: IORegister<0x7F>,
    pub scy: IORegister<0xFF>,
    pub scx: IORegister<0xFF>,
    pub ly: IORegister<0xFF>,
    pub lyc: IORegister<0xFF>,
    pub dma: IORegister<0xFF>,
    pub bgp: IORegister<0xFF>,
    pub opb0: IORegister<0xFF>,
    pub opb1: IORegister<0xFF>,
    pub wy: IORegister<0xFF>,
    pub wx: IORegister<0xFF>,
}

// impl Default for Display {
//     fn default() -> Self {
//         Self {
//             pixels: [0; WIDTH * HEIGHT],
//             dirty: true,
//         }
//     }
// }
//
// impl Display {
//     pub fn clear(&mut self) {
//         self.pixels.fill(0);
//         self.dirty = true;
//     }
//
//     pub fn toggle(&mut self, x: usize, y: usize, value: u8) {
//         let pixel = &mut self.pixels[y * WIDTH + x];
//         *pixel = value;
//         self.dirty = true;
//     }
//
//     pub fn pixels(&self) -> &[u8] { &self.pixels }
//
//     pub fn take_dirty(&mut self) -> bool { std::mem::take(&mut self.dirty) }
// }
