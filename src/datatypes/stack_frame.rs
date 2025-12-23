use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

use crate::datatypes::DeclareVariableType;

#[derive(Clone, Debug, PartialEq)]
pub struct StackVariable {
    pub variable_type : DeclareVariableType,
    pub variable_size : u32, 
    pub offset : u32
}

#[derive(Clone, Debug, PartialEq)]
pub struct StackFrame {
    pub variables: HashMap<String, StackVariable>,
    pub bytes_allocated : u32,
    pub parent : Option<Rc<RefCell<StackFrame>>>,
    pub child : Option<Rc<RefCell<StackFrame>>>
}
