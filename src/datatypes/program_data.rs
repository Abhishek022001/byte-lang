use std::collections::HashMap;

use crate::datatypes::{ast_statements::Statement, stack_frame::StackFrame, token::Token};

#[derive(PartialEq, Clone, Debug)]
pub struct ProgramData {
    pub stack_frames : Vec<StackFrame>,
    pub functions : HashMap<String, usize>,
    pub statements : Vec<Statement>,
    pub source_code : String,
    pub tokens : Vec<Token>,
    pub errors : Vec<String>
}

impl ProgramData {
    pub fn new() -> Self {
        Self { stack_frames: Vec::new(), functions: HashMap::new(), source_code: String::new(), tokens: Vec::new(), statements: Vec::new(), errors: Vec::new() }
    }
}
