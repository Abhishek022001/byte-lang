use std::collections::HashMap;

use crate::datatypes::{ast_statements::{Function, FunctionArg, MemoryLocationsAst, Statement}, stack_frame::{StackFrame, StackVariable}, token::Token};

#[derive(Clone, Debug, PartialEq)]
pub struct StackVariableRef {
    pub local_offset : usize,
    pub var: StackVariable
}

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionStackArgRef {
    pub local_offset : usize,
    pub var: FunctionArg
}

#[derive(PartialEq, Clone, Debug)]
pub struct ProgramData {
    pub stack_frames : Vec<StackFrame>,
    pub functions : HashMap<String, Function>,
    pub statements : Vec<Statement>,
    pub source_code : String,
    pub tokens : Vec<Token>,
    pub errors : Vec<String>
}

impl ProgramData {
    pub fn new() -> Self {
        Self { stack_frames: Vec::new(), functions: HashMap::new(), source_code: String::new(), tokens: Vec::new(), statements: Vec::new(), errors: Vec::new() }
    }

    pub fn get_stack_frame_by_index(&self, index : usize) -> &'_ StackFrame {
        return self.stack_frames.get(index).unwrap();
    }

    pub fn get_stack_variable_ref(&mut self, stack_frame : usize, var_name : &str, offset : usize) -> Option<StackVariableRef> {
        let stack_frame_ref = self.get_stack_frame_by_index(stack_frame);

        match stack_frame_ref.variables.get(var_name) {
            Some(refrence) => {
                return Some(StackVariableRef { local_offset: offset + stack_frame_ref.stack_mem_allocated - refrence.offset - refrence.variable_size, var: refrence.clone() });
            },
            None => {
                if stack_frame_ref.parent == usize::MAX {
                    return None;
                } else {
                    // Also count for x30 stored inside stack
                    return self.get_stack_variable_ref(stack_frame_ref.parent, var_name, offset + 16 + stack_frame_ref.stack_mem_allocated);
                }
            }
        }
    }

    pub fn get_function_stack_arg_ref(&self, stack_frame : usize, identifier : &str) -> Option<FunctionStackArgRef> {
        let function_name = self.get_stack_frame_by_index(stack_frame).function.clone();

        let func_mem_allocated = self.functions.get(&function_name).unwrap().stack_mem_allocated.clone();
        let func_arg = self.functions.get(&function_name).unwrap().args.iter().find(|arg| arg.arg_name == identifier);

        let mut current_stack_frame = stack_frame;
        let mut bytes = 0;

        loop {
            let current_stack_frame_borrow = self.get_stack_frame_by_index(current_stack_frame);
            bytes += current_stack_frame_borrow.stack_mem_allocated + 16;

            if current_stack_frame_borrow.parent == usize::MAX {
                break;
            }

            current_stack_frame = current_stack_frame_borrow.parent;
        }

        if let Some(func_arg_unwrapped) = func_arg {
            if let MemoryLocationsAst::Stack(stack_offset) = func_arg_unwrapped.memory_location {
                return Some(FunctionStackArgRef{local_offset: bytes + func_mem_allocated - stack_offset - func_arg_unwrapped.arg_var_type.get_variable_size(), var: func_arg_unwrapped.clone()});
            }

            return None;
        } else {
            return None;
        }
    }
}
