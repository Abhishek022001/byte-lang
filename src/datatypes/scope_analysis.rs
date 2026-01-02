use std::{collections::HashMap, panic};

use crate::datatypes::{ast_statements::{BuiltInFunctionsAst, CgBranchLinked, CgBuiltInFunctions, CgStatement, CgStatementType, Expression, Function, Literal, Statement, Statements, VariableDeclaration, VariableType}, program_data::ProgramData, stack_frame::{StackFrame, StackVariable}};

macro_rules! throw_err {
    ($self:expr, $error:expr) => {
        $self.throw_err($error);

        continue;
    };
}

pub struct ScopeAnalysis<'a> {
    program_data : &'a mut ProgramData,
    position : usize,
    scope_stack : Vec<usize>
}

impl<'a> ScopeAnalysis<'a> {
    pub fn new(program_data : &'a mut ProgramData) -> Self {
        return Self{position : 0, scope_stack: Vec::new(), program_data};
    }

    pub fn process_all(&mut self) -> () {
        let mut current_function = String::new();

        loop {
            let current_statement = self.current_statement().clone();

            print!(" {:?} ", current_statement);
            
            if current_function.is_empty() {
                if let Statements::FunctionDeclaration(func_declaration) = current_statement.statement_type.clone() {
                    if self.program_data.functions.get(&func_declaration.name).is_some() {
                        throw_err!(self, &format!("Duplicate function: {}", func_declaration.name));
                    }

                    let stack_frame_index = self.program_data.stack_frames.len();

                    self.program_data.stack_frames.push(StackFrame::default());

                    self.program_data.functions.insert(func_declaration.name.clone(), Function{first_stack_frame: stack_frame_index, args: func_declaration.args, return_type: func_declaration.return_type});



                    self.scope_stack.push(stack_frame_index);

                    current_function = func_declaration.name;

                    continue;
                } else if current_statement.statement_type == Statements::EOF {
                    break;
                } else {
                    throw_err!(self, &format!("Found statement outside function: Statement {:?}", current_statement));
                }
            }

            match current_statement.statement_type.clone() {
                Statements::Expression(Expression::BuiltInFunction(BuiltInFunctionsAst::BranchLinked(bl))) => {
                    self.add_statement_to_current_stack_frame(current_statement);
                },
                Statements::VariableDeclaration(var_declaration) => {
                    self.add_var_to_stack_frame(&var_declaration);

                    self.add_statement_to_current_stack_frame(current_statement);
                },
                Statements::StackFramePop => {
                    self.pop_scope();

                    if self.scope_stack.len() == 0 {
                        current_function = String::default();
                    }
                },
                Statements::EOF => {
                    break;
                },
                _ => {
                    self.add_statement_to_current_stack_frame(current_statement);
                }
            };
            
            self.advance_position();
        }

        print!("\n");
    }

    pub fn create_new_scope(&mut self) -> () {
        let new_frame_index = self.program_data.stack_frames.len();

        self.program_data.stack_frames.push(StackFrame::default());

        self.get_current_stack_frame().children.push(new_frame_index);

        self.scope_stack.push(new_frame_index);

        return;
    }

    pub fn throw_err(&mut self, err : &str) -> () {
        self.program_data.errors.push(String::from(err));

        self.advance_position();

        return;
    }

    pub fn add_statement_to_current_stack_frame(&mut self, statement : Statement) -> () {
        self.get_current_stack_frame().statements.push(statement);

        return;
    }

    pub fn get_stack_frame_by_index(&mut self, stack_frame : usize) -> &'_ StackFrame {
        return self.program_data.stack_frames.get(stack_frame).unwrap();
    }

    pub fn check_if_var_exists(&mut self, var : &VariableDeclaration) -> bool {
        let mut current_stack_frame_index : usize = self.get_current_stack_frame_index();

        while current_stack_frame_index != usize::MAX {
            let stack_frame_borrow = self.get_stack_frame_by_index(current_stack_frame_index);

            if let Some(_) = stack_frame_borrow.variables.get(&var.name) {
                return true;
            }

            current_stack_frame_index = stack_frame_borrow.parent;
        }

        return false;
    }

    pub fn add_var_to_stack_frame(&mut self, var : &VariableDeclaration) -> () {
        if self.check_if_var_exists(var) == true {
            self.throw_err(&format!("Duplicate variable: {}", var.name));

            return;
        }

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

        return self.program_data.stack_frames.get_mut(current_index).unwrap();
    }
    
    pub fn get_current_stack_frame_index(&self) -> usize {
        return self.scope_stack.last().unwrap().clone();
    }

    pub fn current_statement(&self) -> &Statement {
        return &self.program_data.statements.get(self.position).unwrap();
    }

    pub fn advance_position(&mut self) -> () {
        self.position += 1;

        return;
    }
}
