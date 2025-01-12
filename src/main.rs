use std::{env, fs, os::unix::process};

use interpreter::Interpreter;
use lexer::Lexer;
use parser::Parser;

mod commands;
mod interpreter;
mod lexer;
mod parser;
mod state;
mod tokens;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("args, {:?}", args);

    if args.len() < 2 {
        panic!("Usage: biryani filename");
    }

    let filename = &args[1];

    let code = fs::read_to_string(filename).expect("Unable to read the file");

    let tokens = Lexer::new(&code).lex();

    println!("tokens: {:?}", tokens);

    let commands = Parser::new(tokens).parse();

    println!("commands :{:?}", commands);

    println!();

    let mut interpreter = Interpreter::new();
    interpreter.execute(commands);
}
