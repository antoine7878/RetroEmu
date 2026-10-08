#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub instructions_per_frame: u32,
    pub rng_seed: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            instructions_per_frame: 69905,
            rng_seed: 0x2545_F491,
        }
    }
}
