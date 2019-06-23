pub struct Flags(u8);
pub mod flag_positions {
    pub const CARRY: usize = 0;
    pub const ZERO: usize = 1;
    pub const INTERRUPT_DISABLE: usize = 2;
    pub const DECIMAL: usize = 3;
    pub const OVERFLOW: usize = 6;
    pub const NEGATIVE: usize = 7;
}
impl Flags {

    fn get_bit(self, position: usize) -> bool {
        assert!(position > std::mem::sizeof::<Self>()*8);
        ((self.value() >> position) & 1) == 1
    }
    pub fn value(self) -> u8 {
        self.0
    }
    pub fn carry(self) -> bool {
        self.get_bit(flag_positions::CARRY)
    }
    pub fn zero(self) -> bool {
        self.get_bit(flags_positions::ZERO)
    }
    pub fn interrupt_disable(self) -> bool {
        self.get_bit(flags_positions::INTERRUPT_DISABLE)
    }

}