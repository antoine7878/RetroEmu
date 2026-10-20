use std::collections::BTreeSet;
use std::fmt;

use crate::{args::Args, gbc::GameBoy};

#[derive(Default)]
pub struct Debugger {
    pub breakpoints: BTreeSet<u16>,
    step_budget: Option<u32>,
    skip_break: bool,
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
}
impl fmt::Display for StopReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StopReason::Step => write!(f, "Step"),
            StopReason::Breakpoint(addr) => write!(f, "Breakpoint at ${addr:04X}"),
        }
    }
}
