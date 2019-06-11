use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Shl, Sub};
pub fn make_mask<OutT>(size: usize) -> OutT where OutT: num::traits::One + Shl<usize, Output=OutT> + Sub<OutT, Output=OutT> {
    assert!(std::mem::size_of::<OutT>()*8 > size);
    (OutT::one() << size) - OutT::one()
}
trait ShiftOps : std::ops::Shl + std::ops::ShlAssign + std::ops::Shr + std::ops::ShrAssign + Sized {

}
trait BitOps : std::ops::Not + BitAnd + BitAndAssign + BitOr + BitOrAssign + BitXor + BitXorAssign + Sized {

}
trait Bits<'a>: Clone + BitOps + ShiftOps {
    fn byte_len(&self) -> usize;
    fn bit_len(&self) -> usize {
        self.byte_len()*8
    }
    unsafe fn as_mut_bytes(&mut self) -> *mut u8;
    unsafe fn as_bytes(&self) -> *const u8;
    fn set_bit(&mut self, bit: usize);
    fn clear_bit(&mut self, bit: usize);
    fn get_bit(&self, bit: usize) -> bool;
}
macro_rules! bit_impl {
    ($($t:ty),*) => ($(
        impl BitOps for $t {

        }
        impl ShiftOps for $t {

        }
        impl<'a> Bits<'a> for $t {
            fn byte_len(&self) -> usize {
                std::mem::size_of::<$t>()
            }
            unsafe fn as_mut_bytes(&mut self) -> *mut u8 {
                std::mem::transmute::<*mut $t, *mut u8>(self)
            }
            unsafe fn as_bytes(&self) -> *const u8 {
                std::mem::transmute::<*const $t, *const u8>(self)
            }
            fn set_bit(&mut self, bit: usize) {
                *self |= (1 << bit) //todo: fix platform byte ordering maybe
            }
            fn clear_bit(&mut self, bit: usize) {
                *self &= !(1 << bit) //todo: fix platform byte ordering maybe
            }
            fn get_bit(&self, bit: usize) -> bool {
                ((self >> bit) & 1) == 1
            }
        }
    )*)
}
bit_impl!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);
pub struct BitVector {
    data: Vec<u8>,
    bit_length: usize,
}
pub struct BitScanner<'a> {
    bytes: &'a [u8],
    bit_position: usize,
}
impl<'a> Iterator for BitScanner<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        let b = self.peek()?;
        self.bit_position+=1;
        Some(b)
    }
}
impl<'a> BitScanner<'a> {
    pub fn new(bytes: &[u8]) -> BitScanner {
        BitScanner {bytes, bit_position: 0}
    }
    pub fn peek(&self) -> Option<bool> {
        if self.pos() >= self.len() {
            None
        } else {
            Some(((self.bytes[self.byte_pos()] << self.current_pos_in_byte()) & 1) == 1)
        }
    }
    pub fn byte_pos(&self) -> usize {
        self.bit_position / 8
    }
    pub fn pos(&self) -> usize {
        self.bit_position
    }
    pub fn len(&self) -> usize {
        self.bytes_len() * 8
    }
    pub fn bytes_len(&self) -> usize {
        self.bytes.len()
    }
    pub fn bits_left(&self) -> usize {
        self.len() - self.pos()
    }
    pub fn atleast_n_bits_left(&self, n: usize) -> bool {
        self.bits_left() >= n
    }
    fn current_pos_in_byte(&self) -> usize {
        self.pos() % 8
    }
    fn is_aligned(&self) -> bool {
        self.current_pos_in_byte() == 0
    }
    fn next_byte_aligned(&mut self) -> Option<u8> {
        if !self.is_aligned() && !self.atleast_n_bits_left(8) {
            None
        } else {
            let b = self.bytes[self.byte_pos()];
            self.bit_position += 8;
            Some(b)
        }
    }
}