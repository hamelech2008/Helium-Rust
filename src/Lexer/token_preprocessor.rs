use std::{env, fs};
use std::path::PathBuf;
use std::process::exit;
use std::sync::Mutex;
use crate::lexer;
use crate::lexer::token::{Token, TokenType};

pub(crate) static LIB_PATHS: Mutex<Vec<String>> = Mutex::new(Vec::new());

pub fn tokens_preprocess(tokens: &[Token]) -> Vec<Token>
{
    let mut fetched_tokens: Vec<Token> = Vec::new();
    let mut program_tokens: Vec<Token> = Vec::with_capacity(tokens.len());
    let mut i = 0;

    while i < tokens.len()
    {
        if tokens[i].token_type != TokenType::Fetch
        {
            program_tokens.push(tokens[i].clone());
            i += 1;
        }
        else
        {
            i += 1; // consume the 'fetch' token

            if i >= tokens.len() || tokens[i].token_type != TokenType::Lesser
            {
                eprintln!("Expected '<' after fetch");
                exit(1);
            }
            i += 1; // consume the '<' token

            if i >= tokens.len()
            {
                eprintln!("Expected a path after fetch <");
                exit(1);
            }
            let path = tokens[i].data.clone();
            i += 1; // consume the path

            if i >= tokens.len() || tokens[i].token_type != TokenType::Greater
            {
                eprintln!("Expected '>' after fetch path");
                exit(1);
            }
            i += 1; // consume the '>' token


            // compile against a source file
            if let Some(file_path) = try_source_file(&path)
            {
                let source_code = fs::read_to_string(&file_path).expect("Could not read file.");
                let mut lexer = lexer::Lexer::new();
                lexer.tokenize(&source_code);

                // resolve recursive fetch directives
                let processed = tokens_preprocess(&lexer.tokens);
                fetched_tokens.extend(processed);
                continue;
            }

            // compile against a built-in library
            let he_path = format!("/usr/local/helium/fetch/{}.he", path);
            let so_path = format!("/usr/local/helium/lib/{}.so", path);


            LIB_PATHS.lock().unwrap().push(so_path); // save all the libraries to link against later
            
            let source_code = fs::read_to_string(&he_path).expect("Could not read file.");
            let mut lexer = lexer::Lexer::new();
            lexer.tokenize(&source_code);

            // resolve recursive fetch directives
            let processed = tokens_preprocess(&lexer.tokens);
            fetched_tokens.extend(processed);

        }
    }


    fetched_tokens.extend(program_tokens);
    fetched_tokens
}

pub fn try_source_file(path: &str) -> Option<String>
{
    let parent_path = PathBuf::from(env::args_os().nth(1).unwrap());
    let parent = parent_path.parent().unwrap();
    let full_path = parent.join(format!("{path}.he"));

    if full_path.is_file()
    {
        full_path.to_str().map(|s| s.to_string())
    }
    else
    {
        None
    }
}