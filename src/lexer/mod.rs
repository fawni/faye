// faye, a pretty lil lisp
// Copyright (c) 2023 fawn
//
// SPDX-License-Identifier: Apache-2.0

use std::{
    str::{Chars, FromStr},
    sync::Arc,
};

pub use error::{Error, ErrorKind};
pub use symbol::Symbol;
pub use token::{Token, TokenKind};

use crate::span::{Source, Span};

mod error;
mod symbol;
mod token;

/// A lexer for the parser
#[derive(Debug)]
pub struct Lexer<'a> {
    input: Chars<'a>,
    byte: usize,
    pub source: Arc<Source>,
}

impl Lexer<'_> {
    /// Create a new lexer instance from a string
    #[must_use]
    pub fn new(input: &str) -> Lexer {
        let source = Arc::new(Source::new(None, input.to_owned()));

        Lexer {
            input: input.chars(),
            byte: 0,
            source,
        }
    }

    pub fn set_name(&mut self, name: String) {
        // TODO: goob
        Arc::get_mut(&mut self.source).unwrap().set_name(name);
    }

    /// Get the current position of the lexer
    #[inline]
    #[must_use]
    pub(crate) fn span(&self) -> Span {
        Span::new(self.byte..self.byte, self.source.clone())
    }

    /// Get the current character
    #[must_use]
    pub fn current(&self) -> Option<char> {
        self.input.as_str().chars().next()
    }

    /// Get the unparsed input
    #[must_use]
    pub fn get_unparsed(&self) -> &str {
        self.input.as_str()
    }

    /// Get the nth character ahead of the current character without advancing
    fn peek(&self, n: usize) -> Option<char> {
        self.input.as_str().chars().nth(n)
    }

    /// Advance the lexer by one character
    fn advance(&mut self) -> Option<char> {
        let c = self.input.next()?;
        self.byte += c.len_utf8();
        Some(c)
    }

    /// Read a word from the input until a separator is reached
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

    /// Parse a value from the input or return an error
    fn parse_or<T: FromStr>(&mut self, err: impl Fn(String) -> ErrorKind) -> Result<T, Error> {
        let span = self.span();
        let word = self.read_word();

        word.parse()
            .map_err(|_| Error::new(err(word), span.join(&self.span())))
    }

    /// Read the next token from the input
    pub fn read(&mut self) -> Result<Option<Token>, Error> {
        let c = loop {
            match self.current() {
                Some(c) if c.is_ascii_whitespace() || c == ',' => {
                    self.advance();
                }
                Some(c) => break c,
                None => return Ok(None),
            }
        };

        let mut span = self.span();
        let kind = match c {
            '(' => {
                self.advance();
                TokenKind::OpenParen
            }
            ')' => {
                self.advance();
                TokenKind::CloseParen
            }
            '[' => {
                self.advance();
                TokenKind::OpenBracket
            }
            ']' => {
                self.advance();
                TokenKind::CloseBracket
            }
            '0'..='9' => TokenKind::Number(self.parse_or(ErrorKind::InvalidNumber)?),
            '+' | '-' if matches!(self.peek(1), Some('0'..='9')) => {
                TokenKind::Number(self.parse_or(ErrorKind::InvalidNumber)?)
            }
            ';' => {
                self.advance();
                let mut comment = String::new();
                while let Some(c) = self.advance() {
                    if c == '\n' {
                        break;
                    }
                    comment.push(c);
                }
                TokenKind::Comment(comment)
            }
            ':' => {
                self.advance();
                TokenKind::Keyword(Symbol(self.read_word()))
            }
            '"' => {
                self.advance();
                let quote_span = span.clone().join(&self.span());
                let mut string = String::new();

                loop {
                    let ch_span = self.span();
                    string.push(match self.advance() {
                        Some('"') => break,
                        Some('\\') => match self.advance() {
                            Some(c @ ('"' | '\\')) => c,
                            Some('n') => '\n',
                            Some('e') => '\x1b',
                            Some(c) => {
                                return Err(Error::new(
                                    ErrorKind::InvalidEscape(c),
                                    ch_span.join(&self.span()),
                                ))
                            }
                            None => return Err(Error::new(ErrorKind::UnclosedString, quote_span)),
                        },
                        Some(c) => c,
                        None => return Err(Error::new(ErrorKind::UnclosedString, quote_span)),
                    });
                }

                if self.current().is_some_and(|c| !c.is_separator()) {
                    self.read_word();
                    return Err(Error::new(
                        ErrorKind::InvalidString,
                        span.join(&self.span()),
                    ));
                }

                TokenKind::String(string)
            }
            '\'' => {
                self.advance();
                let char = match self.advance() {
                    Some('\\') => match self.advance() {
                        Some(c @ ('"' | '\\')) => c,
                        Some('n') => '\n',
                        Some('e') => '\x1b',
                        Some(c) => {
                            return Err(Error::new(
                                ErrorKind::InvalidEscape(c),
                                span.join(&self.span()),
                            ))
                        }
                        None => {
                            return Err(Error::new(
                                ErrorKind::UnclosedChar,
                                span.join(&self.span()),
                            ));
                        }
                    },
                    Some(c) => c,
                    _ => return Err(Error::new(ErrorKind::InvalidChar, span.join(&self.span()))),
                };

                if self.advance() != Some('\'') {
                    self.read_word();
                    return Err(Error::new(ErrorKind::InvalidChar, span.join(&self.span())));
                }

                TokenKind::Char(char)
            }
            _ => {
                let word = self.read_word();
                match word.as_str() {
                    "true" => TokenKind::Bool(true),
                    "false" => TokenKind::Bool(false),
                    "nil" => TokenKind::Nil,
                    _ => TokenKind::Symbol(Symbol::from(word)),
                }
            }
        };

        span.extend(&self.span());

        Ok(Some(Token::new(kind, span)))
    }
}

impl Iterator for Lexer<'_> {
    type Item = Result<Token, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.read().transpose()
    }
}

/// A trait for checking if a character is a separator
pub trait Separator {
    /// Check if the character is a separator
    fn is_separator(&self) -> bool;
}

impl Separator for char {
    fn is_separator(&self) -> bool {
        self.is_ascii_whitespace() || matches!(self, '(' | ')' | '[' | ']' | ';' | ',')
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test {
        ($name:ident: $input:literal, $tokens:expr) => {
            #[test]
            fn $name() {
                let mut lexer = Lexer::new($input);

                for token in $tokens {
                    let x = lexer.next().map(|r| match r {
                        Ok(t) => Ok(t.kind),
                        Err(e) => Err(e.kind),
                    });
                    assert_eq!(x, Some(token));
                }
                assert_eq!(lexer.next(), None);
            }
        };
    }

    test!(lex: "(+ 14 25.5 333 (* 2 5))", [
        Ok(TokenKind::OpenParen),
        Ok(TokenKind::Symbol(Symbol::from("+"))),
        Ok(TokenKind::Number(14.)),
        Ok(TokenKind::Number(25.5)),
        Ok(TokenKind::Number(333.)),
        Ok(TokenKind::OpenParen),
        Ok(TokenKind::Symbol(Symbol::from("*"))),
        Ok(TokenKind::Number(2.)),
        Ok(TokenKind::Number(5.)),
        Ok(TokenKind::CloseParen),
        Ok(TokenKind::CloseParen),
    ]);

    test!(newline: "(+ 14 25.5 333\n(* 2 5 5.x))", [
        Ok(TokenKind::OpenParen),
        Ok(TokenKind::Symbol(Symbol::from("+"))),
        Ok(TokenKind::Number(14.)),
        Ok(TokenKind::Number(25.5)),
        Ok(TokenKind::Number(333.)),
        Ok(TokenKind::OpenParen),
        Ok(TokenKind::Symbol(Symbol::from("*"))),
        Ok(TokenKind::Number(2.)),
        Ok(TokenKind::Number(5.)),
        Err(ErrorKind::InvalidNumber("5.x".into())),
        Ok(TokenKind::CloseParen),
        Ok(TokenKind::CloseParen),
    ]);

    test!(negative_minus: "(- 1 -2 3)", [
        Ok(TokenKind::OpenParen),
        Ok(TokenKind::Symbol(Symbol::from("-"))),
        Ok(TokenKind::Number(1.)),
        Ok(TokenKind::Number(-2.)),
        Ok(TokenKind::Number(3.)),
        Ok(TokenKind::CloseParen),
    ]);

    test!(error_parse_numbers: "2 55 3.144 0.0001 1.1.1", [
        Ok(TokenKind::Number(2.)),
        Ok(TokenKind::Number(55.)),
        Ok(TokenKind::Number(3.144)),
        Ok(TokenKind::Number(0.0001)),
        Err(ErrorKind::InvalidNumber("1.1.1".into())),
    ]);

    test!(error_unclosed_string: "\"hiii", [Err(ErrorKind::UnclosedString)]);

    test!(error_invalid_string: "\"hiii\"222", [Err(ErrorKind::InvalidString)]);
}
