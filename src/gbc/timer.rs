use super::io::IORegister;

#[derive(Default)]
pub struct Timer {
    pub divider: IORegister<0xFF>,
    pub counter: IORegister<0xFF>,
    pub modulo: IORegister<0xFF>,
    pub ctrl: IORegister<0x07>,
}
