#![allow(unused)]

use super::constants::{TIMER_COUNTER, TIMER_CTRL, TIMER_DIVIDER, TIMER_MODULO};
use super::io::{IOReg, IORegister};

#[derive(Default)]
pub struct Timer {
    pub divider: IORegister<0xFF>,
    pub counter: IORegister<0xFF>,
    pub modulo: IORegister<0xFF>,
    pub ctrl: IORegister<0x07>,
}

impl Timer {
    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            TIMER_DIVIDER => self.divider.read(),
            TIMER_COUNTER => self.counter.read(),
            TIMER_MODULO => self.modulo.read(),
            TIMER_CTRL => self.ctrl.read(),
            _ => unreachable!(),
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            TIMER_DIVIDER => self.divider.write(value),
            TIMER_COUNTER => self.counter.write(value),
            TIMER_MODULO => self.modulo.write(value),
            TIMER_CTRL => self.ctrl.write(value),
            _ => unreachable!(),
        }
    }

    pub fn tick(&mut self) { todo!() }
}
