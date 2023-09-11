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
    #[must_use]
    pub fn new(input: &str) -> Lexer {
        Lexer {
            input: input.chars(),
            line: 0,
            col: 0,
        }
    }

    #[inline]
    #[must_use]
    pub const fn location(&self) -> Location {
        (self.line, self.col)
    }

    #[must_use]
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
            if c.is_separator() {
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
            // todo: quote
            // Some('\'') => {
            //     self.advance();
            //     TokenKind::Quote
            // }
            Some('0'..='9') => TokenKind::Number(self.parse_or(ErrorKind::InvalidNumber)?),
            Some('+' | '-') if matches!(self.peek(1), Some('0'..='9')) => {
                TokenKind::Number(self.parse_or(ErrorKind::InvalidNumber)?)
            }
            Some(';') => {
                self.advance();
                let mut comment = String::new();
                while let Some(c) = self.advance() {
                    if c == '\n' {
                        self.newline();
                        break;
                    }
                    comment.push(c);
                }
                TokenKind::Comment(comment)
            }
            Some(':') => {
                self.advance();
                let word = self.read_word();
                TokenKind::Keyword(Symbol(word))
            }
            Some('"') => {
                self.advance();
                let quote_end = self.location();
                let mut string = String::new();

                loop {
                    let ch_start = self.location();
                    string.push(match self.advance() {
                        Some('"') => break,
                        Some('\\') => match self.advance() {
                            Some(c @ ('"' | '\\')) => c,
                            Some('n') => '\n',
                            Some('e') => '\x1b',
                            Some(c) => {
                                return Err(Error::new(
                                    ErrorKind::InvalidEscape(c),
                                    ch_start,
                                    self.location(),
                                ))
                            }
                            None => {
                                return Err(Error::new(ErrorKind::UnclosedString, start, quote_end))
                            }
                        },
                        Some('\n') => {
                            self.newline();
                            '\n'
                        }
                        Some(c) => c,
                        None => {
                            return Err(Error::new(ErrorKind::UnclosedString, start, quote_end))
                        }
                    });
                }

                if self.current().is_some_and(|c| !c.is_separator()) {
                    while self.current().is_some_and(|c| !c.is_separator()) {
                        self.advance();
                    }

                    return Err(Error::new(ErrorKind::InvalidString, start, self.location()));
                }

                TokenKind::String(string)
            }
            Some(_) => {
                let word = self.read_word();
                match word.as_str() {
                    "true" => TokenKind::Bool(true),
                    "false" => TokenKind::Bool(false),
                    "nil" => TokenKind::Nil,
                    _ => TokenKind::Symbol(Symbol::from(word)),
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
    // Quote, // todo
    Comment(String),
    Symbol(Symbol),
    Number(f64),
    Bool(bool),
    String(String),
    Keyword(Symbol),
    Nil,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Symbol(pub String);

impl Symbol {
    pub fn from<T: Into<String>>(s: T) -> Self {
        Self(s.into())
    }
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

trait Separator {
    fn is_separator(&self) -> bool;
}

impl Separator for char {
    fn is_separator(&self) -> bool {
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
    #[must_use]
    pub const fn new(kind: ErrorKind, start: Location, end: Location) -> Self {
        Self { kind, start, end }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ErrorKind::InvalidNumber(n) => write!(f, "`{n}` is not a valid numeric literal"),
            ErrorKind::InvalidEscape(c) => write!(f, "Unknown escape sequence `\\{c}` in string"),
            ErrorKind::InvalidString => write!(f, "Invalid string literal"),
            ErrorKind::UnclosedString => write!(f, "Unclosed string literal"),
        }
    }
}

impl std::error::Error for Error {}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ErrorKind {
    InvalidNumber(String),
    InvalidEscape(char),
    InvalidString,
    UnclosedString,
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
            Some(Ok(Token(
                TokenKind::Symbol(Symbol::from("+")),
                (0, 1),
                (0, 2)
            )))
        );
        assert_eq!(
            lexer.next(),
            Some(Ok(Token(TokenKind::Number(14.), (0, 3), (0, 5))))
        );
        assert_eq!(
            lexer.next(),
            Some(Ok(Token(TokenKind::Number(25.5), (0, 6), (0, 10))))
        );
        assert_eq!(
            lexer.next(),
            Some(Ok(Token(TokenKind::Number(333.), (0, 11), (0, 14))))
        );
        assert_eq!(
            lexer.next(),
            Some(Ok(Token(TokenKind::OpenParen, (0, 15), (0, 16))))
        );
        assert_eq!(
            lexer.next(),
            Some(Ok(Token(
                TokenKind::Symbol(Symbol::from("*")),
                (0, 16),
                (0, 17)
            )))
        );
        assert_eq!(
            lexer.next(),
            Some(Ok(Token(TokenKind::Number(2.), (0, 18), (0, 19))))
        );
        assert_eq!(
            lexer.next(),
            Some(Ok(Token(TokenKind::Number(5.), (0, 20), (0, 21))))
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
            Some(Ok(Token(
                TokenKind::Symbol(Symbol::from("+")),
                (0, 1),
                (0, 2)
            )))
        );
        assert_eq!(
            lexer.next(),
            Some(Ok(Token(TokenKind::Number(14.), (0, 3), (0, 5))))
        );
        assert_eq!(
            lexer.next(),
            Some(Ok(Token(TokenKind::Number(25.5), (0, 6), (0, 10))))
        );
        assert_eq!(
            lexer.next(),
            Some(Ok(Token(TokenKind::Number(333.), (0, 11), (0, 14))))
        );
        assert_eq!(
            lexer.next(),
            Some(Ok(Token(TokenKind::OpenParen, (1, 0), (1, 1))))
        );
        assert_eq!(
            lexer.next(),
            Some(Ok(Token(
                TokenKind::Symbol(Symbol::from("*")),
                (1, 1),
                (1, 2)
            )))
        );
        assert_eq!(
            lexer.next(),
            Some(Ok(Token(TokenKind::Number(2.), (1, 3), (1, 4))))
        );
        assert_eq!(
            lexer.next(),
            Some(Ok(Token(TokenKind::Number(5.), (1, 5), (1, 6))))
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
            Some(Ok(Token(
                TokenKind::Symbol(Symbol::from("-")),
                (0, 1),
                (0, 2)
            )))
        );
        assert_eq!(
            lexer.next(),
            Some(Ok(Token(TokenKind::Number(1.), (0, 3), (0, 4))))
        );
        assert_eq!(
            lexer.next(),
            Some(Ok(Token(TokenKind::Number(-2.), (0, 5), (0, 7))))
        );
        assert_eq!(
            lexer.next(),
            Some(Ok(Token(TokenKind::Number(3.), (0, 8), (0, 9))))
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
            Some(Ok(Token(TokenKind::Number(2.), (0, 0), (0, 1))))
        );
        assert_eq!(
            lexer.next(),
            Some(Ok(Token(TokenKind::Number(55.), (0, 2), (0, 4))))
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

    #[test]
    fn error_invalid_string() {
        let mut lexer = Lexer::new("\"hiii\"222");

        assert_eq!(
            lexer.next(),
            Some(Err(Error::new(ErrorKind::InvalidString, (0, 0), (0, 9))))
        );
    }
}
