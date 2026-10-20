use std::num::ParseIntError;

use super::bus::Bus;
use super::config::Config;
use super::cpu::Cpu;
use super::instruction::Instruction;
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

    pub fn registers(&self) -> Vec<String> {
        let r = self.cpu.registers();
        let flag = |bit, name| if bit == 1 { name } else { '-' };
        let flags = [flag(r.get_z(), 'Z'), flag(r.get_n(), 'N'), flag(r.get_h(), 'H'), flag(r.get_c(), 'C')];
        vec![
            format!("AF {:04X}  {}", r.get_af(), String::from_iter(flags)),
            format!("BC {:04X}", r.get_bc()),
            format!("DE {:04X}", r.get_de()),
            format!("HL {:04X}", r.get_hl()),
            format!("SP {:04X}", r.get_sp()),
            format!("PC {:04X}", r.pc()),
        ]
    }

    pub fn pc(&self) -> u16 { self.cpu.pc() }

    pub fn disassembly(&self, mut addr: u16, count: usize) -> Vec<(u16, String)> {
        let mut lines = Vec::with_capacity(count);
        for _ in 0..count {
            let (text, len) = Instruction::disassemble(addr, |a| self.bus.read_log(a));
            lines.push((addr, text));
            addr = addr.wrapping_add(len);
        }
        lines
    }
}
