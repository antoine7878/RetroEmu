use crate::gbc::io::IORegister;

#[derive(Default)]
pub struct Audio {
    pub nr10: IORegister<0x7F>,
    pub nr11: IORegister<0xFF>,
    pub nr12: IORegister<0xFF>,
    pub nr13: IORegister<0xFF>,
    pub nr14: IORegister<0xC7>,

    pub nr21: IORegister<0xFF>,
    pub nr22: IORegister<0xFF>,
    pub nr23: IORegister<0xFF>,
    pub nr24: IORegister<0xC7>,

    pub nr30: IORegister<0x80>,
    pub nr31: IORegister<0xFF>,
    pub nr32: IORegister<0x60>,
    pub nr33: IORegister<0xFF>,
    pub nr34: IORegister<0xC7>,

    pub nr41: IORegister<0x3F>,
    pub nr42: IORegister<0xFF>,
    pub nr43: IORegister<0xFF>,
    pub nr44: IORegister<0xC0>,

    pub nr50: IORegister<0xFF>,
    pub nr51: IORegister<0xFF>,
    pub nr52: IORegister<0x8F>,

    pub pcm12: IORegister<0xFF>,
    pub pcm34: IORegister<0xFF>,

    pub w_ram: [u8; 16],
}
