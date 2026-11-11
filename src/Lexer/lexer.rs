use regex::{Match, Regex};
use crate::lexer::token::TokenType;



pub(crate) mod token;


pub fn binary_prec(token_type: TokenType) -> Option<i32> {
    match token_type
    {
        TokenType::And => Some(0),
        TokenType::Or => Some(0),

        TokenType::Lesser => Some(1),
        TokenType::Greater => Some(1),

        TokenType::Subtraction => Some(2),
        TokenType::Addition => Some(2),

        TokenType::Division => Some(3),
        TokenType::Multiplication => Some(3),

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
        | (?P<else>\belse\b)
        | (?P<ident>[a-zA-Z_][a-zA-Z0-9_]*)
        | (?P<and>&&)
        | (?P<or>\|\|)
        | (?P<equals>=)
        | (?P<int>\d+)
        | (?P<semi>;)
        | (?P<greater>>)
        | (?P<lesser><)
        | (?P<addition>\+)
        | (?P<multiplication>\*)
        | (?P<subtraction>-)
        | (?P<division>/)
        | (?P<left_bracket>\()
        | (?P<right_bracket>\))
        | (?P<left_curly>\{)
        | (?P<right_curly>})
        "
        ).unwrap();

        // loop on the source code to add then tokens to the vector
        for caps in tokens_re.captures_iter(source_code) {
            if let Some(m) = caps.name("dec")
            {
                self.add_token(TokenType::Dec, m);
            }
            else if let Some(m) = caps.name("ident")
            {
                self.add_token(TokenType::Ident, m);
            }
            else if let Some(m) = caps.name("if")
            {
                self.add_token(TokenType::If, m);
            }
            else if let Some(m) = caps.name("equals")
            {
                self.add_token(TokenType::Equals, m);
            }
            else if let Some(m) = caps.name("int")
            {
                self.add_token(TokenType::IntLiteral, m);
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