use super::cart::Cartridge;
use super::constants::{
    AUDIO_END, AUDIO_START, BOOT_DISABLE, CRAM_END, CRAM_START, DMA_END, DMA_START, ECHO_END, ECHO_START, HRAM_END,
    HRAM_SIZE, HRAM_START, IE, IF, JOYPAD, KEY0, KEY1, NOT_END, NOT_START, OAM_END, OAM_START, PAL_END, PAL_START,
    PPU_END, PPU_START, ROM_END, ROM_START, SB, SC, SVBK, TIMER_END, TIMER_START, U1, U2E, U2S, U3E, U3S, U5, U6E, U6S,
    U7E, U7S, U8E, U8S, VRAM_BS, VRAM_END, VRAM_START, WAVE_END, WAVE_START, WRAM_END, WRAM_START,
};
use super::io::{IOReg, IORegister};
use super::joypad::Joypad;
use super::link::Link;
use super::ppu::Ppu;
use super::timer::Timer;
use super::wram::WRam;

pub struct Bus {
    cart: Cartridge,
    ppu: Ppu,
    wram: WRam,
    hram: [u8; HRAM_SIZE],
    cycle_count: u64,
    ie: IORegister<0xFF>,
    ifl: IORegister<0x1F>,
    joypad: Joypad,
    timer: Timer,
    link: Link,
}

impl Bus {
    pub fn new(rom: Vec<u8>) -> Self {
        Self {
            cart: Cartridge::new(rom),
            ppu: Ppu::default(),
            wram: WRam::default(),
            hram: [0; HRAM_SIZE],
            cycle_count: 0,
            ie: IORegister::default(),
            ifl: IORegister::default(),
            joypad: Joypad::default(),
            timer: Timer::default(),
            link: Link::default(),
        }
    }

    pub fn read_internal(&self, addr: u16) -> u8 {
        match addr {
            ROM_START..ROM_END => self.cart.read_rom(addr),
            VRAM_START..VRAM_END => self.ppu.read(addr),
            CRAM_START..CRAM_END => self.cart.read_ram(addr),
            WRAM_START..WRAM_END | SVBK => self.wram.read(addr),
            ECHO_START..ECHO_END => self.read_internal(addr - 0x2000),
            OAM_START..OAM_END => self.ppu.read(addr),
            NOT_START..NOT_END => 0xFF,
            PPU_START..PPU_END | VRAM_BS => self.ppu.read(addr),
            DMA_START..DMA_END | PAL_START..PAL_END => self.ppu.read(addr),
            HRAM_START..HRAM_END => self.hram[(addr - HRAM_START) as usize],
            TIMER_START..TIMER_END => self.timer.read(addr),
            // TODO:
            AUDIO_START..AUDIO_END => 0xFF,
            KEY0 | KEY1 => 0xFF,
            WAVE_START..WAVE_END => 0xFF,
            BOOT_DISABLE => 0xFF,

            IF => self.ifl.read(),
            IE => self.ie.read(),
            JOYPAD => self.joypad.read(),
            SB | SC => self.link.read(addr),
            U1 | U2S..U2E | U3S..U3E | U5 | U6S..U6E | U7S..U7E | U8S..U8E => 0xFF,
        }
    }

    pub fn read(&mut self, addr: u16) -> u8 {
        self.tick();
        self.read_internal(addr)
    }

    pub fn write_internal(&mut self, addr: u16, value: u8) {
        match addr {
            ROM_START..ROM_END => (),
            VRAM_START..VRAM_END => self.ppu.write(addr, value),
            CRAM_START..CRAM_END => self.cart.write_ram(addr, value),
            WRAM_START..WRAM_END | SVBK => self.wram.write(addr, value),
            ECHO_START..ECHO_END => self.write_internal(addr - 0x2000, value),
            OAM_START..OAM_END => self.ppu.write(addr, value),
            NOT_START..NOT_END => (),
            PPU_START..PPU_END | VRAM_BS => self.ppu.write(addr, value),
            DMA_START..DMA_END | PAL_START..PAL_END => self.ppu.write(addr, value),
            HRAM_START..HRAM_END => self.hram[(addr - HRAM_START) as usize] = value,
            TIMER_START..TIMER_END => self.timer.write(addr, value),
            // TODO:
            AUDIO_START..AUDIO_END => (),
            KEY0 | KEY1 => (),
            WAVE_START..WAVE_END => (),
            BOOT_DISABLE => (),

            IF => self.ifl.write(value),
            IE => self.ie.write(value),
            JOYPAD => self.joypad.write(value),
            SB | SC => self.link.write(addr, value),
            U1 | U2S..U2E | U3S..U3E | U5 | U6S..U6E | U7S..U7E | U8S..U8E => (),
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        self.tick();
        self.write_internal(addr, value)
    }

    /// ticks the system by M-cycles
    pub fn ticks(&mut self, by: u8) {
        for _ in 0..by {
            self.tick();
        }
    }

    /// ticks the system by 1 M-cycles
    pub fn tick(&mut self) {
        // let dots = if self.double_speed { 2 } else { 4 };
        let dots = 4;
        self.ppu.ticks(dots, &mut self.ifl);
        self.timer.tick(&mut self.ifl);
        self.cycle_count += 1;
    }

    pub const fn get_cycle_count(&self) -> u64 { self.cycle_count }
    pub const fn get_pixels(&self) -> &[u8] { self.ppu.get_pixels() }
    pub fn take_redraw(&mut self) -> bool { self.ppu.take_redraw() }
}
