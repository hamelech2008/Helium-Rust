use std::collections::HashMap;
use std::process::exit;
use crate::parser::ast_nodes::{NodeProg, NodeScope};
use crate::parser::ast_nodes::expr::{BinOp, Exprs, NodeBinExpr, NodeExpr};
use crate::parser::ast_nodes::stmt::{NodeStmt, NodeStmtAssign, NodeStmtDec, NodeStmtElse, NodeStmtForLoop, NodeStmtIf, Stmts};
use crate::parser::ast_nodes::term::{NodeTerm, NodeTermBracket, NodeTermIdent, NodeTermIntLiteral, Terms};
mod instructions;


pub struct Generator
{
    prog: NodeProg,
    output: String,
    stack_size: u32,
    vars: HashMap<String, u32>,
    label_counter: u32
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
            vars: HashMap::new(),
            label_counter: 0
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
            Stmts::NodeStmtDec(node_dec) => self.gen_dec_stmt(node_dec),
            Stmts::NodeStmtIf(node_if) => self.gen_if_stmt(node_if),
            Stmts::NodeStmtAssign(node_assign) => self.gen_assign_stmt(node_assign),
            Stmts::NodeStmtForLoop(node_for_loop) => self.gen_for_loop_stmt(node_for_loop),
            _ => exit(0) // TODO: replace with error expected
        }
    }

    fn gen_dec_stmt(&mut self, node_dec: &NodeStmtDec)
    {
        if self.vars.contains_key(&node_dec.ident.data) // variable declared earlier
        {
            exit(1); // TODO: replace with error expected
        }
        self.vars.insert(node_dec.ident.data.to_string(), self.stack_size);
        self.gen_expr(&node_dec.expr);
    }

    fn gen_if_stmt(&mut self, node_if: &NodeStmtIf)
    {
        let after_if_label = self.gen_label();
        let after_else_label = self.gen_label();
        self.gen_expr(node_if.get_expr());

        instructions::pop(self, "%rax");
        instructions::testq(self, "%rax");
        instructions::jz(self, &after_if_label);

        self.gen_scope(node_if.get_scope());

        instructions::jmp(self, &after_else_label);


        instructions::insert_label(self, &after_if_label);
        match node_if.get_else_scope()
        {
            Some(node_else) => self.gen_else_stmt(node_else),
            None => exit(1) // TODO: replace with error expected
        }

        instructions::insert_label(self, &after_else_label);
    }
    fn gen_else_stmt(&mut self, node_else: &NodeStmtElse)
    {
        self.gen_scope(node_else.get_scope());
    }

    fn gen_scope(&mut self, node_scope: &NodeScope)
    {
        for stmt in node_scope.get_stmts()
        {
            self.gen_stmt(stmt);
        }
    }

    fn gen_assign_stmt(&mut self, node_assign: &NodeStmtAssign)
    {
        if !self.vars.contains_key(&node_assign.get_ident().data) // variable doesn't exist
        {
            exit(1); // TODO: replace with error expected
        }
        self.gen_expr(node_assign.get_expr());
        instructions::pop(self, "%rax");
        let stack_location = match self.vars.get(&node_assign.get_ident().data)
        {
            Some(val) => (self.stack_size - val - 1) * 8,
            None => exit(0) // TODO: replace with error expected
        };

        instructions::movq(self, "%rax", &format!("{stack_location}(%rsp)"));
    }


    fn gen_for_loop_stmt(&mut self, node_for_loop: &NodeStmtForLoop)
    {
        let start_label = self.gen_label();
        let end_label = self.gen_label();

        self.gen_stmt(node_for_loop.get_init_stmt());
        instructions::insert_label(self, &start_label);

        self.gen_expr(node_for_loop.get_expr());
        instructions::pop(self, "%rax");

        instructions::testq(self, "%rax");
        instructions::jz(self, &end_label);

        self.gen_scope(node_for_loop.get_scope());
        self.gen_stmt(node_for_loop.get_stmt());
        instructions::jmp(self, &start_label);
        instructions::insert_label(self, &end_label);

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
            Some(val) => (self.stack_size - val - 1) * 8,
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
            BinOp::Subtraction => self.gen_bin_expr_sub(bin_expr),
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

    fn gen_bin_expr_sub(&mut self, bin_expr_add: &NodeBinExpr)
    {
        self.gen_expr(bin_expr_add.get_rhs());
        self.gen_expr(bin_expr_add.get_lhs());
        instructions::pop(self, "%rax");
        instructions::pop(self, "%rbx");
        instructions::subq(self, "%rbx", "%rax");

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
    fn gen_label(&mut self) -> String
    {
        let label = format!("L{}", self.label_counter);
        self.label_counter += 1;
        label
    }

}





