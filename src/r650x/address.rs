#[derive(Clone, Copy, Eq, PartialEq)]
pub enum AddressMode {
    Implied,
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Relative,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndexedIndirect, //($NN, X)
    IndirectIndexed //($NN) Y
}

impl AddressMode {
    pub fn extra_bytes(self) -> usize {
        match self {
            AddressMode::Implied => 0,
            AddressMode::Accumulator => 0,
            AddressMode::Immediate => 1,
            AddressMode::ZeroPage => 1,
            AddressMode::ZeroPageX => 1,
            AddressMode::ZeroPageY => 1,
            AddressMode::Relative => 1,
            AddressMode::Absolute => 2,
            AddressMode::AbsoluteX => 2,
            AddressMode::AbsoluteY => 2,
            AddressMode::Indirect => 2,
            AddressMode::IndexedIndirect => 1,
            AddressMode::IndirectIndexed => 1,
        }
    }
}