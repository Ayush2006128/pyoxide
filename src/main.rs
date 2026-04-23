mod interpreter;
mod lexer;
mod parser;
mod repl;

use repl::repl;
use std::env;
use interpreter::file_runner::run_file;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let file_path = &args[1];
        run_file(file_path);
    } else {
        repl();
    }
}
