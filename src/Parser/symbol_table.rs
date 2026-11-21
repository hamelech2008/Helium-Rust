use std::collections::HashMap;
use std::process::exit;
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

    pub fn get_variable_data_type(&self, ident: &str) -> DataType
    {
        let data_symbol = match self.symbol_table.get(ident).cloned()
        {
            Some(datatype) => datatype,
            None =>
                {
                    eprintln!("Parse Error: Unknown identifier '{}'", ident);
                    exit(1);
                }
        };

        match data_symbol
        {
            Symbol::Variable(datatype) => datatype,
            _ =>
                {
                    eprintln!("Parse Error: Unknown identifier '{}'", ident);
                    exit(1);
                }
        }
    }

    pub fn get_function_data_type(&self, ident: &str) -> DataType
    {
        let data_symbol = match self.symbol_table.get(ident)
        {
            Some(symbol) => symbol,
            None => {
                return DataType::Integer; // the default value of every function is Integer
            },
        };

        match data_symbol
        {
            Symbol::Function { return_type } => return_type.clone(),
            _ => {
                eprintln!("Parse Error: '{}' is not a function", ident);
                exit(1);
            },
        }
    }

}
