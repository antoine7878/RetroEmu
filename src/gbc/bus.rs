use super::display::Display;
use super::keypad::Keypad;
use super::memory::Memory;
use super::timer::Timer;

pub struct Bus {
    memory: Memory,
    pub display: Display,
    pub keypad: Keypad,
    pub delay_timer: Timer,
    pub sound_timer: Timer,
}

impl Bus {
    pub fn new(rom: &[u8]) -> Self {
        Self {
            memory: Memory::new(rom),
            display: Display::default(),
            keypad: Keypad::default(),
            delay_timer: Timer::default(),
            sound_timer: Timer::default(),
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..0x8000 => todo!("Cartridge"),
            0x8000..0xA000 => todo!("Vram"),
            0xA000..0xC000 => todo!("Cram"),
            0xC000..0xE000 => todo!("Wram"),
            0xE000..0xFE00 => self.read(addr - 0xE000),
            0xFE00..0xFEA0 => todo!("OAM"),
            0xFEA0..0xFF00 => todo!("unusable"),
            0xFE00..0xFF80 => todo!("io"),
            0xFF80..0xFFFF => todo!("Hram"),
            0xFFFF => todo!("Interupt"),
        }
    }

    pub fn read_u16(&self, addr: u16) -> u16 { u16::from_be_bytes([self.read(addr), self.read(addr.wrapping_add(1))]) }

    pub fn write(&mut self, addr: u16, value: u8) { self.memory.write(addr, value); }

    pub fn tick_timers(&mut self, by: u32) {
        self.delay_timer.tick(by);
        self.sound_timer.tick(by);
    }
}
