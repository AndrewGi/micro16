pub trait AddressType: Copy + ::num::Unsigned + Into<usize> + Clone + Ord + Sized + PartialEq + PartialOrd +
::num::traits::FromPrimitive + ::num::traits::CheckedAdd + ::num::traits::CheckedSub {}
impl AddressType for u16 {}

