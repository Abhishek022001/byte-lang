use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use super::{VariableDeclaration, VariableType};

#[derive(Clone, Debug, PartialEq)]
pub struct StackVariable {
    pub variable_type : VariableType
}

#[derive(Clone, Debug, PartialEq)]
pub struct StackItem {
    pub offset : u32,
    pub variable : StackVariable
}

#[derive(Clone, Debug, PartialEq)]
pub struct StackFrame {
    pub stack_items: HashMap<String, StackItem>,
    pub bytes_allocated : u32,
    pub parent : Option<Rc<RefCell<StackFrame>>>,
    pub child : Option<Rc<RefCell<StackFrame>>>
}
