pub const MAX_ROM_SIZE: usize = 0x200000;
pub const MEMORY_SIZE: usize = 1 << 16;
pub const PROGRAM_START: u16 = 0;

pub struct Memory {
    raw: [u8; MEMORY_SIZE],
}

impl Default for Memory {
    fn default() -> Self { Self { raw: [0; MEMORY_SIZE] } }
}

impl Memory {
    pub fn read(&self, addr: u16) -> u8 { self.raw[addr as usize % MEMORY_SIZE] }
    pub fn write(&mut self, addr: u16, value: u8) { self.raw[addr as usize % MEMORY_SIZE] = value; }
}
