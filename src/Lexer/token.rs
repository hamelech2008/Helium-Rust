// Enum for token types, declare, for, while, etc...
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TokenType
{
    For, While, If, Else, Function, Return,
    Dec,
    Ident,
    Equals, Greater, Lesser, Addition, Multiplication, Subtraction,
    Division, EqualsTo, GreaterOrEqual, LesserOrEqual, NotEqual,
    And, Or,
    Ternary, Colon,
    IntLiteral,
    SemiColon, LeftBracket, RightBracket, LeftCurly, RightCurly, Comma
}

// token struct containing the type and its data (for e.g: variable names, constant string value, etc...)
#[derive(Debug, Clone)]
pub struct Token
{
    pub token_type: TokenType,
    pub data: String
}