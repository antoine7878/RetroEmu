// #![allow(unused)]
mod argparser;
mod args;
mod debugger;
mod frontend;
mod gbc;

use std::fs;
use std::sync::atomic::AtomicU64;
use std::time::{SystemTime, UNIX_EPOCH};

use frontend::Frontend;
use sdl2::Sdl;

use args::Args;
use gbc::GameBoy;

use crate::gbc::Config;

pub static CYCLES: AtomicU64 = AtomicU64::new(0);

fn main() {
    let args = or_exit(Args::parse());
    let sdl = or_exit(sdl2::init());
    or_exit(run(args, sdl));
}

fn run(args: Args, sdl: Sdl) -> Result<(), Box<dyn std::error::Error>> {
    let rom_path = args.infiles[0].as_str();
    let rom = fs::read(rom_path).map_err(|e| format!("{rom_path}: {e}"))?;
    let seed = SystemTime::now().duration_since(UNIX_EPOCH)?.subsec_nanos();
    let config = Config::new(&rom, seed);
    let mut gb = GameBoy::new(rom, config);
    let mut frontend = Frontend::new(args, sdl)?;
    frontend.run(&mut gb)
}

fn or_exit<T, E: std::fmt::Display>(result: Result<T, E>) -> T {
    result.unwrap_or_else(|e| {
        eprintln!("error: {e}");
        std::process::exit(1)
    })
}
