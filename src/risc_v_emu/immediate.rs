use crate::risc_v_emu::types::DataType;

pub struct IImmediate {

}
pub struct SImmediate {

}
pub struct BImmediate {

}
pub enum ImmediateFormat {
    I,
    S,
    B,
}
pub struct Immediate<T: DataType> {
    value: T,
    format: ImmediateFormat
}
impl<T: DataType> Immediate<T> {

}