pub struct AddressWidth(u8);
impl AddressWidth {
    pub fn max_addressable(self) -> u16 {
        u16::checked_pow(2, self.0 as u32).expect("should fit inside a u16")
    }
}
mod defaults {
    const SP_START: u8 = 0xFF;
}
pub struct Settings {
    address_width: AddressWidth,
    sp_start: u8,
}
