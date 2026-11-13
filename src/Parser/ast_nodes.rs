use crate::parser::ast_nodes::stmt::NodeStmt;
use crate::lexer::token::Token;

// module for term nodes, identifiers and literal values
pub mod term
{
    use crate::lexer::token;
    use crate::parser::ast_nodes::expr::NodeExpr;

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
    pub enum Terms { Ident(NodeTermIdent), IntLiteral(NodeTermIntLiteral), Bracket(NodeTermBracket) }

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
    pub enum BinOp { Addition, Subtraction, Division, Multiplication, Lesser, Greater, EqualsTo }
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


    #[derive(Clone, Debug)]
    pub enum LogicOp { And, Or}
    #[derive(Clone, Debug)]
    pub struct NodeLogicExpr
    {
        lhs: Box<NodeExpr>,
        rhs: Box<NodeExpr>,
        op: LogicOp
    }
    impl NodeLogicExpr
    {
        pub fn new(lhs: NodeExpr, rhs: NodeExpr, op: LogicOp) -> Self
        {
            Self
            {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
                op
            }
        }
        pub fn get_lhs(&self) -> &NodeExpr
        {
            &self.lhs
        }

        pub fn get_rhs(&self) -> &NodeExpr
        {
            &self.rhs
        }

        pub fn get_op(&self) -> &LogicOp
        {
            &self.op
        }
    }


    pub enum Operator 
    {
        BinOp(BinOp),
        LogicOp(LogicOp),
    }


    #[derive(Debug, Clone)]

    pub enum Exprs { Term(term::NodeTerm), BinExpr(NodeBinExpr), LogicExpr(NodeLogicExpr) }
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
        scope: NodeScope,
        else_node: Option<NodeStmtElse>
    }
    impl NodeStmtIf
    {
        pub fn new(expr: NodeExpr, scope: NodeScope) -> Self
        {
            Self
            {
                expr,
                scope,
                else_node: None
            }
        }

        pub fn set_else(&mut self, node: Option<NodeStmtElse>)
        {
            self.else_node = node;
        }
        pub fn get_expr(&self) -> &NodeExpr
        {
            &self.expr
        }

        pub fn get_scope(&self) -> &NodeScope
        {
            &self.scope
        }

        pub fn get_else_scope(&self) -> &Option<NodeStmtElse>
        {
            &self.else_node
        }
    }


    #[derive(Debug, Clone)]
    pub struct NodeStmtElse
    {
        scope: NodeScope
    }
    impl NodeStmtElse
    {
        pub fn new(scope: NodeScope) -> Self
        {
            Self {scope}
        }

        pub fn get_scope(&self) -> &NodeScope
        {
            &self.scope
        }
    }


    #[derive(Debug, Clone)]
    pub struct NodeStmtAssign
    {
        ident: Token,
        expr: NodeExpr
    }
    impl NodeStmtAssign
    {
        pub fn new(ident: Token, expr: NodeExpr) -> Self
        {
            Self
            {
                ident,
                expr
            }
        }

        pub fn get_expr(&self) -> &NodeExpr
        {
            &self.expr
        }

        pub fn get_ident(&self) -> &Token
        {
            &self.ident
        }
    }


    #[derive(Debug, Clone)]
    pub struct NodeStmtForLoop
    {
        init_stmt: Box<NodeStmt>,
        expr: NodeExpr,
        stmt: Box<NodeStmt>,
        scope: NodeScope
    }
    impl NodeStmtForLoop
    {
        pub fn new(init_stmt: NodeStmt, expr: NodeExpr, stmt: NodeStmt, scope: NodeScope) -> Self
        {
            Self
            {
                init_stmt: Box::new(init_stmt),
                stmt: Box::new(stmt),
                expr,
                scope
            }
        }

        pub fn get_init_stmt(&self) -> &NodeStmt
        {
            &self.init_stmt
        }
        pub fn get_stmt(&self) -> &NodeStmt
        {
            &self.stmt
        }
        pub fn get_expr(&self) -> &NodeExpr
        {
            &self.expr
        }

        pub fn get_scope(&self) -> &NodeScope
        {
            &self.scope
        }
    }


    #[derive(Debug, Clone)]
    pub struct NodeStmtWhileLoop
    {
        expr: NodeExpr,
        scope: NodeScope
    }
    
    impl NodeStmtWhileLoop
    {
        pub fn new(expr: NodeExpr, scope: NodeScope) -> Self
        {
            Self
            {
                expr,
                scope
            }
        }
        
        pub fn get_expr(&self) -> &NodeExpr
        {
            &self.expr
        }
        
        pub fn get_scope(&self) -> &NodeScope
        {
            &self.scope
        }
    }
    
    
    #[derive(Debug, Clone)]
    pub enum Stmts
    {
        NodeStmtDec(NodeStmtDec), NodeStmtIf(NodeStmtIf),
        NodeStmtElse(NodeStmtElse), NodeStmtAssign(NodeStmtAssign),
        NodeStmtForLoop(NodeStmtForLoop), NodeStmtWhileLoop(NodeStmtWhileLoop)
    }
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
    
    pub fn get_stmts(&self) -> &Vec<NodeStmt>
    {
        &self.stmts
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