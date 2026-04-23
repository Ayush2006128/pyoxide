use crate::interpreter::Interpreter;
use crate::lexer::{Lexer, Token};
use crate::parser::Parser;
use std::io::{self, Write};

pub fn repl() {
    println!("Welcome to Pyoxide! Type 'exit' to quit");
    let mut interpreter = Interpreter::new();

    loop {
        print!("~~~: ");
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

        let mut lexer = Lexer::new(input);
        let mut tokens = Vec::new();
        loop {
            let tok = lexer.next_token();
            if tok == Token::Eof {
                break;
            }
            tokens.push(tok);
        }

        let mut parser = Parser::new(tokens);
        let stmt = parser.parse_statement();

        interpreter.execute(stmt);
    }
}
