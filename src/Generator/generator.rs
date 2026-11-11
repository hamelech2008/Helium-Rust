use std::collections::HashMap;
use std::process::exit;
use crate::parser::ast_nodes::NodeProg;
use crate::parser::ast_nodes::expr::{BinOp, Exprs, NodeBinExpr, NodeExpr};
use crate::parser::ast_nodes::stmt::{NodeStmt, Stmts};
use crate::parser::ast_nodes::term::{NodeTerm, NodeTermBracket, NodeTermIdent, NodeTermIntLiteral, Terms};
mod instructions;


pub struct Generator
{
    prog: NodeProg,
    output: String,
    stack_size: u32,
    vars: HashMap<String, u32>
}


impl Generator
{
    pub fn new(prog: NodeProg) -> Self
    {
        Self
        {
            prog,
            output: String::with_capacity(4096),
            stack_size: 1,
            vars: HashMap::new()
        }
    }

    pub fn generate_prog(mut self) -> String
    {
        self.output.push_str(".global _start\n");
        self.output.push_str(".section .text\n");
        self.output.push_str("_start:\n");
        let stmts = self.prog.get_stmts().to_vec();

        for stmt in &stmts
        {
            self.gen_stmt(stmt);
        }


        self.output
    }

    fn gen_stmt(&mut self, stmt: &NodeStmt)
    {
        match &stmt.get_var()
        {
            Stmts::NodeStmtDec(node_dec) =>
                {
                    if self.vars.contains_key(&node_dec.ident.data)
                    {
                        exit(0); // TODO: replace with error expected
                    }
                    self.vars.insert(node_dec.ident.data.to_string(), self.stack_size);
                    self.gen_expr(&node_dec.expr);
                },
            _ => exit(0) // TODO: replace with error expected
        }
    }


    // Expressions
    fn gen_expr(&mut self, expr: &NodeExpr)
    {
        match &expr.get_var()
        {
            Exprs::Term(node_term) => self.gen_term(node_term),
            Exprs::BinExpr(node_bin_expr) => self.gen_bin_expr(node_bin_expr),
            _ => exit(0) // TODO: replace with error expected
        }
    }


    // Terms
    fn gen_term(&mut self, term: &NodeTerm)
    {
        match term.get_var()
        {
            Terms::IntLiteral(node_int_literal) => self.gen_int_literal_term(node_int_literal),
            Terms::Ident(node_ident) => self.gen_ident_term(node_ident),
            Terms::Bracket(node_bracket) => self.gen_bracket_term(node_bracket),
            _ => exit(0) // TODO: replace with error expected
        }
    }

    fn gen_int_literal_term(&mut self, node_int_literal: &NodeTermIntLiteral)
    {
        let literal_value = &*node_int_literal.get_value().data;
        instructions::movq_immediate(self, literal_value, "%rax");
        instructions::push(self, "%rax");
    }

    fn gen_ident_term(&mut self, node_ident: &NodeTermIdent)
    {
        let stack_location = match self.vars.get(&node_ident.ident.data)
        {
            Some(val) => val * 8,
            None => exit(0) // TODO: replace with error expected
        };

        instructions::movq(self, &format!("{stack_location}(%rsp)"), "%rax");
        instructions::push(self, "%rax");
    }

    fn gen_bracket_term(&mut self, node_bracket: &NodeTermBracket)
    {
        self.gen_expr(node_bracket.get_expr());
    }


    // Binary Expressions
    fn gen_bin_expr(&mut self, bin_expr: &NodeBinExpr)
    {
        match bin_expr.get_op()
        {
            BinOp::Addition => self.gen_bin_expr_add(bin_expr),
            BinOp::Multiplication => self.gen_bin_expr_mul(bin_expr),
            BinOp::Division => self.gen_bin_expr_div(bin_expr),
            _ => exit(0) // TODO: replace with error expected
        }
    }

    fn gen_bin_expr_add(&mut self, bin_expr_add: &NodeBinExpr)
    {
        self.gen_expr(bin_expr_add.get_rhs());
        self.gen_expr(bin_expr_add.get_lhs());
        instructions::pop(self, "%rax");
        instructions::pop(self, "%rbx");
        instructions::addq(self, "%rbx", "%rax");

    }
    fn gen_bin_expr_mul(&mut self, bin_expr_mul: &NodeBinExpr)
    {
        self.gen_expr(bin_expr_mul.get_rhs());
        self.gen_expr(bin_expr_mul.get_lhs());
        instructions::pop(self, "%rax");
        instructions::pop(self, "%rbx");
        instructions::mulq(self, "%rbx", "%rax");
    }
    fn gen_bin_expr_div(&mut self, bin_expr_div: &NodeBinExpr) {}



    // helper functions


}





