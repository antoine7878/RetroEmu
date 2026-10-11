#[derive(Clone, Copy, Default)]
pub struct Flags {
    z: bool,
    n: bool,
    h: bool,
    c: bool,
}

impl Flags {
    pub fn apply(&mut self, computed: Flags, spec: FlagSpec) {
        let [z, n, h, c] = spec.0;
        self.z = z.resolve(computed.z, self.z);
        self.n = n.resolve(computed.n, self.n);
        self.h = h.resolve(computed.h, self.h);
        self.c = c.resolve(computed.c, self.c);
    }

    pub fn set(&mut self, value: u8) {
        self.z = value & (1 << 7) > 0;
        self.h = value & (1 << 6) > 0;
        self.h = value & (1 << 5) > 0;
        self.c = value & (1 << 4) > 0;
    }
}

impl From<Flags> for u8 {
    fn from(f: Flags) -> Self { (f.z as u8) << 7 | (f.n as u8) << 6 | (f.h as u8) << 5 | (f.c as u8) << 4 }
}

impl FlagEffect {
    const fn resolve(self, computed: bool, current: bool) -> bool {
        match self {
            FlagEffect::Result => computed,
            FlagEffect::Clear => false,
            FlagEffect::Set => true,
            FlagEffect::Keep => current,
        }
    }
}

#[derive(Clone, Copy)]
enum FlagEffect {
    Result,
    Clear,
    Set,
    Keep,
}

impl FlagEffect {
    const fn from_byte(value: u8) -> Self {
        match value {
            b'R' => Self::Result,
            b'1' => Self::Clear,
            b'0' => Self::Set,
            b'-' => Self::Keep,
            _ => panic!("bad flag effect"),
        }
    }
}

#[derive(Clone, Copy)]
pub struct FlagSpec([FlagEffect; 4]); // Z, N, H, C

impl FlagSpec {
    pub const fn parse(s: &str) -> Self {
        let mut ret = [FlagEffect::Keep; 4];
        let s = s.as_bytes();
        let mut i = 0;
        while i < 4 {
            ret[i] = FlagEffect::from_byte(s[i]);
            i += 1;
        }
        Self(ret)
    }
}
