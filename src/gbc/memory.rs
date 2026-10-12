pub const MAX_ROM_SIZE: usize = 0x200000;
pub const MEMORY_SIZE: usize = 0xFFFF;
pub const PROGRAM_START: u16 = 0;

pub struct Memory {
    rom: [u8; MEMORY_SIZE],
}

impl Memory {
    pub const fn new(_rom: &[u8]) -> Self {
        let rom = [0; MEMORY_SIZE];
        Self { rom }
    }

    pub fn read(&self, addr: u16) -> u8 { self.rom[addr as usize % MEMORY_SIZE] }
    pub fn write(&mut self, addr: u16, value: u8) { self.rom[addr as usize % MEMORY_SIZE] = value; }
}
