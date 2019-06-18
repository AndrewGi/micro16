use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Shl, Sub};
use std::convert::TryInto;

pub fn make_mask<OutT>(size: usize) -> OutT where OutT: num::traits::One + Shl<usize, Output=OutT> + Sub<OutT, Output=OutT> {
    assert!(std::mem::size_of::<OutT>()*8 > size);
    (OutT::one() << size) - OutT::one()
}
pub trait ShiftOps : std::ops::Shl + std::ops::ShlAssign + std::ops::Shr + std::ops::ShrAssign + Sized {

}
pub trait BitOps : std::ops::Not + BitAnd + BitAndAssign + BitOr + BitOrAssign + BitXor + BitXorAssign + Sized {

}
pub trait Bits<'a>: Clone + BitOps + ShiftOps + Default {
    fn byte_len(&self) -> usize;
    fn bit_len(&self) -> usize {
        self.byte_len()*8
    }
    unsafe fn as_mut_bytes(&mut self) -> *mut u8;
    unsafe fn as_bytes(&self) -> *const u8;
    fn as_slice(&self) -> &'a [u8] {
        unsafe {
            std::slice::from_raw_parts(self.as_bytes(), self.byte_len())
        }
    }
    fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe {
            std::slice::from_raw_parts_mut(self.as_mut_bytes(), self.byte_len())
        }
    }
    fn set_bit(&mut self, bit: usize);
    fn clear_bit(&mut self, bit: usize);
    fn get_bit(&self, bit: usize) -> bool;
}
pub enum Endianness {
    Little,
    Big
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
bit_impl!(u8, u16, u32, u64, u128);
pub struct BitVector {
    data: Vec<u8>,
    bit_length: usize,
}

pub struct BitScanner<'a> {
    flip_bits: bool,
    bytes: &'a [u8],
    bit_position: usize,
}
impl<'a> Iterator for BitScanner<'a> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        let b = self.peek_bit()?;
        self.bit_position+=1;
        Some(b)
    }
}
fn reverse_bits(b: u8) -> u8 {
    let b = (b & 0xF0) >> 4 | (b & 0x0F) << 4;
    let b = (b & 0xCC) >> 2 | (b & 0x33) << 2;
    let b = (b & 0xAA) >> 1 | (b & 0x55) << 1;
    b
}
impl<'a> BitScanner<'a> {
    pub fn new(bytes: &[u8]) -> BitScanner {
        BitScanner {flip_bits: true, bytes, bit_position: 0}
    }
    pub fn is_done(&self) -> bool {
        self.len() == self.pos()
    }
    pub fn peek_bit(&self) -> Option<bool> {
        if self.pos() >= self.len() {
            None
        } else {
            Some(((self.current_byte()? << self.current_pos_in_byte()) & 1) == 1)
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
    fn current_pos_in_byte(&self) -> u8 {
        (self.pos() % 8).try_into().unwrap()
    }
    fn current_byte(&self) -> Option<u8> {
        if !self.atleast_n_bits_left(8) {
            None
        } else {
            let mut b = self.bytes[self.byte_pos()];
            Some(b)
        }
    }
    fn is_aligned(&self) -> bool {
        self.current_pos_in_byte() == 0
    }
    fn next_byte_aligned(&mut self) -> Option<u8> {
        if !self.is_aligned() && !self.atleast_n_bits_left(8) {
            None
        } else {
            let b = self.current_byte()?;
            self.bit_position += 8;
            Some(b)
        }
    }
    pub fn next_byte(&mut self) -> Option<u8> {
        if !self.atleast_n_bits_left(8) {
            None
        } else if self.is_aligned() {
            self.next_byte_aligned()
        } else {
            let bit_pos = self.current_pos_in_byte();
            let mut out: u8 = self.current_byte()? >> bit_pos;
            self.bit_position += (8-bit_pos) as usize;
            out |= self.current_byte()? << bit_pos;
            self.bit_position += bit_pos as usize;
            Some(out)
        }
    }
    pub fn is_byte_aligned(&self, byte_alignment: u8) -> bool {
        (self.bit_position % (8 << byte_alignment) as usize) == 0
    }
    fn next_sub_byte_aligned(&mut self, amount: u8) -> Option<u8> {
        if amount >= 8 || !self.is_aligned() || !self.atleast_n_bits_left(amount.into()) {
            None
        } else {
            let out: u8 = self.current_byte()? & make_mask::<u8>(amount.into());
            self.bit_position += amount as usize;
            Some(out)
        }
    }
    fn consume_bits_left_in_current_byte(&mut self) -> u8 {
        let bit_pos = self.current_pos_in_byte();
        let out = self.current_byte().unwrap_or(0) >> bit_pos;
        self.bit_position += bit_pos as usize;
        out
    }
    fn next_sub_byte(&mut self, amount: u8) -> Option<u8> {
        if amount == 0 ||amount >= 8 || !self.atleast_n_bits_left(amount.into()) {
            None
        } else if self.is_aligned() {
            self.next_sub_byte_aligned(amount)
        } else {
            let rest_count = (8-self.current_pos_in_byte());
            let mut out = self.consume_bits_left_in_current_byte();
            out |= self.next_sub_byte_aligned(amount-rest_count)? << rest_count;
            Some(out)
        }
    }
    pub fn collect_bits<OutT>(&mut self, amount: usize) -> Option<OutT> where OutT: for<'b> Bits<'b> + From<u8> + Shl<usize, Output=OutT> {
        assert!(std::mem::size_of::<OutT>() <= amount, "trying to collect more bits than the type can contain");
        if !self.atleast_n_bits_left(amount)  {
            None
        } else if (8-self.current_pos_in_byte()) as usize >= amount {
            Some(self.next_sub_byte(amount.try_into().expect("amount should be 8> by now")).expect("should be subbyte").into())
        } else {
            let start_pos = self.pos();
            let mut out: OutT = self.consume_bits_left_in_current_byte().into();
            let mut out_pos = self.pos()-start_pos;
            let mut out_left = amount-out_pos;
            while out_left>=8 {
                let next_byte = self.next_byte().unwrap();
                out |= OutT::from(next_byte) << out_pos;
                out_pos += 8;
                out_left -= 8;
            }
            if out_left > 0 {
                //sub byte left
                out |= OutT::from(self.next_sub_byte(out_left.try_into().expect("amount should be 8> by now")).unwrap()) << out_pos;
            }
            Some(out)
        }
    }
    pub fn collect_type<T>(&mut self) -> Option<T> where T: for<'b> Bits<'b> + From<u8> + Shl<usize, Output=T> {
        self.collect_bits(std::mem::size_of::<T>()*8)
    }
}
impl<'a, T> From<&'a T> for BitScanner<'a> where T: Bits<'a> {
    fn from(bits: &T) -> Self {
        BitScanner::new(bits.as_slice())
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let x = 0x7F7Fu16;
        let mut scanner: BitScanner = (&x).into();
        assert_eq!(scanner.len(), 16);
        assert_eq!(scanner.bits_left(), 16);
        assert!(scanner.atleast_n_bits_left(16));
        assert_eq!(scanner.collect_type::<u16>().expect("expected u16"), x);
        assert!(scanner.is_done());
    }
    #[test]
    fn test2() {
        let x = 0xACACu16;
        let mut scanner: BitScanner = (&x).into();
        assert_eq!(scanner.len(), 16);
        assert_eq!(scanner.bits_left(), 16);
        assert!(scanner.atleast_n_bits_left(16));
        assert_eq!(scanner.collect_bits::<u8>(4).unwrap(), 0xCu8);
        assert_eq!(scanner.collect_bits::<u8>(4).unwrap(), 0xAu8);
        assert!(!scanner.is_done());
    }

}