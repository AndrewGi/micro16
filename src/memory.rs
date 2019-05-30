use std;
use crate::memory::MemoryError::OutOfBounds;
use std::ops::Range;
use std::slice::{Iter, IterMut};

pub enum MemoryError {
    OutOfBounds,
    InvalidAccess,
    ReadOnly,
}
pub trait AddressSpace<'a, Address> {
    fn size(&self) -> usize;
    fn get_view<'b>(&self, range: Range<Address>) -> Result<MemoryView<'b, 'a,Address>, MemoryError> {
        if (range.end as usize) > self.size() {
            Err(MemoryError::OutOfBounds)
        } else {
            Ok(MemoryView{range, parent: &self})
        }
    }
    fn get_mut_view(&mut self, range: Range<Address>) -> Result<MemoryViewMut<Address>, MemoryError> {
        if (range.end as usize) > self.size() {
            Ok(MemoryView{range, parent: &mut self})
        } else {
            Err(MemoryError::OutOfBounds)
        }
    }
    fn read_bytes(&self, range: Range<Address>) -> Result<&[u8], MemoryError>;
    fn read_byte(&self, address: Address) -> Result<u8, MemoryError> {
        Ok(self.read_bytes(Range{start: address, end: address+1})?[0])
    }
    fn write_bytes(&mut self, addr: Address, bytes: &[u8]) -> Result<(), MemoryError> {
        Err(MemoryError::ReadOnly)
    }
    fn write_byte(&mut self, addr: Address, byte: u8) -> Result<(), MemoryError> {
        self.write_bytes(addr, &[byte])
    }
}
pub struct MemoryView<'a, 'b, Address> {
    range: Range<Address>,
    parent: &'a AddressSpace<'b, Address>
}
pub struct MemoryViewMut<'a, Address> {
    range: Range<Address>,
    parent: &'a mut AddressSpace<'a, Address>
}
impl<'a, 'b, Address> AddressSpace<'a, Address> for MemoryView<'a, 'b,  Address> {
    fn size(&self) -> usize {
        &self.range.end - &self.range.start
    }

    fn read_bytes(&self, range: Range<Address>) -> Result<&[u8], MemoryError> {
        if &self.range.start+range.end > &self.range.end {
            Err(MemoryError::OutOfBounds)
        } else {
            self.parent.read_bytes(Range {end: &self.range.start+&range.end, ..range })
        }
    }
}
impl<'a, Address> AddressSpace<'a, Address> for MemoryViewMut<'a, Address> {
    fn size(&self) -> usize {
        &self.range.end - &self.range.start
    }


    fn read_bytes(&self, range: Range<Address>) -> Result<&[u8], MemoryError> {
        if &self.range.start+range.end > &self.range.end {
            Err(MemoryError::OutOfBounds)
        } else {
            self.parent.read_bytes(Range {end: &self.range.start+&range.end, ..range })
        }
    }

    fn write_bytes(&mut self, addr: Address, bytes: &[u8]) -> Result<(), MemoryError> {
        if &self.range.start+bytes.len() > &self.range.end {
            Err(MemoryError::OutOfBounds)
        } else {
            self.parent.write_bytes(&self.range.start+addr, bytes)
        }
    }
}


pub struct DenseStaticMemory {
    data: Vec<u8>
}
impl DenseStaticMemory {
    pub fn new(size: usize) -> DenseStaticMemory {
        DenseStaticMemory { data: vec![0; size] }
    }
    pub fn size(&self) -> usize {
        self.data.len()
    }
    pub fn as_slice(&self) -> &[u8] {
        self.data.as_slice()
    }
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        self.data.as_mut_slice()
    }
}
impl<'a, Address> AddressSpace<'a, Address> for DenseStaticMemory {
    fn size(&self) -> usize {
        self.size()
    }

    fn read_bytes(&self, range: Range<Address>) -> Result<&[u8], MemoryError> {
        if range.end < self.size() {
            Ok(&self.data[range])
        } else {
            Err(MemoryError::OutOfBounds)
        }
    }
    fn write_bytes(&mut self, address: Address, bytes: &[u8]) -> Result<(), MemoryError> {
        if (address as usize) + bytes.len() < self.size() {
            Ok(self.data[address as usize..bytes.len()+address].clone_from_slice(bytes))
        } else {
            Err(MemoryError::OutOfBounds)
        }
    }
}
pub struct SparseAddressSpace<'a, Address> {
    spaces: Vec<(Address, Box<dyn AddressSpace<'a, Address>>)>,
    size: usize,
}
struct OffsetAddressSpace<'a, Address> {
    offset: Address,
    space: &'a AddressSpace<'a, Address>
}
struct OffsetAddressSpaceMut<'a, Address> {
    offset: Address,
    space: &'a mut AddressSpace<'a, Address>
}

impl<'a, Address> SparseAddressSpace<'a, Address> {
    pub fn spaces_iter(&self) -> Iter<OffsetAddressSpace<Address>> {
        self.spaces.iter().map(|space| {
            OffsetAddressSpace { offset: space.0, space: (space.1).as_ref()}
        })
    }
    pub fn spaces_iter_mut(&mut self) -> IterMut<OffsetAddressSpace<Address>> {
        self.spaces.iter_mut().map(|space| {
            OffsetAddressSpaceMut { offset: space.0, space: (space.1).as_mut()}
        })
    }
    pub fn find_space(&self, containing_address: Address) -> Option<&OffsetAddressSpace<Address>> {
        for space in self.spaces_iter() {
            if ((space.0)..(space.0)+(space.1).size()).contains(containing_address) {
                return Some(&space);
            }
        }
        None
    }
    pub fn find_space_mut(&mut self, containing_address: Address) -> Option<&mut OffsetAddressSpaceMut<Address>> {
        for space in self.spaces_iter_mut() {
            if ((space.0)..(space.0)+(space.1).size()).contains(containing_address) {
                return Some(&space);
            }
        }
        None
    }
}
impl<'a, Address> AddressSpace<'a, Address> for SparseAddressSpace<'a, Address> {
    fn size(&self) -> usize {
        self.size
    }

    fn read_bytes(&self, range: Range<Address>) -> Result<&[u8], MemoryError> {
        let space = self.find_space(range.start).ok_or(MemoryError::InvalidAccess)?;
        let start = space.0;
        if range.end > space.0 + (space.1).size() {
           Err(MemoryError::InvalidAccess)
        } else {
            (space.1).read_bytes(range.start-start..range.end-start)
        }
    }
    fn write_bytes(&mut self, address: Address, bytes: &[u8]) -> Result<(), MemoryError> {
        let mut space =  self.find_space_mut(address).ok_or(MemoryError::InvalidAccess)?;
        if &address + bytes.len() > &space.offset + space.space.size() {
            Err(MemoryError::InvalidAccess)
        } else {
            space.space.write_bytes(address-&space.offset, bytes)
        }
    }
}
