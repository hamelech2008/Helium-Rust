use std::process::exit;
use crate::generator::{instructions, Generator, FUNC_PARAM_REGS};
use crate::parser::ast_nodes::term::{NodeTerm, NodeTermArray, NodeTermBracket, NodeTermCharLiteral, NodeTermFuncCall, NodeTermIdent, NodeTermIntLiteral, NodeTermNewObj, NodeTermStringLiteral, NodeTermUninitializedArray, Terms};

impl Generator
{
    // terms generation
    pub(super) fn gen_term(&mut self, term: &NodeTerm)
    {
        match term.get_var()
        {
            Terms::IntLiteral(node_int_literal) => self.gen_int_literal_term(node_int_literal),
            Terms::CharLiteral(node_char_literal) => self.gen_char_literal_term(node_char_literal),
            Terms::StringLiteral(node_string_literal) => self.gen_string_literal_term(node_string_literal),
            Terms::Ident(node_ident) => self.gen_ident_term(node_ident),
            Terms::Bracket(node_bracket) => self.gen_bracket_term(node_bracket),
            Terms::NodeTermFuncCall(node_call) => self.gen_call_term(node_call),
            Terms::Array(node_array) => self.gen_array_term(node_array),
            Terms::UninitializedArray(node_uninitialized_array) => self.gen_initialize_array_term(node_uninitialized_array),
            Terms::NodeTermNewObj(node_new_obj) => self.gen_new_obj(node_new_obj)
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

    pub(super) fn gen_string_literal_term(&mut self, node_string_literal: &NodeTermStringLiteral)
    {

        instructions::movq_immediate(self, &node_string_literal.get_value().data.len().to_string(), "%rdi");
        instructions::call(self, "malloc"); // value returned in rax

        instructions::push(self, "%rax");

        let data = node_string_literal.get_value().data
            .replace("\\n", "\n")
            .replace("\\t", "\t")
            .replace("\\\\", "\\")
            .replace("\\\"", "\"");

        for (i, c) in data.chars().enumerate()
        {
            instructions::movq_immediate(self, &format!("{}", c as u32), "%rbx");
            instructions::movb(self, "%bl", &format!("{}(%rax)", i));
        }

        instructions::movq_immediate(self, "16", "%rdi");
        instructions::call(self, "malloc");

        instructions::pop(self, "%rdi"); // string content addr

        instructions::push(self, "%rax");

        // string size
        instructions::movq_immediate(self, &node_string_literal.get_value().data.len().to_string(), "%rsi");
        instructions::movq(self, "%rax", "%rdx"); // string object addr
        instructions::call(self, "String_String");

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
        for arg in node_call.get_args()
        {
            self.gen_expr(arg);
        }
        
        // pop first 6 arguments to system V calling registers
        for i in (0..node_call.get_args().len().min(6)).rev()
        {
            let reg = FUNC_PARAM_REGS[i];
            instructions::pop(self, reg);
        }

        // node_call.get_args().iter().enumerate().for_each(|(i, var)| {
        //     
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
        instructions::call(self, &node_call.get_ident().data);
        instructions::movq(self, "%r12", "%rsp");
        instructions::pop(self, "%r12");
        
        instructions::push(self, "%rax");
    }

    pub(super) fn gen_array_term(&mut self, node_array: &NodeTermArray)
    {
        for element in node_array.get_elements().iter().rev()
        {
            self.gen_expr(element);
        }
        instructions::movq_immediate(self, &(node_array.get_elements().len() * node_array.get_elements()[0].get_data_type().get_size() as usize).to_string(), "%rdi");
        instructions::call(self, "malloc"); // value returned in rax

        node_array.get_elements().iter().enumerate().for_each(|(i, element)|
            {
                instructions::pop(self, "%rbx");
                instructions::movq(self, "%rbx", &format!("{}(%rax)", i * node_array.get_elements()[0].get_data_type().get_size() as usize));
            });

        instructions::push(self, "%rax");
        // TODO: use a heap allocator to allocate the array
    }


    pub(super) fn gen_initialize_array_term(&mut self, node_uninitialized_array: &NodeTermUninitializedArray)
    {
        self.gen_expr(node_uninitialized_array.get_size());
        instructions::pop(self, "%rdi");
        instructions::call(self, "malloc");
        instructions::push(self, "%rax");
    }


    pub(super) fn gen_new_obj(&mut self, node_new_obj: &NodeTermNewObj)
    {

        instructions::movq_immediate(self, &self.class_table.get_class_size(&node_new_obj.get_ident().data).to_string(), "%rdi");

        instructions::call(self, "malloc");
        instructions::push(self, "%rax");

        node_new_obj.get_args().iter().enumerate().for_each(|(i, var)| {
            let reg = FUNC_PARAM_REGS[i];

            if i < 6
            {
                self.gen_expr(var);
                instructions::pop(self, reg);
            }
        });

        let class_name = &node_new_obj.get_ident().data;
        
        // pass "this" as a pointer to the object
        instructions::movq(self, "0(%rsp)", FUNC_PARAM_REGS[node_new_obj.get_args().len()]); 
        instructions::call(self, &format!("{class_name}_{class_name}"))

    }
}