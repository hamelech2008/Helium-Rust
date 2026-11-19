use std::{env, fs};
use std::path::Path;
use std::process::{exit, Command};
use std::time::Instant;
use crate::lexer::token_preprocessor::tokens_preprocess;

#[path = "Lexer/lexer.rs"]
mod lexer;

#[path = "Parser/parser.rs"]
mod parser;

#[path = "Generator/generator.rs"]
mod generator;
pub mod class_table;

fn main()
{

    // let start = Instant::now();

    let args: Vec<String> = env::args().collect();
    if args.len() != 3
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



    // assemble to an obj file using 'as'
    fs::write(&args[2], output).expect("Could not write to file.");
    let out = Command::new("as")
        .arg(&args[2])
        .arg("-o")
        .arg("test.o")
        .output()
        .expect("Failed to execute assembler");

    if !out.status.success()
    {
        eprintln!("Assembler error:\n{}", String::from_utf8_lossy(&out.stderr));
    }


    // assemble and link with all the libraries using gcc
    // TODO: replace linking with ld
    let mut cmd = Command::new("gcc");
    cmd.arg("-no-pie")
        .arg("-nostartfiles")
        .arg("test.o")
        .arg("-Wl,--no-as-needed");

    {
        let lib_paths = lexer::token_preprocessor::LIB_PATHS.lock().unwrap();
        let mut seen_dirs = std::collections::HashSet::new();

        for lib_path in lib_paths.iter()
        {
            cmd.arg(lib_path);

            // Add each parent directory to rpath once
            if let Some(parent) = Path::new(lib_path).parent()
            {
                if !parent.as_os_str().is_empty() && seen_dirs.insert(parent.to_path_buf())
                {
                    cmd.arg(format!("-Wl,-rpath,{}", parent.display()));
                }
            }
        }
    }

    cmd.arg("-o").arg("test");

    let out = cmd.output().expect("Failed to execute linker");

    if !out.status.success()
    {
        eprintln!("Linker error:\n{}", String::from_utf8_lossy(&out.stderr));
    }
    // let elapsed = start.elapsed();
    // println!("Execution time: {:?}", elapsed);
    // println!("{output}");
}
