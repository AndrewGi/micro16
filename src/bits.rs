
pub const fn make_mask<OutT: Default + std::ops::Add<Output=OutT> + std::ops::Shl<usize, Output=OutT> + std::ops::Sub<Output=OutT> + From<i32>>(size: usize) -> OutT {
    assert!(std::mem::size_of::<OutT>()*8 > size);
    ((OutT::default() + 1.into()) << size) - 1.into()
}
struct BitScanner<'a> {
    bytes: &'a [u8],
    bit_position: usize,
}

impl<'a> BitScanner<'a> {
    pub fn new(bytes: &[u8]) -> BitScanner {
        BitScanner {bytes, bit_position: 0}
    }
    pub fn byte_pos(&self) -> usize {
        self.bit_position / 8
    }
    pub fn pos(&self) -> usize {
        self.bit_position
    }
    pub fn len(&self) -> usize {
        self.bytes.len() * 8
    }
    pub fn collect(&mut self, amount: u8) -> Option<usize> {
        if amount as usize > (std::mem::size_of::<usize>()*8) {
            panic!("too many bits")
        }
        if amount == 0 || self.pos() + amount as usize > self.len() {
            None
        } else {
            let mut out: usize = 0;
            let edge_offset = self.pos() % 8;
            let first_byte = self.bytes[self.byte_pos()];
            None
        }
    }
}