use std::io::stdout;
use std::process::exit;
use crate::parser::ast_nodes::NodeProg;
use string_builder::Builder;
use crate::parser::ast_nodes::stmt::{NodeStmt, Stmts};

pub struct Generator
{
    prog: NodeProg,
    output: Builder
}


impl Generator
{
    pub fn new(prog: NodeProg) -> Self
    {
        Self
        {
            prog,
            output: Builder::default()
        }
    }

    pub fn generate_prog(mut self) -> Builder
    {
        self.output.append(".global _start\n");
        self.output.append(".section .text\n");
        self.output.append("_start:\n");
        let stmts = self.prog.get_stmts();

        for stmt in stmts
        {
            gen_stmt(stmt, &mut self.output);
        }


        self.output
    }


}

fn gen_stmt(stmt: &NodeStmt, builder: &mut Builder)
{
    match &stmt.get_var()
    {
        Stmts::NodeStmtDec(dec) =>
            {
            },
        _ => exit(0) // TODO: replace with error expected
    }
}

