pub mod ast_nodes;

use std::process::exit;
use crate::lexer::{binary_prec};
use crate::parser::ast_nodes::{stmt, NodeProg, NodeScope};
use crate::lexer::token::{Token, TokenType};
use crate::parser::ast_nodes::expr::{BinOp, Exprs, LogicOp, NodeBinExpr, NodeExpr, NodeLogicExpr, NodeTerneryExpr, Operator, TerneryOp};
use crate::parser::ast_nodes::functions::{FuncDecVar, NodeFuncDecl};
use crate::parser::ast_nodes::stmt::{NodeStmt, NodeStmtAssign, NodeStmtElse, NodeStmtIf, NodeStmtReturn, Stmts};
use crate::parser::ast_nodes::stmt::Stmts::{NodeStmtForLoop, NodeStmtWhileLoop};
use crate::parser::ast_nodes::term::{NodeTerm, NodeTermBracket, Terms};
macro_rules! check_next_tokens {
    ($self:expr, $($token:expr),+ $(,)?) => {
        [$($token),+]
            .iter()
            .enumerate()
            .all(|(offset, token)| $self.try_peek_offset(*token, offset as i32))
    };
}

macro_rules! error_expected {
    ($token:expr) =>
    {
        eprintln!("Parse Error: expected token '{}' at line <ADD LINE>", $token);
        exit(-1);
    }
}

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
            if let Some(function) = self.parse_function() {prog.add_function(function);}
            if let Some(stmt) = self.parse_stmt(true) {prog.add_stmt(stmt);}
        }

        prog
    }


    // function parsing

    fn parse_function_param(&mut self) -> Option<FuncDecVar>
    {
        // consume the 'dec' token
        if self.try_consume(TokenType::Dec).is_none() {error_expected!("dec");}
        if let Some(ident) = self.try_consume(TokenType::Ident)
        {
            let func_param = FuncDecVar::new(ident);
            return Some(func_param);
        }
        None
    }
    fn parse_function(&mut self) -> Option<NodeFuncDecl>
    {
        // consume the 'function' token
        if self.try_consume(TokenType::Function).is_none() {error_expected!("function");}

        if let Some(ident) = self.try_consume(TokenType::Ident)
        {

            // consume the '(' token
            if self.try_consume(TokenType::LeftBracket).is_none() {error_expected!("(");}


            let mut func_params = Vec::new();

            while !self.try_peek(TokenType::RightBracket)
            {
                if let Some(func_param) = self.parse_function_param()
                {
                    func_params.push(func_param);
                }

                if self.try_peek(TokenType::Comma)
                {

                    // consume the ',' token
                    if self.try_consume(TokenType::Comma).is_none() {error_expected!(",");}
                }

            }

            // consume the ')' token
            if self.try_consume(TokenType::RightBracket).is_none() {error_expected!(")");}

            let scope = self.parse_scope();
            let node_function_decl = NodeFuncDecl::new(ident, scope, func_params);

            return Some(node_function_decl);
        }
        None
    }


    // statement parsing
    fn parse_stmt(&mut self, expect_semi_colon: bool) -> Option<NodeStmt>
    {

        // check for a declaration statement
        if check_next_tokens!(self, TokenType::Dec, TokenType::Ident, TokenType::Equals)
        {
            // consume 'dec' token
            if self.try_consume(TokenType::Dec).is_none() {error_expected!("dec");}

            // consume ident value
            let ident = match self.try_consume(TokenType::Ident)
            {
                Some(ident) => ident,
                None =>
                    {
                        error_expected!("ident");
                    }
            };

            // consume '=' token
            if self.try_consume(TokenType::Equals).is_none() {error_expected!("=");}
            // consume rhs expression (for e.g: dec var = 20 + 20; - 20 + 20 is the expression)
            if let Some(expr) = self.parse_expr(0)
            {
                // consume ';' token
                if expect_semi_colon
                {
                    if self.try_consume(TokenType::SemiColon).is_none()
                    {
                        error_expected!(";");
                    }
                }

                // generate the declare statement node
                let node_dec = stmt::NodeStmtDec
                {
                    ident,
                    expr
                };
                let stmt = NodeStmt::new(stmt::Stmts::NodeStmtDec(node_dec));

                return Some(stmt)
            }

        }

        // check for an if statement
        if check_next_tokens!(self, TokenType::If, TokenType::LeftBracket)
        {
            // consume the 'if' token
            if self.try_consume(TokenType::If).is_none() {error_expected!("if");}
            // consume the '(' token
            if self.try_consume(TokenType::LeftBracket).is_none() {error_expected!("(");}


            if let Some(expr) = self.parse_expr(0)
            {
                // consume the ')' token
                if self.try_consume(TokenType::RightBracket).is_none() {error_expected!(")");}

                let mut node_if = NodeStmtIf::new(expr, self.parse_scope());

                if self.try_peek(TokenType::Else)
                {
                    node_if.set_else(self.parse_else_stmt());
                }
                let stmt = NodeStmt::new(stmt::Stmts::NodeStmtIf(node_if));
                return Some(stmt);
            }
        }

        // check for an assignment statement
        if check_next_tokens!(self, TokenType::Ident, TokenType::Equals)
        {
            let ident = match self.try_consume(TokenType::Ident)
            {
                Some(ident) => ident,
                None => {error_expected!("ident");}
            };

            // consume the '=' token
            if self.try_consume(TokenType::Equals).is_none() {error_expected!("=");}

            if let Some(expr) = self.parse_expr(0)
            {
                let node_assign = NodeStmtAssign::new(ident, expr);

                if expect_semi_colon
                {
                    // consume the ';' token
                    if self.try_consume(TokenType::SemiColon).is_none() {error_expected!(";");}
                }

                let stmt = NodeStmt::new(stmt::Stmts::NodeStmtAssign(node_assign));
                return Some(stmt);
            }
        }

        // check for a for loop statement
        if check_next_tokens!(self, TokenType::For, TokenType::LeftBracket)
        {

            // consume the 'for' token
            if self.try_consume(TokenType::For).is_none() {error_expected!("for");}
            // consume the '(' token
            if self.try_consume(TokenType::LeftBracket).is_none() {error_expected!("(");}

            // checks for: (stmt; expr; stmt;) scope
            if let Some(init_stmt) = self.parse_stmt(true) &&
                let Some(expr) = self.parse_expr(0)
            {

                // consume the ';' token
                if self.try_consume(TokenType::SemiColon).is_none() {error_expected!(";");}

                if let Some(end_stmt) = self.parse_stmt(false)
                {

                    // consume the ')' token
                    if self.try_consume(TokenType::RightBracket).is_none() {error_expected!(")");}

                    let scope = self.parse_scope();
                    let node_for = stmt::NodeStmtForLoop::new(init_stmt, expr, end_stmt, scope);

                    let stmt = NodeStmt::new(NodeStmtForLoop(node_for));
                    return Some(stmt);
                }
            }
        }

        // check for a while loop statement
        if check_next_tokens!(self, TokenType::While, TokenType::LeftBracket)
        {
            // consume the 'for' token
            if self.try_consume(TokenType::While).is_none() {error_expected!("while");}
            // consume the '(' token
            if self.try_consume(TokenType::LeftBracket).is_none() {error_expected!("(");}

            if let Some(expr) = self.parse_expr(0)
            {
                // consume the ')' token
                if self.try_consume(TokenType::RightBracket).is_none() {error_expected!(")");}

                let scope = self.parse_scope();

                let node_while = stmt::NodeStmtWhileLoop::new(expr, scope);

                let stmt = NodeStmt::new(NodeStmtWhileLoop(node_while));
                return Some(stmt);
            }
        }

        // check for a return statement
        if self.try_peek(TokenType::Return)
        {

            // consume the return statement
            if self.try_consume(TokenType::Return).is_none() {error_expected!("return");}

            if self.try_peek(TokenType::SemiColon)
            {
                // consume the ';' token
                if self.try_consume(TokenType::SemiColon).is_none() {error_expected!(";");}

                let return_node = NodeStmtReturn::new(None);
                return Some(NodeStmt::new(Stmts::NodeStmtReturn(return_node)));
            }
            if let Some(expr) = self.parse_expr(0)
            {
                // consume the ';' token
                if self.try_consume(TokenType::SemiColon).is_none() {error_expected!(";");}

                let return_node = NodeStmtReturn::new(Some(expr));
                return Some(NodeStmt::new(Stmts::NodeStmtReturn(return_node)));
            }

        }

        None
    }

    fn parse_else_stmt(&mut self) -> Option<NodeStmtElse>
    {
        if self.try_consume(TokenType::Else).is_none() {error_expected!("else");}
        let else_node = NodeStmtElse::new(self.parse_scope());
        Some(else_node)
    }


    fn parse_expr(&mut self, min_prec: i32) -> Option<NodeExpr>
    {
        let term_lhs = self.parse_term();
        if term_lhs.is_none() {return None;}

        let term_lhs = match term_lhs
        {
            Some(term) => term,
            None => exit(1)
        };

        let mut expr_lhs = NodeExpr::new(Exprs::Term(term_lhs));

        // loop on the expressions and get the precedence of the current token
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

            // get the operation (+, -, /, *...)
            let op = self.consume();
            let next_min_prec = prec.unwrap() + 1; // get the next precedence

            // parse right hand side of the expression
            let expr_rhs = self.parse_expr(next_min_prec);
            if expr_rhs.is_none() || op.is_none() {exit(1);} // TODO: replace with error expected

            // get the binary operation
            let bin_op: Operator = match op.unwrap().token_type
            {
                TokenType::Addition => Operator::BinOp(BinOp::Addition),
                TokenType::Subtraction => Operator::BinOp(BinOp::Subtraction),
                TokenType::Multiplication => Operator::BinOp(BinOp::Multiplication),
                TokenType::Division => Operator::BinOp(BinOp::Division),
                TokenType::Lesser => Operator::BinOp(BinOp::Lesser),
                TokenType::Greater => Operator::BinOp(BinOp::Greater),
                TokenType::EqualsTo => Operator::BinOp(BinOp::EqualsTo),
                TokenType::And => Operator::LogicOp(LogicOp::And),
                TokenType::Or => Operator::LogicOp(LogicOp::Or),
                TokenType::Ternary => Operator::TerneryOp(TerneryOp::Ternery),

                _ => exit(1), // TODO: Replace with proper error handling
            };

            match bin_op
            {
                Operator::BinOp(bin_op) =>
                    {
                        let bin_node = NodeBinExpr::new(expr_lhs.clone(), expr_rhs.unwrap(), bin_op);
                        expr_lhs.set_var(Exprs::BinExpr(bin_node));
                    },
                Operator::LogicOp(logic_op) =>
                    {
                        let logic_node = NodeLogicExpr::new(expr_lhs.clone(), expr_rhs.unwrap(), logic_op);
                        expr_lhs.set_var(Exprs::LogicExpr(logic_node));
                    },
                Operator::TerneryOp(tern_op) =>
                    {
                        // consume the ':' token for ternery, e.g: 1 ? 2 : 3
                        if self.try_consume(TokenType::Colon).is_none() {error_expected!(":");}

                        // parse the ternery "false" branch
                        if let Some(false_expr_branch) = self.parse_expr(next_min_prec)
                        {
                            let ternery_node = NodeTerneryExpr::new(
                                expr_lhs.clone(), expr_rhs.unwrap(), false_expr_branch, tern_op
                            );

                            expr_lhs.set_var(Exprs::TerneryExpr(ternery_node));
                        }
                    }
            }

        }
        Some(expr_lhs)
    }



    // term parsing
    fn parse_term(&mut self) -> Option<NodeTerm>
    {
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



    fn parse_scope(&mut self) -> NodeScope
    {
        if self.try_consume(TokenType::LeftCurly).is_none()
        {
            if let Some(stmt) = self.parse_stmt(true)
            {
                let mut node_scope = NodeScope::new();
                node_scope.push_stmt(stmt);
                return node_scope;
            }
            exit(1); // TODO: replace with error expected
        }

        let mut node_scope = NodeScope::new();

        while let Some(stmt) = self.parse_stmt(true)
        {
            node_scope.push_stmt(stmt);
        }

        // consume the "}" token
        if self.try_consume(TokenType::RightCurly).is_none() {error_expected!("}");}

        node_scope
    }


    // Helper functions

    // returns the value (if exists) of index
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

    fn try_peek_offset(&mut self, token_type: TokenType, offset: i32) -> bool
    {
        let token_value = match self.peek_offset(offset)
        {
            Some(t) => t,
            None =>
                {
                    return false;
                }
        };

        self.peek_offset(offset).is_some() && token_value.token_type == token_type
    }
    fn try_peek(&mut self, token_type: TokenType) -> bool
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
            None => { return None; }
        };

        if token_value.token_type == token
        {
            Some(self.consume().unwrap())
        }
        else {None}

    }
}