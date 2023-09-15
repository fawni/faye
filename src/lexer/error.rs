// faye, a pretty lil lisp
// Copyright (c) 2023 fawn
//
// SPDX-License-Identifier: Apache-2.0

use super::Location;

/// Lexer errors with a start and end location
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

/// Errors that can occur while lexing a string into tokens
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ErrorKind {
    InvalidNumber(String),
    InvalidEscape(char),
    InvalidString,
    UnclosedString,
}
