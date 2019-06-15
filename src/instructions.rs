
use crate::bits;
use crate::bits::BitScanner;

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
impl Default for Opcode {
    fn default() -> Self {
        Opcode::Nop
    }
}
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
const OPLOCATION_BITS: usize = 3;
impl Default for OpLocationCode {
    fn default() -> Self {
        OpLocationCode::NoLocation
    }
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
const REGCODE_BITS: usize = 3;
impl Default for RegsCode {
    fn default() -> Self {
        RegsCode::NoReg
    }
}
#[derive(Default)]
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
const ARG_CONSTANT_SIZE: usize = 2;
pub enum DecodeOperationError {
    InvalidSize(usize)
}
impl Opcode {
    pub fn from(byte: u8) -> Opcode {
        unsafe {
            std::mem::transmute(byte & bits::make_mask::<u8>(OPCODE_BITS))
        }
    }
}
impl OpLocationCode {
    pub fn from(byte: u8) -> OpLocationCode {
        unsafe {
            std::mem::transmute(byte & bits::make_mask::<u8>(OPLOCATION_BITS))
        }
    }
}
impl RegsCode {
    pub fn from(byte: u8) -> RegsCode {
        unsafe {
            std::mem::transmute(byte & bits::make_mask::<u8>(REGCODE_BITS))
        }
    }

}
impl From<&[u8]> for DecodedOperation {
    fn from(bytes: &[u8]) -> Result<DecodedOperation, DecodeOperationError> {
        let mut scanner = BitScanner::new(bytes);
        let opcode = Opcode::from(scanner.collect_bits(OPCODE_BITS));
        let args_format = OpLocationCode::from(scanner.collect_bits(OPLOCATION_BITS));
        let output_reg = RegsCode::from(scanner.collect_bits(REGCODE_BITS));
        let (arg1_c, arg2_c) = match args_format {
            OpLocationCode::NoLocation => (None, None),
            OpLocationCode::TwoConstants => (Some())
            OpLocationCode::RawRegAndConstant => {},
            OpLocationCode::ConstantAndRawReg => {},
            OpLocationCode::TwoRawRegs => {},
            OpLocationCode::LoadRegAndRawReg => {},
            OpLocationCode::RawRegAndLoadReg => {},
            OpLocationCode::TwoLoadRegs => {},
        }
        DecodedOperation::default()
    }
}