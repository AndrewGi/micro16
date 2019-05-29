use std;
use std::ops::Add;

pub enum MemoryError {
    OutOfBounds,
    InvalidAccess,
}

pub trait AddressSpace<'a, Address> {
    fn size(&self) -> usize;
    fn get_span(&self, start: Address, end: Address) -> Result<Self, MemoryError>;
    fn get_bytes(&self, addr: Address, amount: usize) -> Result<&[u8], MemoryError>;
}

impl<AddressT> std::ops::Index<std::ops::Range<AddressT>> for AddressSpace<'_, AddressT> {
    type Output = Result<Self, MemoryError>;
    fn index(&self, i: std::ops::Range<AddressT>) -> Result<Self, MemoryError> {
        self.get_span(i.start, i.end)
    }
}
impl<AddressT> std::ops::Index<AddressT> for AddressSpace<'_, AddressT> {
    type Output = Result<u8, MemoryError>;
    fn index(&self, address: AddressT) -> Result<u8, MemoryError> {
        Ok(self.get_bytes(address, 1)?[0])
    }
}

pub trait MappableAddressSpace<AddressT, ASpaceT: AddressSpace<AddressT>> {
    type MappedObject = (AddressSpace<'_, AddressT>, ObjectT);

    fn map(address_space: ASpaceT);
    fn find(address: AddressT) -> Option<MappedObjectT>;
}

pub struct DenseStaticMemory<AddressT, Size: usize> {
    data: [u8; Size]
}
impl<Address, Size> AddressSpace for DenseStaticMemory<Address, Size> {
    fn size(&self) -> usize {
        self.data.len()
    }

    fn get_span(&self, start: Address, end: Address) -> Result<Self, MemoryError> {
        DenseStaticMemory { data: &self.data[start..end] }
    }

    fn get_bytes(&self, addr: _, amount: usize) -> Result<&[u8], MemoryError> {
        
    }
}