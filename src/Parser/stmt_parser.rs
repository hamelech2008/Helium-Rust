use std::process::exit;
use crate::lexer::token::TokenType;
use crate::parser::ast_nodes::stmt;
use crate::parser::ast_nodes::stmt::{NodeStmt, NodeStmtAssign, NodeStmtElse, NodeStmtFuncCall, NodeStmtIf, NodeStmtReturn, Stmts};
use crate::parser::ast_nodes::stmt::Stmts::{NodeStmtForLoop, NodeStmtWhileLoop};
use crate::parser::Parser;
use crate::check_next_tokens;
use crate::error_expected;
use crate::parser::symbol_table::Symbol;

impl Parser
{
    // statement parsing
    pub(super) fn parse_stmt(&mut self, expect_semi_colon: bool) -> Option<NodeStmt>
    {
        // check for a declaration statement
        if check_next_tokens!(self, TokenType::Dec, TokenType::Ident, TokenType::Equals)
        {
            return self.parse_dec_stmt(expect_semi_colon);
        }

        // check for an if statement
        if check_next_tokens!(self, TokenType::If, TokenType::LeftBracket)
        {
            return self.parse_if_stmt();
        }

        // check for an assignment statement
        if check_next_tokens!(self, TokenType::Ident, TokenType::Equals)
        {
            return self.parse_assignment_stmt(expect_semi_colon);
        }

        // check for a for loop statement
        if check_next_tokens!(self, TokenType::For, TokenType::LeftBracket)
        {
            return self.parse_for_loop_stmt();
        }

        // check for a while loop statement
        if check_next_tokens!(self, TokenType::While, TokenType::LeftBracket)
        {
            return self.parse_while_loop_stmt();
        }

        // check for a return statement
        if self.try_peek(TokenType::Return)
        {
            return self.parse_return_stmt();
        }

        // check for a function call statement
        if check_next_tokens!(self, TokenType::Ident, TokenType::LeftBracket)
        {
            return self.parse_func_call_stmt();
        }
        None
    }

    fn parse_dec_stmt(&mut self, expect_semi_colon: bool) -> Option<NodeStmt>
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
            let data_type = Symbol::Variable(expr.get_data_type());
            // generate the declare statement node
            let node_dec = stmt::NodeStmtDec
            {
                ident,
                expr
            };

            self.symbol_table.insert(node_dec.ident.data.clone(), data_type);

            let stmt = NodeStmt::new(Stmts::NodeStmtDec(node_dec));

            return Some(stmt);
        }
        None
    }

    fn parse_if_stmt(&mut self) -> Option<NodeStmt>
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
            let stmt = NodeStmt::new(Stmts::NodeStmtIf(node_if));
            return Some(stmt);
        }
        None
    }

    fn parse_else_stmt(&mut self) -> Option<NodeStmtElse>
    {
        if self.try_consume(TokenType::Else).is_none() {error_expected!("else");}
        let else_node = NodeStmtElse::new(self.parse_scope());
        Some(else_node)
    }

    fn parse_assignment_stmt(&mut self, expect_semi_colon: bool) -> Option<NodeStmt>
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
            let data_type = Symbol::Variable(expr.get_data_type());
            self.symbol_table.insert(ident.clone().data, data_type); // TODO: replace .clone

            let node_assign = NodeStmtAssign::new(ident, expr);

            if expect_semi_colon
            {
                // consume the ';' token
                if self.try_consume(TokenType::SemiColon).is_none() {error_expected!(";");}
            }

            let stmt = NodeStmt::new(Stmts::NodeStmtAssign(node_assign));
            return Some(stmt);
        }
        None
    }

    fn parse_for_loop_stmt(&mut self) -> Option<NodeStmt>
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
        None
    }

    fn parse_while_loop_stmt(&mut self) -> Option<NodeStmt>
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
        None
    }

    fn parse_return_stmt(&mut self) -> Option<NodeStmt>
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
        None
    }

    fn parse_func_call_stmt(&mut self) -> Option<NodeStmt>
    {
        if let Some(ident) = self.try_consume(TokenType::Ident)
        {
            // consume the '(' token
            if self.try_consume(TokenType::LeftBracket).is_none() {error_expected!("(");}

            let func_args = self.get_func_args();

            // consume the ')' token
            if self.try_consume(TokenType::RightBracket).is_none() {error_expected!(")");}
            // consume the ';' token
            if self.try_consume(TokenType::SemiColon).is_none() {error_expected!(";");}

            let node_call = NodeStmtFuncCall::new(ident, func_args);
            let stmt = NodeStmt::new(Stmts::NodeStmtFuncCall(node_call));

            return Some(stmt);
        }
        None
    }
}