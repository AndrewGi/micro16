use std;
use crate::memory::MemoryError::OutOfBounds;
use std::ops::Range;
use std::slice::{Iter, IterMut};

pub enum MemoryError {
    OutOfBounds,
    InvalidAccess,
    ReadOnly,
}
trait AddressType: Clone + Copy + Into<usize> + From<usize> + std::ops::Sub<Output=Self> + std::ops::Add<Output=Self> + PartialEq + PartialOrd {}
impl AddressType for u16 {}
pub trait AddressSpace<Address: AddressType> {
    fn size(&self) -> usize;
    fn get_view(&self, range: Range<Address>) -> Result<MemoryView<'_, Address>, MemoryError> where Self: Sized {
        if range.end.into() > self.size() {
            Err(MemoryError::OutOfBounds)
        } else {
            Ok(MemoryView{range, parent: self})
        }
    }
    fn read_bytes(&self, range: Range<Address>) -> Result<&[u8], MemoryError>;
    fn read_byte(&self, address: Address) -> Result<u8, MemoryError> {
        Ok(self.read_bytes(Range{start: address, end: Address::from(address.into()+1)})?[0])
    }
}
pub trait AddressSpaceMut<Address: AddressType>: AddressSpace<Address> {

    fn write_bytes(&mut self, addr: Address, bytes: &[u8]) -> Result<(), MemoryError>;
    fn write_byte(&mut self, addr: Address, byte: u8) -> Result<(), MemoryError> {
        self.write_bytes(addr, &[byte])
    }
    fn get_mut_view(&mut self, range: Range<Address>) -> Result<MemoryViewMut<Address>, MemoryError>  where Self: Sized {
        if range.end.into() > self.size() {
            Ok(MemoryViewMut{range, parent: self})
        } else {
            Err(MemoryError::OutOfBounds)
        }
    }
}
pub struct MemoryView<'a, Address: AddressType> {
    range: Range<Address>,
    parent: &'a dyn AddressSpace<Address>
}
pub struct MemoryViewMut<'a, Address: AddressType> {
    range: Range<Address>,
    parent: &'a mut AddressSpaceMut<Address>
}
impl<'a, Address: AddressType> AddressSpace<Address> for MemoryView<'a, Address> {
    fn size(&self) -> usize {
        (self.range.end - self.range.start).into()
    }

    fn read_bytes(&self, range: Range<Address>) -> Result<&[u8], MemoryError> {
        if self.range.start+range.end > self.range.end {
            Err(MemoryError::OutOfBounds)
        } else {
            self.parent.read_bytes(Range {end: self.range.start+range.end, ..range })
        }
    }
}
impl<'a, Address: AddressType> AddressSpace<Address> for MemoryViewMut<'a, Address> {
    fn size(&self) -> usize {
        (self.range.end - self.range.start).into()
    }

    fn read_bytes(&self, range: Range<Address>) -> Result<&[u8], MemoryError> {
        if self.range.start+range.end > self.range.end {
            Err(MemoryError::OutOfBounds)
        } else {
            self.parent.read_bytes(Range {end: self.range.start+range.end, ..range })
        }
    }
}
impl<'a, Address: AddressType> AddressSpaceMut<Address> for MemoryViewMut<'a, Address> {

    fn write_bytes(&mut self, addr: Address, bytes: &[u8]) -> Result<(), MemoryError> {
        if self.range.start.into()+bytes.len() > self.range.end.into() {
            Err(MemoryError::OutOfBounds)
        } else {
            self.parent.write_bytes(self.range.start+addr, bytes)
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
impl<'a, Address: AddressType> AddressSpace<Address> for DenseStaticMemory {
    fn size(&self) -> usize {
        self.size()
    }

    fn read_bytes(&self, range: Range<Address>) -> Result<&[u8], MemoryError> {
        if range.end.into() < self.size() {
            Ok(&self.data[Range{start: range.start.into(), end:range.end.into()}])
        } else {
            Err(MemoryError::OutOfBounds)
        }
    }
}
impl<'a, Address: AddressType> AddressSpaceMut<Address> for DenseStaticMemory {
    fn write_bytes(&mut self, address: Address, bytes: &[u8]) -> Result<(), MemoryError> {
        if address.into() + bytes.len() < self.size() {
            Ok(self.data[address.into()..bytes.len()+address.into()].clone_from_slice(bytes))
        } else {
            Err(MemoryError::OutOfBounds)
        }
    }
}
pub struct SparseAddressSpace< Address: AddressType> {
    spaces: Vec<(Address, Box<dyn AddressSpace<Address>>)>,
    size: usize,
}
struct OffsetAddressSpace<'a, Address: AddressType> {
    offset: Address,
    space: &'a AddressSpace<Address>
}
struct OffsetAddressSpaceMut<'a, Address: AddressType> {
    offset: Address,
    space: &'a mut AddressSpace<Address>
}

impl<'a, Address> SparseAddressSpace<Address> {
    pub fn spaces_iter(&self) -> impl Iterator<Item=OffsetAddressSpace<Address>> {
        self.spaces.iter().map(|space| {
            OffsetAddressSpace { offset: space.0, space: (space.1).as_ref()}
        })
    }
    pub fn spaces_iter_mut(&mut self) -> impl Iterator<Item = OffsetAddressSpaceMut<Address>> {
        self.spaces.iter_mut().map(|space| {
            OffsetAddressSpaceMut { offset: space.0, space: (space.1).as_mut()}
        })
    }
    pub fn find_space(&self, containing_address: Address) -> Option<&OffsetAddressSpace<Address>> {
        for space in self.spaces_iter() {
            if (space.offset.into()..space.offset.into()+space.space.size()).contains(containing_address) {
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
impl<'a, Address: AddressType> AddressSpace<Address> for SparseAddressSpace<Address> {
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
}
impl<'a, Address: AddressType> AddressSpaceMut<Address> for SparseAddressSpace<Address> {

    fn write_bytes(&mut self, address: Address, bytes: &[u8]) -> Result<(), MemoryError> {
        let mut space =  self.find_space_mut(address).ok_or(MemoryError::InvalidAccess)?;
        if &address + bytes.len() > &space.offset + space.space.size() {
            Err(MemoryError::InvalidAccess)
        } else {
            space.space.write_bytes(address-&space.offset, bytes)
        }
    }
}
