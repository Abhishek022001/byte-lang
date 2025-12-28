use std::panic;

use crate::datatypes::ast_statements::{BranchLinkedAst, BuiltInFunctionsAst, Expression, Format, FunctionArg, FunctionDeclaration, Literal, Statement, Statements, VariableDeclaration, VariableType};
use crate::datatypes::program_data::ProgramData;
use crate::datatypes::token::{BuiltInFunctions, Identifiers, Keywords, Operators, Punctuations, Token, TokenType};

pub struct Parser<'a> {
    program_data: &'a mut ProgramData,
    position: usize,
}

impl<'a> Parser<'a> {
    pub fn new(program_data: &'a mut ProgramData) -> Self {
        return Self{program_data, position: 0};
    }

    pub fn parse_all(&mut self) -> () {
        loop {
            match self.parse_next() {
                Ok(statement) => {
                    self.program_data.statements.push(statement.clone());

                    print!("{:?}", statement);

                    if statement.statement_type == Statements::EOF {
                        break;
                    }
                },
                Err(err) => {
                    panic!("{}", err);
                },
            };
        }
    }

    pub fn parse_format_built_in_function(&mut self, first_token : &Token) -> Result<Statement, String> {
        self.advance_position();

        self.expect_token(TokenType::Punctuation(Punctuations::OpenParenthesis));

        let TokenType::Literal(Literal::String(string_literal)) = self.current_token().kind else {
            panic!("Expected compile time string");
        };

        self.advance_position();

        let mut args : Vec<Token> = Vec::new();

        loop {
            match self.current_token().kind {
                TokenType::Punctuation(Punctuations::Comma) => {
                    self.advance_position();
                    args.push(self.current_token());
                    self.advance_position();
                },
                TokenType::Punctuation(Punctuations::ClosedParenthesis) => {
                    self.advance_position();
                    break;
                },
                _ => panic!("")
            }
        }

        return Ok(Statement::new(first_token, self.current_token().end_pos, Statements::Expression(Expression::BuiltInFunction(BuiltInFunctionsAst::Format(Format{string: string_literal, args_provided: args})))));
    }

    pub fn parse_function_declaration(&mut self, first_token : &Token, func_return_type : VariableType) -> Result<Statement, String> {
        self.advance_position();

        let func_name_tkn = self.current_token();

        let func_name: String = match &func_name_tkn.kind {
            TokenType::Identifiers(Identifiers::Identifier(name)) => name.clone(),
            _ => String::new(),
        };

        if func_name.is_empty() {
            return Err("Parsing func name failed".to_string());
        }

        self.advance_position();

        self.expect_token(TokenType::Punctuation(Punctuations::OpenParenthesis));

        let args : Vec<FunctionArg> = match self.current_token().kind.clone() {
            TokenType::Punctuation(Punctuations::ClosedParenthesis) => Vec::new(),
            TokenType::Keyword(Keywords::VariableType(var_type)) => unimplemented!(),
            _ => return Err("Unknown syntax err".to_string())
        };

        self.advance_position();

        self.expect_token(TokenType::Punctuation(Punctuations::OpenBraces));

        return Ok(Statement::new(first_token, self.current_token().end_pos, Statements::FunctionDeclaration(FunctionDeclaration{
            args,
            name : func_name,
            return_type: func_return_type
        })));
    }

    pub fn parse_variable_declaration(&mut self, first_token : &Token, var_type : VariableType, var_name : &String) -> Result<Statement, String> {
        if var_type == VariableType::Void {
            return Err("Can't declare variable as void".to_string());
        }

        let end_pos;

        self.advance_position();

        let value : Option<Expression> = match self.current_token().kind {
            TokenType::Operator(operator) => {
                if operator == Operators::Assignment {
                    self.advance_position();

                    let value = self.current_token();

                    let option_value : Option<Expression> = match value.kind {
                        TokenType::Literal(identifier) => {
                            match identifier {
                                Literal::String(string_val) => Some(Expression::Literal(Literal::String(string_val))),
                                Literal::Number(num_val) => Some(Expression::Literal(Literal::Number(num_val))),
                                _ => None
                            }
                        },
                        _ => None
                    };

                    if option_value.is_none() {
                        return Err(String::from("Please Provide a valid value"));
                    }

                    let value = option_value.unwrap();

                    self.advance_position();

                    let semicolon_token = self.current_token();

                    if semicolon_token.kind != TokenType::Punctuation(Punctuations::Semicolon) {
                        println!("Expected Semicolon after declaring var");
                    }

                    self.advance_position();

                    end_pos = semicolon_token.end_pos;
        
                    Some(value)
                } else {
                    return Err(String::from("Invalid identifier found"));
                }
            },
           TokenType::Punctuation(Punctuations::Semicolon) => {
                end_pos = self.current_token().end_pos;

                self.advance_position();

                None
            },
            _ => {
                return Err(String::from("Invalid identifier found"));
            }
        };

        
        return Ok(Statement::new(&first_token, end_pos, Statements::VariableDeclaration(VariableDeclaration{name: var_name.clone(), value, variable_type: var_type})));
    }

    pub fn parse_next(&mut self) -> Result<Statement, String> {
        let token = self.current_token();

        match token.kind.clone() {
            TokenType::EOF => {
                return Ok(Statement::new(&token, token.end_pos, Statements::EOF));
            },
            TokenType::BuiltInFunctions(BuiltInFunctions::Format) => {
                return self.parse_format_built_in_function(&token);
            },
            TokenType::BuiltInFunctions(BuiltInFunctions::Assembly) => {
                self.advance_position();

                self.expect_token(TokenType::Punctuation(Punctuations::OpenParenthesis));

                let asm_code = match self.current_token().kind {
                    TokenType::Literal(Literal::String(str)) => {
                        self.advance_position();

                        Expression::Literal(Literal::String(str))
                    },
                    TokenType::BuiltInFunctions(BuiltInFunctions::Format) => {
                        let format = self.parse_next().unwrap();
                        if let Statements::Expression(Expression::BuiltInFunction(BuiltInFunctionsAst::Format(format))) = format.statement_type {
                            Expression::BuiltInFunction(BuiltInFunctionsAst::Format(format))
                        } else {
                            panic!()
                        }
                    }
                    _ => {panic!("Expected string in asm_code");}
                };

                self.expect_token(TokenType::Punctuation(Punctuations::ClosedParenthesis));

                self.expect_token(TokenType::Punctuation(Punctuations::Semicolon));

                return Ok(Statement::new(&token, self.current_token().end_pos, Statements::Expression(Expression::BuiltInFunction(BuiltInFunctionsAst::Assembly(Box::new(asm_code))))));
            },
            TokenType::Punctuation(Punctuations::ClosedBraces) => {
                self.advance_position();

                return Ok(Statement::new(&token, token.end_pos, Statements::StackFramePop))
            },
            TokenType::Keyword(keyword) => {
                match keyword {
                    Keywords::VariableType(var_type) => {
                        self.advance_position();

                        match self.current_token().kind {
                            TokenType::Punctuation(punctuation) => {
                                match punctuation {
                                    Punctuations::Colon => {
                                        return self.parse_function_declaration(&token, var_type);
                                        // Handle func decl
                                    },
                                    _ => {
                                        return Err(String::from("Unknown Punctuation\n"));
                                    }
                                }
                            },
                            TokenType::Identifiers(identifier) => {
                                match identifier {
                                    Identifiers::Identifier(var_name) => {
                                        return self.parse_variable_declaration(&token, var_type, &var_name);
                                    },
                                    _ => {
                                        return Err(String::from("Unknown Identifier\n"));
                                    }
                                }
                            }
                            _ => {
                                return Err(String::from("Unknown Token Type\n"));
                            }
                        }
                    }
                }
            },
            TokenType::BuiltInFunctions(BuiltInFunctions::BranchLinked) => {
                self.advance_position();

                self.expect_token(TokenType::Punctuation(Punctuations::OpenParenthesis));

                let TokenType::Identifiers(Identifiers::Identifier(identifier)) = self.current_token().kind else {
                    panic!("")
                };

                self.advance_position();

                let args : Vec<Identifiers> = Vec::new();

                loop {
                    match self.current_token().kind {
                        TokenType::Punctuation(Punctuations::ClosedParenthesis) => {
                            self.advance_position();
                            break;
                        },
                        TokenType::Punctuation(Punctuations::Comma) => {
                            todo!();
                        },
                        _ => {
                            panic!("");
                        }
                    }
                }

                let end_pos = self.current_token().end_pos;

                self.expect_token(TokenType::Punctuation(Punctuations::Semicolon));

                return Ok(Statement::new(&token, end_pos, Statements::Expression(Expression::BuiltInFunction(BuiltInFunctionsAst::BranchLinked(BranchLinkedAst{args, function_name: identifier})))));
            }
            _ => {
                return Err(format!("Syntax Error!!! {:?}", token.kind));
            }
        };

        //return Err(String::from("Syntax Error"));
    }

    pub fn advance_position(&mut self) -> () {
        self.position += 1;
    }

    pub fn current_token(&mut self) -> Token {
        let tkn = self.program_data.tokens.get(self.position).unwrap().clone(); 
        
        return tkn;
    }

    pub fn expect_token(&mut self, token_type : TokenType) -> () {
        if self.current_token().kind != token_type {
            panic!("Current token type: {:?} Expected: {:?}", self.current_token().kind, token_type);
        }

        self.advance_position();
    }
}
