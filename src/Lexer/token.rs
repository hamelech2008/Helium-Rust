// Enum for token types, declare, for, while, etc...
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TokenType
{
    Fetch,
    Class, New, Dot,
    For, While, If, Else, Function, Return,
    Dec,
    Ident,
    Equals, Greater, Lesser, Addition, Multiplication, Subtraction,
    Division, EqualsTo, GreaterOrEqual, LesserOrEqual, NotEqual,
    And, Or,
    Ternary, Colon,
    IntLiteral, CharLiteral, StringLiteral,
    SemiColon, LeftBracket, RightBracket, LeftCurly, RightCurly, LeftSquare, RightSquare, Comma
}

// token struct containing the type and its data (for e.g: variable names, constant string value, etc...)
#[derive(Debug, Clone)]
pub struct Token
{
    pub token_type: TokenType,
    pub data: String
}