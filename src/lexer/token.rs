// faye, a pretty lil lisp
// Copyright (c) 2023 fawn
//
// SPDX-License-Identifier: Apache-2.0

use super::{Location, Symbol};

/// A token with a start and end location
#[derive(Debug, PartialEq, Clone)]
pub struct Token(pub TokenKind, pub Location, pub Location);

/// The type of a token
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
