use crate::generator::{instructions, Generator};
use crate::parser::ast_nodes::expr::{BinOp, NodeBinExpr};

impl Generator
{
    // Binary Expressions
    pub(super) fn gen_bin_expr(&mut self, bin_expr: &NodeBinExpr)
    {
        match bin_expr.get_op()
        {
            BinOp::Addition => self.gen_bin_expr_add(bin_expr),
            BinOp::Subtraction => self.gen_bin_expr_sub(bin_expr),
            BinOp::Multiplication => self.gen_bin_expr_mul(bin_expr),
            BinOp::Division => self.gen_bin_expr_div(bin_expr),
            BinOp::Lesser => self.gen_bin_expr_less(bin_expr),
            BinOp::Greater => self.gen_bin_expr_greater(bin_expr),
            BinOp::EqualsTo => self.gen_bin_expr_equals_to(bin_expr),
            BinOp::GreaterOrEqual => self.gen_bin_expr_greater_equal(bin_expr),
            BinOp::LesserOrEqual => self.gen_bin_expr_less_equal(bin_expr),
            BinOp::NotEqual => self.gen_bin_expr_not_equal(bin_expr)
        }
    }

    pub(super) fn gen_bin_expr_add(&mut self, bin_expr_add: &NodeBinExpr)
    {
        self.gen_expr(bin_expr_add.get_rhs());
        self.gen_expr(bin_expr_add.get_lhs());
        instructions::pop(self, "%rax");
        instructions::pop(self, "%rbx");
        instructions::addq(self, "%rbx", "%rax");

        instructions::push(self, "%rax");
    }

    pub(super) fn gen_bin_expr_sub(&mut self, bin_expr_add: &NodeBinExpr)
    {
        self.gen_expr(bin_expr_add.get_rhs());
        self.gen_expr(bin_expr_add.get_lhs());
        instructions::pop(self, "%rax");
        instructions::pop(self, "%rbx");
        instructions::subq(self, "%rbx", "%rax");

        instructions::push(self, "%rax");
    }

    pub(super) fn gen_bin_expr_mul(&mut self, bin_expr_mul: &NodeBinExpr)
    {
        self.gen_expr(bin_expr_mul.get_rhs());
        self.gen_expr(bin_expr_mul.get_lhs());
        instructions::pop(self, "%rax");
        instructions::pop(self, "%rbx");
        instructions::mulq(self, "%rbx", "%rax");

        instructions::push(self, "%rax");
    }
    pub(super) fn gen_bin_expr_div(&mut self, bin_expr_div: &NodeBinExpr)
    {
        self.gen_expr(bin_expr_div.get_rhs());
        self.gen_expr(bin_expr_div.get_lhs());
        instructions::pop(self, "%rax");
        instructions::pop(self, "%rbx");


        instructions::xorq(self, "%rdx", "%rdx");
        instructions::divq(self, "%rbx");

        instructions::push(self, "%rax");

    }

    pub(super) fn gen_bin_expr_less(&mut self, bin_expr_less: &NodeBinExpr)
    {
        self.gen_expr(bin_expr_less.get_lhs());
        self.gen_expr(bin_expr_less.get_rhs());
        instructions::pop(self, "%rax");
        instructions::pop(self, "%rbx");

        instructions::movq_immediate(self, "0", "%rcx");
        instructions::movq_immediate(self, "1", "%rdi");

        instructions::cmpq(self, "%rax", "%rbx");
        instructions::cmovl(self, "%rdi", "%rcx");

        instructions::push(self, "%rcx");
    }
    pub(super) fn gen_bin_expr_greater(&mut self, bin_expr_greater: &NodeBinExpr)
    {
        self.gen_expr(bin_expr_greater.get_lhs());
        self.gen_expr(bin_expr_greater.get_rhs());
        instructions::pop(self, "%rax");
        instructions::pop(self, "%rbx");

        instructions::movq_immediate(self, "0", "%rcx");
        instructions::movq_immediate(self, "1", "%rdi");

        instructions::cmpq(self, "%rax", "%rbx");
        instructions::cmovg(self, "%rdi", "%rcx");

        instructions::push(self, "%rcx");
    }

    pub(super) fn gen_bin_expr_equals_to(&mut self, bin_expr_equals: &NodeBinExpr)
    {
        self.gen_expr(bin_expr_equals.get_lhs());
        self.gen_expr(bin_expr_equals.get_rhs());
        instructions::pop(self, "%rax");
        instructions::pop(self, "%rbx");

        instructions::movq_immediate(self, "0", "%rcx");
        instructions::movq_immediate(self, "1", "%rdi");

        instructions::cmpq(self, "%rax", "%rbx");
        instructions::cmove(self, "%rdi", "%rcx");

        instructions::push(self, "%rcx");
    }

    pub(super)fn gen_bin_expr_greater_equal(&mut self, expr_greater_equal: &NodeBinExpr)
    {
        self.gen_expr(expr_greater_equal.get_lhs());
        self.gen_expr(expr_greater_equal.get_rhs());
        instructions::pop(self, "%rax");
        instructions::pop(self, "%rbx");

        instructions::movq_immediate(self, "0", "%rcx");
        instructions::movq_immediate(self, "1", "%rdi");

        instructions::cmpq(self, "%rax", "%rbx");
        instructions::cmovge(self, "%rdi", "%rcx");

        instructions::push(self, "%rcx");
    }

    pub(super) fn gen_bin_expr_less_equal(&mut self, expr_less_equal: &NodeBinExpr)
    {
        self.gen_expr(expr_less_equal.get_lhs());
        self.gen_expr(expr_less_equal.get_rhs());
        instructions::pop(self, "%rax");
        instructions::pop(self, "%rbx");

        instructions::movq_immediate(self, "0", "%rcx");
        instructions::movq_immediate(self, "1", "%rdi");

        instructions::cmpq(self, "%rax", "%rbx");
        instructions::cmovle(self, "%rdi", "%rcx");

        instructions::push(self, "%rcx");
    }

    pub(super) fn gen_bin_expr_not_equal(&mut self, expr_not_equal: &NodeBinExpr)
    {
        self.gen_expr(expr_not_equal.get_lhs());
        self.gen_expr(expr_not_equal.get_rhs());
        instructions::pop(self, "%rax");
        instructions::pop(self, "%rbx");

        instructions::movq_immediate(self, "0", "%rcx");
        instructions::movq_immediate(self, "1", "%rdi");

        instructions::cmpq(self, "%rax", "%rbx");
        instructions::cmovne(self, "%rdi", "%rcx");

        instructions::push(self, "%rcx");
    }
}