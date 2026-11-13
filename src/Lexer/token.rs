// Enum for token types, declare, for, while, etc...
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum TokenType
{
    Exit, For, While, If, Else,
    Dec,
    Ident,
    Equals, Greater, Lesser, Addition, Multiplication, Subtraction, Division, EqualsTo,
    And, Or,
    IntLiteral, StringLiteral,
    SemiColon, LeftBracket, RightBracket, LeftCurly, RightCurly
}

// token struct containing the type and its data (for e.g: variable names, constant string value, etc...)
#[derive(Debug, Clone)]
pub struct Token
{
    pub token_type: TokenType,
    pub data: String
}