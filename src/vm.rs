use crate::memory::SparseAddressSpace;
use std::ops::Add;

type Reg16 = u16;
type Address16 = u16;
pub struct Regs {
    r: [Reg16; 4],
    pc: Reg16,
    sp: Reg16
}
pub struct VMSettings {
    start_pc: Address16,
    stack_base: Address16,

    ram_size: u16,
    ram_address: u16,
}
impl Default for VMSettings {
    fn default() -> Self {
        VMSettings { start_pc: 0x0000, stack_base: 0xB000,
        ram_size: 0x0400 /*1KB*/, ram_address: 0xA000 } //random values oof
    }
}
pub struct VM<'a> {
    name: &'a str,
    settings: VMSettings,

    memory_space: SparseAddressSpace<'a, Address16>,
    regs: Regs,

}
impl<'a> VM<'a> {

}