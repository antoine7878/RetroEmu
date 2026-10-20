use std::time::Duration;

// ----- Memory Size --------------------

pub const WRAM_SIZE: usize = 1 << 12;
pub const HRAM_SIZE: usize = 0xFFFE - 0xFF80;
pub const VRAM_SIZE: usize = 1 << 13;
pub const OAM_SIZE: usize = 40 * 4;

// ----- Memory Map --------------------

pub const ROM_START: u16 = 0x0000;
pub const ROM_END: u16 = 0x8000;
pub const VRAM_START: u16 = 0x8000;
pub const VRAM_END: u16 = 0xA000;
pub const CRAM_START: u16 = 0xA000;
pub const CRAM_END: u16 = 0xC000;
pub const WRAM_START: u16 = 0xC000;
pub const WRAM_END: u16 = 0xE000;
pub const ECHO_START: u16 = 0xE000;
pub const ECHO_END: u16 = 0xFE00;
pub const OAM_START: u16 = 0xFE00;
pub const OAM_END: u16 = 0xFEA0;
pub const NOT_START: u16 = 0xFEA0;
pub const NOT_END: u16 = 0xFF00;
pub const PPU_START: u16 = 0xFF40;
pub const PPU_END: u16 = 0xFF50;
pub const HRAM_START: u16 = 0xFF80;
pub const HRAM_END: u16 = 0xFFFF;

// ----- Registers --------------------

pub const ZERO_FLAG: u8 = 7;
pub const SUB_FLAG: u8 = 6;
pub const HALF_CARRY_FLAG: u8 = 5;
pub const CARRY_FLAG: u8 = 4;

// ----- Console --------------------

// pub const CPU_CLOCK: u32 = 1 << 22;
// pub const CYCLE_PER_SCAN_LINE: u32 = 456;
// pub const CYCLE_PRE_FRAME: u32 = 154 * CYCLE_PER_SCAN_LINE;
pub const SCALE: u32 = 12;
pub const FRAME_RATE: u32 = 60;
pub const FRAME_TIME: Duration = Duration::from_nanos(1_000_000_000 / FRAME_RATE as u64);

pub const PALETTE_ORIGNAL: [[u8; 3]; 4] =
    [[0x2A, 0x41, 0x39], [0x7B, 0x82, 0x0F], [0x5A, 0x79, 0x42], [0x39, 0x59, 0x4A]];
// pub const PALETTE_POCKET: [[u8; 3]; 4] =
//     [[0x18, 0x18, 0x18], [0x4A, 0x51, 0x38], [0x8C, 0x92, 0x6B], [0xC5, 0xCA, 0xA4]];
// pub const PALETTE_LIGHT: [[u8; 3]; 4] =
//     [[0x00, 0x50, 0x3A], [0x00, 0x69, 0x4A], [0x00, 0x9A, 0x70], [0x00, 0xB5, 0x82]];

// ----- PPU --------------------

pub const WIDTH: usize = 160;
pub const HEIGHT: usize = 144;

pub const LCDC: u16 = 0xFF40;
pub const STAT: u16 = 0xFF41;
pub const SCY: u16 = 0xFF42;
pub const SCX: u16 = 0xFF43;
pub const LY: u16 = 0xFF44;
pub const LYC: u16 = 0xFF45;
pub const DMA: u16 = 0xFF46;
pub const BGP: u16 = 0xFF47;
pub const OBP0: u16 = 0xFF48;
pub const OBP1: u16 = 0xFF49;
pub const WY: u16 = 0xFF4A;
pub const WX: u16 = 0xFF4B;
pub const VBK: u16 = 0xFF4F;

pub const LCDC_OFF: u8 = 7;
// pub const LDCL_OSIZE: u8 = 2;

// ----- WRAM --------------------

pub const SVBK: u16 = 0xFF70;

// ----- CARTRIDGE --------------------

pub const CGB_FLAG: u16 = 0x0143;
pub const CARTIDGE_TYPE: usize = 0x0147;
pub const ROM_SIZE_FLAG: usize = 0x0148;
pub const RAM_SIZE_FLAG: usize = 0x149;

// pub const HEADER_START: u16 = 0x0100;
// pub const HEADER_END: u16 = 0x014F;

// ----- CPU --------------------

pub const PROGRAM_START: u16 = 0x0100;
