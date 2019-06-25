#[repr(u8)]
pub enum PrivilegeLevel {
    User = 0,
    Supervisor = 1,
    Hypervisor = 2,
    Machine = 3,
}
#[repr(u8)]
pub enum Use {
    Standard = 11,
    NonStandard,
}
pub enum Accessibility {
    ReadOnly,
    ReadWrite,
    ReadWriteShadow
}