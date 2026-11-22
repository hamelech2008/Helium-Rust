pub mod ast_nodes;

pub mod stmt_parser;
pub mod term_parser;

pub mod class_parser;
pub mod symbol_table;

use std::process::exit;
use crate::class_table::ClassTable;
use crate::lexer::{binary_prec};
use crate::parser::ast_nodes::{DataType, NodeProg, NodeScope};
use crate::lexer::token::{Token, TokenType};
use crate::parser::ast_nodes::expr::{BinOp, Exprs, LogicOp, NodeArrayAccess, NodeBinExpr, NodeExpr, NodeLogicExpr, NodeMemberAccess, NodeTerneryExpr, Operator};
use crate::parser::ast_nodes::functions::{FuncDecVar, NodeFuncDecl};
use crate::parser::ast_nodes::term::{NodeTerm, NodeTermFuncCall, Terms};
use crate::parser::symbol_table::{Symbol, SymbolTable};

#[macro_export]
macro_rules! check_next_tokens {
    ($self:expr, $($token:expr),+ $(,)?) => {
        [$($token),+]
            .iter()
            .enumerate()
            .all(|(offset, token)| $self.try_peek_offset(*token, offset as i32))
    };
}

#[macro_export]
macro_rules! error_expected {
    ($this:ident, $token:expr) =>
    {
        let previous = $this.peek_offset(-1).unwrap();
        let row = previous.row;
        let col = previous.col + previous.data.chars().count() as u32;

        eprintln!("Parse Error: expected token '{}' at {}:{}", $token, row, col);
        exit(-1);
    }
}

#[macro_export]
macro_rules! type_error {
    ($this:ident, $token:expr) =>
    {
        let previous = $this.peek_offset(-1).unwrap();
        let row = previous.row;
        let col = previous.col + previous.data.chars().count() as u32;

        eprintln!("Parse Error: expected type '{}' at {}:{}", $token, row, col);
        exit(-1);
    }
}

pub struct Parser
{

    tokens : Vec<Token>,
    index: i32,
    pub symbol_table : SymbolTable,
    pub class_table: ClassTable
}

impl Parser
{
    pub fn new(tokens: Vec<Token>) -> Self
    {
        Self
        {
            tokens,
            index: 0,
            symbol_table: SymbolTable::new(),
            class_table: ClassTable::new()
        }
    }

    pub fn parse(&mut self) -> NodeProg
    {
        let mut prog: NodeProg = NodeProg::new();

        while self.peek().is_some()
        {
            if let Some(class) = self.parse_class()
            {
                prog.add_class(class);
            }
            else if let Some(function) = self.parse_function() {prog.add_function(function);}
            else if let Some(stmt) = self.parse_stmt(true) {prog.add_stmt(stmt);}
        }

        prog
    }





    // function parsing
    fn parse_function_param(&mut self) -> Option<FuncDecVar> // parse the params of a function when declared
    {
        // consume the 'dec' token
        if self.try_consume(TokenType::Dec).is_none() {error_expected!(self, "dec");}
        if let Some(ident) = self.try_consume(TokenType::Ident)
        {
            // consume the ':' token
            if self.try_consume(TokenType::Colon).is_none() {error_expected!(self, ":");}

            if let Some(data_type) = self.parse_explicit_type()
            {

                // TODO: fix func args and sym table
                let sym = Symbol::Variable(data_type.clone());
                self.symbol_table.insert(ident.clone().data, sym);

                let func_param = FuncDecVar::new(ident, data_type);
                return Some(func_param);
            }


        }
        None
    }
    fn parse_function(&mut self) -> Option<NodeFuncDecl> // parse a function declaration
    {
        // consume the 'function' token
        if self.try_consume(TokenType::Function).is_none() {error_expected!(self, "function");}

        if let Some(ident) = self.try_consume(TokenType::Ident)
        {

            // consume the '(' token
            if self.try_consume(TokenType::LeftBracket).is_none() {error_expected!(self, "(");}


            let mut func_params = Vec::new();

            // consume all the function parameters
            while !self.try_peek(TokenType::RightBracket)
            {
                if let Some(func_param) = self.parse_function_param()
                {
                    func_params.push(func_param);
                }


                if self.try_peek(TokenType::Comma)
                {

                    // consume the ',' token
                    if self.try_consume(TokenType::Comma).is_none() {error_expected!(self, ",");}
                }

            }

            // consume the ')' token
            if self.try_consume(TokenType::RightBracket).is_none() {error_expected!(self, ")");}


            // check for "-> return_type" tokens
            if check_next_tokens!(self, TokenType::Subtraction, TokenType::Greater, TokenType::Ident)
            {
                if self.try_consume(TokenType::Subtraction).is_none() {error_expected!(self, "-");}
                if self.try_consume(TokenType::Greater).is_none() {error_expected!(self, ">");}
                if let Some(return_data_type) = self.parse_explicit_type()
                {
                    let symbol = Symbol::Function {return_type: return_data_type};
                    self.symbol_table.insert(ident.clone().data, symbol);
                }
            }
            let scope : Option<NodeScope>;

            // function declaration (not implementations)
            if self.try_peek(TokenType::SemiColon)
            {
                self.consume(); // consume the ';' token
                scope = None;
            }
            else
            {
                scope = Some(self.parse_scope());
            }
            let node_function_decl = NodeFuncDecl::new(ident, scope, func_params);

            return Some(node_function_decl);
        }
        None
    }

    fn get_func_args(&mut self) -> Vec<NodeExpr> // function to get a list of the arguments that are passed when a function is called
    {
        let mut func_args = Vec::new();

        // consume all the arguments passed to the function
        while !self.try_peek(TokenType::RightBracket)
        {
            if let Some(expr_arg) = self.parse_expr(0)
            {
                func_args.push(expr_arg);
            }

            if self.try_peek(TokenType::Comma)
            {
                // consume the ',' token
                if self.try_consume(TokenType::Comma).is_none() {error_expected!(self, ",");}
            }

        }
        func_args
    }

    fn parse_explicit_type(&mut self) -> Option<DataType>
    {
        // consume the next token
        let token = self.consume().unwrap();

        // if it's not an ident, return None (invalid type)
        if token.token_type != TokenType::Ident {return None;}

        // check for the 'Array' keyword
        if token.data == "Array"
        {
            // if after the keyword there isn't a data type, return None
            if self.try_consume(TokenType::Lesser).is_none() {error_expected!(self, '<');}

            // consume the inner type and make sure the next token is '>'
            let inner_type = self.parse_explicit_type().unwrap();
            if self.try_consume(TokenType::Greater).is_none() {error_expected!(self, '>');}

            // call it recursively
            return Some(DataType::Array(Box::new(inner_type)));
        }

        // else return the basic type
        Some(DataType::try_from(token.data.as_str()).unwrap())
    }







    // expression parsing
    fn parse_expr(&mut self, min_prec: i32) -> Option<NodeExpr>
    {
        let term_lhs = self.parse_term();
        if term_lhs.is_none() {return None;}
        let term_lhs = match term_lhs
        {
            Some(term) => term,
            None => exit(1)
        };
        let expr_lhs_data_type = term_lhs.get_data_type().clone();

        let node_original_lhs = NodeExpr::new(
            Exprs::Term(term_lhs.clone()),
            expr_lhs_data_type
        );

        let mut expr_lhs = self.parse_postfix(node_original_lhs.clone()).unwrap_or_else(|| node_original_lhs);

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
                TokenType::Modulo => Operator::BinOp(BinOp::Modulo),
                TokenType::LesserOrEqual => Operator::BinOp(BinOp::LesserOrEqual),
                TokenType::GreaterOrEqual => Operator::BinOp(BinOp::GreaterOrEqual),
                TokenType::Lesser => Operator::BinOp(BinOp::Lesser),
                TokenType::Greater => Operator::BinOp(BinOp::Greater),
                TokenType::EqualsTo => Operator::BinOp(BinOp::EqualsTo),
                TokenType::NotEqual => Operator::BinOp(BinOp::NotEqual),
                TokenType::And => Operator::LogicOp(LogicOp::And),
                TokenType::Or => Operator::LogicOp(LogicOp::Or),
                TokenType::Ternary => Operator::TerneryOp,

                _ => exit(1), // TODO: Replace with proper error handling
            };

            match bin_op
            {
                Operator::BinOp(bin_op) =>
                    {
                        let expr_rhs = expr_rhs.unwrap();

                        let res_type = self.infer_bin_op_type(&expr_lhs.get_data_type(), &expr_rhs.get_data_type());

                        if res_type.is_none()
                        {
                            type_error!(self, String::from(expr_rhs.get_data_type()));
                        }

                        let res_type = res_type.unwrap();

                        let bin_node = NodeBinExpr::new(expr_lhs.clone(), expr_rhs, bin_op);
                        expr_lhs.set_var(Exprs::BinExpr(bin_node));
                        expr_lhs.set_data_type(res_type);
                    },
                Operator::LogicOp(logic_op) =>
                    {
                        let logic_node = NodeLogicExpr::new(expr_lhs.clone(), expr_rhs.unwrap(), logic_op);
                        expr_lhs.set_var(Exprs::LogicExpr(logic_node));
                    },
                Operator::TerneryOp =>
                    {
                        // consume the ':' token for ternery, e.g: 1 ? 2 : 3
                        if self.try_consume(TokenType::Colon).is_none() {error_expected!(self, ":");}

                        // parse the ternery "false" branch
                        if let Some(false_expr_branch) = self.parse_expr(next_min_prec)
                        {
                            if expr_lhs.get_data_type() != expr_lhs.get_data_type()
                            {
                                type_error!(self, "test"); // TODO: replace with actual type
                            }

                            let ternery_node = NodeTerneryExpr::new(
                                expr_lhs.clone(), expr_rhs.unwrap(), false_expr_branch, expr_lhs.get_data_type()
                            );

                            expr_lhs.set_var(Exprs::TerneryExpr(ternery_node));
                        }
                    }
            }

        }
        Some(expr_lhs)
    }



    fn parse_postfix(&mut self, mut expr: NodeExpr) -> Option<NodeExpr>
    {
        loop
        {
            // get the token (account for last token)
            let token = match self.peek()
            {
                Some(token) => token,
                None => return Some(expr),
            };

            // match the token
            match token.token_type
            {
                // array indexing ('[')
                TokenType::LeftSquare =>
                    {
                        self.consume(); // consume the '[' token

                        // parse the index
                        if let Some(index) = self.parse_expr(0)
                        {
                            // consume the ']' token
                            if self.try_consume(TokenType::RightSquare).is_none() {error_expected!(self, "]");}

                            // get the type of the element
                            let element_type = match expr.get_data_type()
                            {
                                DataType::Array(inner) => (*inner).clone(),
                                _ => return None
                            };

                            // create the access node
                            let node_access = NodeArrayAccess::new(
                                expr, index, element_type.clone()
                            );

                            // update the expression
                            expr = NodeExpr::new(Exprs::ArrayAccess(node_access), element_type);
                        }
                    },

                TokenType::Dot =>
                    {
                        self.consume(); // consume the '.' token
                        if self.try_peek(TokenType::Ident)
                        {
                            if let Some(member) = self.try_consume(TokenType::Ident)
                            {
                                if self.try_peek(TokenType::LeftBracket) // method call
                                {
                                    self.consume(); // consume the '(' token

                                    let mut args = self.get_func_args();

                                    // consume the ')' token
                                    if self.try_consume(TokenType::RightBracket).is_none() {error_expected!(self, ")");}

                                    args.push(expr.clone());

                                    let mut node_method_access = NodeTermFuncCall::new(member.clone(), args);
                                    if let Some(method) = self.class_table.get_class_method(&expr, &member)
                                    {
                                        let method_data_type = self.symbol_table.get_function_data_type(&method.get_ident().data);
                                        node_method_access.prefix_ident(&self.class_table.get_class_name(&expr));
                                        expr = NodeExpr::new(Exprs::Term(NodeTerm::new(Terms::NodeTermFuncCall(node_method_access), method_data_type.clone())), method_data_type);
                                    }

                                }
                                else // member access
                                {

                                    let node_obj_access = NodeMemberAccess::new(expr.clone(), member.clone());

                                    // get the data type of the field
                                    if let Some(class_dec_var) = self.class_table.get_class_member(&expr, &member)
                                    {
                                        expr = NodeExpr::new(Exprs::NodeMemberAccess(node_obj_access), class_dec_var.get_data_type().clone());
                                    }
                                }
                            }
                            else {error_expected!(self, "ident");}
                        }
                    }
                _ => return Some(expr)
            }
        }
    }


    // scope parsing
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
            error_expected!(self, "scope");
        }

        let mut node_scope = NodeScope::new();

        while let Some(stmt) = self.parse_stmt(true)
        {
            node_scope.push_stmt(stmt);
        }

        // consume the "}" token
        if self.try_consume(TokenType::RightCurly).is_none() {error_expected!(self, "}");}

        node_scope
    }


    // get data type of binary operation
    pub fn infer_bin_op_type(&self, lhs: &DataType, rhs: &DataType) -> Option<DataType>
    {
        match (lhs, rhs)
        {
            (DataType::Integer, DataType::Integer) => Some(DataType::Integer),
            (DataType::Float, DataType::Float) | (DataType::Float, DataType::Integer) => Some(DataType::Float),

            (DataType::Char, DataType::Integer) |
            (DataType::Integer, DataType::Char) |
            (DataType::Char, DataType::Char) => Some(DataType::Integer),
            _ => None
        }
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