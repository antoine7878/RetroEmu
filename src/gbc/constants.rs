use std::time::Duration;

//
pub const PROMPT: &str = "\x1b[1;32m(gbdb)\x1b[0m ";

// ----- Registers --------------------

pub const ZERO_FLAG: u8 = 7;
pub const SUB_FLAG: u8 = 6;
pub const HALF_CARRY_FLAG: u8 = 5;
pub const CARRY_FLAG: u8 = 4;

// ----- Console --------------------

pub const SCALE: u32 = 2;
pub const FRAME_RATE: u32 = 60;
pub const CYCLES_PER_FRAME: u64 = 17556;

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

pub const HDMA1: u16 = 0xFF51;
pub const HDMA2: u16 = 0xFF52;
pub const HDMA3: u16 = 0xFF53;
pub const HDMA4: u16 = 0xFF54;
pub const HDMA5: u16 = 0xFF55;
pub const BCPS: u16 = 0xFF68;
pub const BCPD: u16 = 0xFF69;
pub const OCPS: u16 = 0xFF6A;
pub const OCPD: u16 = 0xFF6B;
pub const OPRI: u16 = 0xFF6C;

// ----- WRAM --------------------

pub const SVBK: u16 = 0xFF70;

// ----- AUDIO --------------------

pub const NR10: u16 = 0xFF10;
pub const NR11: u16 = 0xFF11;
pub const NR12: u16 = 0xFF12;
pub const NR13: u16 = 0xFF13;
pub const NR14: u16 = 0xFF14;
pub const NR21: u16 = 0xFF16;
pub const NR22: u16 = 0xFF17;
pub const NR23: u16 = 0xFF18;
pub const NR24: u16 = 0xFF19;
pub const NR30: u16 = 0xFF1A;
pub const NR31: u16 = 0xFF1B;
pub const NR32: u16 = 0xFF1C;
pub const NR33: u16 = 0xFF1D;
pub const NR34: u16 = 0xFF1E;
pub const NR41: u16 = 0xFF20;
pub const NR42: u16 = 0xFF21;
pub const NR43: u16 = 0xFF22;
pub const NR44: u16 = 0xFF23;
pub const NR50: u16 = 0xFF24;
pub const NR51: u16 = 0xFF25;
pub const NR52: u16 = 0xFF26;
pub const PCM12: u16 = 0xFF76;
pub const PCM34: u16 = 0xFF77;

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

// ----- infrared --------------------

pub const RP: u16 = 0xFF56;
