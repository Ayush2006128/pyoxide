use std::fs;
use crate::interpreter::Interpreter;

pub fn run_file(path: &str) {
    if !path.ends_with(".pyo") {
        eprintln!("Error: File must have .pyo extension");
        return;
    }

    let contents = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading file {}: {}", path, e);
            return;
        }
    };

    let mut interpreter = Interpreter::new();
    for line in contents.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        interpreter.run(line);
    }
}
