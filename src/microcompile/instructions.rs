use super::bits;
use std::fmt;
/*
1234567890123456
ooooOOOTTTAAABBB
if Reg == R1 on a constant, load from that memory locations


*/


#[repr(u8)]
#[derive(Clone)]
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
impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}
const OPCODE_BITS: usize = 4;
impl Default for Opcode {
    fn default() -> Self {
        Opcode::Nop
    }
}
#[repr(u8)]
pub enum OpArgsTypes {
    NoLocation =      0b000,
    TwoConstants     =0b001,
    RawRegAndConstant=0b010,
    ConstantAndRawReg=0b011,
    TwoRawRegs       =0b100,
    LoadRegAndRawReg =0b101,
    RawRegAndLoadReg =0b110,
    TwoLoadRegs      =0b111,

}
#[derive(Eq, PartialEq, Clone)]
pub enum Arg {
    Constant(u16),
    RawReg(Reg),
    LoadReg(Reg),
    None
}
impl fmt::Display for Arg {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Arg::Constant(c) => write!(f, "{}", c),
            Arg::RawReg(r) => write!(f, "r{}", r),
            Arg::LoadReg(r) => write!(f, "l{}", r),
            Arg::None => write!(f, "_"),
        }
    }
}
impl OpArgsTypes {

}
impl ToString for OpArgsTypes {
    fn to_string(&self) -> String {
        match self {
            OpArgsTypes::NoLocation => "__",
            OpArgsTypes::TwoConstants => "cc",
            OpArgsTypes::RawRegAndConstant => "rc",
            OpArgsTypes::ConstantAndRawReg => "cr",
            OpArgsTypes::TwoRawRegs => "rr",
            OpArgsTypes::LoadRegAndRawReg => "lr",
            OpArgsTypes::RawRegAndLoadReg => "rl",
            OpArgsTypes::TwoLoadRegs => "ll",
        }.to_string()
    }
}
const OPARGSTYPE_BITS: usize = 3;
impl Default for OpArgsTypes {
    fn default() -> Self {
        OpArgsTypes::NoLocation
    }
}
#[repr(u8)]
#[derive(Eq, PartialEq, Clone)]
pub enum Reg {
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
impl fmt::Display for Reg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
                match self {
                    Reg::NoReg => "_",
                    Reg::R1 => "R1",
                    Reg::R2 => "R2",
                    Reg::R3 => "R3",
                    Reg::R4 => "R4",
                    Reg::SP => "SP",
                    Reg::IP => "IP",
                    Reg::Flags => "FLAGS"
                })
    }
}

impl Default for Reg {
    fn default() -> Self {
        Reg::NoReg
    }
}
#[derive(Default)]
pub struct DecodedOperation {
    opcode: Opcode,
    args_format: OpArgsTypes,
    output_reg: Reg,
    reg1: Reg,
    reg2: Reg,
    arg1_constant: Option<u16>,
    arg2_constant: Option<u16>,
}
const OPERATION_SIZE: usize = 2 + 2 + 2;
const ARG_CONSTANT_SIZE: usize = 2;
pub enum DecodeOperationError {
    InvalidSize(usize),
    RanOutOfBytes,
}
impl Opcode {
    pub fn from(byte: u8) -> Opcode {
        unsafe {
            std::mem::transmute(byte & bits::make_mask::<u8>(OPCODE_BITS))
        }
    }
}
impl OpArgsTypes {
    pub fn from(byte: u8) -> OpArgsTypes {
        unsafe {
            std::mem::transmute(byte & bits::make_mask::<u8>(OPARGSTYPE_BITS))
        }
    }
}
impl Reg {
    pub fn from(byte: u8) -> Reg {
        unsafe {
            std::mem::transmute(byte & bits::make_mask::<u8>(REGCODE_BITS))
        }
    }

}
impl std::convert::TryFrom<&[u8]> for DecodedOperation {
    type Error = DecodeOperationError;
    fn try_from(bytes: &[u8]) -> Result<DecodedOperation, DecodeOperationError> {
        use DecodeOperationError::*;
        if bytes.len() < 2 {
            return Err(RanOutOfBytes);
        }
        let mut scanner = bits::BitScanner::new(bytes);
        let opcode = Opcode::from(scanner.collect_bits(OPCODE_BITS).ok_or(RanOutOfBytes)?);
        let output_reg = Reg::from(scanner.collect_bits(REGCODE_BITS).ok_or(RanOutOfBytes)?);
        let args_format = OpArgsTypes::from(scanner.collect_bits(OPARGSTYPE_BITS).ok_or(RanOutOfBytes)?);
        let reg1 = Reg::from(scanner.collect_bits(REGCODE_BITS).ok_or(RanOutOfBytes)?);
        let reg2 = Reg::from(scanner.collect_bits(REGCODE_BITS).ok_or(RanOutOfBytes)?);
        assert!(scanner.is_byte_aligned(2), "decoded operation didn't align scanner to u16");
        let mut collect_u16 = || {scanner.collect_type::<u16>().ok_or(RanOutOfBytes)};
        let args_c = match args_format {
            OpArgsTypes::TwoConstants => (Some(collect_u16()?), Some(collect_u16()?)),
            OpArgsTypes::RawRegAndConstant => (None, Some(collect_u16()?)),
            OpArgsTypes::ConstantAndRawReg => (Some(collect_u16()?), None),
            _ => (None, None),
        };
        std::mem::drop(collect_u16);
        assert!(scanner.is_byte_aligned(2), "decoded operation didn't align scanner to u16");
        Ok(DecodedOperation {
            opcode,
            args_format,
            output_reg,
            reg1,
            reg2,
            arg1_constant: args_c.0,
            arg2_constant: args_c.1
        })
    }
}