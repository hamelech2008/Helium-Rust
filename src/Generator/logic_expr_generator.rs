use crate::generator::{instructions, Generator};
use crate::parser::ast_nodes::expr::{LogicOp, NodeLogicExpr};

impl Generator
{
    // logic expressions
    pub(super) fn gen_logic_expr(&mut self, node_logic_expr: &NodeLogicExpr)
    {
        match node_logic_expr.get_op()
        {
            LogicOp::Or => self.gen_logic_expr_or(node_logic_expr),
            LogicOp::And => self.gen_logic_expr_and(node_logic_expr)
        }
    }

    pub(super) fn gen_logic_expr_or(&mut self, node_or: &NodeLogicExpr)
    {
        self.gen_expr(node_or.get_rhs());
        self.gen_expr(node_or.get_lhs());
        instructions::pop(self, "%rax");
        instructions::pop(self, "%rbx");

        instructions::testq(self, "%rax");
        instructions::setnz(self, "%al");

        instructions::testq(self, "%rbx");
        instructions::setnz(self, "%bl");

        instructions::or(self, "%bl", "%al");
        instructions::movzx(self, "%al", "%rax");

        instructions::push(self, "%rax");
    }

    pub(super) fn gen_logic_expr_and(&mut self, node_and: &NodeLogicExpr)
    {
        self.gen_expr(node_and.get_rhs());
        self.gen_expr(node_and.get_lhs());
        instructions::pop(self, "%rax");
        instructions::pop(self, "%rbx");

        instructions::testq(self, "%rax");
        instructions::setnz(self, "%al");

        instructions::testq(self, "%rbx");
        instructions::setnz(self, "%bl");

        instructions::and(self, "%bl", "%al");
        instructions::movzx(self, "%al", "%rax");

        instructions::push(self, "%rax");
    }
}