use super::types::DataType;
pub struct Regs<RegType: DataType> {

    regs: [RegType; 32]

}