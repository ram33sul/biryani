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
    let file = "ingredient abc is [2,3,4] plate abc simmer n in abc { plate n }";
    let mut lexer = Lexer::new(file);
    let tokens = lexer.lex();

    println!("{:?}", tokens);

    let mut parser = Parser::new(tokens);
    let commands = parser.parse();

    println!("{:?}", commands);

    let mut interpreter = Interpreter::new();
    interpreter.execute(commands);
}
