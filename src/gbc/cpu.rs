use super::alu;
use super::bus::Bus;
use super::flag::FlagSpec;
use super::instruction::{AluOp, Cond, Instruction, R8, R16, R16Mem, R16Stk};
use super::memory::PROGRAM_START;
use super::registers::Registers;

// const STACK_SIZE: usize = 16;

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
    stack: Vec<u16>,
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
            stack: Vec::new(),
            sp: 0,
            power_mode: PowerMode::default(),
            ime: false,
            registers: Registers::default(),
        }
    }

    pub fn take_pc8(&mut self, bus: &Bus) -> u8 {
        let res = bus.read(self.pc);
        self.pc = self.pc.wrapping_add(2);
        res
    }

    pub fn take_pc16(&mut self, bus: &Bus) -> u16 {
        let r1 = self.take_pc8(bus);
        let r2 = self.take_pc8(bus);
        (r1 as u16) << 8 | (r2 as u16)
    }

    fn map_high_page(addr: u8) -> u16 { 0xFF00 | addr as u16 }

    pub fn step(&mut self, bus: &mut Bus) {
        let opcode = self.take_pc8(bus);
        let instruction = Instruction::decode(opcode);
        self.execute(instruction, bus)
    }

    fn execute(&mut self, instruction: Instruction, bus: &mut Bus) {
        match instruction {
            Instruction::Nop => (),
            Instruction::Stop => self.power_mode = PowerMode::Stop,
            Instruction::Halt => self.power_mode = PowerMode::Halt,
            Instruction::Di => self.ime = false,
            Instruction::Ei => self.ime = true,

            Instruction::Ld(dst, src) => self.ld(bus, dst, src),
            Instruction::LdImm(r) => self.ldn8(bus, r),
            Instruction::LdhImmA => self.ldimma(bus),
            Instruction::LdhAImm => self.ldaimm(bus),
            Instruction::LdhCA => self.ldhca(bus),
            Instruction::LdhAC => self.ldhac(bus),

            Instruction::LdR16MemA(rm) => self.ldr16a(bus, rm),
            Instruction::LdAR16Mem(rm) => self.ldar16(bus, rm),
            Instruction::LdImm16(r) => self.ldn16(bus, r),
            Instruction::LdA16SP => self.lda16sp(bus),
            Instruction::LdSpHl => self.ldsphl(),
            Instruction::LdHlSp8 => self.ldhlsp8(bus),
            Instruction::LdhAbsA => self.ldhabsa(bus),
            Instruction::LdhAAbs => self.ldhaabs(bus),

            Instruction::Alu(op, r) => self.alu(bus, op, r),
            Instruction::AluN(op) => self.alun(bus, op),
            Instruction::AddHl(r) => self.addhl(r),
            Instruction::AddSpImm => self.addspimm(bus),
            Instruction::Inc(r) => self.inc8(bus, r),
            Instruction::Dec(r) => self.dec8(bus, r),
            Instruction::Inc16(r) => self.inc16(r),
            Instruction::Dec16(r) => self.dec16(r),

            Instruction::Jp(c) => self.jp(bus, c),
            Instruction::Jr(c) => self.jr(bus, c),
            Instruction::JpHl => self.jphl(),
            Instruction::Call(c) => self.call(bus, c),
            Instruction::Ret(c) => self.ret(c),
            Instruction::RetI => self.reti(),
            Instruction::Rst(vec) => self.rst(vec as u16),

            Instruction::Push(r) => self.push(r),
            Instruction::Pop(r) => self.pop(r),

            Instruction::Rcla => todo!(),
            Instruction::Rrca => todo!(),
            Instruction::Rla => todo!(),
            Instruction::Rra => todo!(),
            Instruction::Dda => todo!(),
            Instruction::Cpl => todo!(),
            Instruction::Scf => todo!(),
            Instruction::Ccf => todo!(),

            Instruction::Shift(_, _) => todo!(),
            Instruction::Bit(_, _) => todo!(),
            Instruction::Res(_, _) => todo!(),
            Instruction::Set(_, _) => todo!(),

            Instruction::Illegal(_) => (),
        }
    }

    // ----- Registers Getter/Setter ------------------------------

    fn get_r8(&self, bus: &Bus, r: R8) -> u8 {
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
    const fn get_r16(&self, r: R16) -> u16 { self.registers.get_r16(r) }
    const fn set_r16(&mut self, r: R16, value: u16) { self.registers.set_r16(r, value); }

    const fn condition(&self, c: Option<Cond>) -> bool {
        match c {
            None => true,
            Some(Cond::Z) => self.registers.get_z() == 1,
            Some(Cond::NZ) => self.registers.get_z() == 0,
            Some(Cond::C) => self.registers.get_c() == 0,
            Some(Cond::NC) => self.registers.get_c() == 1,
        }
    }

    // ---- Load 8-bit ------------------------------

    fn ld(&mut self, bus: &mut Bus, dst: R8, src: R8) {
        let value = self.get_r8(bus, src);
        self.set_r8(bus, dst, value)
    }

    fn ldn8(&mut self, bus: &mut Bus, r: R8) {
        let n = self.take_pc8(bus);
        self.set_r8(bus, r, n);
    }

    // ----- Load 16-bit ------------------------------

    fn ldimma(&mut self, bus: &mut Bus) {
        let imm = self.take_pc8(bus);
        let addr = Self::map_high_page(imm);
        let value = self.get_r8(bus, R8::A);
        bus.write(addr, value);
    }

    fn ldaimm(&mut self, bus: &mut Bus) {
        let imm = self.take_pc8(bus);
        let addr = Self::map_high_page(imm);
        let value = bus.read(addr);
        self.set_r8(bus, R8::A, value)
    }

    fn ldhca(&mut self, bus: &mut Bus) {
        let c = self.get_r8(bus, R8::C);
        let value = Self::map_high_page(c);
        bus.write(value, self.get_r8(bus, R8::A));
    }

    fn ldhac(&mut self, bus: &mut Bus) {
        let a = self.get_r8(bus, R8::A);
        let value = Self::map_high_page(a);
        bus.write(value, self.get_r8(bus, R8::C));
    }

    fn ldr16a(&mut self, bus: &mut Bus, rm: R16Mem) {
        let value = self.get_r8(bus, R8::A);
        let r = rm.to_r16();
        bus.write(self.get_r16(r), value);
    }

    fn ldar16(&mut self, bus: &mut Bus, rm: R16Mem) {
        let r = rm.to_r16();
        let addr = self.get_r16(r);
        let value = bus.read(addr);
        self.set_r8(bus, R8::A, value);
    }

    fn ldn16(&mut self, bus: &Bus, r: R16) {
        let n = self.take_pc16(bus);
        self.set_r16(r, n);
    }

    fn lda16sp(&mut self, bus: &mut Bus) {
        let sp = self.get_r16(R16::SP);
        let addr = self.take_pc16(bus);
        bus.write(addr, (sp & 0xFF) as u8);
        bus.write(addr.wrapping_add(1), (sp >> 8) as u8);
    }

    fn ldsphl(&mut self) {
        let value = self.get_r16(R16::HL);
        self.set_r16(R16::SP, value);
    }

    fn ldhlsp8(&mut self, bus: &mut Bus) {
        let sp = self.get_r16(R16::SP);
        let imm = self.take_pc8(bus);
        let (result, flags) = alu::add16(sp, imm as u16, false);
        self.set_r16(R16::HL, result);
        self.registers.set_flags(flags, ADD_SP);
    }

    fn ldhabsa(&mut self, bus: &mut Bus) {
        let imm = self.take_pc8(bus);
        let addr = Self::map_high_page(imm);
        let value = self.get_r8(bus, R8::A);
        bus.write(addr, value);
    }

    fn ldhaabs(&mut self, bus: &mut Bus) {
        let imm = self.take_pc8(bus);
        let addr = Self::map_high_page(imm);
        let value = bus.read(addr);
        self.set_r8(bus, R8::A, value);
    }

    // ----- Alu ------------------------------

    fn alu(&mut self, bus: &mut Bus, op: AluOp, r: R8) {
        let a = self.get_r8(bus, R8::A);
        let r = self.get_r8(bus, r);
        let (result, flags) = op.operation()(a, r, self.registers.get_c() == 1);
        self.set_r8(bus, R8::A, result);
        self.registers.set_flags(flags, op.flag_spec());
    }

    fn alun(&mut self, bus: &mut Bus, op: AluOp) {
        let a = self.get_r8(bus, R8::A);
        let imm = self.take_pc8(bus);
        let (result, flags) = op.operation()(a, imm, self.registers.get_c() == 1);
        self.set_r8(bus, R8::A, result);
        self.registers.set_flags(flags, op.flag_spec());
    }

    fn addhl(&mut self, r: R16) {
        let hl = self.get_r16(R16::HL);
        let r = self.get_r16(r);
        let (result, flags) = alu::add16(hl, r, false);
        self.set_r16(R16::HL, result);
        self.registers.set_flags(flags, ADD_HL);
    }

    fn addspimm(&mut self, bus: &mut Bus) {
        let sp = self.get_r16(R16::SP);
        let imm = self.take_pc8(bus) as u16;
        let (result, flags) = alu::add16(sp, imm, false);
        self.set_r16(R16::SP, result);
        self.registers.set_flags(flags, ADD_SP);
    }

    fn inc8(&mut self, bus: &mut Bus, r: R8) {
        let v = self.get_r8(bus, r);
        let (result, flags) = alu::add8(v, 1, false);
        self.set_r8(bus, r, result);
        self.registers.set_flags(flags, INC);
    }

    fn dec8(&mut self, bus: &mut Bus, r: R8) {
        let v = self.get_r8(bus, r);
        let (result, flags) = alu::sub8(v, 1, false);
        self.set_r8(bus, r, result);
        self.registers.set_flags(flags, DEC);
    }

    fn inc16(&mut self, r: R16) {
        let v = self.get_r16(r);
        let (result, flags) = alu::add16(v, 1, false);
        self.set_r16(r, result);
        self.registers.set_flags(flags, NOF);
    }

    fn dec16(&mut self, r: R16) {
        let v = self.get_r16(r);
        let (result, flags) = alu::sub16(v, 1, false);
        self.set_r16(r, result);
        self.registers.set_flags(flags, NOF);
    }

    // ----- Jump ------------------------------

    fn jp(&mut self, bus: &mut Bus, c: Option<Cond>) {
        if !self.condition(c) {
            return;
        }
        let addr = self.take_pc16(bus);
        self.pc = addr;
    }

    fn jr(&mut self, bus: &mut Bus, c: Option<Cond>) {
        if !self.condition(c) {
            return;
        }
        let offset = self.take_pc8(bus) as i8 as i32;
        self.pc = (self.pc as i32).wrapping_add(offset) as u16;
    }

    const fn jphl(&mut self) { self.pc = self.get_r16(R16::HL); }

    fn call(&mut self, bus: &mut Bus, c: Option<Cond>) {
        if !self.condition(c) {
            return;
        }
        let addr = self.take_pc16(bus);
        self.rst(addr)
    }

    fn ret(&mut self, c: Option<Cond>) {
        if !self.condition(c) {
            return;
        }
        self.pc = self.stack.pop().expect("StackUnderflow");
    }

    fn reti(&mut self) {
        self.ime = true;
        self.ret(None);
    }

    fn rst(&mut self, vec: u16) {
        self.stack.push(self.pc);
        self.pc = vec
    }

    fn push(&mut self, r: R16Stk) {
        let value = self.registers.get_r16stck(r);
        self.stack.push(value);
    }

    fn pop(&mut self, r: R16Stk) {
        let value = self.registers.get_r16stck(r);
        self.stack.push(value);
    }

    // ----- Accumulator/Flag ------------------------------

    // ----- Prefix ------------------------------
}

const DEC: FlagSpec = FlagSpec::znhc("R1R-");
const INC: FlagSpec = FlagSpec::znhc("R0R-");
const ADD_HL: FlagSpec = FlagSpec::znhc("-0RR");
const ADD_SP: FlagSpec = FlagSpec::znhc("00RR");
const RLCA: FlagSpec = FlagSpec::znhc("000R");
const CB_RLC: FlagSpec = FlagSpec::znhc("R00R");
const NOF: FlagSpec = FlagSpec::znhc("----");
