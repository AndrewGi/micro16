
use crate::bits;
#[repr(u8)]
pub enum Opcode {
    Nop = 0,
    Mov = 1,
    Add = 2,
    Sub = 3,
    IMul = 4,
    IDiv = 5,
    Cmp = 6,
    Jmp = 7,
    And = 8,
    Or = 9,
    Xor = 11,
    Not = 10,
    Call =12,
    Ret = 13,
    Syscall = 14,
    Int = 15,
}
const OPCODE_BITS: usize = 4;
#[repr(u8)]
pub enum OpLocationCode {
    NoLocation =      0b000,
    TwoConstants     =0b001,
    RawRegAndConstant=0b010,
    ConstantAndRawReg=0b011,
    TwoRawRegs       =0b100,
    LoadRegAndRawReg =0b101,
    RawRegAndLoadReg =0b110,
    TwoLoadRegs      =0b111,

}
#[repr(u8)]
pub enum RegsCode {
    NoReg = 0,
    R1 = 1,
    R2 = 2,
    R3 = 3,
    R4 = 4,
    SP = 5,
    IP = 6,
    Flags = 7
}
const OPLOCATION_BITS: usize = 3;
pub struct DecodedOperation {
    opcode: Opcode,
    args_format: OpLocationCode,
    output_reg: RegsCode,
    arg1: RegsCode,
    arg2: RegsCode,
    arg1_constant: Option<u16>,
    arg2_constnat: Option<u16>,
}
const OPERATION_SIZE: usize = 2 + 2 + 2;
impl Opcode {
    pub const fn from(byte: u8) -> Opcode {
        (byte & bits::make_mask(OPCODE_BITS)) as Opcode
    }
}
impl OpLocationCode {
    pub const fn from(byte: u8) -> OpLocationCode {
        (byte & bits::make_mask(OPLOCATION_BITS)) as OpLocationCode
    }
}
impl RegsCode {
    pub const fn from(byte: u8) -> OpLocationCode {
        (byte & bits::make_mask(OPLOCATION_BITS)) as OpLocationCode
    }

}
impl From<[u8; OPERATION_SIZE]> for DecodedOperation {
    fn from(u: [u8; OPERATION_SIZE]) -> DecodedOperation {
        let opcode = Opcode::from(u[0]);

    }
}