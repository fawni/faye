// faye, a pretty lil lisp
// Copyright (c) 2023 fawn
//
// SPDX-License-Identifier: Apache-2.0

use std::str::{Chars, FromStr};

#[derive(Debug)]
pub struct Lexer<'a> {
    input: Chars<'a>,
    line: usize,
    col: usize,
}

impl Lexer<'_> {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            input: input.chars(),
            line: 0,
            col: 0,
        }
    }

    #[inline]
    pub fn location(&self) -> (usize, usize) {
        (self.line, self.col)
    }

    pub fn current(&self) -> Option<char> {
        self.input.as_str().chars().next()
    }

    fn peek(&self, n: usize) -> Option<char> {
        self.input.as_str().chars().nth(n)
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.input.next();
        self.col += 1;
        c
    }

    fn parse_word_or<T: FromStr>(&mut self, kind: ErrorKind) -> Result<T, Error> {
        let start = self.location();
        let mut text = String::new();
        while let Some(c) = self.current() {
            if c.is_seperator() {
                break;
            }

            text.push(c);
            self.advance();
        }

        text.parse()
            .map_err(|_| Error::new(kind, text, start, self.location()))
    }

    pub fn read(&mut self) -> Result<Option<Token>, Error> {
        loop {
            match self.current() {
                Some(' ') => {
                    self.advance();
                }
                Some('\n') => {
                    self.advance();
                    self.line += 1;
                    self.col = 0;
                }
                Some(_) => break,
                None => return Ok(None),
            }
        }

        let start = self.location();
        let kind = match self.current() {
            Some('(') => {
                self.advance();
                TokenKind::OpenParen
            }
            Some(')') => {
                self.advance();
                TokenKind::CloseParen
            }
            Some('0'..='9') => TokenKind::Number(self.parse_word_or(ErrorKind::InvalidNumber)?),
            Some('+' | '-') if matches!(self.peek(1), Some('0'..='9')) => {
                TokenKind::Number(self.parse_word_or(ErrorKind::InvalidNumber)?)
            }
            Some(_) => TokenKind::Symbol(self.parse_word_or(ErrorKind::InvalidSymbol)?),
            None => return Ok(None),
        };

        Ok(Some(Token(kind, start, self.location())))
    }
}

impl Iterator for Lexer<'_> {
    type Item = Result<Token, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.read().transpose()
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Token(pub TokenKind, pub (usize, usize), pub (usize, usize));

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenKind {
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

impl FromStr for Symbol {
    type Err = ErrorKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Plus),
            "-" => Ok(Self::Minus),
            "*" => Ok(Self::Multiply),
            "/" => Ok(Self::Divide),
            _ => Err(ErrorKind::InvalidSymbol),
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
pub struct Error {
    pub kind: ErrorKind,
    pub text: String,
    pub start: (usize, usize),
    pub end: (usize, usize),
}

impl Error {
    pub fn new(kind: ErrorKind, text: String, start: (usize, usize), end: (usize, usize)) -> Self {
        Self {
            kind,
            text,
            start,
            end,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            ErrorKind::InvalidNumber => write!(f, "'{}' is not a valid numeric literal", self.text),
            ErrorKind::InvalidSymbol => write!(f, "'{}' is not a valid symbol", self.text),
        }
    }
}

impl std::error::Error for Error {}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ErrorKind {
    InvalidSymbol,
    InvalidNumber,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex() {
        let mut lexer = Lexer::new("(+ 14 25.5 333 (* 2 5))");

        assert_eq!(
            lexer.next(),
            Some(Ok(Token(TokenKind::OpenParen, (0, 0), (0, 1))))
        );
        assert_eq!(
            lexer.next(),
            Some(Ok(Token(TokenKind::Symbol(Symbol::Plus), (0, 1), (0, 2))))
        );
        assert_eq!(
            lexer.next(),
            Some(Ok(Token(TokenKind::Number(14.0), (0, 3), (0, 5))))
        );
        assert_eq!(
            lexer.next(),
            Some(Ok(Token(TokenKind::Number(25.5), (0, 6), (0, 10))))
        );
        assert_eq!(
            lexer.next(),
            Some(Ok(Token(TokenKind::Number(333.0), (0, 11), (0, 14))))
        );
        assert_eq!(
            lexer.next(),
            Some(Ok(Token(TokenKind::OpenParen, (0, 15), (0, 16))))
        );
        assert_eq!(
            lexer.next(),
            Some(Ok(Token(
                TokenKind::Symbol(Symbol::Multiply),
                (0, 16),
                (0, 17)
            )))
        );
        assert_eq!(
            lexer.next(),
            Some(Ok(Token(TokenKind::Number(2.0), (0, 18), (0, 19))))
        );
        assert_eq!(
            lexer.next(),
            Some(Ok(Token(TokenKind::Number(5.0), (0, 20), (0, 21))))
        );
        assert_eq!(
            lexer.next(),
            Some(Ok(Token(TokenKind::CloseParen, (0, 21), (0, 22))))
        );
        assert_eq!(
            lexer.next(),
            Some(Ok(Token(TokenKind::CloseParen, (0, 22), (0, 23))))
        );
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn newline() {
        let mut lexer = Lexer::new("(+ 14 25.5 333\n(* 2 5 5.x))");

        assert_eq!(
            lexer.next(),
            Some(Ok(Token(TokenKind::OpenParen, (0, 0), (0, 1))))
        );
        assert_eq!(
            lexer.next(),
            Some(Ok(Token(TokenKind::Symbol(Symbol::Plus), (0, 1), (0, 2))))
        );
        assert_eq!(
            lexer.next(),
            Some(Ok(Token(TokenKind::Number(14.0), (0, 3), (0, 5))))
        );
        assert_eq!(
            lexer.next(),
            Some(Ok(Token(TokenKind::Number(25.5), (0, 6), (0, 10))))
        );
        assert_eq!(
            lexer.next(),
            Some(Ok(Token(TokenKind::Number(333.0), (0, 11), (0, 14))))
        );
        assert_eq!(
            lexer.next(),
            Some(Ok(Token(TokenKind::OpenParen, (1, 0), (1, 1))))
        );
        assert_eq!(
            lexer.next(),
            Some(Ok(Token(
                TokenKind::Symbol(Symbol::Multiply),
                (1, 1),
                (1, 2)
            )))
        );
        assert_eq!(
            lexer.next(),
            Some(Ok(Token(TokenKind::Number(2.0), (1, 3), (1, 4))))
        );
        assert_eq!(
            lexer.next(),
            Some(Ok(Token(TokenKind::Number(5.0), (1, 5), (1, 6))))
        );
        assert_eq!(
            lexer.next(),
            Some(Err(Error::new(
                ErrorKind::InvalidNumber,
                "5.x".into(),
                (1, 7),
                (1, 10),
            )))
        );
    }

    #[test]
    fn negative_minus() {
        let mut lexer = Lexer::new("(- 1 -2 3)");

        assert_eq!(
            lexer.next(),
            Some(Ok(Token(TokenKind::OpenParen, (0, 0), (0, 1))))
        );
        assert_eq!(
            lexer.next(),
            Some(Ok(Token(TokenKind::Symbol(Symbol::Minus), (0, 1), (0, 2))))
        );
        assert_eq!(
            lexer.next(),
            Some(Ok(Token(TokenKind::Number(1.0), (0, 3), (0, 4))))
        );
        assert_eq!(
            lexer.next(),
            Some(Ok(Token(TokenKind::Number(-2.0), (0, 5), (0, 7))))
        );
        assert_eq!(
            lexer.next(),
            Some(Ok(Token(TokenKind::Number(3.0), (0, 8), (0, 9))))
        );
        assert_eq!(
            lexer.next(),
            Some(Ok(Token(TokenKind::CloseParen, (0, 9), (0, 10))))
        );
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn error_parse_numbers() {
        let mut lexer = Lexer::new("2 55 3.144 0.0001 1.1.1");

        assert_eq!(
            lexer.next(),
            Some(Ok(Token(TokenKind::Number(2.0), (0, 0), (0, 1))))
        );
        assert_eq!(
            lexer.next(),
            Some(Ok(Token(TokenKind::Number(55.0), (0, 2), (0, 4))))
        );
        assert_eq!(
            lexer.next(),
            Some(Ok(Token(TokenKind::Number(3.144), (0, 5), (0, 10))))
        );
        assert_eq!(
            lexer.next(),
            Some(Ok(Token(TokenKind::Number(0.0001), (0, 11), (0, 17))))
        );
        assert_eq!(
            lexer.next(),
            Some(Err(Error::new(
                ErrorKind::InvalidNumber,
                "1.1.1".into(),
                (0, 18),
                (0, 18 + 5),
            )))
        );
    }
}
