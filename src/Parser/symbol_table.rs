use std::collections::HashMap;
use crate::parser::ast_nodes::DataType;

#[derive(Clone, Debug)]
pub enum Symbol
{
    Variable(DataType),
    Function
    {
        return_type: DataType,
    }
}

#[derive(Debug)]
pub struct SymbolTable
{
    symbol_table: HashMap<String, Symbol>
}

impl SymbolTable
{
    pub fn new() -> SymbolTable
    {
        SymbolTable { symbol_table: HashMap::new() }
    }

    pub fn insert(&mut self, ident: String, datatype: Symbol)
    {
        self.symbol_table.insert(ident, datatype);
    }

    pub fn get_variable(&self, ident: &str) -> DataType
    {
        let data_symbol = match self.symbol_table.get(ident).cloned()
        {
            Some(datatype) => datatype,
            None => panic!("SymbolTable::get: Unknown identifier {}", ident)
        };

        match data_symbol
        {
            Symbol::Variable(datatype) => datatype,
            _ => panic!("SymbolTable::get: Unknown identifier {}", ident)
        }
    }

    pub fn get_function(&self, ident: &str) -> DataType
    {
        let data_symbol = match self.symbol_table.get(ident)
        {
            Some(symbol) => symbol,
            None => panic!("SymbolTable::get: Unknown identifier {}", ident),
        };

        match data_symbol
        {
            Symbol::Function { return_type } => return_type.clone(),
            _ => panic!("SymbolTable::get: {} is not a function", ident),
        }
    }

}
