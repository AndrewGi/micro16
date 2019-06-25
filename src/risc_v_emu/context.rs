use super::regs;
use super::csr;
use super::types::DataType;
use crate::microvm::memory::address::AddressType;

pub struct Context<RegType: DataType, Address: AddressType> {
    regs: regs::Regs<RegType>,
    csr: csr::CSR,
    a: Address
}