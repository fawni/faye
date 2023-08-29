// faye, a pretty lil lisp
// Copyright (c) 2023 fawn
//
// SPDX-License-Identifier: Apache-2.0

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
    line: usize,
    col: usize,
}

impl Lexer<'_> {
    pub const fn new(input: &str) -> Lexer {
        Lexer {
            input,
            pos: 0,
            line: 0,
            col: 0,
        }
    }

    pub const fn location(&self, len: usize) -> (usize, usize, usize) {
        (self.line, self.col - len, len)
    }

    fn current(&self) -> Option<char> {
        self.input.chars().nth(self.pos)
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.current();
        self.pos += 1;
        self.col += 1;
        c
    }

    fn read(&mut self) -> Result<Token, Error> {
        while self
            .current()
            .ok_or_else(|| Error(ErrorKind::Other, self.location(0)))?
            == ' '
        {
            self.advance();
        }

        match self
            .current()
            .ok_or_else(|| Error(ErrorKind::Other, self.location(0)))?
        {
            '(' => {
                self.advance();
                Ok(Token::OpenParen)
            }
            ')' => {
                self.advance();
                Ok(Token::CloseParen)
            }
            '0'..='9' => {
                let mut s = String::new();
                while let Some(c) = self.current() {
                    if c.is_seperator() {
                        break;
                    }

                    s.push(c);
                    self.advance();
                }
                let len = s.len();
                let n = s
                    .parse::<f64>()
                    .ok()
                    .ok_or_else(|| Error(ErrorKind::InvalidNumber(s), self.location(len)))?;
                Ok(Token::Number(n))
            }
            '\n' => {
                self.advance();
                self.line += 1;
                self.col = 0;
                self.read()
            }
            _ => {
                let mut s = String::new();
                while let Some(c) = self.current() {
                    if c.is_seperator() {
                        break;
                    }

                    s.push(c);
                    self.advance();
                }

                let len = s.len();
                match Symbol::try_from(&*s) {
                    Ok(sym) => Ok(Token::Symbol(sym)),
                    _ => Err(Error(ErrorKind::InvalidSymbol(s), self.location(len))),
                }
            }
        }
    }
}

impl Iterator for Lexer<'_> {
    type Item = Result<Token, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.input.len() {
            None
        } else {
            Some(self.read())
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    OpenParen,
    CloseParen,
    Symbol(Symbol),
    Number(f64),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Symbol {
    Plus,
    Minus,
    Multiply,
    Divide,
}

impl Symbol {
    pub const fn len(&self) -> usize {
        1
    }
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::Multiply => write!(f, "*"),
            Self::Divide => write!(f, "/"),
        }
    }
}

impl TryFrom<&str> for Symbol {
    type Error = ErrorKind;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "+" => Ok(Self::Plus),
            "-" => Ok(Self::Minus),
            "*" => Ok(Self::Multiply),
            "/" => Ok(Self::Divide),
            _ => Err(Self::Error::InvalidSymbol(s.to_owned())),
        }
    }
}

trait Seperator {
    fn is_seperator(&self) -> bool;
}

impl Seperator for char {
    fn is_seperator(&self) -> bool {
        self.is_ascii_whitespace() || *self == '(' || *self == ')'
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Error(pub ErrorKind, pub (usize, usize, usize));

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for Error {}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ErrorKind {
    InvalidSymbol(String),
    InvalidNumber(String),
    Other,
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidSymbol(s) => write!(f, "'{s}' is not a valid symbol"),
            Self::InvalidNumber(s) => write!(f, "Could not parse number '{s}'"),
            Self::Other => write!(f, "ummmmmm... something went wrong idk"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexer() {
        let mut lexer = Lexer::new("(+ 14 25.5 333 (* 2 5))");

        assert_eq!(lexer.next(), Some(Ok(Token::OpenParen)));
        assert_eq!(lexer.next(), Some(Ok(Token::Symbol(Symbol::Plus))));
        assert_eq!(lexer.next(), Some(Ok(Token::Number(14.0))));
        assert_eq!(lexer.next(), Some(Ok(Token::Number(25.5))));
        assert_eq!(lexer.next(), Some(Ok(Token::Number(333.0))));
        assert_eq!(lexer.next(), Some(Ok(Token::OpenParen)));
        assert_eq!(lexer.next(), Some(Ok(Token::Symbol(Symbol::Multiply))));
        assert_eq!(lexer.next(), Some(Ok(Token::Number(2.0))));
        assert_eq!(lexer.next(), Some(Ok(Token::Number(5.0))));
        assert_eq!(lexer.next(), Some(Ok(Token::CloseParen)));
        assert_eq!(lexer.next(), Some(Ok(Token::CloseParen)));
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn parse_numbers() {
        let mut lexer = Lexer::new("2 55 3.144 0.0001 1.1.1");

        assert_eq!(lexer.next(), Some(Ok(Token::Number(2.0))));
        assert_eq!(lexer.next(), Some(Ok(Token::Number(55.0))));
        assert_eq!(lexer.next(), Some(Ok(Token::Number(3.144))));
        assert_eq!(lexer.next(), Some(Ok(Token::Number(0.0001))));
        assert_eq!(
            lexer.next(),
            Some(Err(Error(
                ErrorKind::InvalidNumber("1.1.1".to_owned()),
                (0, 18, 5),
            )))
        );
    }

    #[test]
    fn newline() {
        let mut lexer = Lexer::new("(+ 14 25.5 333\n(* 2 5 5.x))");

        assert_eq!(lexer.next(), Some(Ok(Token::OpenParen)));
        assert_eq!(lexer.next(), Some(Ok(Token::Symbol(Symbol::Plus))));
        assert_eq!(lexer.next(), Some(Ok(Token::Number(14.0))));
        assert_eq!(lexer.next(), Some(Ok(Token::Number(25.5))));
        assert_eq!(lexer.next(), Some(Ok(Token::Number(333.0))));
        assert_eq!(lexer.next(), Some(Ok(Token::OpenParen)));
        assert_eq!(lexer.next(), Some(Ok(Token::Symbol(Symbol::Multiply))));
        assert_eq!(lexer.next(), Some(Ok(Token::Number(2.0))));
        assert_eq!(lexer.next(), Some(Ok(Token::Number(5.0))));
        assert_eq!(
            lexer.next(),
            Some(Err(Error(
                ErrorKind::InvalidNumber("5.x".to_owned()),
                (1, 7, 3),
            )))
        );
    }
}
