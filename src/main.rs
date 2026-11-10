#[path = "Lexer/lexer.rs"]
mod lexer;

#[path = "Parser/parser.rs"]
mod parser;

fn main()
{
    let source_code : String = String::from("dec i = 10 + 1 * 20;");
    let mut lexer = lexer::Lexer::new();
    lexer.tokenize(&source_code);
    let mut parser = parser::Parser::new(lexer.tokens);
    let node_prog = parser.parse();
    println!("{:#?}", node_prog)
}
