use std::{collections::HashMap, panic};

use crate::datatypes::{ast_statements::{Statement, Statements, VariableDeclaration}, program_data::ProgramData, stack_frame::{StackFrame, StackVariable}};

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
                        panic!("Duplicate function: {}", func_declaration.name);
                    }

                    let stack_frame_index = self.program_data.stack_frames.len();

                    self.program_data.stack_frames.push(StackFrame::default());

                    self.program_data.functions.insert(func_declaration.name.clone(), stack_frame_index);

                    self.scope_stack.push(stack_frame_index);

                    current_function = func_declaration.name;

                    continue;
                } else if current_statement.statement_type == Statements::EOF {
                    break;
                } else {
                    print!("Scope: {:?}\nCurrent Function: {:?}\nStatement: {:?}\n", self.scope_stack, current_function, current_statement);

                    panic!("Found statement outside function");
                }
            }

            match current_statement.statement_type.clone() {
                Statements::VariableDeclaration(var_declaration) => {
                    self.add_var_to_stack_frame(&var_declaration);

                    self.add_statement_to_current_stack_frame(current_statement);

                    /*if let Some(init_value) = var_declaration.value {
                        self.add_statement_to_current_stack_frame(CgStatement{

                        });
                    }*/
                },
                Statements::StackFramePop => {
                    self.pop_scope();

                    if self.scope_stack.len() == 0 {
                        current_function = String::default();
                    }
                },
                /*Statements::BuiltInFunctions(func) => {
                    match func {
                        BuiltInFunctionsAst::Assembly(asm_expression) => {
                            let asm_code : String = match asm_expression {
                                Expression::Literal(Literal::String(asm_code)) => asm_code,
                                _ => {panic!("Invalid shit given to asm func");}
                            };

                            self.get_current_stack_frame().statements.push(CgStatement { statement_type: CgStatementType::BuiltInFunction(CgBuiltInFunctions::Assembly(asm_code))});
                        }
                    }
                },*/
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
    }

    pub fn add_statement_to_current_stack_frame(&mut self, statement : Statement) -> () {
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
