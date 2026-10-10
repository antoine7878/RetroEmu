use super::io::IORegister;

#[derive(Default)]
pub struct Link {
    pub buffer: IORegister<0xFF>,
    pub ctrl: IORegister<0x03>,
}
