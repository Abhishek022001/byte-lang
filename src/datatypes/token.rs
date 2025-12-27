use crate::datatypes::ast_statements::{Literal, VariableType};

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
    VariableType(VariableType),
    CompileTime
}

#[derive(Debug, PartialEq, Clone)]
pub enum Identifiers {
    Identifier(String)
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    EOF,
    BuildInCommand(BuildInCommand),
    Operator(Operators),
    Keyword(Keywords),
    Literal(Literal),
    Punctuation(Punctuations),
    BuiltInFunctions(BuiltInFunctions),
    Identifiers(Identifiers)
}

#[derive(Debug, PartialEq, Clone)]
pub enum BuiltInFunctions {
    Loop,
    Compare,
    Assembly,
    Format
}

#[derive(Debug, PartialEq, Clone)]
pub enum Punctuations {
    Colon,
    OpenParenthesis,
    ClosedParenthesis,
    OpenBraces,
    ClosedBraces,
    Comma,
    Semicolon
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operators {
    Assignment
}

#[derive(Debug, PartialEq, Clone)]
pub enum BuildInCommand {
}
