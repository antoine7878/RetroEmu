use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};
use std::fmt;

use crate::args::Args;
use crate::color::{GREY, RESET, WHITE};
use crate::gbc::GameBoy;

const PANEL_HEIGHT: usize = 12;
const HISTORY_LEN: usize = 3;
const ASM_WIDTH: usize = 32;
const REG_WIDTH: usize = 20;
const CONSOLE_WIDTH: usize = ASM_WIDTH + REG_WIDTH + 2;
const CONSOLE_HEIGHT: usize = 10;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum WatchOn {
    Read,
    #[default]
    Write,
}

impl fmt::Display for WatchOn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WatchOn::Read => write!(f, "read"),
            WatchOn::Write => write!(f, "write"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Watchpoint {
    start: u16,
    end: u16,
    value: Option<u8>,
    on: WatchOn,
}

impl fmt::Display for Watchpoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "${:04X}", self.start)?;
        if self.start != self.end {
            write!(f, "..${:04X}", self.end)?;
        }
        write!(f, " on: {}", self.on)?;
        Ok(())
        // if let Some(value) = self.value { writeln!(f, "to {value}") } else { writeln!(f, "change") }
    }
}

#[derive(Default)]
pub struct Debugger {
    breakpoints: HashMap<u16, usize>,
    watchpoints: HashMap<Watchpoint, usize>,
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
        Some(Self {
            breakpoints: HashMap::from_iter(args.breakpoints.clone().into_iter().enumerate().map(|(a, b)| (b, a))),
            ..Default::default()
        })
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
        self.breakpoints.contains_key(&pc).then_some(StopReason::Breakpoint(pc))
    }

    pub fn resume(&mut self) { self.skip_break = true; }

    pub fn handle_command(&mut self, cmd: &str) -> DebugAction {
        self.log(format!("(gbdb) {cmd}"));
        let it = &mut cmd.split_whitespace();
        match it.next() {
            Some("c") => return self.continu(),
            Some("s") => return self.step(it),
            Some("ab") => self.add_breakpoint(it),
            Some("db") => self.delete_breakpoing(it),
            Some("aw") => self.add_watchpoint(it),
            Some("dw") => self.delete_watchpoint(it),
            Some("l") => self.list_points(),
            Some("q") => std::process::exit(0),
            Some("h") => self.help(),
            Some("cls") => self.console.clear(),
            Some(other) => self.log(format!("unknown command: {other}")),
            None => (),
        }
        DebugAction::Stay
    }

    fn continu(&mut self) -> DebugAction {
        self.resume();
        DebugAction::Continue
    }

    fn step<'a, It: Iterator<Item = &'a str>>(&mut self, it: &mut It) -> DebugAction {
        let n = parse_int(it);
        self.step_budget = Some(n);
        self.resume();
        DebugAction::Step
    }

    fn add_breakpoint<'a, It: Iterator<Item = &'a str>>(&mut self, it: &mut It) {
        let max_id = *self.breakpoints.values().max().unwrap_or(&0);
        let Some(addr) = parse_addr(it) else { return };
        if self.breakpoints.insert(addr, max_id).is_some() {
            self.log(format!("Breakpoint {max_id} added: ${addr:04X}"));
        }
    }

    fn delete_breakpoing<'a, It: Iterator<Item = &'a str>>(&mut self, it: &mut It) {
        let point_id = parse_int(it) as usize;
        let Some((&point, _)) = self.breakpoints.iter().find(|&(_, id)| id == &point_id) else { return };
        if let Some(id) = self.breakpoints.remove(&point) {
            self.log(format!("Breakpoint {id} removed at {point:04X}"));
        }
    }

    fn add_watchpoint<'a, It: Iterator<Item = &'a str>>(&mut self, it: &mut It) {
        let max_id = *self.breakpoints.values().max().unwrap_or(&0);
        let on = match it.next() {
            Some("r" | "read") => WatchOn::Read,
            Some("w" | "write") => WatchOn::Write,
            _ => return self.log("Watchpoint not added".to_string()),
        };
        let Some(range) = it.next() else { return self.log("Watchpoint not added".to_string()) };
        let it = &mut range.split("..");
        let Some(start) = parse_addr(it) else { return self.log("Watchpoint not added".to_string()) };
        let end = parse_addr(it).unwrap_or(start);
        let point = Watchpoint { start, end, on, value: None };
        self.watchpoints.insert(point, max_id);
        self.log(format!("Watchpoint {max_id} added: {point}"));
    }

    fn delete_watchpoint<'a, It: Iterator<Item = &'a str>>(&mut self, it: &mut It) {
        let point_id = parse_int(it) as usize;
        let Some((&point, _)) = self.watchpoints.iter().find(|&(_, id)| id == &point_id) else { return };
        if let Some(id) = self.watchpoints.remove(&point) {
            self.log(format!("Watchpoint {id} removed: {point}"));
        }
    }

    fn list_points(&mut self) {
        let mut breakpoints: Vec<_> = self.breakpoints.iter().map(|(&point, &id)| (id, point)).collect();
        breakpoints.sort_by_key(|&(id, _)| id);
        self.log("Breakpoints:".into());
        for (id, point) in breakpoints {
            self.log(format!("  {id}: ${point:04X}"));
        }
        let mut watchpoints: Vec<_> = self.watchpoints.iter().map(|(&point, &id)| (id, point)).collect();
        watchpoints.sort_by_key(|&(id, _)| id);
        self.log("Watchpoints:".into());
        for (id, point) in watchpoints {
            self.log(format!("  {id}: {point}"));
        }
    }

    fn help(&mut self) {
        self.log("Commands:".into());
        self.log("  c                    continue".into());
        self.log("  s [n]                step n instructions (default 1)".into());
        self.log("  ab $<addr>           add breakpoint at hex addr".into());
        self.log("  db <id>              delete breakpoint by id".into());
        self.log("  aw r|w $<a>[..$<b>]  add read/write watchpoint".into());
        self.log("  dw <id>              delete watchpoint by id".into());
        self.log("  l                    list breakpoints & watchpoints".into());
        self.log("  cls                  clear the console".into());
        self.log("  h                    show this help".into());
        self.log("  q                    quit".into());
        self.log("  <enter>              repeat last command".into());
    }

    pub fn log(&mut self, line: String) { self.console.push(line); }

    pub fn draw(&self, game_boy: &GameBoy) {
        let mut out = String::from("\x1b[H\x1b[J");
        out += &self.disassembly_registers_panel(game_boy);
        out += &self.console_panel();
        println!("{out}");
    }

    fn make_asm(&self, game_boy: &GameBoy) -> Vec<String> {
        let current = self.history.len();

        let past = self.history.iter().flat_map(|&addr| game_boy.disassembly(addr, 1));
        let next = game_boy.disassembly(game_boy.pc(), PANEL_HEIGHT - current);

        past.chain(next)
            .enumerate()
            .map(|(i, (addr, text))| {
                let bp = if self.breakpoints.contains_key(&addr) { '●' } else { ' ' };
                let line = pad(&format!("{bp} ${addr:04X}  {text}"), ASM_WIDTH);
                match i.cmp(&current) {
                    Ordering::Less => format!("{GREY}{line}{RESET}"),
                    Ordering::Equal => format!("{WHITE}{line}{RESET}"),
                    Ordering::Greater => line,
                }
            })
            .collect()
    }

    fn disassembly_registers_panel(&self, game_boy: &GameBoy) -> String {
        let asm = self.make_asm(game_boy);
        let regs = game_boy.registers();
        let mut out = format!("┌{:─<ASM_WIDTH$}┐┌{:─<REG_WIDTH$}┐\n", "─ Disassembly ", "─ Registers ");
        for i in 0..PANEL_HEIGHT {
            let left = asm.get(i).cloned().unwrap_or_else(|| pad("", ASM_WIDTH));
            let right = pad(regs.get(i).map_or("", String::as_str), REG_WIDTH);
            out += &format!("│{left}││{right}│\n");
        }
        out += &format!("└{:─<ASM_WIDTH$}┘└{:─<REG_WIDTH$}┘\n", "", "");
        out
    }

    fn console_panel(&self) -> String {
        let mut out = format!("┌{:─<CONSOLE_WIDTH$}┐\n", "─ Console ");
        let height = console_height();
        let start = self.console.len().saturating_sub(height);
        for i in 0..height {
            let line = self.console.get(start + i).map_or("", String::as_str);
            out += &format!("│{}│\n", pad(line, CONSOLE_WIDTH));
        }
        out += &format!("└{:─<CONSOLE_WIDTH$}┘", "");
        out
    }
}

fn pad(s: &str, width: usize) -> String { format!("{s:<width$.width$}") }

fn console_height() -> usize {
    terminal_rows().map_or(CONSOLE_HEIGHT, |rows| rows.saturating_sub(PANEL_HEIGHT + 2 + 2 + 2).max(1))
}

fn terminal_rows() -> Option<usize> {
    let out = std::process::Command::new("stty").arg("size").stdin(std::process::Stdio::inherit()).output().ok()?;
    String::from_utf8(out.stdout).ok()?.split_whitespace().next()?.parse().ok()
}
fn parse_addr<'a, It: Iterator<Item = &'a str>>(it: &mut It) -> Option<u16> {
    it.next()?.strip_prefix('$').and_then(|hex| u16::from_str_radix(hex, 16).ok())
}
fn parse_int<'a, It: Iterator<Item = &'a str>>(it: &mut It) -> u32 {
    it.next().and_then(|s| s.parse().ok()).unwrap_or(1)
}

impl fmt::Display for StopReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StopReason::Step => write!(f, "Step"),
            StopReason::Breakpoint(addr) => write!(f, "Breakpoint at ${addr:04X}"),
        }
    }
}
