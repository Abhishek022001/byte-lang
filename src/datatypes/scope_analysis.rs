use std::{collections::HashMap, panic, vec};

use crate::datatypes::{ast_statements::{BuiltInFunctionsAst, CgBuiltInFunctions, CgStatement, CgStatementType, CgVariableInitialization, Expression, Literal, Statement, Statements, VariableDeclaration}, stack_frame::{StackFrame, StackVariable}, token::BuiltInFunctions};

pub struct ScopeAnalysis<'a> {
    source_code : &'a str,
    statements : &'a Vec<Statement>,
    position : usize,
    functions : HashMap<String, usize>,
    stack_frames : Vec<StackFrame>,
    scope_stack : Vec<usize>
}

impl<'a> ScopeAnalysis<'a> {
    pub fn new(source_code : &'a str, statements : &'a Vec<Statement>) -> Self {
        return Self{source_code, statements, position : 0, stack_frames: Vec::new(), scope_stack: Vec::new(), functions: HashMap::new()};
    }

    pub fn process_all(&mut self) -> (Vec<StackFrame>, HashMap<String, usize>) {
        let mut current_function = String::new();

        print!("Statements: ");

        loop {
            let current_statement = self.current_statement().clone();

            print!(" {:?} ", current_statement);
            
            if current_function.is_empty() {
                if let Statements::FunctionDeclaration(func_declaration) = current_statement.statement_type.clone() {
                    if self.functions.get(&func_declaration.name).is_some() {
                        panic!("Duplicate function: {}", func_declaration.name);
                    }

                    let stack_frame_index = self.stack_frames.len();

                    self.stack_frames.push(StackFrame::default());

                    self.functions.insert(func_declaration.name.clone(), stack_frame_index);

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

                    if let Some(init_value) = var_declaration.value {
                        self.add_statement_to_current_stack_frame(CgStatement{
                            statement_type: CgStatementType::VariableInitialization(CgVariableInitialization{var_name: var_declaration.name.clone(), init_value: init_value, stack_frame: self.get_current_stack_frame_index()})
                        });
                    }
                },
                Statements::StackFramePop => {
                    self.pop_scope();

                    if self.scope_stack.len() == 0 {
                        current_function = String::default();
                    }
                },
                Statements::BuiltInFunctions(func) => {
                    match func {
                        BuiltInFunctionsAst::Assembly(asm_expression) => {
                            let asm_code : String = match asm_expression {
                                Expression::Literal(Literal::String(asm_code)) => asm_code,
                                _ => {panic!("Invalid shit given to asm func");}
                            };

                            self.get_current_stack_frame().statements.push(CgStatement { statement_type: CgStatementType::BuiltInFunction(CgBuiltInFunctions::Assembly(asm_code))});
                        }
                    }
                },
                Statements::EOF => break,
                _ => {}
            };
            
            self.advance_position();
        }

        print!("\n");

        return (self.stack_frames.clone(), self.functions.clone());
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
