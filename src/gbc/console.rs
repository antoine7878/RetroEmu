use std::num::ParseIntError;

use super::bus::Bus;
use super::config::Config;
use super::cpu::Cpu;
use crate::debugger::{Debugger, StopReason};

pub struct GameBoy {
    cpu: Cpu,
    bus: Bus,
    config: Config,
    cycle_count: u64,
}

impl GameBoy {
    pub fn new(rom: Vec<u8>, config: Config) -> Self {
        Self { cpu: Cpu::new(), bus: Bus::new(rom), cycle_count: 0, config }
    }

    pub fn run_frame(&mut self, mut dbg: Option<&mut Debugger>) -> Option<StopReason> {
        let start = self.bus.get_cycle_count();
        while self.bus.get_cycle_count() - start < self.config.instructions_per_frame {
            if let Some(r) = dbg.as_deref_mut().and_then(|d| d.check(self.cpu.pc())) {
                return Some(r);
            }
            self.cycle_count += self.cpu.step(&mut self.bus);
        }
        None
    }

    pub const fn pixels(&self) -> &[u8] { self.bus.get_pixels() }

    pub fn take_redraw(&mut self) -> bool { self.bus.take_redraw() }

    pub fn print_registers(&self) {
        self.cpu.print();
        println!(
            " PCMEM:{:02X},{:02X},{:02X},{:02X}",
            self.bus.read_log(self.cpu.pc()),
            self.bus.read_log(self.cpu.pc() + 1),
            self.bus.read_log(self.cpu.pc() + 2),
            self.bus.read_log(self.cpu.pc() + 3)
        );
    }
}
