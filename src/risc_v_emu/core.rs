use super::context;
use crate::risc_v_emu::types::DataType;
use crate::microvm::memory::address::AddressType;

pub trait CoreSettings: Sized {
    type XLen: DataType;
    type RegType: DataType;
    type Address: AddressType;
}
pub struct Core<Settings: CoreSettings> {
    context: context::Context<Settings::RegType, Settings::Address>,
}