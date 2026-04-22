use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
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

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            chars: input.chars().peekable(),
        }
    }

    pub fn next_token(&mut self) -> Token {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_tokens() {
        let input = "x = 10 + 2.5 * (y - 3)";
        let mut lexer = Lexer::new(input);
        let expected = vec![
            Token::Ident("x".to_string()),
            Token::Assign,
            Token::Float(10.0),
            Token::Plus,
            Token::Float(2.5),
            Token::Star,
            Token::LParen,
            Token::Ident("y".to_string()),
            Token::Minus,
            Token::Float(3.0),
            Token::RParen,
            Token::Eof,
        ];

        for token in expected {
            assert_eq!(lexer.next_token(), token);
        }
    }

    #[test]
    fn test_lexer_whitespace() {
        let input = "  x   =   123  ";
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next_token(), Token::Ident("x".to_string()));
        assert_eq!(lexer.next_token(), Token::Assign);
        assert_eq!(lexer.next_token(), Token::Float(123.0));
        assert_eq!(lexer.next_token(), Token::Eof);
    }
}
