use crate::lexer::Token;

#[derive(Debug, PartialEq)]
pub enum Expr {
    Number(f64),
    Variable(String),
    BinOp(Box<Expr>, Token, Box<Expr>),
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Assign(String, Expr),
    Expr(Expr),
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn current(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::Eof)
    }

    fn consume(&mut self) {
        self.pos += 1;
    }

    pub fn parse_statement(&mut self) -> Stmt {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Token;

    #[test]
    fn test_parse_assign() {
        let tokens = vec![
            Token::Ident("x".to_string()),
            Token::Assign,
            Token::Float(42.0),
            Token::Eof,
        ];
        let mut parser = Parser::new(tokens);
        let stmt = parser.parse_statement();
        assert_eq!(stmt, Stmt::Assign("x".to_string(), Expr::Number(42.0)));
    }

    #[test]
    fn test_parse_precedence() {
        let tokens = vec![
            Token::Float(1.0),
            Token::Plus,
            Token::Float(2.0),
            Token::Star,
            Token::Float(3.0),
            Token::Eof,
        ];
        let mut parser = Parser::new(tokens);
        let stmt = parser.parse_statement();

        // 1 + (2 * 3)
        let expected = Stmt::Expr(Expr::BinOp(
            Box::new(Expr::Number(1.0)),
            Token::Plus,
            Box::new(Expr::BinOp(
                Box::new(Expr::Number(2.0)),
                Token::Star,
                Box::new(Expr::Number(3.0)),
            )),
        ));
        assert_eq!(stmt, expected);
    }
}
