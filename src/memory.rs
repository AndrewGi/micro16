use std;
use std::ops::{Range, Add};
use std::any::Any;
use crate::instructions::RegsCode::SP;

pub enum MemoryError {
    OutOfBounds,
    InvalidAccess,
    ReadOnly,
    Overflow,
    Overlap,
}
pub trait AddressType: ::num::Unsigned + Into<usize> + Clone + Ord + Sized +
                ::num::traits::FromPrimitive + ::num::traits::CheckedAdd + ::num::traits::CheckedSub {}
impl AddressType for u16 {}
pub trait AddressSpace<Address: AddressType>{
    fn size(&self) -> Address;
    fn get_view<'a>(&'a self, range: Range<Address>) -> Result<MemoryView<'a, Address>, MemoryError> where Self: Sized {
        get_memory_view(self, range)
    }
    fn read_bytes(&self, range: Range<Address>) -> Result<&[u8], MemoryError>;
    fn read_byte(&self, address: Address) -> Result<u8, MemoryError> {
        Ok(self.read_bytes(Range{start: address.clone(), end: address + Address::one()})?[0])
    }
    fn write_bytes(&mut self, addr: Address, _bytes: &[u8]) -> Result<(), MemoryError> {
        Err(MemoryError::ReadOnly)
    }
    fn write_byte(&mut self, addr: Address, byte: u8) -> Result<(), MemoryError> {
        self.write_bytes(addr, &[byte])
    }
    fn get_mut_view<'a>(&'a mut self, range: Range<Address>) -> Result<MemoryViewMut<'a, Address>, MemoryError>  where Self: Sized {
        if range.end > self.size() {
            Ok(MemoryViewMut{range, parent: self as &'a mut dyn AddressSpace<Address>})
        } else {
            Err(MemoryError::OutOfBounds)
        }
    }
}
fn get_memory_view<'a, Address: AddressType>(space: &'a dyn AddressSpace<Address>, range: Range<Address>) -> Result<MemoryView<'a, Address>, MemoryError>{
    if range.end > space.size() {
        Err(MemoryError::OutOfBounds)
    } else {
        Ok(MemoryView{range, parent: space as &'a dyn AddressSpace<Address>})
    }
}
pub struct MemoryView<'a, Address: AddressType> {
    range: Range<Address>,
    parent: &'a (dyn AddressSpace<Address> + 'a)
}
pub struct MemoryViewMut<'a, Address: AddressType> {
    range: Range<Address>,
    parent: &'a mut (dyn AddressSpace<Address> + 'a)
}
impl<'a, Address: AddressType> AddressSpace<Address> for MemoryView<'a, Address> {
    fn size(&self) -> Address {
        (self.range.end.clone() - self.range.start.clone())
    }

    fn read_bytes(&self, range: Range<Address>) -> Result<&[u8], MemoryError> {
        if self.range.start.clone()+range.end.clone() > self.range.end {
            Err(MemoryError::OutOfBounds)
        } else {
            self.parent.read_bytes(Range {end: self.range.start.clone()+range.end.clone(), ..range })
        }
    }
}
impl<'a, Address: AddressType> AddressSpace<Address> for MemoryViewMut<'a, Address> {
    fn size(&self) -> Address {
        (self.range.end.clone() - self.range.start.clone())
    }

    fn read_bytes(&self, range: Range<Address>) -> Result<&[u8], MemoryError> {
        if &self.range.start.checked_add(&range.end).ok_or(MemoryError::Overflow)? > &self.range.end {
            Err(MemoryError::OutOfBounds)
        } else {
            self.parent.read_bytes(Range {end: self.range.start.clone()+range.end.clone(), ..range })
        }
    }
    fn write_bytes(&mut self, addr: Address, bytes: &[u8]) -> Result<(), MemoryError> {
        let end = addr.clone() + self.range.start.clone()+Address::from_usize(bytes.len()).ok_or(MemoryError::Overflow)?;
        if &end > &self.range.end {
            Err(MemoryError::OutOfBounds)
        } else {
            self.parent.write_bytes(self.range.start.clone()+addr.clone(), bytes)
        }
    }
}

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

    fn read_bytes(&self, range: Range<Address>) -> Result<&[u8], MemoryError> {
        if range.end < self.size() {
            Ok(&self.data[Range{start: range.start.into(), end:range.end.into()}])
        } else {
            Err(MemoryError::OutOfBounds)
        }
    }
    fn write_bytes(&mut self, address: Address, bytes: &[u8]) -> Result<(), MemoryError> {
        if Address::from_usize(bytes.len()).ok_or(MemoryError::Overflow)? < self.size() {
            Ok(self.data[address.clone().into()..bytes.len()+address.into()].clone_from_slice(bytes))
        } else {
            Err(MemoryError::OutOfBounds)
        }
    }
}
pub struct SparseAddressSpace<Address: AddressType> {
    spaces: Vec<(Address, Box<dyn AddressSpace<Address>>)>,
    size: Address,
}
#[derive(Clone)]
struct OffsetAddressSpace<'a, Address: AddressType> {
    offset: Address,
    space: &'a dyn AddressSpace<Address>
}
struct OffsetAddressSpaceMut<'a, Address: AddressType> {
    offset: Address,
    space: &'a mut dyn AddressSpace<Address>
}
fn does_overlap<Address: AddressType>(first: &dyn AddressSpace<Address>, second: (Address, &dyn AddressSpace<Address>)) -> bool {
    true
}
impl<'a, Address: AddressType> SparseAddressSpace<Address> {
    pub fn new(size: Address) -> SparseAddressSpace<Address> {
        SparseAddressSpace {
            spaces: Vec::with_capacity(4),
            size
        }
    }
    pub fn add_space(&mut self, offset: Address, new_space: Box<dyn AddressSpace<Address>>) -> Result<(), MemoryError>  {
        if offset.clone()+new_space.as_ref().size() > self.size {
            return Err(MemoryError::Overflow)
        }
        for space in self.spaces_iter() {
            if does_overlap(space, new_space) {
                return Err(MemoryError::Overlap)
            }
        }
        self.spaces.push((offset, new_space));
        Ok(())
    }
    pub fn spaces_iter(&self) -> impl Iterator<Item=OffsetAddressSpace<Address>>  {
        self.spaces.iter().map(|space| {
            OffsetAddressSpace { offset: (space.0).clone(), space:  (space.1).as_ref()}
        })
    }
    pub fn spaces_iter_mut(&mut self) -> impl Iterator<Item = OffsetAddressSpaceMut<Address>> {
        self.spaces.iter_mut().map(|space| {
            OffsetAddressSpaceMut { offset: (space.0).clone(), space: (space.1).as_mut()}
        })
    }
    pub fn find_space(&self, containing_address: Address) -> Option<OffsetAddressSpace<Address>> {
        for space in self.spaces_iter() {
            if (space.offset.clone()..space.offset.checked_add(&space.space.size()).unwrap_or(space.offset.clone())).contains(&containing_address) {
                return Some(space);
            }
        }
        None
    }
    pub fn find_space_mut(&mut self, containing_address: Address) -> Option<OffsetAddressSpaceMut<Address>> {
        for mut space in self.spaces_iter_mut() {
            if (space.offset.clone()..space.offset.checked_add(&space.space.size()).unwrap_or(space.offset.clone())).contains(&containing_address) {
                return Some(space);
            }
        }
        None
    }
}
impl<'a, Address: AddressType> AddressSpace<Address> for SparseAddressSpace<Address> {
    fn size(&self) -> Address {
        self.size.clone()
    }

    fn read_bytes(&self, range: Range<Address>) -> Result<&[u8], MemoryError> {
        let space = self.find_space(range.start.clone()).ok_or(MemoryError::InvalidAccess)?;
        if range.end.clone() > space.offset.checked_add(&space.space.size()).ok_or(MemoryError::Overflow)? {
           Err(MemoryError::InvalidAccess)
        } else {
            space.space.read_bytes(range.start.clone()-space.offset.clone()..range.end.clone()-space.offset.clone())
        }
    }
    fn write_bytes(&mut self, address: Address, bytes: &[u8]) -> Result<(), MemoryError> {
        let space =  self.find_space_mut(address.clone()).ok_or(MemoryError::InvalidAccess)?;
        let start = address.clone() - space.offset.clone();
        let end = start.clone()+Address::from_usize(bytes.len()).ok_or(MemoryError::Overflow)?;
        if end > space.offset.checked_add(&space.space.size()).ok_or(MemoryError::Overflow)? {
            Err(MemoryError::InvalidAccess)
        } else {
            space.space.write_bytes(start, bytes)
        }
    }
}
