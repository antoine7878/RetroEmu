mod argparser;
mod args;
mod frontend;
mod gbc;

use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

use args::Args;
use gbc::GameBoy;

use crate::gbc::Config;

macro_rules! exit {
    ($($arg:tt)*) => {{
        eprintln!($($arg)*);
        std::process::exit(1)
    }};
}

fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let rom_path = args.infiles[0].as_str();
    let rom = fs::read(rom_path).map_err(|e| format!("{rom_path}: {e}"))?;
    let seed = SystemTime::now().duration_since(UNIX_EPOCH)?.subsec_nanos();
    let config = Config::new(&rom, seed);
    let mut gb = GameBoy::new(rom, config);
    frontend::run(&mut gb)
}

fn main() {
    let args = match Args::parse() {
        Ok(args) => args,
        Err(e) => exit!("{e}"),
    };
    match run(args) {
        Ok(()) => println!("Goodbye !"),
        Err(e) => exit!("error: {e}"),
    }
}
