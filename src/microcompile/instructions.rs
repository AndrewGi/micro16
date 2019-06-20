use super::bits;
use std::fmt;
/*
1234567890123456
ooooOOOLAAALBBBL
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
#[derive(Eq, PartialEq, Clone)]
pub enum DecodedArg {
    RawConstant(u16),
    LoadConstant(u16),
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
#[repr(u8)]
#[derive(Eq, PartialEq, Clone)]
pub enum Reg {
    Constant = 0,
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
                    Reg::Constant => "C",
                    Reg::R1 => "R1",
                    Reg::R2 => "R2",
                    Reg::R3 => "R3",
                    Reg::R4 => "R4",
                    Reg::SP => "SP",
                    Reg::IP => "IP",
                    Reg::Flags => "FL"
                })
    }
}


pub struct DecodedOperation {
    opcode: Opcode,
    output: DecodedArg,
    arg1: DecodedArg,
    arg2: DecodedArg
}
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
impl Reg {
    pub fn from(byte: u8) -> Reg {
        unsafe {
            std::mem::transmute(byte & bits::make_mask::<u8>(REGCODE_BITS))
        }
    }

}
impl DecodedArg {
    pub fn new(reg: Reg, do_load: bool, maybe_constant: Option<u16>) -> Result<DecodedArg, DecodedOperation> {
        if reg == Reg::Constant {

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
        let load_output = scanner.next()?;
        let arg1 = Reg::from(scanner.collect_bits(REGCODE_BITS).ok_or(RanOutOfBytes)?);
        let load_reg1 = scanner.next()?;
        let arg2 = Reg::from(scanner.collect_bits(REGCODE_BITS).ok_or(RanOutOfBytes)?);
        let load_reg2 = scanner.next()?;
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