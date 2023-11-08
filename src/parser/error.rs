// faye, a pretty lil lisp
// Copyright (c) 2023 fawn
//
// SPDX-License-Identifier: Apache-2.0

use crate::prelude::{LexerError, Span};

/// Parse errors with a start and end location
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
        write!(f, "{}", self.kind)
    }
}

impl std::error::Error for Error {}

impl From<LexerError> for Error {
    fn from(e: LexerError) -> Self {
        let span = e.span.clone();
        Self::new(ErrorKind::Lexer(e), span)
    }
}

/// Errors that can occur while parsing an AST from tokens
#[derive(Debug, PartialEq, Clone)]
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
