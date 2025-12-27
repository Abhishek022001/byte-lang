use crate::datatypes::token::Token;

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
    pub arg_name : String
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
    BuiltInFunctions(BuiltInFunctionsAst),
}

#[derive(Debug, PartialEq, Clone)]
pub enum BuiltInFunctionsAst {
    Assembly(Expression)
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
    Assembly(String)
}

#[derive(Debug, PartialEq, Clone)]
pub struct CgVariableInitialization {
    pub init_value : Expression,
    pub var_name : String,
    pub stack_frame : usize
}
