use super::Error;
use super::bus::Bus;
use super::display::{HEIGHT, WIDTH};
use super::instruction::{Instruction, Reg};
use super::memory::PROGRAM_START;

const STACK_SIZE: usize = 16;

const CPU_CLOCK: u32 = 1 << 22;
const CYCLE_PER_SCAN_LINE: u32 = 456;
const CYCLE_PRE_FRAME: u32 = 154 * CYCLE_PER_SCAN_LINE;

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
    awaited_key: Option<u8>,
    rng: RandomGenerator,
}

impl Cpu {
    pub fn new(seed: u32) -> Self {
        Self {
            v: [0; 16],
            i: 0,
            pc: PROGRAM_START,
            stack: [0; STACK_SIZE],
            sp: 0,
            awaited_key: None,
            rng: RandomGenerator::new(seed),
        }
    }

    pub fn step(&mut self, bus: &mut Bus) -> Result<u32, Error> {
        let pc = self.pc;
        let opcode = bus.read_u16(pc);
        let instruction = Instruction::decode(opcode).ok_or(Error::UnknownOpcode { opcode, pc })?;
        self.pc = pc.wrapping_add(2);
        self.execute(instruction, bus)
            .map_err(|kind| Error::Execution { kind, pc, instruction })?;
        Ok(instruction.cycle_count())
    }

    fn execute(&mut self, _instruction: Instruction, _bus: &mut Bus) -> Result<(), Fault> {
        // match instruction {
        //     ClearScreen => bus.display.clear(),
        //     Return => {
        //         self.sp = self.sp.checked_sub(1).ok_or(Fault::StackUnderflow)?;
        //         self.pc = self.stack[self.sp];
        //     }
        //     Jump(addr) => self.pc = addr.get(),
        //     Call(addr) => {
        //         *self.stack.get_mut(self.sp).ok_or(Fault::StackOverflow)? = self.pc;
        //         self.sp += 1;
        //         self.pc = addr.get();
        //     }
        //     SkipEqImm(x, n) => self.skip_if(self.v(x) == n),
        //     SkipNeqImm(x, n) => self.skip_if(self.v(x) != n),
        //     SkipEqReg(x, y) => self.skip_if(self.v(x) == self.v(y)),
        //     SkipNeqReg(x, y) => self.skip_if(self.v(x) != self.v(y)),
        //     LoadImm(x, n) => self.set_v(x, n),
        //     AddImm(x, n) => self.set_v(x, self.v(x).wrapping_add(n)),
        //     LoadReg(x, y) => self.set_v(x, self.v(y)),
        //     Or(x, y) => self.logic(x, y, |a, b| a | b),
        //     And(x, y) => self.logic(x, y, |a, b| a & b),
        //     Xor(x, y) => self.logic(x, y, |a, b| a ^ b),
        //     AddReg(x, y) => {
        //         let (result, carry) = self.v(x).overflowing_add(self.v(y));
        //         self.set_v_with_flag(x, result, carry);
        //     }
        //     Sub(x, y) => {
        //         let (result, borrow) = self.v(x).overflowing_sub(self.v(y));
        //         self.set_v_with_flag(x, result, !borrow);
        //     }
        //     SubReverse(x, y) => {
        //         let (result, borrow) = self.v(y).overflowing_sub(self.v(x));
        //         self.set_v_with_flag(x, result, !borrow);
        //     }
        //     ShiftRight(x, y) => {
        //         let value = self.v(y);
        //         self.set_v_with_flag(x, value >> 1, value & 1 == 1);
        //     }
        //     ShiftLeft(x, y) => {
        //         let value = self.v(y);
        //         self.set_v_with_flag(x, value << 1, value >> 7 == 1);
        //     }
        //     LoadIndex(addr) => self.i = addr.get(),
        //     JumpOffset(addr) => self.pc = addr.get().wrapping_add(self.v(Reg::V0) as u16),
        //     Random(x, mask) => {
        //         let random = self.rng.next_u8();
        //         self.set_v(x, random & mask);
        //     }
        //     Draw(x, y, height) => {
        //         self.draw(bus, x, y, height);
        //         return Ok(Step::WaitVblank);
        //     }
        //     SkipKey(x) => self.skip_if(bus.keypad.is_pressed(self.v(x))),
        //     SkipNotKey(x) => self.skip_if(!bus.keypad.is_pressed(self.v(x))),
        //     LoadDelay(x) => self.set_v(x, bus.delay_timer.value()),
        //     WaitKey(x) => self.wait_key(bus, x),
        //     SetDelay(x) => bus.delay_timer.set(self.v(x)),
        //     SetSound(x) => bus.sound_timer.set(self.v(x)),
        //     AddIndex(x) => self.i = self.i.wrapping_add(self.v(x) as u16),
        //     // LoadFont(x) => self.i = Memory::font_address(self.v(x)),
        //     Bcd(x) => {
        //         let value = self.v(x);
        //         bus.write(self.i, value / 100);
        //         bus.write(self.i.wrapping_add(1), value / 10 % 10);
        //         bus.write(self.i.wrapping_add(2), value % 10);
        //     }
        //     Store(x) => {
        //         for (offset, r) in x.up_to().enumerate() {
        //             bus.write(self.i.wrapping_add(offset as u16), self.v(r));
        //         }
        //         self.i = self.i.wrapping_add(x.index() as u16 + 1);
        //     }
        //     Load(x) => {
        //         for (offset, r) in x.up_to().enumerate() {
        //             self.set_v(r, bus.read(self.i.wrapping_add(offset as u16)));
        //         }
        //         self.i = self.i.wrapping_add(x.index() as u16 + 1);
        //     }
        // }
        // Ok(Step::Continue)
        todo!();
    }

    fn v(&self, reg: Reg) -> u8 { self.v[reg.index()] }

    fn set_v(&mut self, reg: Reg, value: u8) { self.v[reg.index()] = value; }

    fn set_v_with_flag(&mut self, reg: Reg, value: u8, flag: bool) {
        self.set_v(reg, value);
        // self.set_v(Reg::VF, flag as u8);
    }

    fn skip_if(&mut self, condition: bool) {
        if condition {
            self.pc = self.pc.wrapping_add(2);
        }
    }

    fn logic(&mut self, x: Reg, y: Reg, op: impl Fn(u8, u8) -> u8) {
        self.set_v(x, op(self.v(x), self.v(y)));
        // self.set_v(Reg::VF, 0);
    }

    fn draw(&mut self, bus: &mut Bus, x: Reg, y: Reg, height: u8) {
        let x0 = self.v(x) as usize % WIDTH;
        let y0 = self.v(y) as usize % HEIGHT;

        for row in 0..(height as usize).min(HEIGHT - y0) {
            let py = y0 + row;
            let bits = bus.read(self.i.wrapping_add(row as u16));
            for col in 0..8.min(WIDTH - x0) {
                let px = x0 + col;
                if bits & (0x80 >> col) != 0 {
                    bus.display.toggle(px, py, 0x42);
                }
            }
        }
        // self.set_v(Reg::VF, collision as u8);
    }

    fn wait_key(&mut self, bus: &Bus, x: Reg) {
        match self.awaited_key {
            Some(key) if !bus.keypad.is_pressed(key) => {
                self.set_v(x, key);
                self.awaited_key = None;
                return;
            }
            Some(_) => {}
            None => self.awaited_key = bus.keypad.first_pressed(),
        }
        self.pc = self.pc.wrapping_sub(2);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fault {
    StackOverflow,
    StackUnderflow,
}

struct RandomGenerator(u32);

impl RandomGenerator {
    fn new(seed: u32) -> Self { Self(seed.max(1)) }

    fn next_u8(&mut self) -> u8 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.0 = x;
        (x >> 24) as u8
    }
}
