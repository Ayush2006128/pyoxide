pub mod file_runner;
use crate::lexer::Token;
use crate::parser::{Expr, Stmt};
use std::collections::HashMap;

pub struct Interpreter {
    variables: HashMap<String, f64>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            variables: HashMap::new(),
        }
    }

    fn eval_expr(&self, expr: &Expr) -> f64 {
        match expr {
            Expr::Number(val) => *val,
            Expr::Variable(name) => *self
                .variables
                .get(name)
                .unwrap_or_else(|| panic!("Undefined variable: {}", name)),
            Expr::BinOp(left, op, right) => {
                let left_val = self.eval_expr(left);
                let right_val = self.eval_expr(right);
                match op {
                    Token::Plus => left_val + right_val,
                    Token::Minus => left_val - right_val,
                    Token::Star => left_val * right_val,
                    Token::Slash => left_val / right_val,
                    _ => unreachable!(),
                }
            }
        }
    }

    pub fn execute(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::Assign(name, expr) => {
                let val = self.eval_expr(&expr);
                self.variables.insert(name, val);
            }
            Stmt::Expr(expr) => {
                let result = self.eval_expr(&expr);
                println!("{}", result);
            }
        }
    }

    pub fn run(&mut self, source: &str) {
        use crate::lexer::Lexer;
        use crate::parser::Parser;

        let mut lexer = Lexer::new(source);
        let mut tokens = Vec::new();
        loop {
            let tok = lexer.next_token();
            if tok == Token::Eof {
                break;
            }
            tokens.push(tok);
        }

        let mut parser = Parser::new(tokens);
        // Note: The current parser seems to only parse one statement.
        // If we want to support multiple statements per file/line, we might need to update the parser.
        // For now, I'll follow the REPL's pattern.
        let stmt = parser.parse_statement();
        self.execute(stmt);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Token;
    use crate::parser::{Expr, Stmt};

    #[test]
    fn test_interpreter_eval() {
        let mut interpreter = Interpreter::new();
        
        // x = 10
        interpreter.execute(Stmt::Assign("x".to_string(), Expr::Number(10.0)));
        assert_eq!(interpreter.variables.get("x"), Some(&10.0));

        // y = x + 5
        interpreter.execute(Stmt::Assign("y".to_string(), Expr::BinOp(
            Box::new(Expr::Variable("x".to_string())),
            Token::Plus,
            Box::new(Expr::Number(5.0))
        )));
        assert_eq!(interpreter.variables.get("y"), Some(&15.0));
    }

    #[test]
    #[should_panic(expected = "Undefined variable: z")]
    fn test_interpreter_undefined_var() {
        let interpreter = Interpreter::new();
        interpreter.eval_expr(&Expr::Variable("z".to_string()));
    }

    #[test]
    fn test_end_to_end() {
        let mut interpreter = Interpreter::new();
        
        let inputs = vec![
            "a = 5",
            "b = 10",
            "c = (a + b) * 2",
        ];

        for input in inputs {
            let mut lexer = crate::lexer::Lexer::new(input);
            let mut tokens = Vec::new();
            loop {
                let tok = lexer.next_token();
                if tok == Token::Eof { break; }
                tokens.push(tok);
            }
            let mut parser = crate::parser::Parser::new(tokens);
            let stmt = parser.parse_statement();
            interpreter.execute(stmt);
        }

        assert_eq!(interpreter.variables.get("c"), Some(&30.0));
    }

    #[test]
    fn test_division_by_zero() {
        let interpreter = Interpreter::new();
        let input = "1 / 0";
        let mut lexer = crate::lexer::Lexer::new(input);
        let mut tokens = Vec::new();
        loop {
            let tok = lexer.next_token();
            if tok == Token::Eof { break; }
            tokens.push(tok);
        }
        let mut parser = crate::parser::Parser::new(tokens);
        let stmt = parser.parse_statement();
        if let Stmt::Expr(expr) = stmt {
            let val = interpreter.eval_expr(&expr);
            assert!(val.is_infinite());
        }
    }
}
