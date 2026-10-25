use std::collections::HashSet;

use super::constants::{CARTIDGE_TYPE, CRAM_SIZE, CRAM_START, RAM_SIZE_FLAG, ROM_SIZE_FLAG};

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum Mbc {
    Mbc1,
    Mbc2,
    Mbc3,
    Mbc5,
    Mbc6,
    Mbc7,
    Mmm01,
    Ram,
    Battery,
    Timer,
    Sensor,
    Rumble,
    Camera,
    Tama5,
    HuC1,
    HuC3,
}

// │ 0000–1FFF │ Turns cartridge RAM on (0x0A in the low nibble) or off │
// │ 2000–3FFF │ Picks the ROM bank mapped at 4000–7FFF                 │
// │ 4000–5FFF │ Picks the RAM bank, or the upper ROM bank bits         │
// │ 6000–7FFF │ Switches banking mode                                  │

pub struct Cartridge {
    rom: Vec<u8>,
    ram: Vec<u8>,
    mbcs: HashSet<Mbc>,
    rom_banks: u16,
    ram_banks: u16,
    // ram_on: bool,
    // rom_bank_select: u8,
    // ram_bank_select: u8,
    // banking_mode,
}

impl Cartridge {
    pub fn new(rom: Vec<u8>) -> Self {
        let mbcs = Self::mbcs(&rom);
        let rom_banks = Self::rom_banks(&rom);
        let ram_banks = Self::ram_banks(&rom);
        let s = (ram_banks as usize) * CRAM_SIZE;
        let ram: Vec<u8> = vec![0; s];
        Self { rom, mbcs, rom_banks, ram_banks, ram }
    }

    pub fn reset(&mut self) { self.ram.clear(); }

    pub fn extract_rom(&self) -> &Vec<u8> { &self.rom }
    pub fn read_rom(&self, addr: u16) -> u8 { self.rom[addr as usize] }

    pub fn read_ram(&self, addr: u16) -> u8 {
        if self.ram.is_empty() {
            return 0xFF;
        }
        self.ram[(addr - CRAM_START) as usize]
    }

    pub fn write_ram(&mut self, addr: u16, value: u8) {
        if self.ram.is_empty() {
            return;
        }
        self.ram[(addr - CRAM_START) as usize] = value
    }

    fn mbcs(rom: &[u8]) -> HashSet<Mbc> {
        let s: &[Mbc] = match rom[CARTIDGE_TYPE] {
            0x00 => &[],
            0x01 => &[Mbc::Mbc1],
            0x02 => &[Mbc::Mbc1, Mbc::Ram],
            0x03 => &[Mbc::Mbc1, Mbc::Ram, Mbc::Battery],
            0x05 => &[Mbc::Mbc2],
            0x06 => &[Mbc::Mbc2, Mbc::Battery],
            0x08 => &[Mbc::Ram],
            0x09 => &[Mbc::Ram, Mbc::Battery],
            0x0B => &[Mbc::Mmm01],
            0x0C => &[Mbc::Mmm01, Mbc::Ram],
            0x0D => &[Mbc::Mmm01, Mbc::Ram, Mbc::Battery],
            0x0F => &[Mbc::Mbc3, Mbc::Timer, Mbc::Battery],
            0x10 => &[Mbc::Mbc3, Mbc::Timer, Mbc::Ram, Mbc::Battery],
            0x11 => &[Mbc::Mbc3],
            0x12 => &[Mbc::Mbc3, Mbc::Ram],
            0x13 => &[Mbc::Mbc3, Mbc::Ram, Mbc::Battery],
            0x19 => &[Mbc::Mbc5],
            0x1A => &[Mbc::Mbc5, Mbc::Ram],
            0x1B => &[Mbc::Mbc5, Mbc::Ram, Mbc::Battery],
            0x1C => &[Mbc::Mbc5, Mbc::Rumble],
            0x1D => &[Mbc::Mbc5, Mbc::Rumble, Mbc::Ram],
            0x1E => &[Mbc::Mbc5, Mbc::Rumble, Mbc::Ram, Mbc::Battery],
            0x20 => &[Mbc::Mbc6],
            0x22 => &[Mbc::Mbc7, Mbc::Sensor, Mbc::Rumble, Mbc::Ram, Mbc::Battery],
            0xFC => &[Mbc::Camera],
            0xFD => &[Mbc::Tama5],
            0xFE => &[Mbc::HuC3],
            0xFF => &[Mbc::HuC1, Mbc::Ram, Mbc::Battery],
            _ => unimplemented!("invalide cartridge type"),
        };
        HashSet::from_iter(s.iter().cloned())
    }

    fn rom_banks(rom: &[u8]) -> u16 {
        match rom[ROM_SIZE_FLAG] {
            s @ (0x00..0x08) => 1 << (s + 1),
            0x52 => 72,
            0x53 => 80,
            0x54 => 96,
            _ => unimplemented!("Invalid rom size"),
        }
    }

    fn ram_banks(rom: &[u8]) -> u16 {
        match rom[RAM_SIZE_FLAG] {
            0x00 => 0,
            0x02 => 1,
            0x03 => 4,
            0x04 => 16,
            0x05 => 8,
            _ => unimplemented!("Invalud ram size"),
        }
    }
}
