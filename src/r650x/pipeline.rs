use crate::r650x::decoder::{DecodedInstruction, DecoderError, decode};

pub struct Pipeline {
    raw_instruction: u8,
    decoded_instruction: DecodedInstruction,
    memory_value: u8,
}
impl Pipeline {
    pub fn latch_instruction(&mut self, raw_instruction: u8) {
        self.raw_instruction = raw_instruction
    }
    pub fn latch_memory_value(&mut self, val: u8) {
        self.memory_value = val;
    }
    pub fn m(&self) -> u8 {
        self.memory_value
    }
    pub fn decode(&mut self) -> Result<DecodedInstruction, DecoderError> {
        self.decoded_instruction = decode(self.raw_instruction)?;
        Ok(self.decoded_instruction)
    }
}