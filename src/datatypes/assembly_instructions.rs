use crate::datatypes::ast_statements::CgExpression;

pub mod asm {
    pub const REG_TEMP_STR_64: &str = "x10";
    pub const REG_TEMP_STR_32: &str = "w10";
    pub const REG_TEMP_STR_16: &str = "w10";
    pub const REG_TEMP_STR_8: &str = "w10";

    pub const REG_STACK_PTR: &str = "sp";
    pub const REG_FRAME_STACK_PTR: &str = "x29";

    pub const INST_STR_64: &str = "str";
    pub const INST_STR_32: &str = "str";
    pub const INST_STR_16: &str = "strh";
    pub const INST_STR_8: &str = "strb";
}
