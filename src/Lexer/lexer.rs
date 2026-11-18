use regex::{Match, Regex};
use crate::lexer::token::{TokenType};
pub mod token_preprocessor;



pub(crate) mod token;


pub fn binary_prec(token_type: TokenType) -> Option<i32> {
    match token_type
    {
        TokenType::Ternary => Some(0),

        TokenType::Or => Some(1),

        TokenType::And => Some(2),


        TokenType::EqualsTo => Some(3),
        TokenType::NotEqual => Some(3),

        TokenType::LesserOrEqual => Some(4),
        TokenType::Lesser => Some(4),
        TokenType::GreaterOrEqual => Some(4),
        TokenType::Greater => Some(4),

        TokenType::Subtraction => Some(5),
        TokenType::Addition => Some(5),

        TokenType::Division => Some(6),
        TokenType::Multiplication => Some(6),

        _ => None
    }
}

pub struct Lexer
{
    pub tokens: Vec<token::Token>
}

impl Lexer
{
    pub fn new() -> Self
    {
        Self
        {
            tokens: Vec::new()
        }
    }

    pub fn tokenize(&mut self, source_code : &str)
    {
        // regex filter to search for keywords and expressions, for e.g: &&, ||, exit, etc...
        let tokens_re = Regex::new(
            r"(?x)
        (?P<dec>\bdec\b)
        | (?P<exit>\bexit\b)
        | (?P<for>\bfor\b)
        | (?P<while>\bwhile\b)
        | (?P<if>\bif\b)
        | (?P<fetch>\bfetch\b)
        | (?P<else>\belse\b)
        | (?P<function>\bfunction\b)
        | (?P<return>\breturn\b)
        | (?P<class>\bclass\b)
        | (?P<new>\bnew\b)
        | (?P<ident>[a-zA-Z_][a-zA-Z0-9_]*)
        | (?P<and>&&)
        | (?P<dot>\.)
        | (?P<or>\|\|)
        | (?P<not_equal>!=)
        | (?P<equals_to>==)
        | (?P<equals>=)
        | (?P<char>'[^']')
        | (?P<int>-?\d+)
        | (?P<semi>;)
        | (?P<comma>,)
        | (?P<colon>:)
        | (?P<greater_or_equals>>=)
        | (?P<lesser_or_equals><=)
        | (?P<greater>>)
        | (?P<lesser><)
        | (?P<ternery>\?)
        | (?P<addition>\+)
        | (?P<multiplication>\*)
        | (?P<subtraction>-)
        | (?P<division>/)
        | (?P<left_bracket>\()
        | (?P<right_bracket>\))
        | (?P<left_curly>\{)
        | (?P<right_curly>})
        | (?P<left_square>\[)
        | (?P<right_square>])
        "
        ).unwrap();

        // loop on the source code to add then tokens to the vector
        for caps in tokens_re.captures_iter(source_code)
        {
            if let Some(m) = caps.name("class")
            {
                self.add_token(TokenType::Class, m);
            }
            else if let Some(m) = caps.name("new")
            {
                self.add_token(TokenType::New, m);
            }
            else if let Some(m) = caps.name("dot")
            {
                self.add_token(TokenType::Dot, m);
            }
            else if let Some(m) = caps.name("dec")
            {
                self.add_token(TokenType::Dec, m);
            }
            else if let Some(m) = caps.name("for")
            {
                self.add_token(TokenType::For, m);
            }
            else if let Some(m) = caps.name("function")
            {
                self.add_token(TokenType::Function, m);
            }
            else if let Some(m) = caps.name("return")
            {
                self.add_token(TokenType::Return, m);
            }
            else if let Some(m) = caps.name("while")
            {
                self.add_token(TokenType::While, m);
            }
            else if let Some(m) = caps.name("ident")
            {
                self.add_token(TokenType::Ident, m);
            }
            else if let Some(m) = caps.name("if")
            {
                self.add_token(TokenType::If, m);
            }
            else if let Some(m) = caps.name("fetch")
            {
                self.add_token(TokenType::Fetch, m);
            }
            else if let Some(m) = caps.name("else")
            {
                self.add_token(TokenType::Else, m);
            }
            else if let Some(m) = caps.name("equals")
            {
                self.add_token(TokenType::Equals, m);
            }
            else if let Some(m) = caps.name("int")
            {
                self.add_token(TokenType::IntLiteral, m);
            }
            else if let Some(m) = caps.name("char")
            {
                self.add_token(TokenType::CharLiteral, m);
            }
            else if let Some(m) = caps.name("semi")
            {
                self.add_token(TokenType::SemiColon, m);
            }
            else if let Some(m) = caps.name("addition")
            {
                self.add_token(TokenType::Addition, m);
            }
            else if let Some(m) = caps.name("subtraction")
            {
                self.add_token(TokenType::Subtraction, m);
            }
            else if let Some(m) = caps.name("multiplication")
            {
                self.add_token(TokenType::Multiplication, m);
            }
            else if let Some(m) = caps.name("division")
            {
                self.add_token(TokenType::Division, m);
            }
            else if let Some(m) = caps.name("or")
            {
                self.add_token(TokenType::Or, m);
            }
            else if let Some(m) = caps.name("and")
            {
                self.add_token(TokenType::And, m);
            }
            else if let Some(m) = caps.name("not_equal")
            {
                self.add_token(TokenType::NotEqual, m);
            }
            else if let Some(m) = caps.name("greater_or_equals")
            {
                self.add_token(TokenType::GreaterOrEqual, m);
            }
            else if let Some(m) = caps.name("lesser_or_equals")
            {
                self.add_token(TokenType::LesserOrEqual, m);
            }
            else if let Some(m) = caps.name("lesser")
            {
                self.add_token(TokenType::Lesser, m);
            }
            else if let Some(m) = caps.name("greater")
            {
                self.add_token(TokenType::Greater, m);
            }
            else if let Some(m) = caps.name("equals_to")
            {
                self.add_token(TokenType::EqualsTo, m);
            }
            else if let Some(m) = caps.name("ternery")
            {
                self.add_token(TokenType::Ternary, m);
            }
            else if let Some(m) = caps.name("left_bracket")
            {
                self.add_token(TokenType::LeftBracket, m);
            }
            else if let Some(m) = caps.name("right_bracket")
            {
                self.add_token(TokenType::RightBracket, m);
            }
            else if let Some(m) = caps.name("left_curly")
            {
                self.add_token(TokenType::LeftCurly, m);
            }
            else if let Some(m) = caps.name("right_curly")
            {
                self.add_token(TokenType::RightCurly, m);
            }
            else if let Some(m) = caps.name("left_square")
            {
                self.add_token(TokenType::LeftSquare, m);
            }
            else if let Some(m) = caps.name("right_square")
            {
                self.add_token(TokenType::RightSquare, m);
            }
            else if let Some(m) = caps.name("colon")
            {
                self.add_token(TokenType::Colon, m);
            }
            else if let Some(m) = caps.name("comma")
            {
                self.add_token(TokenType::Comma, m);
            }
        }
    }

    fn add_token(&mut self, token: TokenType, m: Match)
    {
        self.tokens.push(token::Token {
            token_type: token,
            data: m.as_str().to_string()
        });
    }
}