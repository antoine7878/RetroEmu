#![allow(unused)]

use crate::gbc::io::IORegister;

#[derive(Default)]
pub struct Interrupt {
    pub enable: IORegister<0x1F>,
    pub flags: IORegister<0x1F>,
}
