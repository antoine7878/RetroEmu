const ZERO_FLAG: u8 = 8;
const SUB_FLAG: u8 = 7;
const HALF_CARRY_FLAG: u8 = 5;
const CARRY_FLAG: u8 = 4;

#[rustfmt::skip]
pub struct Registers {
    a: u8, f: u8,
    b: u8, c: u8,
    d: u8, e: u8,
    h: u8, l: u8,
    sp: u16,
    cp: u16,
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
}
