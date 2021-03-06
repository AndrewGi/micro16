use std::ops::{Range, Deref, DerefMut};
use std::cmp::Ordering;
use crate::microvm::memory::MemoryError;
use crate::microvm::memory::address::*;
use crate::microvm::memory::address_space::AddressSpace;
pub struct SparseAddressSpace<Address: AddressType> {
    spaces: Vec<OffsetAddressSpace<Address, dyn AddressSpace<Address>, Box<dyn AddressSpace<Address>>>>,
    size: Address,
}
pub struct OffsetAddressSpace<Address, Space, SpaceStorage> where
    Address: AddressType,
    Space: AddressSpace<Address> + ?Sized,
    SpaceStorage: Deref<Target=Space> {
    offset: Address,
    space: SpaceStorage
}
pub struct OffsetAddressSpaceMut<Address, Space, SpaceStorage> where
    Address: AddressType,
    Space: AddressSpace<Address> + ?Sized,
    SpaceStorage: Deref<Target=Space> + DerefMut {
    offset: Address,
    space: SpaceStorage
}
impl<'a, Address, Space, SpaceStorage> OffsetAddressSpaceMut<Address, Space, SpaceStorage> where
    Address: AddressType,
    Space: AddressSpace<Address> + ?Sized,
    SpaceStorage: Deref<Target=Space> + DerefMut {
    pub fn as_ref(&self) -> OffsetAddressSpace<Address, Space, &Space> {
        OffsetAddressSpace { offset: self.offset, space: &self.space.deref() }
    }
}
impl<Address, Space, SpaceStorage> OffsetAddressSpace<Address, Space, SpaceStorage> where
    Address: AddressType,
    Space: AddressSpace<Address> + ?Sized,
    SpaceStorage: Deref<Target=Space> {
    pub fn address_range(&self) -> Range<Address> {
        Range { start: self.offset, end: self.space.size()+self.offset }
    }
    pub fn sub_offset(&self, range: Range<Address>) -> Result<Range<Address>, MemoryError> {
        Ok(Range { start: range.start.checked_sub(&self.offset).ok_or(MemoryError::Underflow)?, end: range.end-self.offset })
    }
    pub fn relative_range(&self, range: Range<Address>) -> Result<Range<Address>, MemoryError> {
        match self.sub_offset(range) {
            Ok(r) => Ok(r),
            Err(e) => {
                if e == MemoryError::Underflow {
                    Err(MemoryError::OutOfBounds)
                } else {
                    Err(e)
                }
            },
        }
    }
    pub fn does_overlap<OSpaceStorage: Deref<Target=Space>>(&self, other: &OffsetAddressSpace<Address, Space, OSpaceStorage>) -> bool {
        let r1 = self.address_range();
        let r2 = other.address_range();
        r1.contains(&r2.start) || r1.contains(&r2.end) || r2.contains(&r1.start) || r2.contains(&r1.end)
    }
}
impl<Address, Space, SpaceStorage> AddressSpace<Address> for OffsetAddressSpaceMut<Address, Space, SpaceStorage> where
    Address: AddressType,
    Space: AddressSpace<Address> + ?Sized,
    SpaceStorage: Deref<Target=Space> + DerefMut {
    fn size(&self) -> Address {
        self.space.deref().size()
    }
    fn read_byte(&self, address: Address) -> Result<u8, MemoryError> {
        self.space.read_byte(self.offset.checked_add(&address).ok_or(MemoryError::Overflow)?)
    }
    fn write_bytes(&mut self, address: Address, bytes: &[u8]) -> Result<(), MemoryError> {
        self.space.deref_mut().write_bytes(address.checked_sub(&self.offset).ok_or(MemoryError::OutOfBounds)?, bytes)
    }
    fn address_in_space(&self, address: Address) -> bool {
        self.as_ref().address_range().contains(&address)
    }
}
impl<Address, Space, SpaceStorage> AddressSpace<Address> for OffsetAddressSpace<Address, Space, SpaceStorage> where
    Address: AddressType,
    Space: AddressSpace<Address> + ?Sized,
    SpaceStorage: Deref<Target=Space> {
    fn size(&self) -> Address {
        self.space.deref().size()
    }
    fn read_byte(&self, address: Address) -> Result<u8, MemoryError> {
        self.space.read_byte(self.offset.checked_add(&address).ok_or(MemoryError::Overflow)?)
    }
    fn address_in_space(&self, address: Address) -> bool {
        self.address_range().contains(&address)
    }
}
impl<'a, Address: AddressType> SparseAddressSpace< Address> {
    pub fn new(size: Address) -> SparseAddressSpace< Address> {
        SparseAddressSpace {
            spaces: Vec::with_capacity(4),
            size
        }
    }
    pub fn add_space(&mut self, offset: Address, new_space: Box<dyn AddressSpace<Address>>) -> Result<(), MemoryError>  {
        if offset+new_space.as_ref().size() > self.size {
            return Err(MemoryError::Overflow)
        }
        let new_offset_space = OffsetAddressSpace {
            offset: offset,
            space: new_space
        };
        for space in self.spaces.iter() {
            if space.does_overlap(&new_offset_space) {
                return Err(MemoryError::Overlap)
            }
        }
        let position = self.find_space_position(offset).err().ok_or(MemoryError::Overlap)?;
        self.spaces.insert(position, new_offset_space);
        Ok(())
    }
    fn find_space_position(&self, containing_address: Address) -> Result<usize, usize> {
        self.spaces.binary_search_by(|space| {
            if space.offset > containing_address {
                Ordering::Greater
            } else if space.address_in_space(containing_address) {
                Ordering::Equal
            } else {
                Ordering::Less
            }
        })
    }
    pub fn find_space(&'a self, containing_address: Address) -> Option<OffsetAddressSpace<Address, dyn AddressSpace<Address>+'a, &'a dyn AddressSpace<Address>>> {
        let i = self.find_space_position(containing_address).ok()?;
        let oa = self.spaces.get(i)?;
        let out = OffsetAddressSpace { offset: oa.offset, space: oa.space.deref() };
        return Some(out);

    }

    pub fn find_space_mut(&'a mut self, containing_address: Address) -> Option<OffsetAddressSpace<Address, dyn AddressSpace<Address>+'a, &'a mut dyn AddressSpace<Address>>> {
        let i = self.find_space_position(containing_address).ok()?;
        let oa = self.spaces.get_mut(i)?;
        let space = oa.space.deref_mut();
        //FIXME:
        // work around for https://github.com/rust-lang/rust/issues/53613
        let fixxed_space: &mut dyn AddressSpace<Address> = unsafe {
            std::mem::transmute(space)
        };

        Some(OffsetAddressSpace { offset: oa.offset, space: fixxed_space })

    }
}
impl< Address: AddressType> AddressSpace<Address> for SparseAddressSpace< Address> {
    fn size(&self) -> Address {
        self.size.clone()
    }

    fn read_byte(&self, address: Address) -> Result<u8, MemoryError> {
        self.find_space(address).ok_or(MemoryError::InvalidAccess)?.read_byte(address)
    }
    fn write_bytes(&mut self, address: Address, bytes: &[u8]) -> Result<(), MemoryError> {
        let space =  self.find_space_mut(address).ok_or(MemoryError::InvalidAccess)?;
        let start = address - space.offset;
        let end = start+Address::from_usize(bytes.len()).ok_or(MemoryError::Overflow)?;
        if end > space.offset.checked_add(&space.space.size()).ok_or(MemoryError::Overflow)? {
            Err(MemoryError::InvalidAccess)
        } else {
            space.space.write_bytes(start, bytes)
        }
    }
}
