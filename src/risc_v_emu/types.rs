
pub trait DataType: Sized + Copy {
    type Signed: num::traits::Signed + Sized + Copy;
    type Unsigned: num::traits::Unsigned + Sized + Copy;
    fn byte_len(self) -> usize {
        std::mem::size_of::<Self>()
    }
    fn signed(self) -> Self::Signed;
    fn unsigned(self) -> Self::Unsigned;
    fn store_signed(&mut self, i: Self::Signed);
    fn store_unsigned(&mut self, i: Self::Unsigned);
}

macro_rules! data_type_impl {
    ($(($name:ident, $unsigned:ty, $signed:ty)),*) => ($(
        #[derive(Copy, Clone)]
        pub struct $name($unsigned);
        impl DataType for $name {
            type Signed = $signed;
            type Unsigned = $unsigned;
            fn signed(self) -> Self::Signed {
                unsafe {
                    std::mem::transmute::<Self::Unsigned, Self::Signed>(self.0)
                }
            }
            fn unsigned(self) -> Self::Unsigned {
                self.0
            }
            fn store_signed(&mut self, i: Self::Signed) {
                self.0 = std::mem::transmute::<Self::Signed, Self::Unsigned>(i);
            }
            fn store_unsigned(&mut self, i: Self::Unsigned) {
                self.0 = i;
            }
        }
    )*)
}
data_type_impl!(
    (HalfWord, u8, i8),
    (Word, u16, i16),
    (DoubleWord, u32, i32),
    (QuadWord, u64, i64),
    (OctoWord, u128, i128)
);
