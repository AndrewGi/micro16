
type Reg16 = u16;
pub struct Regs {
    r: [Reg16; 4],
    pc: Reg16,
    sp: Reg16
}

pub struct CPU<'a> {
    name: &'a str,
    regs: Regs
}