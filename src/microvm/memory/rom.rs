use crate::microvm::memory::address_space::{DenseStaticMemory, AddressSpace};
use crate::microvm::memory::address::AddressType;

pub struct ROM {
    memory: DenseStaticMemory
}
impl<Address: AddressType> AddressSpace<Address> for ROM {
    fn size(&self) -> Address {
        self.memory.size()
    }

    fn read_byte(&self, address: Address) -> Result<u8, super::MemoryError> {
        self.memory.read_byte(address)
    }
}
impl ROM {
    pub fn new(size: impl AddressType) -> ROM {
        ROM {
            memory: DenseStaticMemory::new(size)
        }
    }
    pub fn get_mut(&mut self) -> &mut DenseStaticMemory {
        &mut self.memory
    }
}