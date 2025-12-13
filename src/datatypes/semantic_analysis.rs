use std::{cell::RefCell, collections::HashMap, hash::Hash, rc::Rc};

use crate::datatypes::{StackFrame, SymbolType};

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
                if((var.variable_type == DeclareVariableType::String && matches!(var.value, Expression::Literal(Literal::String(_))))
                    ||
                    (var.variable_type == DeclareVariableType::Number && matches!(var.value, Expression::Literal(Literal::Number(_))))) == false {
                    return Err(String::from(format!("Invalid Variable Declaration at line {} and col {}: {:?}", statement.line, statement.col, self.untokenized_input.get(statement.start_pos..statement.end_pos).unwrap()))); 
                }

                self.add_symbol(Symbol{symbol_type: SymbolType::Variable}, var.name);
            },
            Statements::EOF => {
                return Ok(statement.clone());
            },
            _ => ()
        };

        self.position += 1;

        return Ok(statement.clone());
    }

    pub fn add_symbol(&mut self, symbol : Symbol, symbol_name : String) {
        if self.symbol_table.get(&symbol_name).is_some() {
            panic!("Already defined symbol with the name: {:?}", symbol_name);
        }

        self.symbol_table.insert(symbol_name, symbol);
    }
}
