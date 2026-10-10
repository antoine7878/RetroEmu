use super::instruction::{R8, R16};

const ZERO_FLAG: u8 = 8;
const SUB_FLAG: u8 = 7;
const HALF_CARRY_FLAG: u8 = 5;
const CARRY_FLAG: u8 = 4;

#[rustfmt::skip]
#[derive(Default)]
pub struct Registers {
    pub a: u8, pub f: u8,
    pub b: u8, pub c: u8,
    pub d: u8, pub e: u8,
    pub h: u8, pub l: u8,
    pub sp: u16,
}

impl Registers {
    fn merge_reg(hi: u8, lo: u8) -> u16 { (hi as u16) << 8 | lo as u16 }
    pub fn get_af(&self) -> u16 { Self::merge_reg(self.a, self.f) }
    pub fn get_bc(&self) -> u16 { Self::merge_reg(self.b, self.c) }
    pub fn get_de(&self) -> u16 { Self::merge_reg(self.d, self.e) }
    pub fn get_hl(&self) -> u16 { Self::merge_reg(self.h, self.l) }

    fn split_reg(value: u16) -> (u8, u8) { (((value & 0xFF00) >> 8) as u8, (value & 0x00FF) as u8) }
    pub fn set_af(&mut self, value: u16) { (self.a, self.f) = Self::split_reg(value); }
    pub fn set_bc(&mut self, value: u16) { (self.b, self.c) = Self::split_reg(value); }
    pub fn set_de(&mut self, value: u16) { (self.d, self.e) = Self::split_reg(value); }
    pub fn set_hl(&mut self, value: u16) { (self.h, self.l) = Self::split_reg(value); }

    pub fn get_z(&mut self) -> u8 { (self.f >> ZERO_FLAG) & 1 }
    pub fn get_n(&mut self) -> u8 { (self.f >> SUB_FLAG) & 1 }
    pub fn get_h(&mut self) -> u8 { (self.f >> HALF_CARRY_FLAG) & 1 }
    pub fn get_c(&mut self) -> u8 { (self.f >> CARRY_FLAG) & 1 }

    pub fn set_z(&mut self) { self.f |= 1 << ZERO_FLAG }
    pub fn set_n(&mut self) { self.f |= 1 << SUB_FLAG }
    pub fn set_h(&mut self) { self.f |= 1 << HALF_CARRY_FLAG }
    pub fn set_c(&mut self) { self.f |= 1 << CARRY_FLAG }

    pub fn reset_z(&mut self) { self.f &= !(1 << ZERO_FLAG) }
    pub fn reset_n(&mut self) { self.f &= !(1 << SUB_FLAG) }
    pub fn reset_h(&mut self) { self.f &= !(1 << HALF_CARRY_FLAG) }
    pub fn reset_c(&mut self) { self.f &= !(1 << CARRY_FLAG) }

    pub fn get_r16(&self, r: R16) -> u16 {
        match r {
            R16::BC => self.get_bc(),
            R16::DE => self.get_de(),
            R16::HL => self.get_hl(),
            R16::SP => self.sp,
        }
    }

    pub fn set_r16(&mut self, r: R16, value: u16) {
        match r {
            R16::BC => self.set_bc(value),
            R16::DE => self.set_de(value),
            R16::HL => self.set_hl(value),
            R16::SP => self.sp = value,
        }
    }

    pub fn get_r8(&self, r: R8) -> u8 {
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

    pub fn set_r8(&mut self, r: R8, value: u8) {
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
}
