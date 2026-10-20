use std::collections::HashSet;

use super::constants::{CARTIDGE_TYPE, RAM_SIZE_FLAG, ROM_SIZE_FLAG};

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum MBC {
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

pub struct Cartridge {
    rom: Vec<u8>,
    cram: Vec<u8>,
    mbcs: HashSet<MBC>,
    rom_banks: u16,
    ram_banks: u16,
}

impl Cartridge {
    pub fn new(rom: Vec<u8>) -> Self {
        let mbcs = Self::mbcs(&rom);
        let rom_banks = Self::rom_banks(&rom);
        let ram_banks = Self::ram_banks(&rom);
        let s = (ram_banks as usize) * (1 << 12);
        let cram: Vec<u8> = vec![0; s];
        Self { rom, mbcs, rom_banks, ram_banks, cram }
    }

    pub fn read_rom(&self, addr: u16) -> u8 { self.rom[addr as usize] }
    pub fn read_ram(&self, addr: u16) -> u8 { self.cram[addr as usize] }
    pub fn write_ram(&mut self, addr: u16, value: u8) { self.rom[addr as usize] = value }

    fn mbcs(rom: &[u8]) -> HashSet<MBC> {
        let s: &[MBC] = match rom[CARTIDGE_TYPE] {
            0x00 => &[],
            0x01 => &[MBC::Mbc1],
            0x02 => &[MBC::Mbc1, MBC::Ram],
            0x03 => &[MBC::Mbc1, MBC::Ram, MBC::Battery],
            0x05 => &[MBC::Mbc2],
            0x06 => &[MBC::Mbc2, MBC::Battery],
            0x08 => &[MBC::Ram],
            0x09 => &[MBC::Ram, MBC::Battery],
            0x0B => &[MBC::Mmm01],
            0x0C => &[MBC::Mmm01, MBC::Ram],
            0x0D => &[MBC::Mmm01, MBC::Ram, MBC::Battery],
            0x0F => &[MBC::Mbc3, MBC::Timer, MBC::Battery],
            0x10 => &[MBC::Mbc3, MBC::Timer, MBC::Ram, MBC::Battery],
            0x11 => &[MBC::Mbc3],
            0x12 => &[MBC::Mbc3, MBC::Ram],
            0x13 => &[MBC::Mbc3, MBC::Ram, MBC::Battery],
            0x19 => &[MBC::Mbc5],
            0x1A => &[MBC::Mbc5, MBC::Ram],
            0x1B => &[MBC::Mbc5, MBC::Ram, MBC::Battery],
            0x1C => &[MBC::Mbc5, MBC::Rumble],
            0x1D => &[MBC::Mbc5, MBC::Rumble, MBC::Ram],
            0x1E => &[MBC::Mbc5, MBC::Rumble, MBC::Ram, MBC::Battery],
            0x20 => &[MBC::Mbc6],
            0x22 => &[MBC::Mbc7, MBC::Sensor, MBC::Rumble, MBC::Ram, MBC::Battery],
            0xFC => &[MBC::Camera],
            0xFD => &[MBC::Tama5],
            0xFE => &[MBC::HuC3],
            0xFF => &[MBC::HuC1, MBC::Ram, MBC::Battery],
            _ => panic!("invalide cartridge type"),
        };
        HashSet::from_iter(s.iter().cloned())
    }

    fn rom_banks(rom: &[u8]) -> u16 {
        match rom[ROM_SIZE_FLAG] {
            s @ (0x00 | 0x01 | 0x02 | 0x03 | 0x04 | 0x05 | 0x06 | 0x07 | 0x08) => 1 << (s + 1),
            0x52 => 72,
            0x53 => 80,
            0x54 => 96,
            _ => panic!("Invalid rom size"),
        }
    }

    fn ram_banks(rom: &[u8]) -> u16 {
        match rom[RAM_SIZE_FLAG] {
            0x00 => 0,
            0x02 => 1,
            0x03 => 4,
            0x04 => 16,
            0x05 => 8,
            _ => panic!("Invalud ram size"),
        }
    }
}
