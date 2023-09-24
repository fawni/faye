// faye, a pretty lil lisp
// Copyright (c) 2023 fawn
//
// SPDX-License-Identifier: Apache-2.0

use super::Location;

#[derive(Debug, PartialEq, Clone)]
pub struct Token(pub TokenKind, pub Location, pub Location);

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    Comment(String),
    Symbol(Symbol),
    Number(f64),
    Bool(bool),
    String(String),
    Char(char),
    Keyword(Symbol),
    Nil,
}

/// A symbol used to identify a function or a variable
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Symbol(pub String);

impl Symbol {
    /// Create a new symbol from a string
    pub fn from<T: Into<String>>(s: T) -> Self {
        Self(s.into())
    }
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
