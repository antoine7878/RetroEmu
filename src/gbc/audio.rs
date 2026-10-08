pub const SAMPLE_RATE: u32 = 44_100;

const TONE_HZ: f32 = 440.0;
const VOLUME: f32 = 0.25;

#[derive(Default)]
pub struct Beeper {
    phase: f32,
    samples: Vec<f32>,
}

impl Beeper {
    pub fn generate(&mut self, count: usize, timer_value: u8) {
        let on = timer_value > 0;
        let phase_inc = TONE_HZ / SAMPLE_RATE as f32;
        for _ in 0..count {
            let sample = match (on, self.phase < 0.5) {
                (false, _) => 0.0,
                (true, true) => VOLUME,
                (true, false) => -VOLUME,
            };
            self.samples.push(sample);
            self.phase = (self.phase + phase_inc) % 1.0;
        }
    }

    pub fn drain(&mut self, out: &mut Vec<f32>) { out.append(&mut self.samples); }
}
