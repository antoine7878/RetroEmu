use super::config::GCMode;
use super::constants::{SB, SC};
use super::io::{IOReg, IORegister};

#[derive(Default)]
pub struct Link {
    buffer: IORegister<SB, 0xFF>,
    ctrl: IORegister<SC, 0x83>,
}

impl Link {
    pub fn init_reg(&mut self, mode: GCMode) {
        self.buffer.init(mode);
        self.ctrl.init(mode);
    }

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
