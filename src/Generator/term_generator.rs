use std::process::exit;
use crate::generator::{instructions, Generator, FUNC_PARAM_REGS};
use crate::parser::ast_nodes::term::{NodeTerm, NodeTermArray, NodeTermBracket, NodeTermCharLiteral, NodeTermFuncCall, NodeTermIdent, NodeTermIntLiteral, Terms};

impl Generator
{
    // terms generation
    pub(super) fn gen_term(&mut self, term: &NodeTerm)
    {
        match term.get_var()
        {
            Terms::IntLiteral(node_int_literal) => self.gen_int_literal_term(node_int_literal),
            Terms::CharLiteral(node_char_literal) => self.gen_char_literal_term(node_char_literal),
            Terms::Ident(node_ident) => self.gen_ident_term(node_ident),
            Terms::Bracket(node_bracket) => self.gen_bracket_term(node_bracket),
            Terms::NodeTermFuncCall(node_call) => self.gen_call_term(node_call),
            Terms::Array(node_array) => self.gen_array_term(node_array),
        }
    }

    pub(super) fn gen_int_literal_term(&mut self, node_int_literal: &NodeTermIntLiteral)
    {
        let literal_value = &*node_int_literal.get_value().data;
        instructions::movq_immediate(self, literal_value, "%rax");
        instructions::push(self, "%rax");
    }

    pub(super) fn gen_char_literal_term(&mut self, node_char_literal: &NodeTermCharLiteral)
    {
        let literal_value = &*node_char_literal.get_value().data;
        instructions::movq_immediate(self, literal_value, "%rax");
        instructions::push(self, "%rax");
    }

    pub(super) fn gen_ident_term(&mut self, node_ident: &NodeTermIdent)
    {

        let stack_location = match self.get(&node_ident.ident.data)
        {
            Some(val) => (self.stack_size - val - 1) * 8,
            None => exit(1) // TODO: replace with error expected
        };


        instructions::movq(self, &format!("{stack_location}(%rsp)"), "%rax");
        instructions::push(self, "%rax");
    }

    pub(super) fn gen_bracket_term(&mut self, node_bracket: &NodeTermBracket)
    {
        self.gen_expr(node_bracket.get_expr());
    }

    pub(super) fn gen_call_term(&mut self, node_call: &NodeTermFuncCall)
    {
        node_call.get_args().iter().enumerate().for_each(|(i, var)| {
            let reg = FUNC_PARAM_REGS[i];

            if i < 6
            {
                self.gen_expr(var);
                instructions::pop(self, reg);
            }
        });

        instructions::call(self, &node_call.get_ident().data);
        instructions::push(self, "%rax");
    }

    pub(super) fn gen_array_term(&mut self, node_array: &NodeTermArray)
    {
        // TODO: use a heap allocator to allocate the array
    }

}