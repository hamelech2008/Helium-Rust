use crate::parser::ast_nodes::stmt::NodeStmt;
use crate::lexer::token::Token;


#[derive(Clone, Debug, PartialEq)]
pub enum DataType
{
    Integer, Char, Float, Array(Box<DataType>)
}


impl TryFrom<&str> for DataType // used only to infer atomic types
{
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error>
    {
        match value
        {
            "Integer" => Ok(Self::Integer),
            "Float" => Ok(Self::Float),
            "Char" => Ok(Self::Char),
            _ => Err(format!("Unknown type '{}'", value)),
        }
    }
}

impl From<DataType> for String
{
    fn from(value: DataType) -> String
    {
        match value
        {
            DataType::Integer => String::from("Integer"),
            DataType::Float => String::from("Float"),
            DataType::Char => String::from("Char"),
            DataType::Array(array) => {
                let mut str = String::new();

                str.push_str("Array<");
                str.push_str(&String::from(*array));
                str.push_str(">");
                str
            },
        }
    }
}

// module for term nodes, identifiers and literal values
pub mod term
{
    use crate::lexer::token::Token;
    use crate::parser::ast_nodes::DataType;
    use crate::parser::ast_nodes::expr::{Exprs, NodeExpr};

    #[derive(Debug, Clone)]

    pub struct NodeTermIntLiteral
    {
        pub value: Token
    }
    impl NodeTermIntLiteral
    {
        pub fn get_value(&self) -> &Token
        {
            &self.value
        }
    }

    #[derive(Debug, Clone)]
    pub struct NodeTermCharLiteral
    {
        value: Token
    }
    impl NodeTermCharLiteral
    {
        pub fn new(value: Token) -> Self {Self{value}}
        pub fn get_value(&self) -> &Token
        {
            &self.value
        }
    }


    #[derive(Debug, Clone)]
    pub struct NodeTermIdent
    {
        pub ident: Token,
        pub data_type: DataType
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
    pub struct NodeTermFuncCall
    {
        ident: Token,
        exprs: Vec<NodeExpr>
    }
    
    impl NodeTermFuncCall
    {
        pub fn new(ident: Token, exprs: Vec<NodeExpr>) -> Self
        {
            Self
            {
                ident,
                exprs
            }
        }

        pub fn get_ident(&self) -> &Token
        {
            &self.ident
        }

        pub fn get_args(&self) -> &Vec<NodeExpr>
        {
            &self.exprs
        }
    }


    #[derive(Debug, Clone)]
    pub struct NodeTermArray
    {
        exprs: Vec<NodeExpr>,
        data_type: DataType
    }
    impl NodeTermArray
    {
        pub fn new(exprs: Vec<NodeExpr>, data_type: DataType) -> Self
        {
            Self
            {
                exprs,
                data_type
            }
        }

        pub fn get_exprs(&self) -> &Vec<NodeExpr>
        {
            &self.exprs
        }

        pub fn get_data_type(&self) -> &DataType
        {
            &self.data_type
        }
    }

    #[derive(Debug, Clone)]
    pub enum Terms
    {
        Ident(NodeTermIdent), IntLiteral(NodeTermIntLiteral),
        CharLiteral(NodeTermCharLiteral), Bracket(NodeTermBracket), NodeTermFuncCall(NodeTermFuncCall),
        Array(NodeTermArray)
    }

    #[derive(Debug, Clone)]

    pub struct NodeTerm
    {
        var : Terms,
        data_type: DataType
    }
    impl NodeTerm
    {
        pub fn new(term: Terms, data_type: DataType) -> Self
        {
            Self { var: term, data_type }
        }

        pub fn get_var(&self) -> &Terms
        {
            &self.var
        }

        pub fn get_data_type(&self) -> &DataType
        {
            &self.data_type
        }
    }
}

// module for expressions, terms, binary and logic expressions
pub mod expr
{
    use crate::parser::ast_nodes::{term, DataType};


    #[derive(Clone, Debug)]
    pub enum BinOp { Addition, Subtraction, Division, Multiplication, Lesser, Greater, EqualsTo, GreaterOrEqual, LesserOrEqual, NotEqual }
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
        op: LogicOp,
        data_type: DataType
    }
    impl NodeLogicExpr
    {
        pub fn new(lhs: NodeExpr, rhs: NodeExpr, op: LogicOp) -> Self
        {
            Self
            {
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
                op,
                data_type: DataType::Integer
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



    #[derive(Clone, Debug)]
    pub struct NodeTerneryExpr
    {
        condition: Box<NodeExpr>,
        true_expr: Box<NodeExpr>,
        false_expr: Box<NodeExpr>,
        data_type: DataType
    }

    impl NodeTerneryExpr
    {
        pub fn new(condition: NodeExpr, true_expr: NodeExpr, false_expr: NodeExpr, data_type: DataType) -> Self
        {
            Self
            {
                condition: Box::new(condition),
                true_expr: Box::new(true_expr),
                false_expr: Box::new(false_expr),
                data_type
            }
        }

        pub fn get_condition(&self) -> &NodeExpr
        {
            &self.condition
        }

        pub fn get_true_expr(&self) -> &NodeExpr
        {
            &self.true_expr
        }

        pub fn get_false_expr(&self) -> &NodeExpr
        {
            &self.false_expr
        }
    }

    pub enum Operator
    {
        BinOp(BinOp),
        LogicOp(LogicOp),
        TerneryOp
    }


    #[derive(Debug, Clone)]
    pub struct NodeArrayAccess
    {
        pub array : Box<NodeExpr>,
        pub index : Box<NodeExpr>,
        pub data_type: DataType
    }
    impl NodeArrayAccess
    {
        pub fn new(array: NodeExpr, index: NodeExpr, data_type: DataType) -> Self
        {
            Self
            {
                array: Box::new(array),
                index: Box::new(index),
                data_type
            }
        }
    }

    #[derive(Debug, Clone)]

    pub enum Exprs
    {
        Term(term::NodeTerm), BinExpr(NodeBinExpr),
        LogicExpr(NodeLogicExpr), TerneryExpr(NodeTerneryExpr),
        ArrayAccess(NodeArrayAccess)
    }
    #[derive(Debug, Clone)]

    pub struct NodeExpr
    {
        var: Exprs,
        data_type: DataType
    }
    impl NodeExpr
    {
        pub fn new(expr: Exprs, data_type: DataType) -> Self
        {
            Self { var: expr, data_type }
        }

        pub fn get_var(&self) -> &Exprs
        {
            &self.var
        }

        pub fn set_var(&mut self, expr: Exprs)
        {
            self.var = expr;
        }

        pub fn set_data_type(&mut self, data_type: DataType)
        {
            self.data_type = data_type;
        }

        pub fn get_data_type(&self) -> DataType
        {
            self.data_type.clone()
        }
    }
}

// module for statements, for variable creations, assignments, func calls etc...
pub mod stmt
{
    use crate::parser::ast_nodes::{Token, NodeScope};
    use crate::parser::ast_nodes::expr::NodeExpr;

    #[derive(Debug, Clone)]

    pub struct NodeStmtDec
    {
        pub ident: Token,
        pub expr: NodeExpr
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
    pub struct NodeStmtReturn
    {
        expr: Option<NodeExpr>
    }
    impl NodeStmtReturn
    {
        pub fn new(expr: Option<NodeExpr>) -> Self
        {
            Self
            {
                expr
            }
        }
        pub fn get_expr(&self) -> &Option<NodeExpr>
        {
            &self.expr
        }
    }

    #[derive(Debug, Clone)]
    pub struct NodeStmtFuncCall
    {
        ident: Token,
        exprs: Vec<NodeExpr>
    }

    impl NodeStmtFuncCall
    {
        pub fn new(ident: Token, exprs: Vec<NodeExpr>) -> Self
        {
            Self
            {
                ident,
                exprs
            }
        }
        
        pub fn get_ident(&self) -> &Token
        {
            &self.ident
        }
        
        pub fn get_args(&self) -> &Vec<NodeExpr>
        {
            &self.exprs
        }
    }

    #[derive(Debug, Clone)]
    pub enum Stmts
    {
        NodeStmtDec(NodeStmtDec), NodeStmtIf(NodeStmtIf),
        NodeStmtAssign(NodeStmtAssign),
        NodeStmtForLoop(NodeStmtForLoop), NodeStmtWhileLoop(NodeStmtWhileLoop),
        NodeStmtReturn(NodeStmtReturn), NodeStmtFuncCall(NodeStmtFuncCall)
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




pub mod functions
{
    use crate::lexer::token::Token;
    use crate::parser::ast_nodes::{DataType, NodeScope};

    #[derive(Clone, Debug)]
    pub struct FuncDecVar
    {
        ident: Token,
        data_type: DataType
    }
    impl FuncDecVar
    {
        pub fn new(ident: Token, data_type: DataType) -> Self
        {
            Self
            {
                ident,
                data_type
            }
        }

        pub fn get_ident(&self) -> &Token
        {
            &self.ident
        }
    }

    #[derive(Clone, Debug)]
    pub struct NodeFuncDecl
    {
        ident: Token,
        declared_vars: Vec<FuncDecVar>,
        scope: NodeScope,
    }

    impl NodeFuncDecl
    {
        pub fn new(ident: Token, scope: NodeScope, declared_vars: Vec<FuncDecVar>) -> Self
        {
            Self
            {
                ident,
                declared_vars,
                scope
            }
        }

        pub fn get_ident(&self) -> &Token
        {
            &self.ident
        }

        pub fn get_declared_vars(&self) -> &Vec<FuncDecVar>
        {
            &self.declared_vars
        }

        pub fn get_scope(&self) -> &NodeScope
        {
            &self.scope
        }
    }
}


// the program entry point, the start of the AST
#[derive(Debug)]
pub struct NodeProg
{
    stmts : Vec<NodeStmt>,
    functions: Vec<functions::NodeFuncDecl>
}
impl NodeProg
{
    pub fn new() -> Self
    {
        Self { stmts: Vec::new(), functions: Vec::new() }
    }

    pub fn add_stmt(&mut self, stmt: NodeStmt)
    {
        self.stmts.push(stmt);
    }

    pub fn get_stmts(&self) -> &Vec<NodeStmt>
    {
        &self.stmts
    }


    pub fn get_functions(&self) -> &Vec<functions::NodeFuncDecl>
    {
        &self.functions
    }

    pub fn add_function(&mut self, function: functions::NodeFuncDecl)
    {
        self.functions.push(function);
    }
}