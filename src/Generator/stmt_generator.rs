use std::collections::HashMap;
use std::process::exit;
use crate::generator::{instructions, Generator, FUNC_PARAM_REGS};
use crate::parser::ast_nodes::expr::{Exprs, NodeArrayAccess, NodeExpr, NodeMemberAccess};
use crate::parser::ast_nodes::NodeScope;
use crate::parser::ast_nodes::stmt::{NodeStmt, NodeStmtAssign, NodeStmtComplexAssign, NodeStmtComplexCall, NodeStmtDec, NodeStmtElse, NodeStmtForLoop, NodeStmtFuncCall, NodeStmtIf, NodeStmtReturn, NodeStmtWhileLoop, Stmts};

impl Generator
{
    // statements generation
    pub(super) fn gen_stmt(&mut self, stmt: &NodeStmt)
    {
        match &stmt.get_var()
        {
            Stmts::NodeStmtDec(node_dec) => self.gen_dec_stmt(node_dec),
            Stmts::NodeStmtIf(node_if) => self.gen_if_stmt(node_if),
            Stmts::NodeStmtAssign(node_assign) => self.gen_assign_stmt(node_assign),
            Stmts::NodeStmtForLoop(node_for_loop) => self.gen_for_loop_stmt(node_for_loop),
            Stmts::NodeStmtWhileLoop(node_while_loop) => self.gen_while_loop_stmt(node_while_loop),
            Stmts::NodeStmtReturn(node_return) => self.gen_return_stmt(node_return),
            Stmts::NodeStmtFuncCall(node_call) => self.gen_func_call_stmt(node_call),
            Stmts::NodeStmtComplexAssign(node_assign) => self.gen_complex_assign_stmt(node_assign),
            Stmts::NodeStmtComplexCall(node_call) => self.gen_complex_call_stmt(node_call),
        }
    }

    pub(super) fn gen_dec_stmt(&mut self, node_dec: &NodeStmtDec)
    {
        if self.contains_key(&node_dec.ident.data) // variable declared earlier
        {
            eprintln!("Variable \"{}\" is already defined", node_dec.ident.data);
            exit(1);
        }
        self.insert(node_dec.ident.data.to_string(), self.stack_size);
        self.gen_expr(&node_dec.expr);
    }

    pub(super) fn gen_if_stmt(&mut self, node_if: &NodeStmtIf)
    {
        let after_if_label = self.gen_label();
        let after_else_label = self.gen_label();
        self.gen_expr(node_if.get_expr());

        instructions::pop(self, "%rax");
        instructions::testq(self, "%rax");
        instructions::jz(self, &after_if_label);

        self.gen_scope(node_if.get_scope());

        if node_if.get_else_scope().is_some()
        {
            instructions::jmp(self, &after_else_label);
        }


        instructions::insert_label(self, &after_if_label);
        match node_if.get_else_scope()
        {
            Some(node_else) =>
                {
                    self.gen_else_stmt(node_else);
                    instructions::insert_label(self, &after_else_label);
                },
            None => ()
        }


    }
    pub(super) fn gen_else_stmt(&mut self, node_else: &NodeStmtElse)
    {
        self.gen_scope(node_else.get_scope());
    }

    pub(super) fn gen_scope(&mut self, node_scope: &NodeScope)
    {
        let scope_stack_size = self.stack_size; // save the current stack size
        self.vars.push(HashMap::new());

        for stmt in node_scope.get_stmts()
        {
            self.gen_stmt(stmt);
        }

        let vars_declared = (self.stack_size as i32 - scope_stack_size as i32).abs();

        // use the size of the stack before the scope and account for the variables declared in that scope
        instructions::addq(self, &format!("${}", vars_declared * 8), "%rsp");
        self.stack_size = scope_stack_size;
        self.vars.pop();
    }

    pub(super) fn gen_assign_stmt(&mut self, node_assign: &NodeStmtAssign)
    {
        if !self.contains_key(&node_assign.get_ident().data) // variable doesn't exist
        {
            eprintln!("Unknown variable {}\n", node_assign.get_ident().data);
            exit(1); // TODO: replace with error expected
        }
        self.gen_expr(node_assign.get_expr());
        instructions::pop(self, "%rax");
        let stack_location = match self.get(&node_assign.get_ident().data)
        {
            Some(val) => (self.stack_size - val - 1) * 8,
            None => exit(0) // TODO: replace with error expected
        };

        instructions::movq(self, "%rax", &format!("{stack_location}(%rsp)"));
    }


    pub(super) fn gen_for_loop_stmt(&mut self, node_for_loop: &NodeStmtForLoop)
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


    pub(super) fn gen_while_loop_stmt(&mut self, node_while_loop: &NodeStmtWhileLoop)
    {
        let loop_start = self.gen_label();
        let loop_end = self.gen_label();

        instructions::insert_label(self, &loop_start);

        self.gen_expr(node_while_loop.get_expr());
        instructions::pop(self, "%rax");

        instructions::testq(self, "%rax");
        instructions::jz(self, &loop_end);

        self.gen_scope(node_while_loop.get_scope());

        instructions::jmp(self, &loop_start);

        instructions::insert_label(self, &loop_end);

    }


    pub(super) fn gen_return_stmt(&mut self, node_return: &NodeStmtReturn)
    {
        if let Some(expr) = node_return.get_expr()
        {
            self.gen_expr(expr);
            instructions::pop(self, "%rax");
        }
        else
        {
            instructions::movq_immediate(self, "0", "%rax");
        }

        instructions::movq(self, "%rbp", "%rsp");
        instructions::pop(self, "%rbp");
        instructions::retq(self);
    }

    pub(super) fn gen_func_call_stmt(&mut self, node_call_func: &NodeStmtFuncCall)
    {
        for var in node_call_func.get_args()
        {
            self.gen_expr(var);
        }

        // pop first 6 arguments to system V calling registers
        for i in (0..node_call_func.get_args().len().min(6)).rev()
        {
            let reg = FUNC_PARAM_REGS[i];
            instructions::pop(self, reg);
        }

        // node_call_func.get_args().iter().enumerate().for_each(|(i, var)| {
        //     let reg = FUNC_PARAM_REGS[i];
        //
        //     if i < 6
        //     {
        //         // self.gen_expr(var);
        //         instructions::pop(self, reg);
        //     }
        // });

        instructions::push(self, "%r12");
        instructions::movq(self, "%rsp", "%r12");
        instructions::and(self, "$-16", "%rsp"); // alight rsp to 16 bytes
        instructions::call(self, &node_call_func.get_ident().data);

        instructions::movq(self, "%r12", "%rsp");
        instructions::pop(self, "%r12");
    }

    pub fn gen_complex_call_stmt(&mut self, node_complex_call: &NodeStmtComplexCall)
    {
        self.gen_expr(node_complex_call.get_expr());
    }



    pub(super) fn gen_complex_assign_stmt(&mut self, node_complex_assign: &NodeStmtComplexAssign)
    {
        self.gen_expr_addr(node_complex_assign.get_assigned_expr());
        self.gen_expr(node_complex_assign.get_expr());

        instructions::pop(self, "%rax"); // the value to assign
        instructions::pop(self, "%rbx"); // the value to assign to
        let size = node_complex_assign.get_assigned_expr().get_data_type().get_size();
        match size
        {
            8 => instructions::movq(self,"%rax", "(%rbx)"),
            1 => instructions::movb(self, "%al", "(%rbx)"),
            _ => {}
        }


    }







    pub fn gen_expr_addr(&mut self, expr: &NodeExpr)
    {
        match &expr.get_var()
        {
            Exprs::ArrayAccess(node_array_access) => self.gen_array_index_addr(node_array_access),
            Exprs::NodeMemberAccess(node_member_access) => self.gen_member_access_addr(node_member_access),
            _ => unreachable!("Expression is not assignable"),
        }
    }

    pub fn gen_array_index_addr(&mut self, node_array_access: &NodeArrayAccess)
    {
        self.gen_expr(&*node_array_access.array);
        self.gen_expr(&*node_array_access.index);
        instructions::pop(self, "%rax"); // index
        instructions::pop(self, "%rbx"); // array

        instructions::leaq(self, &format!("(%rbx, %rax, {})", node_array_access.data_type.get_size()), "%rax"); // calculate offset fitting for a byte rbx[rax * 8]
        instructions::push(self, "%rax");
    }

    pub fn gen_member_access_addr(&mut self, node_member_access: &NodeMemberAccess)
    {
        let member_offset = self.class_table.get_member_offset(node_member_access.get_object(), node_member_access.get_member());
        self.gen_expr(node_member_access.get_object()); // generate the object expression
        instructions::pop(self, "%rax");

        instructions::leaq(self, &format!("{}(%rax)", member_offset), "%rax");
        instructions::push(self, "%rax");
    }

}