mod interpreter;
mod lexer;
mod parser;

use interpreter::Interpreter;
use lexer::{Lexer, Token};
use parser::Parser;
use std::io::{self, Write};

fn main() {
    let mut interpreter = Interpreter::new();
    println!("Pyoxid 0.1 python interpreter written in rust");

    loop {
        print!(">>> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "exit" {
            break;
        }
        if input.is_empty() {
            continue;
        }

        // 1. Lex
        let mut lexer = Lexer::new(input);
        let mut tokens = Vec::new();
        loop {
            let tok = lexer.next_token();
            if tok == Token::Eof {
                break;
            }
            tokens.push(tok);
        }

        // 2. Parse
        let mut parser = Parser::new(tokens);
        let stmt = parser.parse_statement();

        // 3. Evaluate
        interpreter.execute(stmt);
    }
}
