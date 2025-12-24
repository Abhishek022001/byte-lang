use crate::datatypes::DeclareVariableType;

use super::print_string::PrintString;
use super::data_string::DataString;
use super::data_number::DataNumber;
use super::compare::Compare;
use super::loop_token::LoopToken;
use super::function_struct::FunctionStruct;
use super::data_boolean::DataBoolean;
use super::call_function::CallFunction;

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
    VariableType(DeclareVariableType)
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
