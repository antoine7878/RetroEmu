#![allow(unused)]

use super::constants::{IE, IF};
use super::io_reg::IORegister;

#[derive(Default)]
pub struct Interrupt {
    pub enable: IORegister<IE, 0xFF>,
    pub flags: IORegister<IF, 0x1F>,
}
