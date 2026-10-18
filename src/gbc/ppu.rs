use super::io::{IOReg, IORegister};

pub const VRAM_SIZE: usize = 1 << 12;
pub const OAM_SIZE: usize = 40 * 4;
pub const WIDTH: usize = 160;
pub const HEIGHT: usize = 144;

pub struct Ppu {
    vram: [u8; VRAM_SIZE],
    oam: [u8; OAM_SIZE],

    lcdc: IORegister<0xFF>,
    stat: IORegister<0x7F>,
    scy: IORegister<0xFF>,
    scx: IORegister<0xFF>,
    ly: IORegister<0xFF>,
    lyc: IORegister<0xFF>,
    dma: IORegister<0xFF>,
    bgp: IORegister<0xFF>,
    opb0: IORegister<0xFF>,
    opb1: IORegister<0xFF>,
    wy: IORegister<0xFF>,
    wx: IORegister<0xFF>,
}

impl Default for Ppu {
    fn default() -> Self {
        Self {
            vram: [0; VRAM_SIZE],
            oam: [0; OAM_SIZE],
            lcdc: IORegister::default(),
            stat: IORegister::default(),
            scy: IORegister::default(),
            scx: IORegister::default(),
            ly: IORegister::default(),
            lyc: IORegister::default(),
            dma: IORegister::default(),
            bgp: IORegister::default(),
            opb0: IORegister::default(),
            opb1: IORegister::default(),
            wy: IORegister::default(),
            wx: IORegister::default(),
        }
    }
}

impl Ppu {
    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            VRAMS..VRAME => self.vram[addr as usize],
            LCDC => self.lcdc.read(),
            STAT => self.stat.read(),
            SCY => self.scy.read(),
            SCX => self.scx.read(),
            LY => self.ly.read(),
            LYC => self.lyc.read(),
            DMA => self.dma.read(),
            BGP => self.bgp.read(),
            OBP0 => self.opb0.read(),
            OBP1 => self.opb1.read(),
            WY => self.wy.read(),
            WX => self.wx.read(),
            _ => panic!("Address not in PPU"),
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x8000..0x9FFF => self.vram[addr as usize] = value,
            LCDC => self.lcdc.write(value),
            STAT => self.stat.write(value),
            SCY => self.scy.write(value),
            SCX => self.scx.write(value),
            LY => self.ly.write(value),
            LYC => self.lyc.write(value),
            DMA => self.dma.write(value),
            BGP => self.bgp.write(value),
            OBP0 => self.opb0.write(value),
            OBP1 => self.opb1.write(value),
            WY => self.wy.write(value),
            WX => self.wx.write(value),
            _ => panic!("Address not in PPU"),
        }
    }
}

const VRAMS: u16 = 0x8000;
const VRAME: u16 = 0x9FFF;
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
