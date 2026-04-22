use std::collections::HashMap;
use std::io::{self, Write};
use std::iter::Peekable;
use std::str::Chars;

// lexer for interpreter
#[derive(Debug, Clone, PartialEq)]
enum Token {
    Ident(String),
    Float(f64),
    Assign,
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
    Eof,
}

struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Lexer {
            chars: input.chars().peekable(),
        }
    }

    fn next_token(&mut self) -> Token {
        while let Some(&c) = self.chars.peek() {
            match c {
                ' ' | '\t' | '\r' | '\n' => {
                    self.chars.next();
                }
                'a'..='z' | 'A'..='Z' | '_' => return self.read_ident(),
                '0'..='9' | '.' => return self.read_number(),
                '=' => {
                    self.chars.next();
                    return Token::Assign;
                }
                '+' => {
                    self.chars.next();
                    return Token::Plus;
                }
                '-' => {
                    self.chars.next();
                    return Token::Minus;
                }
                '*' => {
                    self.chars.next();
                    return Token::Star;
                }
                '/' => {
                    self.chars.next();
                    return Token::Slash;
                }
                '(' => {
                    self.chars.next();
                    return Token::LParen;
                }
                ')' => {
                    self.chars.next();
                    return Token::RParen;
                }
                _ => panic!("Unexpected character: {}", c),
            }
        }
        Token::Eof
    }

    fn read_ident(&mut self) -> Token {
        let mut ident = String::new();
        while let Some(&c) = self.chars.peek() {
            if c.is_alphanumeric() || c == '_' {
                ident.push(self.chars.next().unwrap());
            } else {
                break;
            }
        }
        Token::Ident(ident)
    }

    fn read_number(&mut self) -> Token {
        let mut num_str = String::new();
        while let Some(&c) = self.chars.peek() {
            if c.is_ascii_digit() || c == '.' {
                num_str.push(self.chars.next().unwrap());
            } else {
                break;
            }
        }
        Token::Float(num_str.parse().unwrap())
    }
}
// ast and parser
#[derive(Debug)]
enum Expr {
    Number(f64),
    Variable(String),
    BinOp(Box<Expr>, Token, Box<Expr>),
}

#[derive(Debug)]
enum Stmt {
    Assign(String, Expr),
    Expr(Expr),
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn current(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::Eof)
    }

    fn consume(&mut self) {
        self.pos += 1;
    }

    // Parses: identifier = expr OR expr
    fn parse_statement(&mut self) -> Stmt {
        if let Token::Ident(name) = self.current() {
            if self.tokens.get(self.pos + 1) == Some(&Token::Assign) {
                let var_name = name.clone();
                self.consume(); // consume ident
                self.consume(); // consume '='
                let expr = self.parse_expr();
                return Stmt::Assign(var_name, expr);
            }
        }
        Stmt::Expr(self.parse_expr())
    }

    // Handles + and -
    fn parse_expr(&mut self) -> Expr {
        let mut left = self.parse_term();
        while matches!(self.current(), Token::Plus | Token::Minus) {
            let op = self.current().clone();
            self.consume();
            let right = self.parse_term();
            left = Expr::BinOp(Box::new(left), op, Box::new(right));
        }
        left
    }

    // Handles * and /
    fn parse_term(&mut self) -> Expr {
        let mut left = self.parse_factor();
        while matches!(self.current(), Token::Star | Token::Slash) {
            let op = self.current().clone();
            self.consume();
            let right = self.parse_factor();
            left = Expr::BinOp(Box::new(left), op, Box::new(right));
        }
        left
    }

    // Handles numbers, variables, and ( expr )
    fn parse_factor(&mut self) -> Expr {
        match self.current().clone() {
            Token::Float(val) => {
                self.consume();
                Expr::Number(val)
            }
            Token::Ident(name) => {
                self.consume();
                Expr::Variable(name)
            }
            Token::LParen => {
                self.consume(); // consume '('
                let expr = self.parse_expr();
                if *self.current() == Token::RParen {
                    self.consume(); // consume ')'
                } else {
                    panic!("Expected ')'");
                }
                expr
            }
            _ => panic!(
                "Expected number, variable, or '(' but found {:?}",
                self.current()
            ),
        }
    }
}

// --- 3. EVALUATOR ---

struct Interpreter {
    variables: HashMap<String, f64>,
}

impl Interpreter {
    fn new() -> Self {
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

    fn execute(&mut self, stmt: Stmt) {
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

// --- 4. REPL ---
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
