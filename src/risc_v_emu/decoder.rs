use super::instructions::*;
use super::super::microvm::bits;
use std::convert::TryInto;

pub struct RawInstructionLine<'a> {
    raw: &'a [u8],
    format: InstructionFormat,
}
pub enum DecoderError {
    InvalidOpcode,
}
pub struct Opcode {

}
pub enum WhichReg {

}
#[derive(Copy, Clone)]
pub enum FunctSize {
    Funct3,
    Funct37
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Funct3(u8);
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Funct7(u8);
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Funct37(Funct3, Funct7);
#[derive(Copy, Clone)]
pub enum Funct {
    Funct3(Funct3),
    Funct37(Funct37)
}
impl Funct {
    pub fn size(&self) -> FunctSize {
        match *self {
            Funct::Funct3(_) => FunctSize::Funct3,
            Funct::Funct37(_) => FunctSize::Funct37,
        }
    }
    pub fn funct3(&self) -> Funct3 {
        match *self {
            Funct::Funct3(f) => f,
            Funct::Funct37(f37) => f37.0,
        }
    }
    pub fn funct7(&self) -> Option<Funct7> {
        match *self {
            Funct::Funct3(_) => None,
            Funct::Funct37(f37) => Some(f37.1),
        }
    }
    pub fn value(&self) -> u16 {
        (self.funct3().0 as u16) | ((self.funct7().unwrap_or(Funct7(0)).0 as u16) << 7)
    }
}
pub mod instruction_line {
    use super::*;
    use crate::risc_v_emu::immediate::Immediate;
    use crate::risc_v_emu::types::DataType;

    pub struct RType {
        opcode: Opcode,
        rd: WhichReg,
        rs1: WhichReg,
        rs2: WhichReg,
        funct: Funct37
    }
    pub struct IType<UT: DataType> {
        opcode: Opcode,
        rd: WhichReg,
        rs1: WhichReg,
        imm: Immediate<UT>,
        funct: Funct3
    }
    pub struct SType<UT: DataType> {
        opcode: Opcode,
        imm: Immediate<UT>,
        funct: Funct3,
        rs1: WhichReg,
    }
    pub struct BType<UT: DataType> {
        opcode: Opcode,
        imm: Immediate<UT>,
        funct: Funct3,
        rs1: WhichReg,
        rs2: WhichReg
    }

}
