mod ast_nodes;

use std::process::exit;
use crate::lexer::{binary_prec};
use crate::parser::ast_nodes::NodeProg;
use crate::lexer::token::{Token, TokenType};
use crate::parser::ast_nodes::expr::{BinOp, Exprs, NodeBinExpr, NodeExpr};
use crate::parser::ast_nodes::stmt::NodeStmt;

pub struct Parser
{

    tokens : Vec<Token>,
    index: i32
}

impl Parser
{
    pub fn new(tokens: Vec<Token>) -> Self
    {
        Self
        {
            tokens,
            index: 0
        }
    }

    pub fn parse(&mut self) -> NodeProg
    {
        let mut prog: NodeProg = NodeProg::new();

        while self.peek().is_some()
        {
            if let Some(stmt) = self.parse_stmt() {prog.add_stmt(stmt);}
        }

        prog
    }



    fn parse_stmt(&mut self) -> Option<NodeStmt>
    {
        if self.try_peek(TokenType::Dec) && self.try_peek_offset(TokenType::Ident, 1)
            && self.try_peek_offset(TokenType::Equals, 2)
        {
            self.try_consume(TokenType::Dec); // consume 'dec' token

            // consume ident value
            let ident = match self.try_consume(TokenType::Ident)
            {
                Some(i) => i.clone(),
                None => exit(0) // TODO: replace with error expected
            };
            self.try_consume(TokenType::Equals); // consume '=' token
            // consume rhs expression (for e.g: dec var = 20 + 20; - 20 + 20 is the expression)
            if let Some(expr) = self.parse_expr(0)
            {
                self.try_consume(TokenType::SemiColon); // consume ';' token

                // generate the declare statement node
                let node_dec = ast_nodes::stmt::NodeStmtDec
                {
                    ident,
                    expr
                };
                let stmt = NodeStmt::new(ast_nodes::stmt::Stmts::NodeStmtDec(node_dec));

                return Some(stmt)
            }

        }
        None
    }

    fn parse_expr(&mut self, min_prec: i32) -> Option<NodeExpr>
    {
        let term_lhs = self.parse_term();
        if term_lhs.is_none() {return None;}

        let term_lhs = match term_lhs
        {
            Some(term) => term,
            None => exit(0)
        };

        let mut expr_lhs = NodeExpr::new(Exprs::Term(term_lhs));

        loop
        {
            let curr_token = self.peek();
            let prec : Option<i32>;

            if curr_token.is_some()
            {
                prec = binary_prec(curr_token.unwrap().token_type);
                if prec.is_none() || prec < Option::from(min_prec) {break;}
            }
            else {break;}

            let op = self.consume();
            let next_min_prec = prec.unwrap() + 1;

            let expr_rhs = self.parse_expr(next_min_prec);
            if expr_rhs.is_none() || op.is_none() {exit(0);} // TODO: replace with error expected

            match op.unwrap().token_type
            {
                TokenType::Addition =>
                    {
                        let add = NodeBinExpr::new(expr_lhs.clone(), expr_rhs.unwrap(),BinOp::Addition);
                        expr_lhs.set_var(Exprs::BinExpr(add));
                    },
                TokenType::Multiplication =>
                    {
                        let mul = NodeBinExpr::new(expr_lhs.clone(), expr_rhs.unwrap(),BinOp::Multiplication);
                        expr_lhs.set_var(Exprs::BinExpr(mul));                    }


                _ => {}
            }
        }
        Some(expr_lhs)
    }


    fn parse_term(&mut self) -> Option<ast_nodes::term::NodeTerm>
    {
        if let Some(literal) = self.try_consume(TokenType::IntLiteral)
        {
            let node_term_literal = ast_nodes::term::NodeTermIntLiteral
            {
                value: literal.clone()
            };

            let term =
                ast_nodes::term::NodeTerm::new(ast_nodes::term::Terms::IntLiteral(node_term_literal));

            return Some(term);
        }
        None
    }



    // Helper functions

    // return the value (if exists) of index
    fn peek(&self) -> Option<&Token>
    {
        self.peek_offset(0)
    }

    // return the value (if exists) of index + offset
    fn peek_offset(&self, offset: i32) -> Option<&Token>
    {
        // if exceeds upper and lower bounds of the vector, return none
        if self.index + offset < 0 || self.index + offset >= self.tokens.len() as i32
        {
            None
        }
        else
        {
            // return the value at index + offset
            Some(&self.tokens[(self.index + offset) as usize])
        }
    }

    fn try_peek_offset(&self, token_type: TokenType, offset: i32) -> bool
    {
        let token_value = match self.peek_offset(offset)
        {
            Some(t) => t,
            None => exit(0) // TODO: replace with an error expected function
        };

        self.peek_offset(offset).is_some() && token_value.token_type == token_type
    }
    fn try_peek(&self, token_type: TokenType) -> bool
    {
        self.try_peek_offset(token_type, 0)
    }


    // consumes the value at index and returns it
    fn consume(&mut self) -> Option<Token>
    {
        if self.peek().is_some()
        {
            // return the value of index and increase index
            let val = Some(self.tokens[self.index as usize].clone());
            self.index += 1;
            val
        }
        else {None}
    }


    // trys to consume the current token if it's a specific one
    fn try_consume(&mut self, token: TokenType) -> Option<Token>
    {
        // extract the value from the tokens
        let token_value = match self.peek()
        {
            Some(t) => t,
            None => exit(0) // TODO: replace with an error expected function
        };

        // check for the current token and return it if found (else returns None)
        if self.peek().is_some() && token_value.token_type == token
        {
            self.consume()
        }
        else {None}
    }
}