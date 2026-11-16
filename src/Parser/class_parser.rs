use std::collections::HashMap;
use std::process::exit;
use crate::lexer::token::TokenType;
use crate::parser::ast_nodes::class::{ClassDecVar, NodeClassDecl};
use crate::parser::Parser;

use crate::error_expected;
impl Parser
{
    // class parsing
    pub(super) fn parse_class(&mut self) -> Option<NodeClassDecl>
    {
        // consume the 'class' token
        if self.try_consume(TokenType::Class).is_none() {return None;}

        if let Some(ident) = self.try_consume(TokenType::Ident)
        {
            // consume the '{' token, note: class does not contain a scope
            if self.try_consume(TokenType::LeftCurly).is_none() {return None;}

            let mut fields = HashMap::new();
            let mut methods = HashMap::new();


            let mut offset = 0;
            loop
            {
                if self.try_peek(TokenType::Dec) //field
                {
                    if let Some(mut field) = self.parse_field()
                    {
                        field.set_offset(offset);
                        let ident = field.get_ident().clone().data;
                        fields.insert(ident, field);
                        offset += 1;
                    }
                }
                else {break;}
            }

            let mut node_class = NodeClassDecl::new(methods, fields, offset);

            self.class_table.define_class(ident.clone().data, node_class.clone());


            loop
            {
                if self.try_peek(TokenType::Function) // method
                {
                    if let Some(method) = self.parse_function()
                    {
                        let ident = method.get_ident().clone().data;
                        node_class.add_method(ident, method);
                    }
                }
                else {break;}
            }

            // consume the '}' token
            if self.try_consume(TokenType::RightCurly).is_none() {return None;}

            self.class_table.define_class(ident.data, node_class.clone());
            return Some(node_class);
        }
        None
    }

    fn parse_field(&mut self) -> Option<ClassDecVar>
    {
        // consume the 'dec' token
        if self.try_consume(TokenType::Dec).is_none() {error_expected!("dec");}

        if let Some(ident) = self.try_consume(TokenType::Ident)
        {
            // consume the ':' token
            if self.try_consume(TokenType::Colon).is_none() {error_expected!(":");}

            if let Some(data_type) = self.parse_explicit_type()
            {
                // consume the ';' token
                if self.try_consume(TokenType::SemiColon).is_none() {error_expected!(";");}

                let dec_field = ClassDecVar::new(ident, data_type);
                return Some(dec_field);
            }
        }
        None
    }
}