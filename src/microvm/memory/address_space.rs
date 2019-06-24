use std;
use std::ops::{Range, Deref, DerefMut, Add};
use crate::microvm::memory::MemoryError;
use crate::microvm::memory::address::*;
use crate::microvm::vm::VMError;
use std::cmp::Ordering;

pub trait AddressSpace<Address: AddressType>{
    fn size(&self) -> Address;
    fn read_byte(&self, address: Address) -> Result<u8, MemoryError>;
    fn write_bytes(&mut self, addr: Address, _bytes: &[u8]) -> Result<(), MemoryError> {
        Err(MemoryError::ReadOnly)
    }
    fn write_byte(&mut self, addr: Address, byte: u8) -> Result<(), MemoryError> {
        self.write_bytes(addr, &[byte])
    }
    fn address_in_space(&self, address: Address) -> bool {
        address < self.size()
    }
}
/*
pub struct MemoryView<'a, Address: AddressType> {
    range: Range<Address>,
    parent: &'a dyn AddressSpace<Address>
}
pub struct MemoryViewMut<'a, Address: AddressType> {
    range: Range<Address>,
    parent: &'a mut dyn AddressSpace<Address>
}
impl<'a, Address: AddressType> AddressSpace<Address> for MemoryView<'a, Address> {
    fn size(&self) -> Address {
        (self.range.end - self.range.start)
    }
    fn read_byte(&self, address: Address) -> Result<u8, MemoryError> {
        self.parent.read_byte(self.range.start + address)
    }
    fn address_in_space(&self, address: Address) -> bool {
        self.range.contains(&address)
    }
}

impl<'a, Address: AddressType> AddressSpace<Address> for MemoryViewMut<'a, Address> {
    fn size(&self) -> Address {
        (self.range.end.clone() - self.range.start)
    }

    fn read_byte(&self, address: Address) -> Result<u8, MemoryError> {
        if &self.range.start.checked_add(&range.end).ok_or(MemoryError::Overflow)? > &self.range.end {
            Err(MemoryError::OutOfBounds)
        } else {
            self.parent.read_bytes(Range {end: self.range.start+range.end.clone(), ..range })
        }
    }
    fn write_bytes(&mut self, addr: Address, bytes: &[u8]) -> Result<(), MemoryError> {
        let end = addr.clone() + self.range.start+Address::from_usize(bytes.len()).ok_or(MemoryError::Overflow)?;
        if &end > &self.range.end {
            Err(MemoryError::OutOfBounds)
        } else {
            self.parent.write_bytes(self.range.start+addr.clone(), bytes)
        }
    }
}*/

pub struct DenseStaticMemory {
    data: Vec<u8>
}
impl DenseStaticMemory {
    pub fn new<Address: AddressType>(size: Address) -> DenseStaticMemory {
        DenseStaticMemory { data: vec![0; size.into()] }
    }
    pub fn size<Address: AddressType>(&self) -> Address {
        Address::from_usize(self.data.len()).unwrap()
    }
    pub fn as_slice(&self) -> &[u8] {
        self.data.as_slice()
    }
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        self.data.as_mut_slice()
    }
}
impl<'a, Address: AddressType> AddressSpace<Address> for DenseStaticMemory {
    fn size(&self) -> Address {
        self.size()
    }

    fn read_byte(&self, address: Address) -> Result<u8, MemoryError> {
        if self.address_in_space(address) {
            Ok(self.data[address.into()])
        } else {
            Err(MemoryError::OutOfBounds)
        }
    }
    fn write_bytes(&mut self, address: Address, bytes: &[u8]) -> Result<(), MemoryError> {
        if Address::from_usize(bytes.len()).ok_or(MemoryError::Overflow)? < self.size() {
            Ok(self.data[address.into()..bytes.len()+address.into()].clone_from_slice(bytes))
        } else {
            Err(MemoryError::OutOfBounds)
        }
    }
}