use super::bus::Bus;
use super::instruction::{Instruction, R8, R16};
use super::memory::PROGRAM_START;
use super::registers::{FlagSpec, Registers};

const STACK_SIZE: usize = 16;

const CPU_CLOCK: u32 = 1 << 22;
const CYCLE_PER_SCAN_LINE: u32 = 456;
const CYCLE_PRE_FRAME: u32 = 154 * CYCLE_PER_SCAN_LINE;

#[derive(Debug, PartialEq, Eq, Default)]
pub enum PowerMode {
    #[default]
    Normal,
    Halt,
    Stop,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Step {
    Continue,
    WaitVblank,
}

pub struct Cpu {
    v: [u8; 16],
    i: u16,
    pc: u16,
    stack: [u16; STACK_SIZE],
    sp: usize,
    power_mode: PowerMode,
    ime: bool,
    registers: Registers,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            v: [0; 16],
            i: 0,
            pc: PROGRAM_START,
            stack: [0; STACK_SIZE],
            sp: 0,
            power_mode: PowerMode::default(),
            ime: false,
            registers: Registers::default(),
        }
    }

    pub fn take_8(&mut self, bus: &Bus) -> u8 {
        let res = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(2);
        res
    }

    // fn read_pc_ff8(&mut self, bus: &Bus) -> u16 { 0xFF00 | self.read_pc_8(bus) as u16 }
    fn a(addr: u8) -> u16 { 0xFF00 | addr as u16 }
    // pub fn read_pc_16(&mut self, bus: &Bus) -> u16 { (self.read_pc_8(bus) as u16) << 8 | (self.read_pc_8(bus) as u16) }

    pub fn step(&mut self, bus: &mut Bus) {
        let opcode = self.take_8(bus);
        let instruction = Instruction::decode(opcode);
        self.execute(instruction, bus)
    }

    fn execute(&mut self, instruction: Instruction, bus: &mut Bus) {
        let flags = match instruction {
            Instruction::Nop => (),
            Instruction::Stop => self.power_mode = PowerMode::Stop,
            Instruction::Halt => self.power_mode = PowerMode::Halt,
            Instruction::Di => self.ime = false,
            Instruction::Ei => self.ime = true,

            Instruction::Ld(R8::HlInd, R8::HlInd) => unreachable!(),
            Instruction::Ld(r, R8::HlInd) => self.registers.set_r8(r, bus.read(self.registers.get_hl())),
            Instruction::Ld(R8::HlInd, r) => bus.write(self.registers.get_hl(), self.registers.get_r8(r)),
            Instruction::Ld(dst, src) => self.registers.set_r8(dst, self.registers.get_r8(src)),
            Instruction::LdN(r) => self.ldn(bus, r),
            Instruction::LdhImmA => bus.write(Self::a(self.take_8(bus)), self.registers.a),
            Instruction::LdhAImm => self.registers.a = bus.read(Self::a(self.take_8(bus))),
            Instruction::LdhCA => bus.write(Self::a(self.registers.c), self.registers.a),
            Instruction::LdhAC => self.registers.a = bus.read(Self::a(self.registers.c)),

            Instruction::LdhAbsA => todo!(),
            Instruction::LdhAAbs => todo!(),
            Instruction::Ld16N(_) => todo!(),
            Instruction::LdA16SP => todo!(),
            Instruction::LdR16MemA(_) => todo!(),
            Instruction::LdAR16Mem(_) => todo!(),
            Instruction::LdSpHl => todo!(),
            Instruction::LdHlSp8 => todo!(),
            Instruction::Alu(_, R8::HlInd) => todo!(),
            Instruction::Alu(_, _) => todo!(),
            Instruction::AluN(_) => todo!(),

            Instruction::Inc(r) => self.inc8(bus, r),
            Instruction::Dec(r) => self.dec8(bus, r),
            Instruction::Inc16(r) => self.inc16(r),
            Instruction::Dec16(r) => self.dec16(r),

            Instruction::AddHl(_) => todo!(),
            Instruction::AddSpImm => todo!(),
            Instruction::Jr(None) => todo!(),
            Instruction::Jr(Some(_)) => todo!(),
            Instruction::Jp(None) => todo!(),
            Instruction::Jp(Some(_)) => todo!(),
            Instruction::JpHl => todo!(),
            Instruction::Call(None) => todo!(),
            Instruction::Call(Some(_)) => todo!(),
            Instruction::Ret(None) => todo!(),
            Instruction::Ret(Some(_)) => todo!(),
            Instruction::RetI => todo!(),
            Instruction::Rst(_) => todo!(),
            Instruction::Pop(_) => todo!(),
            Instruction::Push(_) => todo!(),
            Instruction::Rcla => todo!(),
            Instruction::Rrca => todo!(),
            Instruction::Rla => todo!(),
            Instruction::Rra => todo!(),
            Instruction::Dda => todo!(),
            Instruction::Cpl => todo!(),
            Instruction::Scf => todo!(),
            Instruction::Ccf => todo!(),
            Instruction::Shift(_, R8::HlInd) => todo!(),
            Instruction::Shift(_, _) => todo!(),
            Instruction::Bit(_, R8::HlInd) => todo!(),
            Instruction::Bit(_, _) => todo!(),
            Instruction::Res(_, R8::HlInd) => todo!(),
            Instruction::Res(_, _) => todo!(),
            Instruction::Set(_, R8::HlInd) => todo!(),
            Instruction::Set(_, _) => todo!(),
            Instruction::Illegal(_) => todo!(),
        }
    }

    fn get_r8(&self, bus: &mut Bus, r: R8) -> u8 {
        match r {
            R8::HlInd => bus.read(self.registers.get_hl()),
            _ => self.registers.get_r8(r),
        }
    }

    fn set_r8(&mut self, bus: &mut Bus, r: R8, value: u8) {
        match r {
            R8::HlInd => bus.write(self.registers.get_hl(), value),
            _ => self.registers.set_r8(r, value),
        }
    }

    fn dec8(&mut self, bus: &mut Bus, r: R8) {
        let value = self.get_r8(bus, r);
        self.set_r8(bus, r, value.wrapping_add(1));
    }

    fn inc8(&mut self, bus: &mut Bus, r: R8) {
        let value = self.get_r8(bus, r);
        self.set_r8(bus, r, value.wrapping_add(1));
    }

    fn dec16(&mut self, r: R16) {
        let value = self.registers.get_r16(r);
        self.registers.set_r16(r, value.wrapping_add(1));
    }

    fn inc16(&mut self, r: R16) {
        let value = self.registers.get_r16(r);
        self.registers.set_r16(r, value.wrapping_add(1));
    }

    fn ldn(&mut self, bus: &Bus, r: R8) {
        let n = self.take_8(bus);
        self.registers.set_r8(r, n);
    }
}

pub fn add8(a: u8, b: u8, carry: bool) -> (u8, Flags) {
    let c = carry as u8;
    let sum = a as u16 + b as u16 + c as u16;
    let result = sum as u8;

    let flags = Flags {
        z: result == 0,
        n: false, h: (a & 0x0F) + (b & 0x0F) + c > 0x0F,
        c: sum > 0xFF,
    };

    (result, flags)
}

pub fn sub8(a: u8, b: u8, carry: bool) -> (u8, Flags) { ... }
pub fn and8(a: u8, b: u8) -> (u8, Flags) { ... }
pub fn rlc(v: u8) -> (u8, Flags) { ... }
pub fn add16(a: u16, b: u16) -> (u16, Flags) { ... } 

const INC: FlagSpec = FlagSpec::parse("R0R-");
const ADD_HL: FlagSpec = FlagSpec::parse("-0RR");
const ADD_SP: FlagSpec = FlagSpec::parse("00RR");
const RLCA: FlagSpec = FlagSpec::parse("000R");
const CB_RLC: FlagSpec = FlagSpec::parse("R00R");
const AND: FlagSpec = FlagSpec::parse("R010");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fault {
    StackOverflow,
    StackUnderflow,
}

// impl AluOp {
//     pub const fn flag_spec(self) -> FlagSpec {
//         match self {
//             AluOp::Add | AluOp::Adc => FlagSpec::parse("Z0HC"),
//             AluOp::Sub | AluOp::Sbc | AluOp::Cp => FlagSpec::parse("Z1HC"),
//             AluOp::And => FlagSpec::parse("Z010"),
//             AluOp::Xor | AluOp::Or => FlagSpec::parse("Z000"),
//         }
//     }
// }
