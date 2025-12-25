use std::{collections::HashMap, vec};

use crate::datatypes::{ast_statements::{CgStatement, CgStatementType, CgVariableInitialization, Statement, Statements, VariableDeclaration}, stack_frame::{StackFrame, StackVariable}};

pub struct ScopeAnalysis<'a> {
    source_code : &'a str,
    statements : &'a Vec<Statement>,
    position : usize,
    stack_frames : Vec<StackFrame>,
    scope_stack : Vec<usize>
}

impl<'a> ScopeAnalysis<'a> {
    pub fn new(source_code : &'a str, statements : &'a Vec<Statement>) -> Self {
        return Self{source_code, statements, position : 0, stack_frames: Vec::new(), scope_stack: Vec::new()};
    }

    pub fn process_all(&mut self) -> Vec<StackFrame> {
        self.stack_frames.push(StackFrame::default());
        self.scope_stack.push(0);

        loop {
            let current_statement = self.current_statement();
            
            match current_statement.statement_type.clone() {
                Statements::VariableDeclaration(var_declaration) => {
                    self.add_var_to_stack_frame(&var_declaration);

                    if let Some(init_value) = var_declaration.value {
                        self.add_statement_to_current_stack_frame(CgStatement{
                            statement_type: CgStatementType::VariableInitialization(CgVariableInitialization{var_name: var_declaration.name.clone(), init_value: init_value, stack_frame: self.get_current_stack_frame_index()})
                        });
                    }
                },
                Statements::EOF => break,
                _ => {}
            };
            
            self.advance_position();
        }

        return self.stack_frames.clone();
    }

    pub fn create_new_scope(&mut self) -> () {
        let new_frame_index = self.stack_frames.len();

        self.stack_frames.push(StackFrame::default());

        self.get_current_stack_frame().children.push(new_frame_index);

        self.scope_stack.push(new_frame_index);
    }

    pub fn add_statement_to_current_stack_frame(&mut self, statement : CgStatement) -> () {
        self.get_current_stack_frame().statements.push(statement);

        return;
    }

    pub fn add_var_to_stack_frame(&mut self, var : &VariableDeclaration) -> () {
        let current_stack_frame = self.get_current_stack_frame();

        current_stack_frame.variables.insert(var.name.clone(), StackVariable{variable_type: var.variable_type.clone(), variable_size: var.variable_type.get_variable_size(), offset: current_stack_frame.stack_mem_allocated.clone()});

        current_stack_frame.stack_mem_allocated += var.variable_type.get_variable_size();

        return;
    }

    pub fn pop_scope(&mut self) -> () {
        self.scope_stack.pop();

        return;
   }

    pub fn get_current_stack_frame(&mut self) -> &'_ mut StackFrame {
        let current_index = self.get_current_stack_frame_index();

        return self.stack_frames.get_mut(current_index).unwrap();
    }
    
    pub fn get_current_stack_frame_index(&self) -> usize {
        return self.scope_stack.last().unwrap().clone();
    }

    pub fn current_statement(&self) -> &Statement {
        return &self.statements.get(self.position).unwrap();
    }

    pub fn advance_position(&mut self) -> () {
        self.position += 1;

        return;
    }
}
