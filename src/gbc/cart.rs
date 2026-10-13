#[derive(Default)]
pub struct Cartridge {
    rom: Vec<u8>,
}

impl Cartridge {
    pub fn new(rom: &[u8]) -> Self { Self { rom: Vec::from(rom) } }
    pub fn read(&self, addr: u16) -> u8 { self.rom[addr as usize] }
}
