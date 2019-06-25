pub enum InstructionFormat {
    R,
    I,
    S,
    U,
    B,
    J
}

const BaseOpcodeFlag: u8 = 0b11;
#[repr(u8)]
pub enum BaseOpcodes {
    Load        = 0b00000,
    LoadFP      = 0b00001,
    Custom0     = 0b00010,
    MiscMem     = 0b00011,
    OpImm       = 0b00100,
    Auipc       = 0b00101,
    OpImm32     = 0b00110,
    B48A        = 0b00111,

    Store       = 0b01000,
    StoreFP     = 0b01001,
    Custom1     = 0b01010,
    Amo         = 0b01011,
    Op          = 0b01100,
    Lui         = 0b01101,
    Op32        = 0b01110,
    B64         = 0b01111,

    MAdd        = 0b10000,
    MSub        = 0b10001,
    NMSub       = 0b10010,
    NMAdd       = 0b10011,
    OpFP        = 0b10100,
    Reserved0   = 0b10101,
    Custom2     = 0b10110,
    B48B        = 0b10111,

    Branch      = 0b11000,
    Jalr        = 0b11001,
    Reserved1   = 0b11010,
    Jal         = 0b11011,
    System      = 0b11100,
    Reserved2   = 0b11101,
    Custom3     = 0b11110,
    B80         = 0b11111,



}

mod bit_lengths {
    const OPCODE: usize = 6;
    const RS: usize = 6;
    const RD: usize = 6;
    const FUNCT3: usize = 2;
}