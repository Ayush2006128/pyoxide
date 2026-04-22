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
