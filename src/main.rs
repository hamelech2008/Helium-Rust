use std::{env, fs};
use std::process::exit;
use std::time::Instant;

#[path = "Lexer/lexer.rs"]
mod lexer;

#[path = "Parser/parser.rs"]
mod parser;

#[path = "Generator/generator.rs"]
mod generator;
pub mod class_table;

fn main()
{

    let start = Instant::now();

    let args: Vec<String> = env::args().collect();
    if args.len() != 2
    {
        println!("Error: Invalid number of arguments!");
        exit(1);
    }

    let file_path = &args[1];
    let source_code : String = fs::read_to_string(file_path).expect("Could not read file.");

    let mut lexer = lexer::Lexer::new();
    lexer.tokenize(&source_code);
    let tokens =  lexer::token_preprocessor::tokens_preprocess(&lexer.tokens);
    // println!("{:#?}", lexer.tokens);

    let mut parser = parser::Parser::new(tokens);
    let node_prog = parser.parse();

    // println!("{:#?}", node_prog.get_classes());
    let generator = generator::Generator::new(node_prog, parser.class_table);
    let output = generator.generate_prog();

    // println!("{:#?}", parser.class_table);

    let elapsed = start.elapsed();
    println!("Execution time: {:?}", elapsed);
    println!("{output}");
}
