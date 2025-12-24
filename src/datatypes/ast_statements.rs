#[derive(Debug, PartialEq, Clone)]
pub struct Statement {
    pub col: usize,
    pub line: usize,
    pub start_pos: usize,
    pub end_pos: usize,
    pub statement_type: Statements
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statements {
    Terminate,
    EOF,
    VariableDeclaration(VariableDeclaration),
    BuildInFunctions(BuildInFunctionsAst),
}

#[derive(Debug, PartialEq, Clone)]
pub enum BuildInFunctionsAst {
    Println(String)
}

#[derive(Debug, PartialEq, Clone)]
pub struct VariableDeclaration {
    pub name: String,
    pub variable_type: DeclareVariableType,
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
pub enum DeclareVariableType {
    I8,
    I16,
    I32
}

impl DeclareVariableType {
    #[inline]
    pub fn get_variable_size(&self) -> usize {
        return match self {
            DeclareVariableType::I8 => 1,
            DeclareVariableType::I16 => 2,
            DeclareVariableType::I32 => 4
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
}

#[derive(Debug, PartialEq, Clone)]
pub struct CgVariableInitialization {
    pub init_value : Expression,
    pub var_name : String,
    pub stack_frame : usize
}
