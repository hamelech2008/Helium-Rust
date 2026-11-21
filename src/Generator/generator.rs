use std::collections::HashMap;
use std::process::exit;
use crate::class_table::ClassTable;
use crate::parser::ast_nodes::{NodeProg, NodeScope};
use crate::parser::ast_nodes::expr::{Exprs, NodeArrayAccess, NodeExpr, NodeMemberAccess, NodeTerneryExpr};
use crate::parser::ast_nodes::functions::NodeFuncDecl;
mod instructions;
mod stmt_generator;
mod term_generator;

mod bin_expr_generator;

mod logic_expr_generator;

const FUNC_PARAM_REGS: [&'static str; 6] = [
    "%rdi",
    "%rsi",
    "%rdx",
    "%rcx",
    "%r8",
    "%r9",
];


pub struct Generator
{
    prog: NodeProg,
    output: String,
    stack_size: u32,
    vars: Vec<HashMap<String, u32>>,
    label_counter: u32,
    class_table: ClassTable
}


impl Generator
{
    pub fn insert(&mut self, ident: String, stack_lock : u32)
    {
        // inserting only on the current scope
        self.vars.last_mut().unwrap().insert(ident, stack_lock);
    }
    pub fn contains_key(&self, ident: &String) -> bool
    {
        // checking for variables from all parent scopes
        self.vars.iter().rev().any(|scope| scope.contains_key(ident))
    }

    pub fn get(&self, ident: &String) -> Option<&u32>
    {
        // getting the variable stack offset from all the parent scopes
        self.vars.iter().rev().find_map(|scope| scope.get(ident))
    }

    pub fn new(prog: NodeProg, class_table: ClassTable) -> Self
    {
        Self
        {
            prog,
            output: String::with_capacity(4096),
            stack_size: 1,
            vars: Vec::new(),
            label_counter: 0,
            class_table
        }
    }

    pub fn generate_prog(mut self) -> String
    {
        self.output.push_str(".global _start\n");
        self.output.push_str(".section .text\n");
        self.output.push_str("_start:\n");
        self.output.push_str("\tcall main\n");
        self.output.push_str("\tmovq %rax, %rdi\n");
        self.output.push_str("\tmovq $60, %rax\n");
        self.output.push_str("\tsyscall\n");

        let funcs = self.prog.get_functions().to_vec();
        let stmts = self.prog.get_stmts().to_vec();


        let methods: Vec<_> = self.class_table
            .get_itr()
            .flat_map(|(class_name, class)| {
                class.get_methods()
                    .values()
                    .cloned()
                    .map(move |method| (class_name.clone(), method))
            })
            .collect();

        for (class_name, mut method) in methods
        {
            method.prefix_name(&class_name);
            self.gen_function(&method);
        }


        for func in &funcs
        {
            self.gen_function(func);
        }

        for stmt in &stmts
        {
            self.gen_stmt(stmt);
        }


        self.output
    }

    // functions generation
    fn gen_function(&mut self, func: &NodeFuncDecl)
    {
        if func.get_scope().is_none() {return;}
        let initial_stack_size = self.stack_size; // save the stack size before the function
        self.vars.push(HashMap::new());

        let func_label = &func.get_ident().data;
        instructions::insert_label(self, &func_label);

        instructions::push(self, "%rbp");
        instructions::movq(self, "%rsp", "%rbp");

        func.get_declared_vars().iter().enumerate().for_each(|(i, var)| {
            if i < 6
            {
                let reg = FUNC_PARAM_REGS[i];


                if self.contains_key(&var.get_ident().data) // variable declared earlier
                {
                    eprintln!("Variable '{}' has already been declared.", var.get_ident().data);
                    exit(1); // TODO: replace with error expected
                }
                self.insert(var.get_ident().data.to_string(), self.stack_size);
                instructions::push(self, reg);
            }
        });


        self.gen_scope(&func.get_scope().clone().unwrap());

        instructions::movq_immediate(self, "0", "%rax");
        instructions::movq(self, "%rbp", "%rsp");
        instructions::pop(self, "%rbp");
        instructions::retq(self);

        self.stack_size = initial_stack_size; // use the saved stack size
        self.vars.pop();
    }





    // expressions generation
    fn gen_expr(&mut self, expr: &NodeExpr)
    {
        match &expr.get_var()
        {
            Exprs::Term(node_term) => self.gen_term(node_term),
            Exprs::BinExpr(node_bin_expr) => self.gen_bin_expr(node_bin_expr),
            Exprs::LogicExpr(node_logic_expr) => self.gen_logic_expr(node_logic_expr),
            Exprs::TerneryExpr(node_ternery_expr) => self.gen_ternery_expr(node_ternery_expr),
            Exprs::ArrayAccess(node_array_access) => self.gen_array_index_expr(node_array_access),
            Exprs::NodeMemberAccess(node_member_access) => self.gen_member_access(node_member_access)

        }
    }



    fn gen_ternery_expr(&mut self, node_ternery_expr: &NodeTerneryExpr)
    {
        let false_branch_start = self.gen_label();
        let false_branch_end = self.gen_label();
        self.gen_expr(node_ternery_expr.get_condition());
        instructions::pop(self, "%rax");

        instructions::testq(self, "%rax");
        instructions::jz(self, &false_branch_start);

        let initial_stack_size = self.stack_size; // save the init stack size

        // generate the "true" branch
        self.gen_expr(node_ternery_expr.get_true_expr()); // gen_expr already pushes onto the stack
        let true_stack_size = self.stack_size; // save the true branch stack size
        instructions::jmp(self, &false_branch_end);


        // generate the "false" branch
        instructions::insert_label(self, &false_branch_start);
        self.stack_size = initial_stack_size; // reset the stack size for the false branch

        // gen_expr already pushes onto the stack
        self.gen_expr(node_ternery_expr.get_false_expr());
        let false_stack_size = self.stack_size; // save the false branch stack size
        instructions::insert_label(self, &false_branch_end);


        assert_eq!(true_stack_size, false_stack_size); // make sure both stack sizes are the same

        self.stack_size = true_stack_size;
    }


    fn gen_array_index_expr(&mut self, node_array_access: &NodeArrayAccess)
    {
        self.gen_expr(&*node_array_access.array);
        self.gen_expr(&*node_array_access.index);
        instructions::pop(self, "%rax"); // index
        instructions::pop(self, "%rbx"); // array

        let size = node_array_access.data_type.get_size();
        let address = format!("(%rbx, %rax, {})", size);

        match size
        {
            8 => instructions::movq(self, &address, "%rax"),
            1 => instructions::movzbq(self, &address, "%rax"),
            _ => panic!("Unsupported array element size: {}", size),
        }

        instructions::push(self, "%rax");
    }


    fn gen_member_access(&mut self, member_access: &NodeMemberAccess)
    {
        let field = self.class_table.get_class_member(member_access.get_object(), member_access.get_member());

        if field.is_none()
        {
            eprintln!("Unknown field {}", member_access.get_member().data);
            exit(1);
        }

        let offset = field.unwrap().get_offset();
        self.gen_expr(member_access.get_object());
        instructions::pop(self, "%rax");

        instructions::movq(self, &format!("{}(%rax)", offset), "%rbx");
        instructions::push(self, "%rbx");
    }



    // helper functions
    fn gen_label(&mut self) -> String
    {
        let label = format!("L{}", self.label_counter);
        self.label_counter += 1;
        label
    }
}





