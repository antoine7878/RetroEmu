use std::time::Instant;

use super::bus::Bus;
use super::config::Config;
use super::cpu::Cpu;

pub const FRAME_RATE: u32 = 60;

pub struct GameBoy {
    cpu: Cpu,
    bus: Bus,
    config: Config,
    cycle_count: u64,
}

impl GameBoy {
    pub fn new(rom: &[u8], config: Config) -> Self {
        Self { cpu: Cpu::new(), bus: Bus::new(rom), cycle_count: 0, config }
    }

    pub fn run_frame(&mut self) {
        let start_time = Instant::now();

        let mut cycle_count: u32 = self.config.instructions_per_frame;
        while cycle_count > 0 {
            self.cpu.step(&mut self.bus);
        }
    }

    fn read(&mut self, addr: u16) -> u8 {
        self.cycle_count += 1;
        self.bus.read(addr)
    }

    fn write(&mut self, addr: u16, value: u8) {
        self.cycle_count += 1;
        self.bus.write(addr, value);
    }

    // pub fn pixels(&self) -> &[u8] { self.bus.display.pixels() }
    // pub fn take_redraw(&mut self) -> bool { self.bus.display.take_dirty() }
}
