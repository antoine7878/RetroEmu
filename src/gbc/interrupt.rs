#![allow(unused)]

use super::io_reg::IORegister;
use super::memory_map::{IE, IF};

#[derive(Default)]
pub struct Interrupt {
    pub enable: IORegister<IE, 0xFF>,
    pub flags: IORegister<IF, 0x1F>,
}
