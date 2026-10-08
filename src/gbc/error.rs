use std::fmt::{self};

use super::cpu::Fault;
use super::instruction::Instruction;
use super::memory::MAX_ROM_SIZE;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GBError {
    // RomTooLarge {
    //     size: usize,
    // },
    // UnknownOpcode {
    //     opcode: u16,
    //     pc: u16,
    // },
    // Execution {
    //     kind: Fault,
    //     pc: u16,
    //     instruction: Instruction,
    // },
    UnknownR8(u8),
}

impl fmt::Display for GBError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GBError::UnknownR8(r) => write!(f, "Unknown R8 {r}"),
            // Error::RomTooLarge { size } => write!(f, "ROM is {size} bytes, the maximum is {MAX_ROM_SIZE}"),
            // Error::UnknownOpcode { opcode, pc } => write!(f, "unknown opcode {opcode:04X} at {pc:03X}"),
            // Error::Execution { kind, pc, instruction } => write!(f, "{kind:?} at {pc:03X} ({instruction:?})"),
        }
    }
}

impl std::error::Error for GBError {}
