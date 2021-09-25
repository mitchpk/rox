use anyhow::*;
use lexer::Lexer;
use std::env;
use std::fs;
use std::io;
use std::io::prelude::*;

mod ast;
mod lexer;
mod token;
mod parser;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: {} [script]", args[0]);
        std::process::exit(1);
    } else if args.len() == 2 {
        run_file(&args[1])
    } else {
        run_prompt()
    }
}

fn run_file(path: &String) -> Result<()> {
    let code = fs::read_to_string(path)?;
    run(code)
}

fn run_prompt() -> Result<()> {
    let input = io::stdin();

    loop {
        print!("> ");
        io::stdout().flush()?;
        let mut line = String::new();
        let size = input.read_line(&mut line)?;

        if size == 0 {
            break;
        }

        let result = run(line.trim_end().into());
        if result.is_err() {
            println!("Error: {}", result.unwrap_err());
        }
    }

    Ok(())
}

fn run(code: String) -> Result<()> {
    let mut lexer = Lexer::new(code);
    let tokens = lexer.scan_tokens()?;
    for e in tokens {
        println!("{}", e);
    }
    Ok(())
}
