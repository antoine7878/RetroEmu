mod argparser;
mod args;
mod color;
mod debugger;
mod frontend;
mod gbc;

use std::fs;
use std::sync::atomic::{AtomicU64, Ordering};

use frontend::Frontend;

use args::Args;
use gbc::GameBoy;

use crate::gbc::Config;

pub static CYCLES: AtomicU64 = AtomicU64::new(0);

fn main() {
    if let Err(e) = run() {
        let cycle = CYCLES.load(Ordering::Relaxed);
        if cycle > 0 {
            eprintln!("error at cycle {cycle} : {e}");
        } else {
            eprintln!("error: {e}");
        }
        std::process::exit(1)
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse()?;
    let sdl = sdl2::init()?;
    let rom_path = args.infiles[0].as_str();
    let rom = fs::read(rom_path).map_err(|e| format!("{rom_path}: {e}"))?;
    let config = Config::new(&rom)?;
    let mut gb = GameBoy::new(rom, config);
    let mut frontend = Frontend::new(args, sdl)?;
    frontend.run(&mut gb)
}
