use crate::datatypes::ast_statements::VariableType;

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub kind: TokenType,
    pub line: usize,
    pub col: usize,
    pub start_pos: usize,
    pub end_pos: usize
}

#[derive(Debug, PartialEq, Clone)]
pub enum Keywords {
    VariableType(VariableType)
}

#[derive(Debug, PartialEq, Clone)]
pub enum Identifiers {
    StringLiteral(String),
    NumberLiteral(i32),
    Identifier(String)
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    EOF,
    BuildInCommand(BuildInCommand),
    Operator(Operators),
    Semicolon,
    Keyword(Keywords),
    Punctuation(Punctuations),
    BuildInFunctions(BuildInFunctions),
    Identifiers(Identifiers)
}

#[derive(Debug, PartialEq, Clone)]
pub enum BuildInFunctions {
    Loop,
    Compare
}

#[derive(Debug, PartialEq, Clone)]
pub enum Punctuations {
    Colon,
    OpenParenthesis,
    ClosedParenthesis,
    Comma
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operators {
    Assignment
}

#[derive(Debug, PartialEq, Clone)]
pub enum BuildInCommand {
    Terminate
}
