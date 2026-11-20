use std::process::exit;
use crate::lexer::token::{Token, TokenType};
use crate::parser::ast_nodes::term::{NodeTerm, NodeTermArray, NodeTermBracket, NodeTermFuncCall, NodeTermNewObj, NodeTermUninitializedArray, Terms};
use crate::parser::{ast_nodes, Parser};
use crate::parser::ast_nodes::expr::NodeExpr;
use crate::{check_next_tokens, type_error};
use crate::error_expected;
use crate::parser::ast_nodes::DataType;

impl Parser
{
    // term parsing
    pub(super) fn parse_term(&mut self) -> Option<NodeTerm>
    {
        if check_next_tokens!(self, TokenType::New, TokenType::Ident)
        {
            return self.parse_object_creation_term();
        }

        // check for an array term
        if self.try_peek(TokenType::LeftSquare)
        {
            if self.try_peek_offset(TokenType::Ident, 1)
            {

                // consume the '[' token
                if self.try_consume(TokenType::LeftSquare).is_none() {error_expected!(self, '[');}

                if let Some(element_data_type) =self.parse_explicit_type()
                {
                    if self.try_consume(TokenType::Comma).is_none() {error_expected!(self, ',');}

                    if let Some(size) = self.parse_expr(0)
                    {
                        // consume the ']' token
                        if self.try_consume(TokenType::RightSquare).is_none() {error_expected!(self, ']');}

                        let data_type = DataType::Array(Box::new(element_data_type));

                        let node_term_arr = NodeTermUninitializedArray::new(size, data_type.clone());
                        let node_term = NodeTerm::new(Terms::UninitializedArray(node_term_arr), data_type);
                        return Some(node_term);
                    }
                }
            }
            return self.parse_array_term();
        }

        // check for a function call term
        if check_next_tokens!(self, TokenType::Ident, TokenType::LeftBracket)
        {
            return self.parse_func_call_term();
        }

        // check for an int literal term for e.g: 1, 3, 10...
        if let Some(int_literal) = self.try_consume(TokenType::IntLiteral)
        {
            return self.parse_int_literal_term(&int_literal);
        }

        // check for a char literal term
        if let Some(char_literal) = self.try_consume(TokenType::CharLiteral)
        {
            return self.parse_char_literal_term(&char_literal);
        }

        // check for a string literal term
        if let Some(string_literal) = self.try_consume(TokenType::StringLiteral)
        {
            return self.parse_string_literal_term(&string_literal);
        }

        // check for an identifier term - variables
        if let Some(ident) = self.try_consume(TokenType::Ident)
        {
            return self.parse_ident_term(ident);
        }

        // check for a brackets term for e.g: (1 + 5 * 3)
        if  let Some(_open_bracket) = self.try_consume(TokenType::LeftBracket) &&
            let Some(expr) = self.parse_expr(0)
        {
            return self.parse_bracket_term(expr);
        }

        None
    }

    fn parse_object_creation_term(&mut self) -> Option<NodeTerm>
    {
        // consume the opening 'new' token and expecting a type
        if self.try_consume(TokenType::New).is_none() {error_expected!(self, "new");}

        if let Some(ident) = self.try_consume(TokenType::Ident)
        {
            let data_type = DataType::Custom(0, ident.clone().data);

            // consume the '(' token
            if self.try_consume(TokenType::LeftBracket).is_none() {error_expected!(self, "(");}

            let func_args = self.get_func_args(); // consume the constructor args

            // consume the ')' token
            if self.try_consume(TokenType::RightBracket).is_none() {error_expected!(self, ")");}

            let obj_creation = NodeTermNewObj::new(ident, data_type.clone(), func_args);
            let term = NodeTerm::new(Terms::NodeTermNewObj(obj_creation), data_type);
            return Some(term);
        }
        None
    }

    fn parse_array_term(&mut self) -> Option<NodeTerm>
    {
        let mut elements = Vec::new();
        // consume the opening '[' token and expecting a type
        if self.try_consume(TokenType::LeftSquare).is_none() {error_expected!(self, '[');}
        if self.try_consume(TokenType::RightSquare).is_some() {type_error!(self, "any");}

        loop
        {
            // loop through all the elements and pushing them
            if let Some(element) = self.parse_expr(0)
            {
                elements.push(element);
                // continue while separated by a ','
                if self.try_consume(TokenType::Comma).is_some() {continue;}

                // break on the ']' token
                if self.try_consume(TokenType::RightSquare).is_some(){break;}
            }
            return None;
        }
        // get the first element type and make sure all the elements are from the same type
        let element_type = elements.first().unwrap().get_data_type().clone();
        for element in &elements
        {
            if element.get_data_type() != element_type
            {
                type_error!(self, String::from(element_type));
            }
        }

        // create the data type of type array<element_type>
        let data_type = DataType::Array(Box::new(element_type));

        let node_term_arr = NodeTermArray::new(elements, data_type.clone());
        let node_term = NodeTerm::new(Terms::Array(node_term_arr), data_type);

        Some(node_term)
    }

    fn parse_int_literal_term(&self, literal: &Token) -> Option<NodeTerm>
    {
        let node_term_literal = ast_nodes::term::NodeTermIntLiteral
        {
            value: literal.clone()
        };

        let term = NodeTerm::new(Terms::IntLiteral(node_term_literal), DataType::Integer);

        Some(term)
    }

    fn parse_char_literal_term(&self, literal: &Token) -> Option<NodeTerm>
    {
        let node_term_literal = ast_nodes::term::NodeTermCharLiteral::new(literal.clone());

        let term = NodeTerm::new(Terms::CharLiteral(node_term_literal), DataType::Char);

        Some(term)
    }

    fn parse_string_literal_term(&self, literal: &Token) -> Option<NodeTerm>
    {
        let node_term_literal = ast_nodes::term::NodeTermStringLiteral::new(literal.clone());

        let term = NodeTerm::new(Terms::StringLiteral(node_term_literal), DataType::Custom(0, "String".to_string())); // custom data type, note: must link against the standard library

        Some(term)
    }

    fn parse_ident_term(&self, ident: Token) -> Option<NodeTerm>
    {
        let data_type = self.symbol_table.get_variable(&ident.data);
        let node_term_ident = ast_nodes::term::NodeTermIdent
        {
            ident,
            data_type
        };

        // TODO: find a way to pass the data type without cloning
        let data_type = node_term_ident.clone().data_type;
        let term = NodeTerm::new(Terms::Ident(node_term_ident), data_type);
        Some(term)
    }

    fn parse_bracket_term(&mut self, expr: NodeExpr) -> Option<NodeTerm>
    {
        // consume the ')' token
        if self.try_consume(TokenType::RightBracket).is_none() {error_expected!(self, ")");}
        let data_type = expr.get_data_type();

        let term_bracket = Terms::Bracket(NodeTermBracket::new(expr));
        Some(NodeTerm::new(term_bracket, data_type))
    }

    fn parse_func_call_term(&mut self) -> Option<NodeTerm>
    {
        if let Some(ident) = self.try_consume(TokenType::Ident)
        {
            // consume the '(' token
            if self.try_consume(TokenType::LeftBracket).is_none() {error_expected!(self, "(");}

            let func_args = self.get_func_args();


            // consume the ')' token
            if self.try_consume(TokenType::RightBracket).is_none() {error_expected!(self, ")");}

            let node_call = NodeTermFuncCall::new(ident, func_args);

            let data_type = self.symbol_table.get_function(&node_call.get_ident().data);
            let term = NodeTerm::new(Terms::NodeTermFuncCall(node_call), data_type);
            return Some(term);
        }
        None
    }

}