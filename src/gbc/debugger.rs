use std::{collections::HashSet, fmt};

use crate::{args::Args, gbc::GameBoy};

#[derive(Default)]
pub struct Debugger {
    breakpoints: HashSet<u16>,
    step_budget: Option<u32>, // Some(n) = stop after n instructions
}

pub enum StopReason {
    Breakpoint(u16),
    Step,
}

impl Debugger {
    pub fn new(args: &Args) -> Option<Self> {
        if !args.debug {
            return None;
        }
        Some(Self { step_budget: None, breakpoints: args.breakpoints.clone() })
    }

    pub fn check(&mut self, pc: u16) -> Option<StopReason> {
        if let Some(n) = &mut self.step_budget {
            if *n == 0 {
                self.step_budget = None;
                return Some(StopReason::Step);
            }
            *n -= 1;
        }
        self.breakpoints.contains(&pc).then_some(StopReason::Breakpoint(pc))
    }

    pub fn step(&mut self, n: u32) { self.step_budget = Some(n); }

    // pub fn handle_command(&mut self, cmd: &str, game_boy: &mut GameBoy) {
    //     // let Some(dbg) = self.as_mut() else { return };
    //     let mut it = cmd.split_whitespace();
    //     match it.next() {
    //         Some("c") => self.mode = Mode::Running,
    //         Some("s") => {
    //             let n = it.next().and_then(|s| s.parse().ok()).unwrap_or(1);
    //             self.step(n);
    //             self.mode = Mode::Running;
    //         }
    //         Some(other) => {
    //             println!("unknown command: {other}");
    //             self.pause();
    //         }
    //         None => self.pause(),
    //     }
    // }
}

impl fmt::Display for StopReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StopReason::Step => write!(f, "Step"),
            StopReason::Breakpoint(addr) => write!(f, "Breakpoint at {addr:04x}"),
        }
    }
}
