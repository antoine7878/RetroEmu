use std::ops::{AddAssign, BitOr};

#[derive(Default)]
pub struct IORegister<const USED_MASK: u8> {
    raw: u8,
}

impl<const USED_MASK: u8> IORegister<USED_MASK> {
    pub fn set(&mut self, bit: u8) { self.raw |= 1 << bit }
    pub fn clear(&mut self, bit: u8) { self.raw &= !(1 << bit) }
    pub fn is_set(&self, bit: u8) -> bool { self.raw & (1 << bit) > 0 }
    pub fn is_clear(&self, bit: u8) -> bool { self.raw & (1 << bit) == 0 }
}

impl<const USED_MASK: u8> AddAssign<u8> for IORegister<USED_MASK> {
    fn add_assign(&mut self, rhs: u8) { self.raw = self.raw.wrapping_add(rhs); }
}

impl<const USED_MASK: u8> BitOr<u8> for &IORegister<USED_MASK> {
    type Output = u8;
    fn bitor(self, rhs: u8) -> Self::Output { self.read() | rhs }
}

// impl<const USED_MASK: u8> BitAnd<u8> for &IORegister<USED_MASK> {
//     type Output = u8;
//     fn bitand(self, rhs: u8) -> Self::Output { self.read() & rhs }
// }

impl<const USED_MASK: u8> PartialEq for IORegister<USED_MASK> {
    fn eq(&self, other: &Self) -> bool { self.read() == other.read() }
}

impl<const USED_MASK: u8> IOReg for IORegister<USED_MASK> {
    fn read(&self) -> u8 { self.raw & USED_MASK }
    fn write(&mut self, value: u8) { self.raw = value & USED_MASK }
}
pub trait IOReg {
    fn read(&self) -> u8;
    fn write(&mut self, value: u8);
}

// #[derive(Default)]
// pub struct IO {
//     joypad: Joypad,
//     link: Link,
//     timer: Timer,
//     interrupt: Interrupt,
//     audio: Audio,
//     bank: IORegister<0xFF>,
// }
//
// impl IO {
//     pub fn read(&self, addr: u16) -> u8 {
//         match addr {
//             IO::P1_JOYP => self.joypad.read(),
//             IO::SB => self.link.buffer.read(),
//             IO::SC => self.link.ctrl.read(),
//             IO::DIV => self.timer.divider.read(),
//             IO::TIMA => self.timer.counter.read(),
//             IO::TMA => self.timer.modulo.read(),
//             IO::TAC => self.timer.ctrl.read(),
//             IO::IF => self.interrupt.flags.read(),
//             IO::NR10 => self.audio.nr10.read(),
//             IO::NR11 => self.audio.nr11.read(),
//             IO::NR12 => self.audio.nr12.read(),
//             IO::NR13 => self.audio.nr13.read(),
//             IO::NR14 => self.audio.nr14.read(),
//             IO::NR21 => self.audio.nr21.read(),
//             IO::NR22 => self.audio.nr22.read(),
//             IO::NR23 => self.audio.nr23.read(),
//             IO::NR24 => self.audio.nr24.read(),
//             IO::NR30 => self.audio.nr30.read(),
//             IO::NR31 => self.audio.nr31.read(),
//             IO::NR32 => self.audio.nr32.read(),
//             IO::NR33 => self.audio.nr33.read(),
//             IO::NR34 => self.audio.nr34.read(),
//             IO::NR41 => self.audio.nr41.read(),
//             IO::NR42 => self.audio.nr42.read(),
//             IO::NR43 => self.audio.nr43.read(),
//             IO::NR44 => self.audio.nr44.read(),
//             IO::NR50 => self.audio.nr50.read(),
//             IO::NR51 => self.audio.nr51.read(),
//             IO::NR52 => self.audio.nr52.read(),
//             IO::WR_START..=IO::WR_END => self.audio.w_ram[(addr - IO::WR_START) as usize],
//
//             // IO::LCDC => self.display.lcdc.read(),
//             // IO::STAT => self.display.stat.read(),
//             // IO::SCY => self.display.scy.read(),
//             // IO::SCX => self.display.scx.read(),
//             // IO::LY => self.display.ly.read(),
//             // IO::LYC => self.display.lyc.read(),
//             // IO::DMA => self.display.dma.read(),
//             // IO::BGP => self.display.bgp.read(),
//             // IO::OBP0 => self.display.opb0.read(),
//             // IO::OBP1 => self.display.opb1.read(),
//             // IO::WY => self.display.wy.read(),
//             // IO::WX => self.display.wx.read(),
//
//             //
//             IO::BANK => self.bank.read(),
//             IO::PCM12 => self.audio.pcm12.read(),
//             IO::PCM34 => self.audio.pcm34.read(),
//             IO::IE => self.interrupt.enable.read(),
//             _ => todo!("CGB"),
//         }
//     }
//
//     pub fn write(&mut self, addr: u16, value: u8) {
//         match addr {
//             IO::P1_JOYP => self.joypad.write(value),
//             IO::SB => self.link.buffer.write(value),
//             IO::SC => self.link.ctrl.write(value),
//             IO::DIV => self.timer.divider.write(value),
//             IO::TIMA => self.timer.counter.write(value),
//             IO::TMA => self.timer.modulo.write(value),
//             IO::TAC => self.timer.ctrl.write(value),
//             IO::IF => self.interrupt.flags.write(value),
//             IO::NR10 => self.audio.nr10.write(value),
//             IO::NR11 => self.audio.nr11.write(value),
//             IO::NR12 => self.audio.nr12.write(value),
//             IO::NR13 => self.audio.nr13.write(value),
//             IO::NR14 => self.audio.nr14.write(value),
//             IO::NR21 => self.audio.nr21.write(value),
//             IO::NR22 => self.audio.nr22.write(value),
//             IO::NR23 => self.audio.nr23.write(value),
//             IO::NR24 => self.audio.nr24.write(value),
//             IO::NR30 => self.audio.nr30.write(value),
//             IO::NR31 => self.audio.nr31.write(value),
//             IO::NR32 => self.audio.nr32.write(value),
//             IO::NR33 => self.audio.nr33.write(value),
//             IO::NR34 => self.audio.nr34.write(value),
//             IO::NR41 => self.audio.nr41.write(value),
//             IO::NR42 => self.audio.nr42.write(value),
//             IO::NR43 => self.audio.nr43.write(value),
//             IO::NR44 => self.audio.nr44.write(value),
//             IO::NR50 => self.audio.nr50.write(value),
//             IO::NR51 => self.audio.nr51.write(value),
//             IO::NR52 => self.audio.nr52.write(value),
//             IO::WR_START..=IO::WR_END => self.audio.w_ram[(addr - IO::WR_START) as usize] = value,
//
//             // IO::LCDC => self.display.lcdc.write(value),
//             // IO::STAT => self.display.stat.write(value),
//             // IO::SCY => self.display.scy.write(value),
//             // IO::SCX => self.display.scx.write(value),
//             // IO::LY => self.display.ly.write(value),
//             // IO::LYC => self.display.lyc.write(value),
//             // IO::DMA => self.display.dma.write(value),
//             // IO::BGP => self.display.bgp.write(value),
//             // IO::OBP0 => self.display.opb0.write(value),
//             // IO::OBP1 => self.display.opb1.write(value),
//             // IO::WY => self.display.wy.write(value),
//             // IO::WX => self.display.wx.write(value),
//
//             //
//             IO::BANK => self.bank.write(value),
//             IO::PCM12 => self.audio.pcm12.write(value),
//             IO::PCM34 => self.audio.pcm34.write(value),
//             IO::IE => self.interrupt.enable.write(value),
//             _ => todo!("CGB"),
//         }
//     }
//
//     // Joypad
//     const P1_JOYP: u16 = 0xFF00;
//     // Link cable
//     const SB: u16 = 0xFF01;
//     const SC: u16 = 0xFF02;
//     // Timer
//     const DIV: u16 = 0xFF04;
//     const TIMA: u16 = 0xFF05;
//     const TMA: u16 = 0xFF06;
//     const TAC: u16 = 0xFF07;
//     // Interupt
//     const IF: u16 = 0xFF0F;
//     // Audio
//     const NR10: u16 = 0xFF10;
//     const NR11: u16 = 0xFF11;
//     const NR12: u16 = 0xFF12;
//     const NR13: u16 = 0xFF13;
//     const NR14: u16 = 0xFF14;
//     const NR21: u16 = 0xFF16;
//     const NR22: u16 = 0xFF17;
//     const NR23: u16 = 0xFF18;
//     const NR24: u16 = 0xFF19;
//     const NR30: u16 = 0xFF1A;
//     const NR31: u16 = 0xFF1B;
//     const NR32: u16 = 0xFF1C;
//     const NR33: u16 = 0xFF1D;
//     const NR34: u16 = 0xFF1E;
//     const NR41: u16 = 0xFF20;
//     const NR42: u16 = 0xFF21;
//     const NR43: u16 = 0xFF22;
//     const NR44: u16 = 0xFF23;
//     const NR50: u16 = 0xFF24;
//     const NR51: u16 = 0xFF25;
//     const NR52: u16 = 0xFF26;
//     const WR_START: u16 = 0xFF30;
//     const WR_END: u16 = 0xFF3F;
//     // Display
//     const LCDC: u16 = 0xFF40;
//     const STAT: u16 = 0xFF41;
//     const SCY: u16 = 0xFF42;
//     const SCX: u16 = 0xFF43;
//     const LY: u16 = 0xFF44;
//     const LYC: u16 = 0xFF45;
//     const DMA: u16 = 0xFF46;
//     const BGP: u16 = 0xFF47;
//     const OBP0: u16 = 0xFF48;
//     const OBP1: u16 = 0xFF49;
//     const WY: u16 = 0xFF4A;
//     const WX: u16 = 0xFF4B;
//     // GBC
//     const KEY0: u16 = 0xFF4C;
//     const KEY1: u16 = 0xFF4D;
//     const VBK: u16 = 0xFF4F;
//     const BANK: u16 = 0xFF50;
//     const HDMA1: u16 = 0xFF51;
//     const HDMA2: u16 = 0xFF52;
//     const HDMA3: u16 = 0xFF53;
//     const HDMA4: u16 = 0xFF54;
//     const HDMA5: u16 = 0xFF55;
//     const RP: u16 = 0xFF56;
//     const BCPS: u16 = 0xFF68;
//     const BCPD: u16 = 0xFF69;
//     const OCPS: u16 = 0xFF6A;
//     const OCPD: u16 = 0xFF6B;
//     const OPRI: u16 = 0xFF6C;
//     const SVBK: u16 = 0xFF70;
//     const PCM12: u16 = 0xFF76;
//     const PCM34: u16 = 0xFF77;
//     const IE: u16 = 0xFFFF;
// }
