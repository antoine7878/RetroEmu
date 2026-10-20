use std::collections::BTreeSet;
use std::fmt;

use crate::{args::Args, gbc::GameBoy};

#[derive(Default)]
pub struct Debugger {
    breakpoints: BTreeSet<u16>,
    step_budget: Option<u32>,
    skip_break: bool,
}

pub enum StopReason {
    Breakpoint(u16),
    Step,
}

pub enum DebugAction {
    Continue,
    Step,
    Stay,
}

impl Debugger {
    pub fn new(args: &Args) -> Option<Self> {
        if !args.debug {
            return None;
        }
        Some(Self { step_budget: None, breakpoints: args.breakpoints.clone(), skip_break: false })
    }

    pub fn check(&mut self, pc: u16) -> Option<StopReason> {
        if let Some(n) = &mut self.step_budget {
            if *n == 0 {
                self.step_budget = None;
                return Some(StopReason::Step);
            }
            *n -= 1;
        }
        if std::mem::take(&mut self.skip_break) {
            return None;
        }
        self.breakpoints.contains(&pc).then_some(StopReason::Breakpoint(pc))
    }

    pub fn resume(&mut self) { self.skip_break = true; }

    pub fn step(&mut self, n: u32) {
        self.step_budget = Some(n);
        self.resume();
    }

    pub fn handle_command(&mut self, cmd: &str, game_boy: &GameBoy) -> DebugAction {
        let mut it = cmd.split_whitespace();
        match it.next() {
            Some("c") => {
                self.resume();
                return DebugAction::Continue;
            }
            Some("s") => {
                self.step(parse_int(it.next()));
                return DebugAction::Step;
            }
            Some("r") => game_boy.print_registers(),
            Some("b") => {
                if let Some(addr) = parse_addr(it.next())
                    && self.breakpoints.insert(addr)
                {
                    println!("Breakpoint added at ${addr:04X}");
                }
            }
            Some("d") => {
                if let Some(addr) = parse_addr(it.next())
                    && self.breakpoints.remove(&addr)
                {
                    println!("Breakpoint removed at ${addr:04X}");
                }
            }
            Some("bl") => {
                println!("Breakpoints:");
                for b in self.breakpoints.iter() {
                    println!("${b:04X}");
                }
            }
            Some("h") => {
                println!("Commands:");
                println!("  c            continue");
                println!("  s [n]        step n instructions (default 1)");
                println!("  r            print registers");
                println!("  b $<addr>    set breakpoint at hex addr");
                println!("  d $<addr>    delete breakpoint at hex addr");
                println!("  bl           list breakpoints");
                println!("  h            show this help");
                println!("  <enter>      repeat last command");
            }
            Some(other) => {
                println!("unknown command: {other}");
            }
            None => (),
        }
        DebugAction::Stay
    }
}

fn parse_addr(s: Option<&str>) -> Option<u16> {
    s?.strip_prefix('$').and_then(|hex| u16::from_str_radix(hex, 16).ok())
}
fn parse_int(s: Option<&str>) -> u32 { s.and_then(|s| s.parse().ok()).unwrap_or(1) }

impl fmt::Display for StopReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StopReason::Step => write!(f, "Step"),
            StopReason::Breakpoint(addr) => write!(f, "Breakpoint at ${addr:04X}"),
        }
    }
}
