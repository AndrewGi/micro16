#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Instruction {
    LDA,
    LDX,
    LDY,
    STA,
    STX,
    STY,

    TAX,
    TAY,
    TXA,
    TYA,

    TSX,
    TXS,
    PHA,
    PHP,
    PLA,
    PLP,

    AND,
    EOR,
    ORA,
    BIT,

    ADC,
    SBC,
    CMP,
    CPX,
    CPY,

    INC,
    INX,
    INY,
    DEC,
    DEX,
    DEY,

    ASL,
    LSR,
    ROL,
    ROR,

    JMP,
    JSR,
    RTS,

    BCC,
    BCS,
    BEQ,
    BMI,
    BNE,
    BPL,
    BVC,
    BVS,

    CLC,
    CLD,
    CLI,
    CLV,
    SEC,
    SED,
    SEI,

    BRK,
    NOP,
    RTI

}