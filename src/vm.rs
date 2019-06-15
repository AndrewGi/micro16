use crate::memory::{SparseAddressSpace, DenseStaticMemory, MemoryError};
use std::ops::Add;

type Reg16 = u16;
type Address16 = u16;
pub struct Regs {
    r1: Reg16,
    r2: Reg16,
    r3: Reg16,
    r4: Reg16,
    pc: Reg16,
    sp: Reg16
}
#[derive(Clone)]
pub struct VMSettings {
    start_pc: Address16,
    stack_base: Address16,

    ram_size: u16,
    ram_address: Address16,
}
const DEFAULT_START_PC: Address16 = 0x0000;
const DEFAULT_STACK_BASE: Address16 = 0xB000;
const DEFAULT_RAM_SIZE: u16 = 0x0400; // 1KB
const DEFAULT_RAM_ADDRESS: Address16 = 0xA000;
impl Default for VMSettings {
    fn default() -> Self {
        VMSettings { start_pc: DEFAULT_START_PC, stack_base: DEFAULT_STACK_BASE,
        ram_size: DEFAULT_RAM_SIZE, ram_address: DEFAULT_RAM_ADDRESS } //random values oof
    }
}
pub struct VM<'a> {
    name: &'a str,
    settings: VMSettings,

    memory_space: SparseAddressSpace<Address16>,
    regs: Regs,
}
impl<'a> VM<'a> {
    pub fn new(name: &'a str, settings: VMSettings) -> Result<VM<'a>, MemoryError> {
        let mut space = SparseAddressSpace::new(Address16::max_value());
        let ram = DenseStaticMemory::new(settings.ram_size.clone());
        space.add_space(settings.ram_address, Box::new(ram))?;
        Ok(VM {
            name,
            memory_space: space,
            regs: Regs {pc: settings.start_pc, ..Default()},
            settings,
        })
    }
    pub fn step() {

    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let vm = VM::new("test", VMSettings::default())?;

    }
}