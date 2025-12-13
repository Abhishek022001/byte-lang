#[derive(Debug, PartialEq, Clone)]
pub enum SymbolType {
    Function,
    Variable
}

#[derive(Debug, PartialEq, Clone)]
pub struct Symbol {
    pub symbol_type : SymbolType
}
