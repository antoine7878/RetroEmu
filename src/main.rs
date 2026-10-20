#![allow(unused)]
mod argparser;
mod args;
mod frontend;
mod gbc;

use std::fs;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use sdl2::log::{self, Category, Priority};

use args::Args;
use gbc::GameBoy;

use crate::gbc::Config;

static START: OnceLock<Instant> = OnceLock::new();
pub static CYCLES: AtomicU64 = AtomicU64::new(0);

fn my_logger(priority: Priority, _: Category, msg: &str) {
    let ms = START.get_or_init(Instant::now).elapsed().as_secs_f64() * 1000.0;
    let c = CYCLES.load(Ordering::Relaxed);
    let (tag, color) = match priority {
        Priority::Critical | Priority::Error => ("ERR", "\x1b[31m"),
        Priority::Warn => ("WRN", "\x1b[33m"),
        Priority::Info => ("INF", "\x1b[32m"),
        _ => ("DBG", "\x1b[36m"),
    };
    eprintln!("\x1b[90m{ms:>9.3}ms {c:>10}\x1b[0m {color}{tag}\x1b[0m {msg}");
}

fn main() {
    or_exit(sdl2::init());
    log::set_output_function(my_logger);
    println!("RetroEmu start");
    or_exit(Args::init());
    or_exit(run());
    println!("Goodbye !");
    log::log("End");
}

fn or_exit<T, E: std::fmt::Display>(result: Result<T, E>) -> T {
    result.unwrap_or_else(|e| {
        eprintln!("error: {e}");
        std::process::exit(1)
    })
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let rom_path = Args::get().infiles[0].as_str();
    let rom = fs::read(rom_path).map_err(|e| format!("{rom_path}: {e}"))?;
    let seed = SystemTime::now().duration_since(UNIX_EPOCH)?.subsec_nanos();
    let config = Config::new(&rom, seed);
    let mut gb = GameBoy::new(rom, config);
    frontend::run(&mut gb)
}
