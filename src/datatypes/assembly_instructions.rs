use crate::datatypes::ast_statements::CgExpression;

pub mod asm {
    use std::fmt::format;

    use crate::datatypes::ast_statements::VariableType;

    pub enum StackDestination {
        StackPointer,
        StackFramePointer
    }

    impl StackDestination {
        pub fn register(&self) -> String {
            let res : &str = match self {
                StackDestination::StackPointer => "sp",
                StackDestination::StackFramePointer => "x29"
            };

            return String::from(res);
        }
    }

    pub const TEMP_64: &str = "x10";
    pub const TEMP_32: &str = "w10";

    pub const STACK_PTR: &str = "sp";
    pub const STACK_FRAME_PTR: &str = "x29";

    pub const STORE_64: &str = "str";
    pub const STORE_32: &str = "str";
    pub const STORE_16: &str = "strh";
    pub const STORE_8: &str = "strb";

    pub const LOAD_SIGNED_64: &str = "ldr";
    pub const LOAD_SIGNED_32: &str = "ldrsw";
    pub const LOAD_SIGNED_16: &str = "ldrsh";
    pub const LOAD_SIGNED_8:  &str = "ldrsb";

    pub const LOAD_UNSIGNED_64: &str = "ldr";
    pub const LOAD_UNSIGNED_32: &str = "ldr";
    pub const LOAD_UNSIGNED_16: &str = "ldrh";
    pub const LOAD_UNSIGNED_8: &str = "ldrb";

    pub fn temp_reg_for_type(var_type : VariableType) -> String {
        let res : &str = match var_type {
            VariableType::I8 => "w10",
            VariableType::I16 => "w10",
            VariableType::I32 => "x10",
            VariableType::I64 => "x10",
            VariableType::U8 => "w10",
            VariableType::U16 => "w10",
            VariableType::U32 => "w10",
            VariableType::U64 => "x10",
            _ => unreachable!()
        };

        return String::from(res);
    }

    pub fn store_literal_to_stack(var_type : VariableType, num : i64, offset : usize) -> String {
        let temp_reg = temp_reg_for_type(var_type.clone());

        return format!("mov {}, {}\n{} {}, [x29, #-{}]\n", temp_reg, num, store_instruction_for_type(var_type), temp_reg, offset);
    }

    pub fn store_reg_to_stack(reg : &str, offset : usize, var_type : VariableType, stack_destination : StackDestination) -> String {
        return format!("{} {}, [{}, #-{}]\n", store_instruction_for_type(var_type), reg, stack_destination.register(), offset);
    }

    pub fn load_instruction_for_type(var_type : VariableType) -> String {
        let res : &str = match var_type {
            VariableType::I8 => "ldrsb",
            VariableType::I16 => "ldrsh",
            VariableType::I32 => "ldrsw",
            VariableType::I64 => "ldr",
            VariableType::U8 => "ldrb",
            VariableType::U16 => "ldrh",
            VariableType::U32 => "ldr",
            VariableType::U64 => "ldr",
            _ => unreachable!()
        };

        return String::from(res);
    }

    pub fn store_instruction_for_type(var_type : VariableType) -> String {
        let res : &str = match var_type {
            VariableType::I8 => "strb",
            VariableType::I16 => "strh",
            VariableType::I32 => "str",
            VariableType::I64 => "str",
            VariableType::U8 => "strb",
            VariableType::U16 => "strh",
            VariableType::U32 => "str",
            VariableType::U64 => "str",
            _ => unreachable!()
        };

        return String::from(res);
    }

    pub fn allocate_stack_memory(bytes : usize) -> String {
        return format!("sub sp, sp, #{}\n", bytes);
    }

    pub fn deallocate_stack_memory(bytes : usize) -> String {
        return format!("add sp, sp, #{}\n", bytes);
    }

    pub fn mov_num_to_reg(reg : &str, num : i64) -> String {
        return format!("mov {}, #{}\n", reg, num);
    }

    pub fn jump_to_function(function_name : &str) -> String {
        return format!("bl _{}\n", function_name);
    }

    pub fn create_stack_frame(stack_memory_allocate : usize) -> String {
        if stack_memory_allocate == 0 {
            return format!("stp x29, x30, [sp, #-16]!\nmov x29, sp\n");
        } else {
            return format!("stp x29, x30, [sp, #-16]!\nmov x29, sp\nsub sp, sp, #{}\n", stack_memory_allocate);
        }
    }

    pub fn destroy_stack_frame(stack_memory_allocated : usize) -> String {
        if stack_memory_allocated == 0 {
            return format!("ldp x29, x30, [sp], #16\nret\n");
        } else {
            return format!("add sp, sp, #{}\nldp x29, x30, [sp], #16\nret\n", stack_memory_allocated);
        }
    }

    pub fn stack_var_to_reg(reg : &str, offset : usize, var_type : VariableType) -> String {
        return format!("{} {}, [x29, #-{}]\n", load_instruction_for_type(var_type), reg, offset);
    }
}
