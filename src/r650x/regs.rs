pub struct Regs {
    pub pc: u16,
    pub sp: u8,
    pub accumulator: u8,
    pub x: u8,
    pub y: u8,
    pub psr: super::flags::PSR,
}