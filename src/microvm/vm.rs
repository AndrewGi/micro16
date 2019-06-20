use super::memory::{SparseAddressSpace, DenseStaticMemory, MemoryError};
use super::super::microcompile::instructions::Args;
use crate::microcompile::instructions::{Arg, DecodedArg};

mod traits {
    pub trait Regs<Reg> {
        type RegIdentifier;
        fn get_reg(&self, ident: RegIdentifier) -> Reg;
        fn set_reg(&mut self, ident: RegIdentifier, value: Reg);
    }
    pub trait Context<Regs: Regs> {

    }
}
#[derive(Debug, Clone)]
pub enum VMError {
    Memory(MemoryError),
    InvalidArguments,
    InvalidInstructionFormat,
    InvalidSettings
}
pub trait VM<'a, Context: traits::VM> {
    fn step() -> Result<(), VMError>;
}
