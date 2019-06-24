
use std::convert::TryInto;
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PSR(u8); //Processor Status Register

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MCR(u8); //Mode Control Register

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IFR(u8); //Interrupt Flag Register

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IER(u8); //Interrupt Enable Register

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PSRFlag {
    Carry = 0,
    Zero = 1,
    InterruptDisable = 2,
    Decimal = 3,
    Overflow = 6,
    Negative = 7
}
impl Into<u8> for PSRFlag {
    fn into(self) -> u8 {
        self as u8
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum MCRFlag {
    Carry = 0,
    Zero = 1,
    InterruptDisable = 2,
    Decimal = 3,
    Overflow = 6,
    Negative = 7
}
pub trait FlagRegister<T>: Sized where
 T: num::traits::Unsigned + std::ops::Shl<u8, Output=T> + std::ops::Shr<u8, Output=T> + std::ops::Not<Output=T>
 + std::ops::BitAnd<Output=T> + num::traits::identities::One + std::ops::BitAndAssign<T> + std::ops::BitOrAssign<T>
{
    type FlagType: Into<u8>;
    fn value(&self) -> T;
    fn value_mut(&mut self) -> &mut T;
    fn get_bit(&self, position: u8) -> bool {
        assert!((position as usize) < std::mem::size_of::<T>()*8);
        ((self.value() >> position) & T::one()) == T::one()
    }
    fn set_bit(&mut self, position: u8) {
        assert!((position as usize) < std::mem::size_of::<T>()*8);
        *self.value_mut() |= (T::one() << position);
    }
    fn clear_bit(&mut self, position: u8) {
        assert!((position as usize) < std::mem::size_of::<T>()*8);
        *self.value_mut() &= !(T::one() << position);
    }
    fn set(&mut self, which: Self::FlagType) {
        self.set_bit(which.into())
    }
    fn clear(&mut self, which: Self::FlagType) {
        self.clear_bit(which.into())
    }
    fn get(&mut self, which: Self::FlagType) -> bool {
        self.get_bit(which.into())
    }
}
impl FlagRegister<u8> for PSR {
    type FlagType = PSRFlag;
    fn value(&self) -> u8 {
        self.0
    }
    fn value_mut(&mut self) -> &mut u8 {
        &mut self.0
    }
}