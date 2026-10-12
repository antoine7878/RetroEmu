use super::flag::{FlagSpec, Flags};
use super::instruction::{R8, R16, R16Stk};

const ZERO_FLAG: u8 = 7;
const SUB_FLAG: u8 = 6;
const HALF_CARRY_FLAG: u8 = 5;
const CARRY_FLAG: u8 = 4;

#[rustfmt::skip]
#[derive(Default)]
pub struct Registers {
    a: u8, f: u8,
    b: u8, c: u8,
    d: u8, e: u8,
    h: u8, l: u8,
    sp: u16,
}

impl Registers {
    const fn merge_reg(hi: u8, lo: u8) -> u16 { (hi as u16) << 8 | lo as u16 }
    pub const fn get_af(&self) -> u16 { Self::merge_reg(self.a, self.f) }
    pub const fn get_bc(&self) -> u16 { Self::merge_reg(self.b, self.c) }
    pub const fn get_de(&self) -> u16 { Self::merge_reg(self.d, self.e) }
    pub const fn get_hl(&self) -> u16 { Self::merge_reg(self.h, self.l) }

    const fn split_reg(value: u16) -> (u8, u8) { (((value & 0xFF00) >> 8) as u8, (value & 0x00FF) as u8) }
    pub const fn set_af(&mut self, value: u16) { (self.a, self.f) = Self::split_reg(value); }
    pub const fn set_bc(&mut self, value: u16) { (self.b, self.c) = Self::split_reg(value); }
    pub const fn set_de(&mut self, value: u16) { (self.d, self.e) = Self::split_reg(value); }
    pub const fn set_hl(&mut self, value: u16) { (self.h, self.l) = Self::split_reg(value); }

    pub const fn set_flags(&mut self, flags: Flags, spec: FlagSpec) {
        let [z, n, h, c] = spec.0;
        self.f |= z.resolve(self.get_z(), flags.z) << ZERO_FLAG;
        self.f |= n.resolve(self.get_n(), flags.n) << SUB_FLAG;
        self.f |= h.resolve(self.get_h(), flags.h) << HALF_CARRY_FLAG;
        self.f |= c.resolve(self.get_c(), flags.c) << CARRY_FLAG;
    }

    pub const fn get_z(&self) -> u8 { (self.f >> ZERO_FLAG) & 1 }
    pub const fn get_n(&self) -> u8 { (self.f >> SUB_FLAG) & 1 }
    pub const fn get_h(&self) -> u8 { (self.f >> HALF_CARRY_FLAG) & 1 }
    pub const fn get_c(&self) -> u8 { (self.f >> CARRY_FLAG) & 1 }

    pub const fn get_r8(&self, r: R8) -> u8 {
        match r {
            R8::B => self.b,
            R8::C => self.c,
            R8::D => self.d,
            R8::E => self.e,
            R8::H => self.h,
            R8::L => self.l,
            R8::A => self.a,
            R8::HlInd => unreachable!(),
        }
    }

    pub const fn set_r8(&mut self, r: R8, value: u8) {
        match r {
            R8::B => self.b = value,
            R8::C => self.c = value,
            R8::D => self.d = value,
            R8::E => self.e = value,
            R8::H => self.h = value,
            R8::L => self.l = value,
            R8::A => self.a = value,
            R8::HlInd => unreachable!(),
        }
    }

    pub const fn get_r16(&self, r: R16) -> u16 {
        match r {
            R16::BC => self.get_bc(),
            R16::DE => self.get_de(),
            R16::HL => self.get_hl(),
            R16::SP => self.sp,
        }
    }

    pub const fn set_r16(&mut self, r: R16, value: u16) {
        match r {
            R16::BC => self.set_bc(value),
            R16::DE => self.set_de(value),
            R16::HL => self.set_hl(value),
            R16::SP => self.sp = value,
        }
    }

    pub const fn get_r16stck(&self, r: R16Stk) -> u16 {
        match r {
            R16Stk::BC => self.get_bc(),
            R16Stk::DE => self.get_de(),
            R16Stk::HL => self.get_hl(),
            R16Stk::AF => self.get_af(),
        }
    }

    pub const fn set_r16stck(&mut self, r: R16Stk, value: u16) {
        match r {
            R16Stk::BC => self.set_bc(value),
            R16Stk::DE => self.set_de(value),
            R16Stk::HL => self.set_hl(value),
            R16Stk::AF => self.set_af(value),
        }
    }
}
