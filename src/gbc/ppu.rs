pub const VRAM_SIZE: usize = 1 << 12;

pub struct Ppu {
    vram: [u8; VRAM_SIZE],
    oam: [u8; 40 * 4],
}

impl Default for Ppu {
    fn default() -> Self { Self { vram: [0; VRAM_SIZE] } }
}

impl Ppu {
    pub fn read(&self, addr: u16) -> u8 { self.vram[addr as usize % VRAM_SIZE] }
}
