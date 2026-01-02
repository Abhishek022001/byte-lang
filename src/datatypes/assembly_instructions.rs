use crate::datatypes::ast_statements::CgExpression;

pub mod asm {
    pub const TEMP_64: &str = "x10";
    pub const TEMP_32: &str = "w10";
    pub const TEMP_16: &str = "w10";
    pub const TEMP_8: &str = "w10";

    pub const STACK_PTR: &str = "sp";
    pub const STACK_FRAME_PTR: &str = "x29";

    pub const STORE_64: &str = "str";
    pub const STORE_32: &str = "str";
    pub const STORE_16: &str = "strh";
    pub const STORE_8: &str = "strb";

    pub const LOAD_SIGNED_64: &str = "ldr";
    pub const LOAD_SIGNED_32: &str = "ldr";
    pub const LOAD_SIGNED_16: &str = "ldrh";
    pub const LOAD_SIGNED_8: &str = "ldrb";

}
