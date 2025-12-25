use std::collections::HashMap;
use crate::datatypes::{ast_statements::{CgStatement, CgStatementType, Expression, Statement, Statements}, stack_frame::StackFrame};

pub struct CodeGenerator<'a> {
    stack_frames: &'a Vec<StackFrame>,
}

impl<'a> CodeGenerator<'a> {
    pub fn new(stack_frames : &'a Vec<StackFrame>) -> Self {
        return Self{stack_frames};
    }

    pub fn generate_statement(&mut self, statement : &CgStatement) -> Result<String, String> {
        match statement.statement_type.clone() {
            CgStatementType::VariableInitialization(var_init) => {
                return Ok(format!("mov x0 #10"));
            },
            _ => ()
        };

        return Err("Something".to_string());
    }

    pub fn process_stack_frame(&mut self, stack_frame : usize) -> String {
        let mut result = String::new();

        for statement in self.get_stack_frame_by_index(stack_frame).statements.clone().iter() {
            let analyzed_statement_err = self.generate_statement(statement);

            match analyzed_statement_err {
                Err(err) => panic!("{:?}", err),
                Ok(asm_code) => {
                    result.push_str(&asm_code);
                }
            }
        }

        return result;
    }

    pub fn process_stack_frame_and_children(&mut self, stack_frame_index : usize) -> String {
        let compiled_code = self.traverse_stack_frame_children(stack_frame_index);

        return compiled_code;
    }

    pub fn traverse_stack_frame_children(&mut self, stack_frame_index : usize) -> String {
        let mut result = String::new();

        let children = self.get_stack_frame_by_index(stack_frame_index).children.clone();

        let compiled_code = self.process_stack_frame(stack_frame_index);

        result.push_str(&compiled_code);

        for child in children.iter() {
            let child_compiled = self.traverse_stack_frame_children(child.clone());

            result.push_str(&child_compiled);
        }

        return compiled_code;
    }

    pub fn get_stack_frame_by_index(&self, index : usize) -> &'_ StackFrame {
        return self.stack_frames.get(index).unwrap();
    }
}
