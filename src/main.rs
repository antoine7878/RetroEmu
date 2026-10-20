mod argparser;
mod args;
mod frontend;
mod gbc;

use std::fs;
use std::sync::OnceLock;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use sdl2::log::{self, Category, Priority};

use args::Args;
use gbc::GameBoy;

use crate::gbc::Config;
static START: OnceLock<Instant> = OnceLock::new();

fn my_logger(priority: Priority, category: Category, msg: &str) {
    let t = START.get_or_init(Instant::now).elapsed().as_secs_f64();
    eprintln!("[{:.6}] [{:?}][{:?}] {}", t * 1000., priority, category, msg);
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
