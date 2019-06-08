
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
        self.bytes_len() * 8
    }
    pub fn bytes_len(&self) -> usize {
        self.bytes.len()
    }

    pub fn collect(&mut self, out: &mut [u8]) -> Option<usize> {
        let amount = out.len();
        if amount == 0 || self.pos() >= self.len() {
            None
        } else {
            let edge_offset = self.pos() % 8;
            let start_byte_index = self.byte_pos();
            let end_bit_index = self.pos() + amount;
            let end_byte_index = end_bit_index / 8;
            let first_byte = self.bytes[start_byte_index];
            out[0] |= (first_byte >> edge_offset);
            self.bit_position += edge_offset;
            if end_byte_index > start_byte_index {
                //We need to scan more than one byte
                let mut out_bit_position = 8-edge_offset;
                if end_byte_index > start_byte_index + 1 {
                    //consume whole bytes
                    for byte in self.bytes[start_byte_index+1..end_byte_index] {
                        out[    ]
                    }
                }
            }
            for self.bytes[self.byte_pos()..]
        }
    }
}