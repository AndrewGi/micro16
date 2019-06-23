use super::memory::{MemoryError};
use crate::microvm::memory::address::AddressType;

mod traits {
    use crate::microvm::mmu::MMU;
    use crate::microvm::memory::address::AddressType;

    pub trait Regs<Reg> {
        type RegIdentifier;
        fn get_reg(&self, ident: u32) -> Reg;
        fn set_reg(&mut self, ident: u32, value: Reg);
    }
    pub trait Context<'a, Address: AddressType> {
        type Regs;
        fn mmu() -> &'a MMU< Address>;
        fn mmu_mut() -> &'a mut MMU< Address>;
    }
}
#[derive(Debug, Clone)]
pub enum VMError {
    Memory(MemoryError),
    InvalidArguments,
    InvalidInstructionFormat,
    InvalidSettings
}
pub trait VM<'a, Address: AddressType, Context: traits::Context<'a, Address>> {
    fn cycle() -> Result<(), VMError>;
}
