use super::regs::Regs;
use crate::r650x::alu::ALU;
use crate::r650x::pipeline::Pipeline;
use crate::microvm::memory::sparse::SparseAddressSpace;
use crate::microvm::memory::address_space::AddressSpace;
use crate::microvm::memory::MemoryError;

pub struct Core {
    pipeline: Pipeline,
    regs: Regs,
    alu: ALU,
    space: SparseAddressSpace<u16>
}

impl Core {
    fn fetch(&mut self) -> Result<u8, MemoryError> {
        let b =self.space.read_byte(self.regs.pc)?;
        self.regs.pc += 1;
        Ok(b)
    }
}