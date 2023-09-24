// faye, a pretty lil lisp
// Copyright (c) 2023 fawn
//
// SPDX-License-Identifier: Apache-2.0

use crate::prelude::{LexerError, Location};

/// Parse errors with a start and end location
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
        write!(f, "{}", self.kind)
    }
}

impl std::error::Error for Error {}

impl From<LexerError> for Error {
    fn from(e: LexerError) -> Self {
        let start = e.start;
        let end = e.end;
        Self::new(ErrorKind::Lexer(e), start, end)
    }
}

/// Errors that can occur while parsing an AST from tokens
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ErrorKind {
    Lexer(LexerError),
    UnexpectedCloseBracket,
    UnclosedBracket,
    UnmatchedBracket,
    Unreachable,
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Lexer(e) => write!(f, "{e}"),
            Self::UnexpectedCloseBracket => write!(f, "Unexpected closing bracket"),
            Self::UnclosedBracket => write!(f, "Unclosed parenthesis"),
            Self::UnmatchedBracket => write!(f, "Unmatched bracket"),
            Self::Unreachable => write!(f, "Unexpected parsing state reached"),
        }
    }
}
