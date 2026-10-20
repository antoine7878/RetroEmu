use super::cart::Cartridge;
use super::constants::{
    CRAM_END, CRAM_START, ECHO_END, ECHO_START, HRAM_END, HRAM_SIZE, HRAM_START, NOT_END, NOT_START, OAM_END,
    OAM_START, PPU_END, PPU_START, ROM_END, ROM_START, VRAM_END, VRAM_START, WRAM_END, WRAM_START,
};
use super::io::{IOReg, IORegister};
use super::ppu::Ppu;
use super::wram::WRam;

const A: u16 = 0xFFFE - 0xFF80;

pub struct Bus {
    cart: Cartridge,
    ppu: Ppu,
    wram: WRam,
    hram: [u8; HRAM_SIZE],
    cycle_count: u64,
    ie: IORegister<0x1F>,
    ifl: IORegister<0x1F>,
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
        }
    }

    pub fn read_log(&self, addr: u16) -> u8 {
        match addr {
            ROM_START..ROM_END => self.cart.read_rom(addr),
            VRAM_START..VRAM_END => self.ppu.read(addr),
            CRAM_START..CRAM_END => self.cart.read_ram(addr),
            WRAM_START..WRAM_END => self.wram.read(addr),
            ECHO_START..ECHO_END => self.read_log(addr - 0x2000),
            OAM_START..OAM_END => self.ppu.read(addr),
            NOT_START..NOT_END => 0xFF,
            PPU_START..PPU_END => self.ppu.read(addr),
            HRAM_START..HRAM_END => self.hram[(addr - HRAM_START) as usize],
            0xFF0F => self.ifl.read(),
            0xFFFF => self.ie.read(),
            _ => todo!("io regs: {:04X}", addr),
        }
    }

    pub fn read(&mut self, addr: u16) -> u8 {
        self.tick();
        self.read_log(addr)
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        self.tick();
        match addr {
            ROM_START..ROM_END => (),
            VRAM_START..VRAM_END => self.ppu.write(addr, value),
            CRAM_START..CRAM_END => self.cart.write_ram(addr, value),
            WRAM_START..WRAM_END => self.wram.write(addr, value),
            ECHO_START..ECHO_END => self.write(addr - 0x2000, value),
            OAM_START..OAM_END => self.ppu.write(addr, value),
            NOT_START..NOT_END => (),
            PPU_START..PPU_END => self.ppu.write(addr, value),
            HRAM_START..HRAM_END => self.hram[(addr - HRAM_START) as usize] = value,
            0xFF0F => self.ifl.write(value),
            0xFFFF => self.ie.write(value),
            _ => todo!("io regs: {:04X}", addr),
        }
    }

    pub fn tick(&mut self) {
        crate::CYCLES.store(self.cycle_count, std::sync::atomic::Ordering::Relaxed);
        // let dots = if self.double_speed { 2 } else { 4 };
        let dots = 4;
        self.ppu.ticks(dots, &mut self.ifl);
        self.cycle_count += 1;
    }

    pub const fn get_cycle_count(&self) -> u64 { self.cycle_count }
    pub const fn get_pixels(&self) -> &[u8] { self.ppu.get_pixels() }
    pub fn take_redraw(&mut self) -> bool { self.ppu.take_redraw() }
}

// pub fn read_u16(&mut self, addr: u16) -> u16 {
//     u16::from_be_bytes([self.read(addr), self.read(addr.wrapping_add(1))])
// }
// pub fn ticks(&mut self, ticks: u8) {
//     for _ in 0..ticks {
//         self.tick();
//     }
// }
