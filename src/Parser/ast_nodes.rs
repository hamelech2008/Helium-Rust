use crate::parser::ast_nodes::stmt::NodeStmt;
use crate::lexer::token::Token;

// module for term nodes, identifiers and literal values
pub mod term
{
    use crate::lexer::token;
    use crate::parser::ast_nodes::expr::NodeExpr;

    #[derive(Debug, Clone)]
    pub enum Terms { Ident(NodeTermIdent), IntLiteral(NodeTermIntLiteral), Bracket(NodeTermBracket) }

    #[derive(Debug, Clone)]

    pub struct NodeTermIntLiteral
    {
        pub value: token::Token
    }
    impl NodeTermIntLiteral
    {
        pub fn get_value(&self) -> &token::Token
        {
            &self.value
        }
    }

    #[derive(Debug, Clone)]
    pub struct NodeTermIdent
    {
        pub ident: token::Token
    }

    #[derive(Clone, Debug)]
    pub struct NodeTermBracket
    {
        pub expr: Box<NodeExpr>
    }
    impl NodeTermBracket
    {
        pub fn new(expression: NodeExpr) -> Self
        {
            Self {expr: Box::new(expression)}
        }
        pub fn get_expr(&self) -> &NodeExpr
        {
            &self.expr
        }
    }

    #[derive(Debug, Clone)]

    pub struct NodeTerm
    {
        var : Terms
    }

    impl NodeTerm
    {
        pub fn new(term: Terms) -> Self
        {
            Self { var: term }
        }

        pub fn get_var(&self) -> &Terms
        {
            &self.var
        }
    }
}

// module for expressions, terms, binary and logic expressions
pub mod expr
{
    use crate::parser::ast_nodes::term;


    #[derive(Clone, Debug)]
    pub enum BinOp { Addition, Subtraction, Division, Multiplication }
    #[derive(Clone, Debug)]
    pub struct NodeBinExpr
    {
        lhs: Box<NodeExpr>,
        rhs: Box<NodeExpr>,
        op: BinOp
    }

    impl NodeBinExpr
    {
        pub fn new(lhs: NodeExpr, rhs: NodeExpr, op: BinOp) -> Self
        {
            Self
            {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
                op
            }
        }

        pub fn get_op(&self) -> &BinOp
        {
            &self.op
        }

        pub fn get_rhs(&self) -> &NodeExpr
        {
            &self.rhs
        }

        pub fn get_lhs(&self) -> &NodeExpr
        {
            &self.lhs
        }
    }



    #[derive(Debug, Clone)]

    pub enum Exprs { Term(term::NodeTerm), BinExpr(NodeBinExpr) }
    #[derive(Debug, Clone)]

    pub struct NodeExpr
    {
        var: Exprs
    }

    impl NodeExpr
    {
        pub fn new(expr: Exprs) -> Self
        {
            Self { var: expr }
        }

        pub fn get_var(&self) -> &Exprs
        {
            &self.var
        }

        pub fn set_var(&mut self, expr: Exprs)
        {
            self.var = expr;
        }
    }
}

// module for statements, for variable creations, assignments, func calls etc...
pub mod stmt
{
    use crate::parser::ast_nodes::{expr, Token, NodeScope};
    use crate::parser::ast_nodes::expr::NodeExpr;

    #[derive(Debug, Clone)]

    pub struct NodeStmtDec
    {
        pub ident: Token,
        pub expr: expr::NodeExpr
    }

    #[derive(Debug, Clone)]
    pub struct NodeStmtIf
    {
        expr: NodeExpr,
        scope: NodeScope
    }
    impl NodeStmtIf
    {
        pub fn new(expr: NodeExpr, scope: NodeScope) -> Self
        {
            Self
            {
                expr,
                scope
            }
        }
    }

    #[derive(Debug, Clone)]
    pub enum Stmts { NodeStmtDec(NodeStmtDec), NodeStmtIf(NodeStmtIf) }
    #[derive(Debug, Clone)]
    pub struct NodeStmt
    {
        var : Stmts
    }

    impl NodeStmt
    {
        pub fn new(stmt_type: Stmts) -> Self
        {
            Self { var: stmt_type }
        }

        pub fn get_var(&self) -> &Stmts
        {
            &self.var
        }
    }
}



#[derive(Clone, Debug)]
pub struct NodeScope
{
    stmts: Vec<NodeStmt>
}
impl NodeScope
{
    pub fn new() -> Self
    {
        Self {stmts: Vec::new()}
    }

    pub fn push_stmt(&mut self, stmt: NodeStmt)
    {
        self.stmts.push(stmt);
    }
}


// the program entry point, the start of the AST
#[derive(Debug)]
pub struct NodeProg
{
    stmts : Vec<NodeStmt>
}
impl NodeProg
{
    pub fn new() -> Self
    {
        Self { stmts: Vec::new() }
    }

    pub fn add_stmt(&mut self, stmt: NodeStmt)
    {
        self.stmts.push(stmt);
    }

    pub fn get_stmts(&self) -> &Vec<NodeStmt>
    {
        &self.stmts
    }
}