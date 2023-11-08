// faye, a pretty lil lisp
// Copyright (c) 2023 fawn
//
// SPDX-License-Identifier: Apache-2.0

use crate::span::Span;

/// Lexer errors with a start and end location
#[derive(Debug, PartialEq, Clone)]
pub struct Error {
    pub kind: ErrorKind,
    pub span: Span,
}

impl Error {
    #[must_use]
    pub const fn new(kind: ErrorKind, span: Span) -> Self {
        Self { kind, span }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ErrorKind::InvalidNumber(n) => write!(f, "`{n}` is not a valid numeric literal"),
            ErrorKind::InvalidEscape(c) => write!(f, "Unknown escape sequence `\\{c}` in string"),
            ErrorKind::InvalidString => write!(f, "Invalid string literal"),
            ErrorKind::UnclosedString => write!(f, "Unclosed string literal"),
            ErrorKind::InvalidChar => write!(f, "Invalid character literal"),
            ErrorKind::UnclosedChar => write!(f, "Unclosed character literal"),
        }
    }
}

impl std::error::Error for Error {}

/// Errors that can occur while lexing a string into tokens
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ErrorKind {
    InvalidNumber(String),
    InvalidEscape(char),
    InvalidString,
    UnclosedString,
    InvalidChar,
    UnclosedChar,
}
