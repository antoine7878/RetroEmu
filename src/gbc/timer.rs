#![allow(unused)]

use super::constants::{DIV, TAC, TIMA, TMA};
use super::io::{IOReg, IORegister};

#[derive(Default)]
pub struct Timer {
    div: IORegister<0xFF>,
    tima: IORegister<0xFF>,
    tma: IORegister<0xFF>,
    tac: IORegister<0x07>,
    tima_clock: u8,
    div_clock: u8,
}

impl Timer {
    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            DIV => self.div.read(),
            TIMA => self.tima.read(),
            TMA => self.tma.read(),
            TAC => self.tac.read(),
            _ => unreachable!(),
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            DIV => {
                self.div.write(0);
                self.div_clock = 0;
            }
            TIMA => self.tima.write(value),
            TMA => self.tma.write(value),
            TAC => {
                self.tac.write(value);
                self.reset_tima_clock();
            }
            _ => unreachable!(),
        }
    }

    const CLOCKS: [u8; 4] = [255, 3, 15, 63];
    fn reset_tima_clock(&mut self) { self.tima_clock = Self::CLOCKS[self.tac.read() as usize & 0b11]; }

    pub fn tick(&mut self, ifl: &mut IORegister<0x1F>) {
        self.tick_div();
        if self.tac.read() & 0b100 == 0 {
            return;
        }
        if self.tima_clock == 0 {
            self.tick_tima(ifl);
            self.reset_tima_clock();
        } else {
            self.tima_clock -= 1;
        }
    }

    fn tick_div(&mut self) {
        if self.div_clock == 63 {
            self.div += 1;
            self.div_clock = 0;
        } else {
            self.div_clock += 1;
        }
    }

    fn tick_tima(&mut self, ifl: &mut IORegister<0x1F>) {
        if self.tima.read() == 0xFF {
            self.tima.write(self.tma.read());
            ifl.set(2);
        } else {
            self.tima += 1;
        }
    }
}
