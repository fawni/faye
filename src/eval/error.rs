// faye, a pretty lil lisp
// Copyright (c) 2023 fawn
//
// SPDX-License-Identifier: Apache-2.0

use super::Expr;
use crate::prelude::{Location, Symbol};

/// An evaluation error with a start and end location
#[derive(Debug, PartialEq, Clone)]
pub struct Error {
    pub kind: ErrorKind,
    pub start: Location,
    pub end: Location,
}

impl Error {
    /// Create a new evaluation error
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

/// Errors that can occur while evaluating an expression
#[derive(Debug, PartialEq, Clone)]
pub enum ErrorKind {
    UnknownSymbol(Symbol),
    MissingArguments,
    TooManyArguments,
    InvalidFunction(Expr),
    InvalidArgument(Expr),
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownSymbol(sym) => write!(f, "Could not resolve symbol '{sym}' in scope"),
            Self::MissingArguments => write!(f, "Function is missing arguments"),
            Self::TooManyArguments => write!(f, "Function has extra arguments"),
            Self::InvalidFunction(v) => write!(f, "`{v}` is not a function"),
            Self::InvalidArgument(v) => {
                write!(f, "`{v}` is not a valid argument for this function")
            }
        }
    }
}
