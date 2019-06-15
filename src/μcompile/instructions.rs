
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
impl ToString for Opcode {
    fn to_string(&self) -> String {
        match self {
            Opcode::Nop => "nop",
            Opcode::Mov => "mov",
            Opcode::Add => "add",
            Opcode::Sub => "sub",
            Opcode::IMul => "imul",
            Opcode::IDiv => "idiv",
            Opcode::Cmp => "cmp",
            Opcode::Jmp => "jmp",
            Opcode::And => "and",
            Opcode::Or => "or",
            Opcode::Xor => "xor",
            Opcode::Not => "not",
            Opcode::Call => "call",
            Opcode::Ret => "ret",
            Opcode::Syscall => "syscall",
            Opcode::Int => "int"
        }.to_string()
    }
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
impl ToString for OpLocationCode {
    fn to_string(&self) -> String {
        match self {
            OpLocationCode::NoLocation => "__",
            OpLocationCode::TwoConstants => "cc",
            OpLocationCode::RawRegAndConstant => "rc",
            OpLocationCode::ConstantAndRawReg => "cr",
            OpLocationCode::TwoRawRegs => "rr",
            OpLocationCode::LoadRegAndRawReg => "lr",
            OpLocationCode::RawRegAndLoadReg => "rl",
            OpLocationCode::TwoLoadRegs => "ll",
        }.to_string()
    }
}
const OPLOCATION_BITS: usize = 3;
impl Default for OpLocationCode {
    fn default() -> Self {
        OpLocationCode::NoLocation
    }
}
#[repr(u8)]
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
impl ToString for Reg {
    fn to_string(&self) -> String {
        match self {
            Reg::NoReg => "_",
            Reg::R1 => "R1",
            Reg::R2 => "R2",
            Reg::R3 => "R3",
            Reg::R4 => "R4",
            Reg::SP => "SP",
            Reg::IP => "IP",
            Reg::Flags => "FLAGS"
        }.to_string()
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
    args_format: OpLocationCode,
    output_reg: Reg,
    arg1: Reg,
    arg2: Reg,
    arg1_constant: Option<u16>,
    arg2_constant: Option<u16>,
}
const OPERATION_SIZE: usize = 2 + 2 + 2;

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
impl Reg {
    pub fn from(byte: u8) -> Reg {
        unsafe {
            std::mem::transmute(byte & bits::make_mask::<u8>(REGCODE_BITS))
        }
    }

}
impl From<[u8; OPERATION_SIZE]> for DecodedOperation {
    fn from(u: [u8; OPERATION_SIZE]) -> DecodedOperation {
        let opcode = Opcode::from(u[0]);
        DecodedOperation::default()
    }
}