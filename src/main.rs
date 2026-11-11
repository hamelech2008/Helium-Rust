#[path = "Lexer/lexer.rs"]
mod lexer;

#[path = "Parser/parser.rs"]
mod parser;

#[path = "Generator/generator.rs"]
mod generator;

fn main()
{
    let source_code : String = String::from(
        "dec a = 10;\
         if (1 + 1) {}\
         ");
    let mut lexer = lexer::Lexer::new();
    lexer.tokenize(&source_code);
    let mut parser = parser::Parser::new(lexer.tokens);
    let node_prog = parser.parse();

    // println!("{:#?}", node_prog);
    let generator = generator::Generator::new(node_prog);
    let output = generator.generate_prog();

    println!("{output}");
}
