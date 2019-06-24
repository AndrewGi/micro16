use crate::r650x::address::AddressMode;
use crate::r650x::flags::MCRFlag::Zero;

pub enum DecoderError {
    UnrecognizedInstruction
}
#[derive(Copy, Clone)]
pub struct DecodedInstruction {
    instruction: super::instructions::Instruction,
    address_mode: AddressMode
}
pub fn decode(b: u8) -> Result<DecodedInstruction, DecoderError> {
    use super::instructions::Instruction::*;
    use super::address::AddressMode::*;
    let high = (b>>4) & 0xF;
    let low  = b & 0xF;
    let ni = |i, m| Ok(DecodedInstruction {instruction: i, address_mode: m});
    let im = |i| ni(i, Implied);
    match low {
        0 => {
            match high {
                0x0 => ni(BRK, Implied),
                0x1 => ni(BPL, Relative),
                0x2 => ni(JSR, Absolute),
                0x3 => ni(BMI, Relative),
                0x4 => ni(RTI, Implied),
                0x5 => ni(BVC, Relative),
                0x6 => ni(RTS, Implied),
                0x7 => ni(BVS, Relative),
                0x8 => Err(DecoderError::UnrecognizedInstruction),
                0x9 => ni(BCC, Relative),
                0xA => ni(LDY, Immediate),
                0xB => ni(BCS, Relative),
                0xC => ni(CPY, Immediate),
                0xD => ni(BNE, Relative),
                0xE => ni(CPX, Immediate),
                0xF => ni(BEQ, Relative),
                _ => panic!("high too big"),
            }
        }
        1 => {
            match high {
                0x0 => ni(ORA, IndexedIndirect),
                0x1 => ni(ORA, IndirectIndexed),
                0x2 => ni(AND, IndexedIndirect),
                0x3 => ni(AND, IndirectIndexed),
                0x4 => ni(EOR, IndexedIndirect),
                0x5 => ni(EOR, IndirectIndexed),
                0x6 => ni(ADC, IndexedIndirect),
                0x7 => ni(ADC, IndirectIndexed),
                0x8 => ni(STA, IndexedIndirect),
                0x9 => ni(STA, IndirectIndexed),
                0xA => ni(LDA, IndexedIndirect),
                0xB => ni(LDA, IndirectIndexed),
                0xC => ni(CMP, IndexedIndirect),
                0xD => ni(CMP, IndirectIndexed),
                0xE => ni(SBC, IndexedIndirect),
                0xF => ni(SBC, IndirectIndexed),
                _ => panic!("high to big")
            }
        }
        2 => {
            match high {
                0xA => ni(LDX, Immediate),
                _ if high <= 0xF => Err(DecoderError::UnrecognizedInstruction),
                _ => panic!("high too big")
            }
        }
        3 => Err(DecoderError::UnrecognizedInstruction), //No instructions have a LSD of 3
        4 => {
            match high {
                0x2 => ni(BIT, ZeroPage),
                0x8 => ni(STY, ZeroPage),
                0x9 => ni(STY, ZeroPageX),
                0xA => ni(LDY, ZeroPage),
                0xB => ni(LDY, ZeroPageX),
                0xC => ni(CPY, ZeroPage),
                0xE => ni(CPX, ZeroPage),
                _ if high <= 0xF => Err(DecoderError::UnrecognizedInstruction),
                _ => panic!("high too big")
            }
        }
        5 => {
            match high {
                0x0 => ni(ORA, ZeroPage),
                0x1 => ni(ORA, ZeroPageX),
                0x2 => ni(AND, ZeroPage),
                0x3 => ni(AND, ZeroPageX),
                0x4 => ni(EOR, ZeroPage),
                0x5 => ni(EOR, ZeroPageX),
                0x6 => ni(ADC, ZeroPage),
                0x7 => ni(ADC, ZeroPageX),
                0x8 => ni(STA, ZeroPage),
                0x9 => ni(STA, ZeroPageX),
                0xA => ni(LDA, ZeroPage),
                0xB => ni(LDA, ZeroPageX),
                0xC => ni(CMP, ZeroPage),
                0xD => ni(CMP, ZeroPageX),
                0xE => ni(SBC, ZeroPage),
                0xF => ni(SBC, ZeroPageX),
                _ => panic!("high to big")
            }
        }
        6 => {
            match high {
                0x0 => ni(ASL, ZeroPage),
                0x1 => ni(ASL, ZeroPageX),
                0x2 => ni(ROL, ZeroPage),
                0x3 => ni(ROL, ZeroPageX),
                0x4 => ni(LSR, ZeroPage),
                0x5 => ni(LSR, ZeroPageX),
                0x6 => ni(ROR, ZeroPage),
                0x7 => ni(ROR, ZeroPageX),
                0x8 => ni(STX, ZeroPage),
                0x9 => ni(STX, ZeroPageY),
                0xA => ni(LDX, ZeroPage),
                0xB => ni(LDX, ZeroPageY),
                0xC => ni(DEC, ZeroPage),
                0xD => ni(DEC, ZeroPageX),
                0xE => ni(INC, ZeroPage),
                0xF => ni(INC, ZeroPageX),
                _ => panic!("high to big")
            }
        }
        7 => Err(DecoderError::UnrecognizedInstruction), //RMB, SMB?
        8 => {
            match high {
                0x0 => im(PHP),
                0x1 => im(CLC),
                0x2 => im(PLP),
                0x3 => im(SEC),
                0x4 => im(PHA),
                0x5 => im(CLI),
                0x6 => im(PLA),
                0x7 => im(SEI),
                0x8 => im(DEY),
                0x9 => im(TYA),
                0xA => im(TAY),
                0xB => im(CLV),
                0xC => im(INY),
                0xD => im(CLD),
                0xE => im(INX),
                0xF => im(SED),
                _ => panic!("high to big")
            }
        }
        9 => {
            match high {
                0x0 => ni(ORA, Immediate),
                0x1 => ni(ORA, AbsoluteY),
                0x2 => ni(AND, Immediate),
                0x3 => ni(AND, AbsoluteY),
                0x4 => ni(EOR, Immediate),
                0x5 => ni(EOR, AbsoluteY),
                0x6 => ni(ADC, Immediate),
                0x7 => ni(ADC, AbsoluteY),
                0x8 => Err(DecoderError::UnrecognizedInstruction),
                0x9 => ni(STA, AbsoluteY),
                0xA => ni(LDA, Immediate),
                0xB => ni(LDA, AbsoluteY),
                0xC => ni(CMP, Immediate),
                0xD => ni(CMP, AbsoluteY),
                0xE => ni(CMP, Immediate),
                0xF => ni(CMP, AbsoluteY),
                _ => panic!("high to big")
            }
        }
        0xA => {
            match high {
                0x0 => ni(ASL, Accumulator),
                0x2 => ni(ROL, Accumulator),
                0x4 => ni(LSR, Accumulator),
                0x6 => ni(ROR, Accumulator),
                0x8 => im(TXA),
                0x9 => im(TXS),
                0xA => im(TAX),
                0xB => im(TSX),
                0xC => im(DEX),
                0xE => im(NOP),
                _ if high <= 0xF => Err(DecoderError::UnrecognizedInstruction),
                _ => panic!("high to big")
            }
        }
        0xB => Err(DecoderError::UnrecognizedInstruction), //No instructions have a LSD of 0xB
        0xC => {
            match high {
                0x2 => ni(BIT, Absolute),
                0x4 => ni(JMP, Absolute),
                0x6 => ni(JMP, Indirect),
                0x8 => ni(STY, Absolute),
                0xA => ni(LDY, Absolute),
                0xB => ni(LDY, AbsoluteX),
                0xC => ni(CPY, Absolute),
                0xE => ni(CPX, Absolute),
                _ if high <= 0xF => Err(DecoderError::UnrecognizedInstruction),
                _ => panic!("high too big")
            }
        }
        0xD => {
            match high {
                0x0 => ni(ORA, Absolute),
                0x1 => ni(ORA, AbsoluteX),
                0x2 => ni(AND, Absolute),
                0x3 => ni(AND, AbsoluteX),
                0x4 => ni(EOR, Absolute),
                0x5 => ni(EOR, AbsoluteX),
                0x6 => ni(ADC, Absolute),
                0x7 => ni(ADC, AbsoluteX),
                0x8 => ni(STA, Absolute),
                0x9 => ni(STA, AbsoluteX),
                0xA => ni(LDA, Absolute),
                0xB => ni(LDA, AbsoluteX),
                0xC => ni(CMP, Absolute),
                0xD => ni(CMP, AbsoluteX),
                0xE => ni(SBC, Absolute),
                0xF => ni(SBC, AbsoluteX),
                _ => panic!("high to big")
            }
        }
        0xE => {

            match high {
                0x0 => ni(ASL, Absolute),
                0x1 => ni(ASL, AbsoluteX),
                0x2 => ni(ROL, Absolute),
                0x3 => ni(ROL, AbsoluteX),
                0x4 => ni(LSR, Absolute),
                0x5 => ni(LSR, AbsoluteX),
                0x6 => ni(ROR, Absolute),
                0x7 => ni(ROR, AbsoluteX),
                0x8 => ni(STX, Absolute),
                0x9 => Err(DecoderError::UnrecognizedInstruction),
                0xA => ni(LDX, Absolute),
                0xB => ni(LDX, AbsoluteY),
                0xC => ni(DEC, Absolute),
                0xD => ni(DEC, AbsoluteX),
                0xE => ni(INC, Absolute),
                0xF => ni(INC, AbsoluteX),
                _ => panic!("high to big")
            }
        }
        0xF => Err(DecoderError::UnrecognizedInstruction), //No instructions have a LSD of 0xF
        _ => panic!("low too high")
    }
}
#[cfg(test)]
mod tests {
    #[cfg(test)]
    fn test1() {
        let mut counter = 0;
        for i in 0u8..255 {
            if super::decode(i).is_ok() {
                counter+=1;
            }
        }
        assert_eq!(counter, 151);
    }
}