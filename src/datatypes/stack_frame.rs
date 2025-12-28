use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use crate::datatypes::ast_statements::{CgStatement, Statement, VariableType};

#[derive(Clone, Debug, PartialEq)]
pub struct StackVariable {
    pub variable_type : VariableType,
    pub variable_size : usize, 
    pub offset : usize
}

#[derive(Clone, Debug, PartialEq)]
pub struct StackFrame {
    pub variables: HashMap<String, StackVariable>,
    pub stack_mem_allocated : usize,
    pub statements : Vec<Statement>,
    pub cg_statements : Vec<CgStatement>,
    pub children : Vec<usize>,
    pub parent : usize
}

impl StackFrame {
    pub fn new(parent : usize) -> Self {
        return Self { variables: HashMap::new(), stack_mem_allocated: 0, statements: Vec::new(), cg_statements: Vec::new(), children: Vec::new(), parent: parent }
    }

    pub fn default() -> Self {
        Self { variables: HashMap::new(), stack_mem_allocated: 0, statements: Vec::new(), cg_statements: Vec::new(), children: Vec::new(), parent: usize::MAX }
    }
}
