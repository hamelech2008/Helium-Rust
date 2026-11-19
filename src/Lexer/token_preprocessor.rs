use std::fs;
use std::process::exit;
use crate::lexer;
use crate::lexer::token::{Token, TokenType};

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

            let he_path = format!("/usr/local/helium/fetch/{}.he", path);
            let so_path = format!("/usr/local/helium/lib/{}.so", path);

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
