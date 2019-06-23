use super::memory;
use crate::microvm::memory::address::AddressType;


pub struct MMU<Address: AddressType> {
    space: memory::sparse::SparseAddressSpace<Address>
}