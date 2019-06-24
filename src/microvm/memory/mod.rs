
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MemoryError {
    OutOfBounds,
    InvalidAccess,
    ReadOnly,
    Overflow,
    Underflow,
    Overlap,
}
pub mod address;
pub mod address_space;
pub mod sparse;
pub mod zero;
pub mod rom;