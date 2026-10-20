#![allow(unused)]
mod argparser;
mod args;
mod debugger;
mod frontend;
mod gbc;

use std::fs;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use frontend::Frontend;
use sdl2::Sdl;
use sdl2::log::{self, Category, Priority};

use args::Args;
use gbc::GameBoy;

use crate::gbc::Config;

static START: OnceLock<Instant> = OnceLock::new();
pub static CYCLES: AtomicU64 = AtomicU64::new(0);

fn main() {
    let args = or_exit(Args::parse());
    init_debug(&args);
    let sdl = or_exit(sdl2::init());
    or_exit(run(args, sdl));
    log::log("RetroEmu End");
}

fn my_logger(priority: Priority, _: Category, msg: &str) {
    let ms = START.get_or_init(Instant::now).elapsed().as_secs_f64() * 1000.0;
    let c = CYCLES.load(Ordering::Relaxed);
    let (tag, color) = match priority {
        Priority::Critical | Priority::Error => ("ERR", "\x1b[31m"),
        Priority::Warn => ("WRN", "\x1b[33m"),
        Priority::Info => ("INF", "\x1b[32m"),
        _ => ("DBG", "\x1b[36m"),
    };
    eprintln!("\x1b[90m{ms:>9.2}ms {c:>10}\x1b[0m {color}{tag}\x1b[0m {msg}");
}

fn init_debug(args: &Args) {
    if args.debug {
        sdl2::hint::set("SDL_LOGGING", "*=debug");
    }
    log::set_output_function(my_logger);
    log::log("RetroEmu Start");
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
