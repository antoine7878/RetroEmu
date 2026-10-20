use super::bus::Bus;
use super::config::Config;
use super::cpu::Cpu;

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

    pub fn run_frame(&mut self) {
        let start_cycle = self.bus.get_cycle_count();
        while self.bus.get_cycle_count() - start_cycle < self.config.instructions_per_frame {
            let c = self.cpu.step(&mut self.bus);
            self.cycle_count += c;
        }
    }

    pub const fn pixels(&self) -> &[u8] { self.bus.get_pixels() }

    pub fn take_redraw(&mut self) -> bool { self.bus.take_redraw() }
}
