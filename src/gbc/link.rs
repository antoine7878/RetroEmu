#![allow(unused)]

use super::constants::{SB, SC};
use super::io::{IOReg, IORegister};

#[derive(Default)]
pub struct Link {
    buffer: IORegister<0xFF>,
    ctrl: IORegister<0x03>,
}

impl Link {
    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            SB => self.buffer.read(),
            SC => self.ctrl.read(),
            _ => unreachable!(),
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            SB => self.buffer.write(value),
            SC => self.ctrl.write(value),
            _ => unreachable!(),
        }
    }
}
