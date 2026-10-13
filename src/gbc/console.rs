use super::bus::Bus;
use super::config::Config;
use super::cpu::Cpu;

pub const FRAME_RATE: u32 = 60;

pub struct GameBoy {
    cpu: Cpu,
    bus: Bus,
    config: Config,
}
impl GameBoy {
    pub fn new(rom: &[u8], config: Config) -> Self { Self { cpu: Cpu::new(), bus: Bus::new(rom), config } }

    pub fn run_frame(&mut self) {
        let mut cycle_count: u32 = self.config.instructions_per_frame;
        while cycle_count > 0 {
            let c = self.cpu.step(&mut self.bus);
            // self.bus.tick_timers(c);
            // cycle_count -= c;
        }
    }

    pub fn set_key(&mut self, key: u8, pressed: bool) { self.bus.keypad.set(key, pressed); }

    pub fn pixels(&self) -> &[u8] { self.bus.display.pixels() }

    pub fn take_redraw(&mut self) -> bool { self.bus.display.take_dirty() }
}
