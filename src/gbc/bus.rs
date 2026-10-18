use super::cart::Cartridge;
use super::memory::Memory;
use super::ppu::Ppu;

#[derive(Default)]
pub struct Bus {
    memory: Memory,
    cart: Cartridge,
    ppu: Ppu,
}

const PPU_ST: u16 = 0x8000;
const PPU_E: u16 = 0xFF4B;

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
            PPU_ST..=PPU_E => self.ppu.read(addr),
            0xFE00..0xFF80 | 0xFFFF => self.read_io(addr),
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
            PPU_ST..=PPU_E => self.ppu.write(addr, value),
            0xFE00..0xFF80 | 0xFFFF => self.write_io(addr, value),
            0xFF80..0xFFFF => todo!("Hram"),
        }
    }

    pub fn read_u16(&mut self, addr: u16) -> u16 {
        u16::from_be_bytes([self.read(addr), self.read(addr.wrapping_add(1))])
    }

    pub fn read_io(&self, addr: u16) -> u8 {
        match addr {
            // Self::P1_JOYP => self.joypad.read(),
            // Self::SB => self.link.buffer.read(),
            // Self::SC => self.link.ctrl.read(),
            // Self::DIV => self.timer.divider.read(),
            // Self::TIMA => self.timer.counter.read(),
            // Self::TMA => self.timer.modulo.read(),
            // Self::TAC => self.timer.ctrl.read(),
            // Self::IF => self.interrupt.flags.read(),
            // Self::NR10 => self.audio.nr10.read(),
            // Self::NR11 => self.audio.nr11.read(),
            // Self::NR12 => self.audio.nr12.read(),
            // Self::NR13 => self.audio.nr13.read(),
            // Self::NR14 => self.audio.nr14.read(),
            // Self::NR21 => self.audio.nr21.read(),
            // Self::NR22 => self.audio.nr22.read(),
            // Self::NR23 => self.audio.nr23.read(),
            // Self::NR24 => self.audio.nr24.read(),
            // Self::NR30 => self.audio.nr30.read(),
            // Self::NR31 => self.audio.nr31.read(),
            // Self::NR32 => self.audio.nr32.read(),
            // Self::NR33 => self.audio.nr33.read(),
            // Self::NR34 => self.audio.nr34.read(),
            // Self::NR41 => self.audio.nr41.read(),
            // Self::NR42 => self.audio.nr42.read(),
            // Self::NR43 => self.audio.nr43.read(),
            // Self::NR44 => self.audio.nr44.read(),
            // Self::NR50 => self.audio.nr50.read(),
            // Self::NR51 => self.audio.nr51.read(),
            // Self::NR52 => self.audio.nr52.read(),
            // Self::WR_START..=IO::WR_END => self.audio.w_ram[(addr - IO::WR_START) as usize],

            // Self::BANK => self.bank.read(),
            // Self::PCM12 => self.audio.pcm12.read(),
            // Self::PCM34 => self.audio.pcm34.read(),
            // Self::IE => self.interrupt.enable.read(),
            _ => todo!("CGB"),
        }
    }

    pub fn write_io(&mut self, addr: u16, value: u8) {
        match addr {
            // Self::P1_JOYP => self.joypad.write(value),
            // Self::SB => self.link.buffer.write(value),
            // Self::SC => self.link.ctrl.write(value),
            // Self::DIV => self.timer.divider.write(value),
            // Self::TIMA => self.timer.counter.write(value),
            // Self::TMA => self.timer.modulo.write(value),
            // Self::TAC => self.timer.ctrl.write(value),
            // Self::IF => self.interrupt.flags.write(value),
            // Self::NR10 => self.audio.nr10.write(value),
            // Self::NR11 => self.audio.nr11.write(value),
            // Self::NR12 => self.audio.nr12.write(value),
            // Self::NR13 => self.audio.nr13.write(value),
            // Self::NR14 => self.audio.nr14.write(value),
            // Self::NR21 => self.audio.nr21.write(value),
            // Self::NR22 => self.audio.nr22.write(value),
            // Self::NR23 => self.audio.nr23.write(value),
            // Self::NR24 => self.audio.nr24.write(value),
            // Self::NR30 => self.audio.nr30.write(value),
            // Self::NR31 => self.audio.nr31.write(value),
            // Self::NR32 => self.audio.nr32.write(value),
            // Self::NR33 => self.audio.nr33.write(value),
            // Self::NR34 => self.audio.nr34.write(value),
            // Self::NR41 => self.audio.nr41.write(value),
            // Self::NR42 => self.audio.nr42.write(value),
            // Self::NR43 => self.audio.nr43.write(value),
            // Self::NR44 => self.audio.nr44.write(value),
            // Self::NR50 => self.audio.nr50.write(value),
            // Self::NR51 => self.audio.nr51.write(value),
            // Self::NR52 => self.audio.nr52.write(value),
            // Self::WR_START..=IO::WR_END => self.audio.w_ram[(addr - IO::WR_START) as usize] = value,

            // Self::BANK => self.bank.write(value),
            // Self::PCM12 => self.audio.pcm12.write(value),
            // Self::PCM34 => self.audio.pcm34.write(value),
            // Self::IE => self.interrupt.enable.write(value),
            _ => todo!("CGB"),
        }
    }

    // Joypad
    const P1_JOYP: u16 = 0xFF00;
    // Link cable
    const SB: u16 = 0xFF01;
    const SC: u16 = 0xFF02;
    // Timer
    const DIV: u16 = 0xFF04;
    const TIMA: u16 = 0xFF05;
    const TMA: u16 = 0xFF06;
    const TAC: u16 = 0xFF07;
    // Interupt
    const IF: u16 = 0xFF0F;
    // Audio
    const NR10: u16 = 0xFF10;
    const NR11: u16 = 0xFF11;
    const NR12: u16 = 0xFF12;
    const NR13: u16 = 0xFF13;
    const NR14: u16 = 0xFF14;
    const NR21: u16 = 0xFF16;
    const NR22: u16 = 0xFF17;
    const NR23: u16 = 0xFF18;
    const NR24: u16 = 0xFF19;
    const NR30: u16 = 0xFF1A;
    const NR31: u16 = 0xFF1B;
    const NR32: u16 = 0xFF1C;
    const NR33: u16 = 0xFF1D;
    const NR34: u16 = 0xFF1E;
    const NR41: u16 = 0xFF20;
    const NR42: u16 = 0xFF21;
    const NR43: u16 = 0xFF22;
    const NR44: u16 = 0xFF23;
    const NR50: u16 = 0xFF24;
    const NR51: u16 = 0xFF25;
    const NR52: u16 = 0xFF26;
    const WR_START: u16 = 0xFF30;
    const WR_END: u16 = 0xFF3F;
    // Display
    const LCDC: u16 = 0xFF40;
    const STAT: u16 = 0xFF41;
    const SCY: u16 = 0xFF42;
    const SCX: u16 = 0xFF43;
    const LY: u16 = 0xFF44;
    const LYC: u16 = 0xFF45;
    const DMA: u16 = 0xFF46;
    const BGP: u16 = 0xFF47;
    const OBP0: u16 = 0xFF48;
    const OBP1: u16 = 0xFF49;
    const WY: u16 = 0xFF4A;
    const WX: u16 = 0xFF4B;
    // GBC
    const KEY0: u16 = 0xFF4C;
    const KEY1: u16 = 0xFF4D;
    const VBK: u16 = 0xFF4F;
    const BANK: u16 = 0xFF50;
    const HDMA1: u16 = 0xFF51;
    const HDMA2: u16 = 0xFF52;
    const HDMA3: u16 = 0xFF53;
    const HDMA4: u16 = 0xFF54;
    const HDMA5: u16 = 0xFF55;
    const RP: u16 = 0xFF56;
    const BCPS: u16 = 0xFF68;
    const BCPD: u16 = 0xFF69;
    const OCPS: u16 = 0xFF6A;
    const OCPD: u16 = 0xFF6B;
    const OPRI: u16 = 0xFF6C;
    const SVBK: u16 = 0xFF70;
    const PCM12: u16 = 0xFF76;
    const PCM34: u16 = 0xFF77;
    const IE: u16 = 0xFFFF;
}
