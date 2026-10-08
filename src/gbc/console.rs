use audio::Beeper;
use bus::Bus;
use cpu::Cpu;
use memory::MAX_ROM_SIZE;

pub const FRAME_RATE: u32 = 60;
const SAMPLES_PER_FRAME: usize = (SAMPLE_RATE / FRAME_RATE) as usize;

pub struct Chip8 {
    cpu: Cpu,
    bus: Bus,
    beeper: Beeper,
    config: Config,
}

impl Chip8 {
    pub fn new(rom: &[u8], config: Config) -> Result<Self, Error> {
        if rom.len() > MAX_ROM_SIZE {
            return Err(Error::RomTooLarge { size: rom.len() });
        }
        Ok(Self {
            cpu: Cpu::new(config.rng_seed),
            bus: Bus::new(rom),
            beeper: Beeper::default(),
            config,
        })
    }

    pub fn run_frame(&mut self) -> Result<(), Error> {
        let mut cycle_count: u32 = self.config.instructions_per_frame;
        while cycle_count > 0 {
            let c = self.cpu.step(&mut self.bus)?;
            self.bus.tick_timers(c);
            cycle_count -= c;
        }
        self.beeper
            .generate(SAMPLES_PER_FRAME, self.bus.sound_timer.value() as u8);
        Ok(())
    }

    pub fn set_key(&mut self, key: u8, pressed: bool) {
        self.bus.keypad.set(key, pressed);
    }

    pub fn pixels(&self) -> &[u8] {
        self.bus.display.pixels()
    }

    pub fn take_redraw(&mut self) -> bool {
        self.bus.display.take_dirty()
    }

    pub fn drain_audio(&mut self, out: &mut Vec<f32>) {
        self.beeper.drain(out);
    }
}
