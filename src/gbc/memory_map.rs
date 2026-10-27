use super::constants::{
    BCPD, BCPS, BGP, DIV, DMA, HDMA1, HDMA2, HDMA3, HDMA4, HDMA5, LCDC, LY, LYC, NR10, NR11, NR12, NR13, NR14, NR21,
    NR22, NR23, NR24, NR30, NR31, NR32, NR33, NR34, NR41, NR42, NR43, NR44, NR50, NR51, NR52, OBP0, OBP1, OCPD, OCPS,
    OPRI, PCM12, PCM34, RP, SB, SC, SCX, SCY, STAT, SVBK, TAC, TIMA, TMA, VBK, WX, WY,
};

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

pub fn memeory_location(addr: u16) -> &'static str {
    match addr {
        ROM_START..ROM_END => "ROM",
        VRAM_START..VRAM_END => "VRAM",
        CRAM_START..CRAM_END => "CRAM",
        WRAM_START..WRAM_END => "WRAM",
        ECHO_START..ECHO_END => "ECHO",
        OAM_START..OAM_END => "OAM",
        NOT_START..NOT_END => "UNUSABLE",
        JOYPAD => "JOYP",
        SB => "SB",
        SC => "SC",
        DIV => "DIV",
        TIMA => "TIMA",
        TMA => "TMA",
        TAC => "TAC",
        IF => "IF",
        NR10 => "NR10",
        NR11 => "NR11",
        NR12 => "NR12",
        NR13 => "NR13",
        NR14 => "NR14",
        NR21 => "NR21",
        NR22 => "NR22",
        NR23 => "NR23",
        NR24 => "NR24",
        NR30 => "NR30",
        NR31 => "NR31",
        NR32 => "NR32",
        NR33 => "NR33",
        NR34 => "NR34",
        NR41 => "NR41",
        NR42 => "NR42",
        NR43 => "NR43",
        NR44 => "NR44",
        NR50 => "NR50",
        NR51 => "NR51",
        NR52 => "NR52",
        WAVE_START..WAVE_END => "WAVE",
        LCDC => "LCDC",
        STAT => "STAT",
        SCY => "SCY",
        SCX => "SCX",
        LY => "LY",
        LYC => "LYC",
        DMA => "DMA",
        BGP => "BGP",
        OBP0 => "OBP0",
        OBP1 => "OBP1",
        WY => "WY",
        WX => "WX",
        KEY0 => "KEY0",
        KEY1 => "KEY1",
        VBK => "VBK",
        BOOT_DISABLE => "BOOT",
        HDMA1 => "HDMA1",
        HDMA2 => "HDMA2",
        HDMA3 => "HDMA3",
        HDMA4 => "HDMA4",
        HDMA5 => "HDMA5",
        RP => "RP",
        BCPS => "BCPS",
        BCPD => "BCPD",
        OCPS => "OCPS",
        OCPD => "OCPD",
        OPRI => "OPRI",
        SVBK => "SVBK",
        PCM12 => "PCM12",
        PCM34 => "PCM34",
        HRAM_START..HRAM_END => "HRAM",
        IE => "IE",
        _ => "IO",
    }
}
