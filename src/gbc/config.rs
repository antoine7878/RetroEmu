#![allow(unused)]

use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};

use super::constants::CGB_FLAG;

pub const FRAME_RATE: u32 = 60;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GCMode {
    Dmg,
    Enhanced,
    Gbc,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub instructions_per_frame: u64,
    pub rng_seed: u32,
    pub mode: GCMode,
}

impl Config {
    pub fn new(rom: &[u8]) -> Result<Self, Box<dyn Error>> {
        let rng_seed = SystemTime::now().duration_since(UNIX_EPOCH)?.subsec_nanos();
        let mode = Self::mode(rom);
        Ok(Self { rng_seed, mode, instructions_per_frame: 69905 })
    }

    fn mode(rom: &[u8]) -> GCMode {
        match rom[CGB_FLAG as usize] {
            0x80 => GCMode::Gbc,
            0xC0 => GCMode::Enhanced,
            _ => GCMode::Dmg,
        }
    }
}
