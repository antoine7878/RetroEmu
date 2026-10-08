use super::error::GBError;

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum R8 { B, C, D, E, H, L, HlInd, A }

impl R8 {
    const ALL: [Self; 8] = [Self::B, Self::C, Self::D, Self::E, Self::H, Self::L, Self::HlInd, Self::A];
    fn oct(o: u8) -> Self { Self::ALL[(o & 0b111) as usize] }
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum R16 { BC, DE, HL, SP }

impl R16 {
    const ALL: [Self; 4] = [Self::BC, Self::DE, Self::HL, Self::SP];
    fn oct(o: u8) -> Self { Self::ALL[((o & 0b111) >> 1) as usize] }
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum R16Stk { BC, DE, HL, AF }

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum R16Mem { BC, DE, HlInc, HlDec }

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cond { NZ, Z, NC, C }

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AluOp { Add, Adc, Sub, Sbc, And, Xor, Or, Cp }

impl AluOp {
    const ALL: [Self; 8] = [Self::Add, Self::Adc, Self::Sub, Self::Sbc, Self::And, Self::Xor, Self::Or, Self::Cp];
    fn oct(o: u8) -> Self { Self::ALL[(o & 0b111) as usize] }
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShiftOp { Rlc, Rrc, Rl, Rr, Sla, Sra, Swap, Srl }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Instruction {
    // Control
    Nop,
    Stop,
    Halt,

    // 8-bit loads
    Ld(R8, R8),
    LdN(R8),

    // 16-bit loads
    Ld16N(R16),

    // Arithmetic
    Alu(AluOp, R8),
    AluN(AluOp),
    Inc(R8),
    Dec(R8),
    Inc16(R16),
    Dec16(R16),
    AddHl(R16),
}

impl Instruction {
    pub fn decode(op_code: u8) -> Result<Self, GBError> {
        use Instruction as I;

        let hi = op_code >> 6;
        let mi = (op_code & 0o070) >> 3;
        let lo = op_code & 0o007;
        let instruction = match (hi, mi, lo) {
            // Control
            (0, 0, 0) => I::Nop,
            (0, 2, 0) => I::Stop,
            (1, 6, 6) => I::Halt,

            // Arithmetic
            (0, r, 4) => I::Inc(R8::oct(r)),
            (0, r, 5) => I::Dec(R8::oct(r)),
            (2, op, r) => I::Alu(AluOp::oct(op), R8::oct(r)),
            (3, op, 6) => I::AluN(AluOp::oct(op)),
            (0, r, 3) if r & 1 == 0 => I::Inc16(R16::oct(r)),
            (0, r, 3) => I::Dec16(R16::oct(r)),
            (0, r, 1) if r & 1 == 0 => I::Ld16N(R16::oct(r)),
            (0, r, 1) => I::AddHl(R16::oct(r)),

            // Ld
            (0, r, 6) => I::LdN(R8::oct(r)),
            (1, src, dst) => I::Ld(R8::oct(src), R8::oct(dst)),

            _ => todo!(),
        };
        Ok(instruction)
    }

    pub fn cycle_count(&self) -> u32 { 1 }
}

// impl fmt::Display for Instruction {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         // use Instruction as I;
//         match *self {
//             _ => todo!(),
//             // I::ClearScreen => write!(f, "CLS"),
//             // I::Return => write!(f, "RET"),
//             // I::Jump(a) => write!(f, "JP   {a:03X}"),
//             // I::Call(a) => write!(f, "CALL {a:03X}"),
//             // I::SkipEqImm(x, n) => write!(f, "SE   V{x:X}, {n:02X}"),
//             // I::SkipNeqImm(x, n) => write!(f, "SNE  V{x:X}, {n:02X}"),
//             // I::SkipEqReg(x, y) => write!(f, "SE   V{x:X}, V{y:X}"),
//             // I::SkipNeqReg(x, y) => write!(f, "SNE  V{x:X}, V{y:X}"),
//             // I::LoadImm(x, n) => write!(f, "LD   V{x:X}, {n:02X}"),
//             // I::AddImm(x, n) => write!(f, "ADD  V{x:X}, {n:02X}"),
//             // I::LoadReg(x, y) => write!(f, "LD   V{x:X}, V{y:X}"),
//             // I::Or(x, y) => write!(f, "OR   V{x:X}, V{y:X}"),
//             // I::And(x, y) => write!(f, "AND  V{x:X}, V{y:X}"),
//             // I::Xor(x, y) => write!(f, "XOR  V{x:X}, V{y:X}"),
//             // I::AddReg(x, y) => write!(f, "ADD  V{x:X}, V{y:X}"),
//             // I::Sub(x, y) => write!(f, "SUB  V{x:X}, V{y:X}"),
//             // I::ShiftRight(x, y) => write!(f, "SHR  V{x:X}, V{y:X}"),
//             // I::SubReverse(x, y) => write!(f, "SUBN V{x:X}, V{y:X}"),
//             // I::ShiftLeft(x, y) => write!(f, "SHL  V{x:X}, V{y:X}"),
//             // I::LoadIndex(a) => write!(f, "LD   I, {a:03X}"),
//             // I::JumpOffset(a) => write!(f, "JP   V0, {a:03X}"),
//             // I::Random(x, n) => write!(f, "RND  V{x:X}, {n:02X}"),
//             // I::Draw(x, y, n) => write!(f, "DRW  V{x:X}, V{y:X}, {n:X}"),
//             // I::SkipKey(x) => write!(f, "SKP  V{x:X}"),
//             // I::SkipNotKey(x) => write!(f, "SKNP V{x:X}"),
//             // I::LoadDelay(x) => write!(f, "LD   V{x:X}, DT"),
//             // I::WaitKey(x) => write!(f, "LD   V{x:X}, K"),
//             // I::SetDelay(x) => write!(f, "LD   DT, V{x:X}"),
//             // I::SetSound(x) => write!(f, "LD   ST, V{x:X}"),
//             // I::AddIndex(x) => write!(f, "ADD  I, V{x:X}"),
//             // I::LoadFont(x) => write!(f, "LD   F, V{x:X}"),
//             // I::Bcd(x) => write!(f, "LD   B, V{x:X}"),
//             // I::Store(x) => write!(f, "LD   [I], V{x:X}"),
//             // I::Load(x) => write!(f, "LD   V{x:X}, [I]"),
//         }
//     }
// }
