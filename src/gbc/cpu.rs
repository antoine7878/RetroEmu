use super::bus::Bus;
use super::instruction::Instruction;
use super::instruction::R8;
use super::memory::PROGRAM_START;
use super::registers::Registers;

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
        match instruction {
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

            Instruction::Inc(R8::HlInd) => self.inc_hl(bus),
            Instruction::Dec(R8::HlInd) => self.dec_hl(bus),
            Instruction::Inc(r) => self.registers.set_r8(r, self.registers.get_r8(r).wrapping_add(1)),
            Instruction::Dec(r) => self.registers.set_r8(r, self.registers.get_r8(r).wrapping_sub(1)),
            Instruction::Inc16(r) => self.registers.set_r16(r, self.registers.get_r16(r).wrapping_add(1)),
            Instruction::Dec16(r) => self.registers.set_r16(r, self.registers.get_r16(r).wrapping_sub(1)),

            Instruction::AddHl(r) => self.registers.set_hl(self.registers.get_r16(r)),
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

    fn inc_hl(&mut self, bus: &mut Bus) {
        let hl = self.registers.get_hl();
        bus.write(hl, bus.read(hl).wrapping_add(1))
    }

    fn dec_hl(&mut self, bus: &mut Bus) {
        let hl = self.registers.get_hl();
        bus.write(hl, bus.read(hl).wrapping_sub(1))
    }

    fn ldn(&mut self, bus: &Bus, r: R8) {
        let n = self.take_8(bus);
        self.registers.set_r8(r, n);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fault {
    StackOverflow,
    StackUnderflow,
}
