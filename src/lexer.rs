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
    pub const fn location(&self) -> Location {
        (self.line, self.col)
    }

    pub fn current(&self) -> Option<char> {
        self.input.as_str().chars().next()
    }

    fn peek(&self, n: usize) -> Option<char> {
        self.input.as_str().chars().nth(n)
    }

    fn advance(&mut self) -> Option<char> {
        self.col += 1;
        self.input.next()
    }

    fn newline(&mut self) {
        self.line += 1;
        self.col = 0;
    }

    fn read_word(&mut self) -> String {
        let mut word = String::new();
        while let Some(c) = self.current() {
            if c.is_seperator() {
                break;
            }

            word.push(c);
            self.advance();
        }

        word
    }

    fn parse_or<T: FromStr>(&mut self, err: impl Fn(String) -> ErrorKind) -> Result<T, Error> {
        let start = self.location();
        let word = self.read_word();

        word.parse()
            .map_err(|_| Error::new(err(word), start, self.location()))
    }

    pub fn read(&mut self) -> Result<Option<Token>, Error> {
        loop {
            match self.current() {
                Some(' ') => {
                    self.advance();
                }
                Some('\n') => {
                    self.advance();
                    self.newline();
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
            Some('0'..='9') => TokenKind::Number(self.parse_or(ErrorKind::InvalidNumber)?),
            Some('+' | '-') if matches!(self.peek(1), Some('0'..='9')) => {
                TokenKind::Number(self.parse_or(ErrorKind::InvalidNumber)?)
            }
            Some(';') => {
                while let Some(c) = self.advance() {
                    if c == '\n' {
                        break;
                    }
                    self.advance();
                }
                self.newline();
                TokenKind::Comment
            }
            Some('"') => {
                self.advance();
                let quote_end = self.location();
                let mut string = String::new();

                loop {
                    match self.current() {
                        Some('"') => {
                            self.advance();
                            break;
                        }
                        Some('\n') => {
                            self.advance();
                            self.newline()
                        }
                        Some('\\') => {
                            let esc_start = self.location();
                            self.advance();
                            match self.advance() {
                                Some(c @ ('"' | '\\')) => string.push(c),
                                Some('n') => string.push('\n'),
                                Some(c) => {
                                    return Err(Error::new(
                                        ErrorKind::InvalidEscape(c),
                                        esc_start,
                                        self.location(),
                                    ))
                                }
                                None => {
                                    return Err(Error::new(
                                        ErrorKind::UnclosedString,
                                        start,
                                        quote_end,
                                    ))
                                }
                            }
                        }
                        Some(c) => {
                            string.push(c);
                            self.advance();
                        }
                        None => {
                            return Err(Error::new(ErrorKind::UnclosedString, start, quote_end))
                        }
                    }
                }

                TokenKind::String(string.to_owned())
            }
            Some(_) => {
                let word = self.read_word();
                match word.as_str() {
                    "true" => TokenKind::Bool(true),
                    "false" => TokenKind::Bool(false),
                    _ => TokenKind::Symbol(word.parse().map_err(|_| {
                        Error::new(ErrorKind::InvalidSymbol(word), start, self.location())
                    })?),
                }
            }
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

#[derive(Debug, PartialEq, Clone)]
pub struct Token(pub TokenKind, pub Location, pub Location);

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    OpenParen,
    CloseParen,
    Comment,
    Symbol(Symbol),
    Number(f64),
    Bool(bool),
    String(String),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Symbol {
    Plus,
    Minus,
    Multiply,
    Divide,
    Equal,
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::Multiply => write!(f, "*"),
            Self::Divide => write!(f, "/"),
            Self::Equal => write!(f, "="),
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
            "=" => Ok(Self::Equal),
            _ => Err(ErrorKind::InvalidSymbol(s.to_owned())),
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

pub type Location = (usize, usize);

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Error {
    pub kind: ErrorKind,
    pub start: Location,
    pub end: Location,
}

impl Error {
    pub const fn new(kind: ErrorKind, start: Location, end: Location) -> Self {
        Self { kind, start, end }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ErrorKind::InvalidNumber(n) => write!(f, "`{n}` is not a valid numeric literal"),
            ErrorKind::InvalidSymbol(s) => write!(f, "`{s}` is not a valid symbol"),
            ErrorKind::UnclosedString => write!(f, "Unclosed string literal"),
            ErrorKind::InvalidEscape(c) => write!(f, "Unknown escape `\\{c}` in string"),
        }
    }
}

impl std::error::Error for Error {}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ErrorKind {
    InvalidSymbol(String),
    InvalidNumber(String),
    UnclosedString,
    InvalidEscape(char),
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
                ErrorKind::InvalidNumber("5.x".into()),
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
                ErrorKind::InvalidNumber("1.1.1".into()),
                (0, 18),
                (0, 18 + 5),
            )))
        );
    }

    #[test]
    fn error_unclosed_string() {
        let mut lexer = Lexer::new("\"hiii");

        assert_eq!(
            lexer.next(),
            Some(Err(Error::new(ErrorKind::UnclosedString, (0, 0), (0, 1),)))
        );
    }
}
