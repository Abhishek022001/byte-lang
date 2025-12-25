use std::{cell::{Ref, RefCell}, clone, collections::HashMap, hash::Hash, rc::Rc, thread::current};

use crate::datatypes::{ast_statements::{CgStatement, CgStatementType, DeclareVariableType, Expression, Literal}, stack_frame::StackFrame};

pub struct SemanticAnaytis<'a> {
    untokenized_input: &'a str,
    stack_frames : &'a Vec<StackFrame>,
}

impl<'a> SemanticAnaytis<'a> {
    pub fn new(untokenized_input: &'a str, stack_frames : &'a Vec<StackFrame>) -> Self {
        Self {
            untokenized_input,
            stack_frames
        }
    }

    pub fn process_statement(&mut self, statement : &'_ CgStatement) -> Option<String> {
        match statement.statement_type.clone() {
            CgStatementType::VariableInitialization(var_init) => {
                let var_stack_frame = self.get_stack_frame_by_index(var_init.stack_frame);
                let var_in_stack_frame = var_stack_frame.variables.get(&var_init.var_name).unwrap();

                let init_valid = match (var_in_stack_frame.variable_type.clone(), var_init.init_value) {
                    (DeclareVariableType::I8  | DeclareVariableType::I16 | DeclareVariableType::I32,
                    Expression::Literal(Literal::Number(_))) => true,
                    _ => false
                };

                if !init_valid {
                    return Some(String::from("Invalid var Declaration"));
                    //return Err(String::from(format!("Invalid Variable Declaration at line {} and col {}: {:?}", statement.line, statement.col, self.untokenized_input.get(statement.start_pos..statement.end_pos).unwrap()))); 
                }

                return None;
            },
            _ => ()
        };

        return None;
    }

    pub fn process_stack_frame(&mut self, stack_frame : usize) -> () {
        for statement in self.get_stack_frame_by_index(stack_frame).statements.clone().iter() {
            let analyzed_statement_err = self.process_statement(statement);

            if let Some(err) = analyzed_statement_err {
                panic!("{:?}", err);
            }
        }

        return;
    }

    pub fn process_stack_frame_and_children(&mut self, stack_frame_index : usize) -> () {
        self.traverse_stack_frame_children(stack_frame_index);
    }

    pub fn traverse_stack_frame_children(&mut self, stack_frame_index : usize) -> () {
        let children = self.get_stack_frame_by_index(stack_frame_index).children.clone();

        for child in children.iter() {
            self.traverse_stack_frame_children(child.clone());
        }

        self.process_stack_frame(stack_frame_index);
    }

    pub fn get_stack_frame_by_index(&self, index : usize) -> &'_ StackFrame {
        return self.stack_frames.get(index).unwrap();
    }
}
