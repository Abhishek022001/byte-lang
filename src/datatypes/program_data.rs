use std::collections::HashMap;

use crate::datatypes::{ast_statements::{Function, FunctionArg, Statement}, stack_frame::{StackFrame, StackVariable}, token::Token};

#[derive(Clone, Debug, PartialEq)]
pub struct StackVariableRef {
    pub local_offset : usize,
    pub var: StackVariable
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

    pub fn get_stack_variable(&mut self, stack_frame : usize, var_name : &str, offset : usize) -> StackVariableRef {
        let stack_frame_ref = self.get_stack_frame_by_index(stack_frame);

        match stack_frame_ref.variables.get(var_name) {
            Some(refrence) => {
                return StackVariableRef { local_offset: offset + stack_frame_ref.stack_mem_allocated - refrence.offset - refrence.variable_size, var: refrence.clone() };
            },
            None => {
                if stack_frame_ref.parent == usize::MAX {
                    unreachable!();
                } else {
                    // Also count for x30 stored inside stack
                    return self.get_stack_variable(stack_frame_ref.parent, var_name, offset + 16 + stack_frame_ref.stack_mem_allocated);
                }
            }
        }
    }
}
