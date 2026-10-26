use std::time::Duration;

//
pub const PROMPT: &str = "\x1b[1;32m(gbdb)\x1b[0m ";

// ----- Memory Size --------------------

pub const WRAM_SIZE: usize = (WRAM_END - WRAM_START) as usize;
pub const HRAM_SIZE: usize = (HRAM_END - HRAM_START) as usize;
pub const VRAM_SIZE: usize = (VRAM_END - VRAM_START) as usize;
pub const OAM_SIZE: usize = (OAM_END - OAM_START) as usize;
pub const CRAM_SIZE: usize = 1 << 13;

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
pub const PPU_END: u16 = 0xFF4C;

pub const KEY0: u16 = 0xFF4C;
pub const KEY1: u16 = 0xFF4D;

pub const VRAM_BS: u16 = 0xFF4F;
pub const DMA_START: u16 = 0xFF51;
pub const DMA_END: u16 = 0xFF56;
pub const PAL_START: u16 = 0xFF68;
pub const PAL_END: u16 = 0xFF6D;

pub const HRAM_START: u16 = 0xFF80;
pub const HRAM_END: u16 = 0xFFFF;
pub const TIMER_START: u16 = 0xFF04;
pub const TIMER_END: u16 = 0xFF08;
pub const AUDIO_START: u16 = 0xFF10;
pub const AUDIO_END: u16 = 0xFF27;
pub const WAVE_START: u16 = 0xFF30;
pub const WAVE_END: u16 = 0xFF40;

pub const IF: u16 = 0xFF0F;
pub const IE: u16 = 0xFFFF;
pub const JOYPAD: u16 = 0xFF00;

pub const BOOT_DISABLE: u16 = 0xFF50;

pub const U1: u16 = 0xFF03;
pub const U2S: u16 = 0xFF08;
pub const U2E: u16 = 0xFF0F;
pub const U3S: u16 = 0xFF27;
pub const U3E: u16 = 0xFF30;
pub const U5: u16 = 0xFF4E;
pub const U6S: u16 = 0xFF56;
pub const U6E: u16 = 0xFF68;
pub const U7S: u16 = 0xFF6D;
pub const U7E: u16 = 0xFF70;
pub const U8S: u16 = 0xFF71;
pub const U8E: u16 = 0xFF80;

// ----- Registers --------------------

pub const ZERO_FLAG: u8 = 7;
pub const SUB_FLAG: u8 = 6;
pub const HALF_CARRY_FLAG: u8 = 5;
pub const CARRY_FLAG: u8 = 4;

// ----- Console --------------------

pub const SCALE: u32 = 2;
pub const FRAME_RATE: u32 = 60;

pub const PALETTE_ORIGNAL: [[u8; 3]; 4] =
    [[0x2A, 0x41, 0x39], [0x7B, 0x82, 0x0F], [0x5A, 0x79, 0x42], [0x39, 0x59, 0x4A]];

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

// ----- TIMER --------------------

pub const DIV: u16 = 0xFF04;
pub const TIMA: u16 = 0xFF05;
pub const TMA: u16 = 0xFF06;
pub const TAC: u16 = 0xFF07;

// ----- link --------------------

pub const SB: u16 = 0xFF01;
pub const SC: u16 = 0xFF02;
