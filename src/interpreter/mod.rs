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
}
