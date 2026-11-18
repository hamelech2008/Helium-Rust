use std::collections::hash_map::Iter;
use std::collections::HashMap;
use std::process::exit;
use crate::lexer::token::Token;
use crate::parser::ast_nodes::class::{ClassDecVar, NodeClassDecl};
use crate::parser::ast_nodes::DataType;
use crate::parser::ast_nodes::expr::NodeExpr;
use crate::parser::ast_nodes::functions::NodeFuncDecl;

#[derive(Debug)]
pub struct ClassTable
{
    class_table: HashMap<String, NodeClassDecl>
}

impl ClassTable
{
    pub fn new() -> Self {Self{class_table: HashMap::new()}}

    pub fn get_class_member(&self, parent_class: &NodeExpr, member: &Token) -> Option<&ClassDecVar>
    {
        self.class_table.get(&String::from(parent_class.get_data_type()))?.fields.get(&member.data)
    }

    pub fn get_class_method(&self, parent_class: &NodeExpr, method: &Token) -> Option<&NodeFuncDecl>
    {
        self.class_table.get(&String::from(parent_class.get_data_type()))?.get_methods().get(&method.data)

    }

    pub fn get_class_name(&self, class: &NodeExpr) -> String
    {
        String::from(class.get_data_type())
    }

    pub fn define_class(&mut self, class_ident: String, class_decl: NodeClassDecl)
    {
        self.class_table.insert(class_ident, class_decl);
    }

    pub fn get_class_size(&self, class_ident: &String) -> u32
    {
        let class_decl = self.class_table.get(class_ident);
        if class_decl.is_none()
        {
            eprintln!("Unknown class {}", class_ident);
            exit(1);
        }

        *class_decl.unwrap().get_size()
    }

    pub fn get_member_offset(&self, parent_class: &NodeExpr, member: &Token) -> u32
    {
        let class_ident = &String::from(parent_class.get_data_type());
        let class_decl = self.class_table.get(class_ident);
        if class_decl.is_none()
        {
            eprintln!("Unknown class {}", class_ident);
            exit(1);
        }

        let member_var = class_decl.unwrap().fields.get(&member.data);
        if member_var.is_none()
        {
            eprintln!("Unknown member {}", member.data);
            exit(1);
        }

        class_decl.unwrap().fields.get(&member.data).unwrap().offset
    }



    pub fn get_member_data_type(&self, parent_class: &NodeExpr, member: &Token) -> DataType
    {
        let class_ident = &String::from(parent_class.get_data_type());
        let class_decl = self.class_table.get(class_ident);
        if class_decl.is_none()
        {
            eprintln!("Unknown class {}", class_ident);
            exit(1);
        }

        let member_var = class_decl.unwrap().fields.get(&member.data);
        if member_var.is_none()
        {
            eprintln!("Unknown member {}", member.data);
            exit(1);
        }

        class_decl.unwrap().fields.get(&member.data).unwrap().get_data_type().clone()
    }

    pub fn get_itr(&self) -> Iter<String, NodeClassDecl>
    {
        self.class_table.iter()
    }
}