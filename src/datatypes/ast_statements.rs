use crate::datatypes::token::{BuiltInFunctions, Identifiers, MemoryLocations, Token, TokenType};

#[derive(Debug, PartialEq, Clone)]
pub struct Statement {
    pub col: usize,
    pub line: usize,
    pub start_pos: usize,
    pub end_pos: usize,
    pub statement_type: Statements
}

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    pub return_type: VariableType,
    pub args: Vec<FunctionArg>,
    pub first_stack_frame: usize
}

impl Statement {
    #[inline]
    pub fn new(token: &Token, end_pos: usize, statement_type: Statements) -> Self {
        Self {
            col: token.col,
            line: token.line,
            start_pos: token.start_pos,
            end_pos,
            statement_type,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionArg {
    pub arg_var_type : VariableType,
    pub arg_name : String,
    pub memory_location : MemoryLocationsAst
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionDeclaration {
    pub args : Vec<FunctionArg>,
    pub name : String,
    pub return_type : VariableType,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statements {
    Terminate,
    EOF,
    VariableDeclaration(VariableDeclaration),
    FunctionDeclaration(FunctionDeclaration),
    StackFramePop,
    Expression(Expression),
}

#[derive(Debug, PartialEq, Clone)]
pub enum BuiltInFunctionsAst {
    Assembly(Box<Expression>),
    Format(Format),
    BranchLinked(BranchLinkedAst)
}

#[derive(Debug, PartialEq, Clone)]
pub enum MemoryLocationsAst {
    Stack,
    Register(String)
}

#[derive(Debug, PartialEq, Clone)]
pub struct BranchLinkedAst {
    pub args : Vec<Expression>,
    pub function_name : String
}

impl BuiltInFunctionsAst {
    pub fn GetReturnType(&self) -> Literal {
        return match self {
            BuiltInFunctionsAst::Format(_) => Literal::String(String::new()),
            BuiltInFunctionsAst::Assembly(_) => Literal::String(String::new()),
            _ => Literal::String(String::new())
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Format {
    pub string : String,
    pub args_provided : Vec<Token>,
}

impl Format {
    pub fn parse(&self) -> String {
        let mut result = String::new();

        let mut position : usize = 0;

        for arg in self.args_provided.clone() {
            loop {
                match self.string.chars().nth(position).unwrap() {
                    '{' => {
                        let mut type_str = String::new();
                        
                        position += 1;

                        loop {
                            match self.string.chars().nth(position).unwrap() {
                                '}' => {
                                    position += 1;
                                    break;
                                },
                                _ => {
                                    type_str.push(self.string.chars().nth(position).unwrap());
                                    position += 1;
                                }
                            }
                        }

                        match (type_str.as_str(), arg.kind) {
                            ("i32", TokenType::Literal(Literal::Number(num))) => {
                                result.push_str(&num.to_string());
                            }
                            _ => {
                                panic!("Invalid Format");
                            }
                        }

                        break;
                    },
                    _ => {
                        result.push(self.string.chars().nth(position).unwrap());

                        position += 1;
                    }
                }
            }
        }

        while position < self.string.len() {
            result.push(self.string.chars().nth(position).unwrap());
            position += 1;
        }

        return result;
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct VariableDeclaration {
    pub name: String,
    pub variable_type: VariableType,
    pub value: Option<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Literal(Literal),
    Identifier(Identifiers),
    BuiltInFunction(BuiltInFunctionsAst)
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    String(String),
    Number(i32)
}

#[derive(Debug, PartialEq, Clone)]
pub enum VariableType {
    I8,
    I16,
    I32,
    Void
}

impl VariableType {
    #[inline]
    pub fn get_variable_size(&self) -> usize {
        return match self {
            VariableType::I8 => 1,
            VariableType::I16 => 2,
            VariableType::I32 => 4,
            VariableType::Void => 0
        }
    }

}

// Code gen specific Structs
#[derive(Debug, PartialEq, Clone)]
pub struct CgStatement {
    pub statement_type : CgStatementType
}

#[derive(Debug, PartialEq, Clone)]
pub enum CgStatementType {
    VariableInitialization(CgVariableInitialization),
    BuiltInFunction(CgBuiltInFunctions)
}

#[derive(Debug, PartialEq, Clone)]
pub enum CgBuiltInFunctions {
    Assembly(String),
    BranchLinked(CgBranchLinked)
}

#[derive(Debug, PartialEq, Clone)]
pub struct CgBranchLinked {
    pub function_name : String,
    pub args : Vec<CgExpression>
}

#[derive(Debug, PartialEq, Clone)]
pub enum CgExpression {
    StackVariableIdentifier(String),
    Literal(Literal)
}

#[derive(Debug, PartialEq, Clone)]
pub struct CgVariableInitialization {
    pub init_value : CgExpression,
    pub var_name : String,
    pub stack_frame : usize
}
