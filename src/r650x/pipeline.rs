use crate::r650x::decoder::{DecodedInstruction, DecoderError, decode};

pub struct Pipeline {
    raw_instruction: u8,
    decoded_instruction: DecodedInstruction,
}
impl Pipeline {
    pub fn latch(&mut self, raw_instruction: u8) {
        self.raw_instruction = raw_instruction
    }
    pub fn decode(&mut self) -> Result<DecodedInstruction, DecoderError> {
        self.decoded_instruction = decode(self.raw_instruction)?;
        Ok(self.decoded_instruction)
    }
}