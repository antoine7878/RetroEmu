use std::ops::{AddAssign, BitOr};

#[derive(Default)]
pub struct IORegister<const USED_MASK: u8> {
    raw: u8,
}

impl<const USED_MASK: u8> IORegister<USED_MASK> {
    pub fn set(&mut self, bit: u8) { self.raw |= 1 << bit }
    pub fn clear(&mut self, bit: u8) { self.raw &= !(1 << bit) }
    pub fn is_set(&self, bit: u8) -> bool { self.raw & (1 << bit) > 0 }
    pub fn is_clear(&self, bit: u8) -> bool { self.raw & (1 << bit) == 0 }
}

impl<const USED_MASK: u8> AddAssign<u8> for IORegister<USED_MASK> {
    fn add_assign(&mut self, rhs: u8) { self.raw = self.raw.wrapping_add(rhs); }
}

impl<const USED_MASK: u8> BitOr<u8> for &IORegister<USED_MASK> {
    type Output = u8;
    fn bitor(self, rhs: u8) -> Self::Output { self.read() | rhs }
}

// impl<const USED_MASK: u8> BitAnd<u8> for &IORegister<USED_MASK> {
//     type Output = u8;
//     fn bitand(self, rhs: u8) -> Self::Output { self.read() & rhs }
// }

impl<const USED_MASK: u8> PartialEq for IORegister<USED_MASK> {
    fn eq(&self, other: &Self) -> bool { self.read() == other.read() }
}

impl<const USED_MASK: u8> IOReg for IORegister<USED_MASK> {
    fn read(&self) -> u8 { self.raw & USED_MASK }
    fn write(&mut self, value: u8) { self.raw = value & USED_MASK }
}
pub trait IOReg {
    fn read(&self) -> u8;
    fn write(&mut self, value: u8);
}
