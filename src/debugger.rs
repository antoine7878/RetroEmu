use std::cmp::Ordering;
use std::collections::{BTreeSet, VecDeque};
use std::fmt;
use std::io::Write;

use crate::{args::Args, gbc::GameBoy};

const PANEL_HEIGHT: usize = 12;
const HISTORY_LEN: usize = 3;
const ASM_WIDTH: usize = 32;
const REG_WIDTH: usize = 20;
const CONSOLE_WIDTH: usize = ASM_WIDTH + REG_WIDTH + 2;
const CONSOLE_HEIGHT: usize = 10;

#[derive(Default)]
pub struct Debugger {
    breakpoints: BTreeSet<u16>,
    step_budget: Option<u32>,
    skip_break: bool,
    console: Vec<String>,
    history: VecDeque<u16>,
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
        Some(Self { breakpoints: args.breakpoints.clone(), ..Default::default() })
    }

    pub fn check(&mut self, pc: u16) -> Option<StopReason> {
        let reason = self.stop_reason(pc);
        if reason.is_none() {
            if self.history.len() == HISTORY_LEN {
                self.history.pop_front();
            }
            self.history.push_back(pc);
        }
        reason
    }

    fn stop_reason(&mut self, pc: u16) -> Option<StopReason> {
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

    pub fn handle_command(&mut self, cmd: &str) -> DebugAction {
        self.log(format!("(gbdb) {cmd}"));
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
            Some("b") => {
                if let Some(addr) = parse_addr(it.next())
                    && self.breakpoints.insert(addr)
                {
                    self.log(format!("Breakpoint added at ${addr:04X}"));
                }
            }
            Some("d") => {
                if let Some(addr) = parse_addr(it.next())
                    && self.breakpoints.remove(&addr)
                {
                    self.log(format!("Breakpoint removed at ${addr:04X}"));
                }
            }
            Some("bl") => {
                let list: Vec<String> = self.breakpoints.iter().map(|b| format!("${b:04X}")).collect();
                self.log(format!("Breakpoints: {}", list.join(" ")));
            }
            Some("h") => {
                self.log("Commands:".into());
                self.log("  c            continue".into());
                self.log("  s [n]        step n instructions (default 1)".into());
                self.log("  b $<addr>    set breakpoint at hex addr".into());
                self.log("  d $<addr>    delete breakpoint at hex addr".into());
                self.log("  bl           list breakpoints".into());
                self.log("  h            show this help".into());
                self.log("  <enter>      repeat last command".into());
            }
            Some(other) => {
                self.log(format!("unknown command: {other}"));
            }
            None => (),
        }
        DebugAction::Stay
    }

    pub fn log(&mut self, line: String) { self.console.push(line); }

    pub fn draw(&self, game_boy: &GameBoy) {
        let current = self.history.len();
        let past = self.history.iter().flat_map(|&addr| game_boy.disassembly(addr, 1));
        let next = game_boy.disassembly(game_boy.pc(), PANEL_HEIGHT - current);
        let asm: Vec<String> = past
            .chain(next)
            .enumerate()
            .map(|(i, (addr, text))| {
                let bp = if self.breakpoints.contains(&addr) { '●' } else { ' ' };
                let line = pad(&format!("{bp} ${addr:04X}  {text}"), ASM_WIDTH);
                match i.cmp(&current) {
                    Ordering::Less => format!("\x1b[2m{line}\x1b[0m"),
                    Ordering::Equal => format!("\x1b[7m{line}\x1b[0m"),
                    Ordering::Greater => line,
                }
            })
            .collect();
        let regs = game_boy.registers();

        let mut out = String::from("\x1b[H\x1b[J");
        out += &format!("┌{:─<ASM_WIDTH$}┐┌{:─<REG_WIDTH$}┐\n", "─ Disassembly ", "─ Registers ");
        for i in 0..PANEL_HEIGHT {
            let left = asm.get(i).cloned().unwrap_or_else(|| pad("", ASM_WIDTH));
            let right = pad(regs.get(i).map_or("", String::as_str), REG_WIDTH);
            out += &format!("│{left}││{right}│\n");
        }
        out += &format!("└{:─<ASM_WIDTH$}┘└{:─<REG_WIDTH$}┘\n", "", "");
        out += &format!("┌{:─<CONSOLE_WIDTH$}┐\n", "─ Console ");
        let start = self.console.len().saturating_sub(CONSOLE_HEIGHT);
        for i in 0..CONSOLE_HEIGHT {
            let line = self.console.get(start + i).map_or("", String::as_str);
            out += &format!("│{}│\n", pad(line, CONSOLE_WIDTH));
        }
        out += &format!("└{:─<CONSOLE_WIDTH$}┘\n(gbdb) ", "");

        let mut stdout = std::io::stdout().lock();
        let _ = stdout.write_all(out.as_bytes());
        let _ = stdout.flush();
    }
}

fn pad(s: &str, width: usize) -> String { format!("{s:<width$.width$}") }

fn parse_addr(s: Option<&str>) -> Option<u16> { s?.strip_prefix('$').and_then(|hex| u16::from_str_radix(hex, 16).ok()) }
fn parse_int(s: Option<&str>) -> u32 { s.and_then(|s| s.parse().ok()).unwrap_or(1) }

impl fmt::Display for StopReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StopReason::Step => write!(f, "Step"),
            StopReason::Breakpoint(addr) => write!(f, "Breakpoint at ${addr:04X}"),
        }
    }
}
