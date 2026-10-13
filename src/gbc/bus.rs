use super::cart::Cartridge;
use super::io::IO;
use super::memory::Memory;
use super::ppu::Ppu;

#[derive(Default)]
pub struct Bus {
    memory: Memory,
    cart: Cartridge,
    ppu: Ppu,
    io: IO,
}

impl Bus {
    pub fn new(rom: &[u8]) -> Self { Self { cart: Cartridge::new(rom), ..Default::default() } }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..0x8000 => self.cart.read(addr),
            0x8000..0xA000 => todo!("Vram"),
            0xA000..0xC000 => todo!("Cram"),
            0xC000..0xE000 => todo!("Wram"),
            0xE000..0xFE00 => self.read(addr - 0xE000),
            0xFE00..0xFEA0 => todo!("OAM"),
            0xFEA0..0xFF00 => 0xFF,
            0xFE00..0xFF80 | 0xFFFF => self.io.read(addr),
            0xFF80..0xFFFF => todo!("Hram"),
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..0x8000 => (),
            0x8000..0xA000 => todo!("Vram"),
            0xA000..0xC000 => todo!("Cram"),
            0xC000..0xE000 => todo!("Wram"),
            0xE000..0xFE00 => self.write(addr - 0xE000, value),
            0xFE00..0xFEA0 => todo!("OAM"),
            0xFEA0..0xFF00 => (),
            0xFE00..0xFF80 | 0xFFFF => self.io.write(addr, value),
            0xFF80..0xFFFF => todo!("Hram"),
        }
    }

    pub fn read_u16(&self, addr: u16) -> u16 { u16::from_be_bytes([self.read(addr), self.read(addr.wrapping_add(1))]) }
}
