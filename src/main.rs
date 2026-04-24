mod interpreter;
mod lexer;
mod parser;
mod repl;

use interpreter::file_runner::run_file;
use repl::repl;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let file_path = &args[1];
        run_file(file_path);
    } else if args.len() > 2 {
        eprintln!("Usage: {} [file]", args[0]);
    } else {
        repl();
    }
}
