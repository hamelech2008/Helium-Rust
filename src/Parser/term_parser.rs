use std::process::exit;
use crate::lexer::token::{Token, TokenType};
use crate::parser::ast_nodes::term::{NodeTerm, NodeTermBracket, NodeTermFuncCall, Terms};
use crate::parser::{ast_nodes, Parser};
use crate::parser::ast_nodes::expr::NodeExpr;
use crate::check_next_tokens;
use crate::error_expected;
impl Parser
{
    // term parsing
    pub(super) fn parse_term(&mut self) -> Option<NodeTerm>
    {
        // check for a function call term
        if check_next_tokens!(self, TokenType::Ident, TokenType::LeftBracket)
        {
            return self.parse_func_call_term();
        }

        // check for an in literal term for e.g: 1, 3, 10...
        if let Some(int_literal) = self.try_consume(TokenType::IntLiteral)
        {
            return self.parse_int_literal_term(&int_literal);
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

    fn parse_int_literal_term(&self, literal: &Token) -> Option<NodeTerm>
    {
        let node_term_literal = ast_nodes::term::NodeTermIntLiteral
        {
            value: literal.clone()
        };

        let term = NodeTerm::new(Terms::IntLiteral(node_term_literal));

        Some(term)
    }

    fn parse_ident_term(&self, ident: Token) -> Option<NodeTerm>
    {
        let node_term_ident = ast_nodes::term::NodeTermIdent
        {
            ident
        };

        let term = NodeTerm::new(Terms::Ident(node_term_ident));
        Some(term)
    }

    fn parse_bracket_term(&mut self, expr: NodeExpr) -> Option<NodeTerm>
    {
        // consume the ')' token
        if self.try_consume(TokenType::RightBracket).is_none() {error_expected!(")");}
        let term_bracket = Terms::Bracket(NodeTermBracket::new(expr));
        Some(NodeTerm::new(term_bracket))
    }

    fn parse_func_call_term(&mut self) -> Option<NodeTerm>
    {
        if let Some(ident) = self.try_consume(TokenType::Ident)
        {
            // consume the '(' token
            if self.try_consume(TokenType::LeftBracket).is_none() {error_expected!("(");}

            let func_args = self.get_func_args();


            // consume the ')' token
            if self.try_consume(TokenType::RightBracket).is_none() {error_expected!(")");}

            let node_call = NodeTermFuncCall::new(ident, func_args);
            let term = NodeTerm::new(Terms::NodeTermFuncCall(node_call));
            return Some(term);
        }
        None
    }

}