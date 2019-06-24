use crate::microvm::memory::address_space::AddressSpace;
use crate::microvm::memory::address::AddressType;
use crate::microvm::memory::MemoryError;

pub struct ZerosSpace {
    size: usize
}
impl<Address: AddressType> AddressSpace<Address> for ZerosSpace {
    fn size(&self) -> Address {
        Address::from_usize(self.size).expect("size too big")
    }

    fn read_byte(&self, address: Address) -> Result<u8, MemoryError> {
        if self.address_in_space(address) {
            Ok(0)
        } else {
            Err(MemoryError::OutOfBounds)
        }
    }
}
