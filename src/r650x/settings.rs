pub struct AddressWidth(u8);
impl AddressWidth {
    pub fn max_addressable(self) -> u16 {
        u16::checked_pow(2, self.0).expect("should fit inside a u16")
    }
}
pub struct Settings {
    address_width: AddressWidth,
}