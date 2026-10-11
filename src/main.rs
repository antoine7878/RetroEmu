mod frontend;
mod gbc;

use std::time::{SystemTime, UNIX_EPOCH};
use std::{env, fs, process};

use crate::gbc::Config;

fn main() {
    let Some(path) = env::args().nth(1) else {
        eprintln!("usage: chip8 <rom.ch8>");
        process::exit(1);
    };

    if let Err(e) = run(&path) {
        eprintln!("error: {e}");
        process::exit(1);
    }
}

fn run(_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
    // let rom = fs::read(path).map_err(|e| format!("{path}: {e}"))?;
    // let seed = SystemTime::now().duration_since(UNIX_EPOCH)?.subsec_nanos();
    // let config = Config { rng_seed: seed, ..Config::default() };
    // let mut chip = Chip8::new(&rom, config)?;
    // frontend::run(&mut chip)
}
