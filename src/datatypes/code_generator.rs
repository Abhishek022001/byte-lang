use std::collections::HashMap;
use crate::datatypes::ast_statements::{CgStatement, Expression, Statement, Statements};

pub struct CodeGenerator<'a> {
    input: &'a Vec<CqStatement>,
    position: usize,
}

impl<'a> CodeGenerator<'a> {
    pub fn new(input: &'a Vec<CgStatement>) -> Self {
        return Self{input, position: 0};
    }

    pub fn generate_next(&mut self) -> Option<String> {
        let current = self.current_statement();
        self.position += 1;

        match current.unwrap().statement_type {
            CgStatements::EOF => {
                return None;
            },
            Statements::Terminate => {
                return Some(String::from("mov x0, #0\nmov x16, #1\nsvc #0X80\n"));
            },
            Statements::VariableDeclaration(var) => {
                println!("{:?}", var);

                let value = match var.value {
                    Expression::Literal(literal) => {
                        match literal {
                            Literal::Number(num) => num,
                            Literal::String(_) => 10
                        }
                    },
                };

                return Some(String::from(&format!("mov x0, #{}\nstr x0, [sp]\nsub sp, sp, #16\n", value)));
            },
            Statements::BuildInFunctions(func) => {
                match func {
                    _ => {}
                };
            },
            _ => ()
        };

        return None;
    }

    pub fn current_statement(&mut self) -> Option<CgStatement> {
        match self.input.get(self.position) {
            Some(statement) => {return Some(statement.clone())},
            None => {return None}
        };
    }
}
