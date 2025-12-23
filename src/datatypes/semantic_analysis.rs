use std::{cell::{Ref, RefCell}, clone, collections::HashMap, hash::Hash, rc::Rc, thread::current};

use crate::datatypes::{StackFrame, StackVariable, SymbolType};

use super::{Statements, DeclareVariableType, Expression, Literal, Token, Statement, Symbol};

pub struct SemanticAnaytis<'a> {
    input: &'a Vec<Statement>,
    untokenized_input: &'a str,
    position: usize,
    first_stack_frame: Rc<RefCell<StackFrame>>,
    current_stack_frame: Rc<RefCell<StackFrame>>
}

impl<'a> SemanticAnaytis<'a> {
    pub fn new(input: &'a Vec<Statement>, untokenized_input: &'a str) -> Self {
        let stack_frame = Rc::new(RefCell::new(StackFrame {
            stack_items: HashMap::new(),
            bytes_allocated: 0,
            parent: None,
            child: None,
        }));

        Self {
            input,
            untokenized_input,
            position: 0,
            first_stack_frame: Rc::clone(&stack_frame),
            current_stack_frame: stack_frame,
        }
    }

    pub fn analyze_all(&mut self) -> Vec<Statement> {
        let mut result : Vec<Statement> = Vec::new();

        loop {
            match self.analyze_next() {
                Ok(statement) => {
                    result.push(statement.clone());

                    if statement.statement_type == Statements::EOF {
                        break;
                    }
                },
                Err(err) => {
                    println!("Error: {}", err);
                    break;
                }
            };
        };

        return result;
    }

    pub fn analyze_next(&mut self) -> Result<Statement, String> {
        let statement = self.input.get(self.position).unwrap();
        println!("{:?}", statement);

        match statement.statement_type.clone() {
            Statements::VariableDeclaration(var) => {
                if let Some(var_value) = var.value {
                    let init_valid = match (var.variable_type.clone(), var_value) {
                        (DeclareVariableType::I8  | DeclareVariableType::I16 | DeclareVariableType::I32,
                        Expression::Literal(Literal::Number(_))) => true,
                        _ => false
                    };

                    if !init_valid {
                        return Err(String::from(format!("Invalid Variable Declaration at line {} and col {}: {:?}", statement.line, statement.col, self.untokenized_input.get(statement.start_pos..statement.end_pos).unwrap()))); 
                    }
                }

                self.add_stack_variable(var.variable_type, var.name);
            },
            Statements::EOF => {
                return Ok(statement.clone());
            },
            _ => ()
        };

        self.position += 1;

        return Ok(statement.clone());
    }

    pub fn find_var_in_stack(&self, name: &str) -> Option<StackVariable> {
        let mut current = Rc::clone(&self.current_stack_frame);

        loop {
            {
                let current_borrow = current.borrow();

                if let Some(variable) = current_borrow.variables.get(name) {
                    return Some(variable.clone());
                }
            }

            let parent = current.borrow().parent.clone();

            if let Some(p) = parent {
                current = Rc::clone(&p);

                continue;
            }

            return None;
        }
    }

    pub fn add_stack_variable(&mut self, variable_type : DeclareVariableType, name: String) -> () {
        let found_variable = self.find_var_in_stack(name.as_str());
        if let Some(_) = found_variable {
            panic!("Already defined symbol with the name: {:?}", name);
        } else {
            self.current_stack_frame.borrow_mut().variables.insert(name, variable);
        }

        return;
    }
}
