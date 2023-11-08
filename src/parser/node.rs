// faye, a pretty lil lisp
// Copyright (c) 2023 fawn
//
// SPDX-License-Identifier: Apache-2.0

use super::{Error, ErrorKind};
use crate::prelude::{Span, Symbol, Token, TokenKind};

/// A node in the AST with a start and end location
#[derive(Debug, PartialEq, Clone)]
pub struct Node {
    pub kind: NodeKind,
    pub span: Span,
}

/// The type of a node in the AST
#[derive(Debug, PartialEq, Clone)]
pub enum NodeKind {
    Number(f64),
    Bool(bool),
    String(String),
    Char(char),
    Symbol(Symbol),
    Keyword(Symbol),
    List(Vec<Node>),
    Vector(Vec<Node>),
    Nil,
}

impl Node {
    #[must_use]
    pub const fn new(kind: NodeKind, span: Span) -> Self {
        Self { kind, span }
    }

    /// Push a child node onto a list node
    pub fn push_node(&mut self, child: Self) -> Result<(), Error> {
        match &mut self.kind {
            NodeKind::List(c) | NodeKind::Vector(c) => {
                c.push(child);
                Ok(())
            }
            _ => Err(Error::new(ErrorKind::Unreachable, child.span)),
        }
    }
}

impl TryFrom<Token> for Node {
    type Error = Error;

    /// Perform a conversion from a token to a node
    fn try_from(token: Token) -> Result<Self, Self::Error> {
        let kind = match token.kind {
            TokenKind::Number(n) => NodeKind::Number(n),
            TokenKind::Bool(b) => NodeKind::Bool(b),
            TokenKind::Symbol(s) => NodeKind::Symbol(s),
            TokenKind::String(s) => NodeKind::String(s),
            TokenKind::Char(c) => NodeKind::Char(c),
            TokenKind::Keyword(k) => NodeKind::Keyword(k),
            TokenKind::Nil => NodeKind::Nil,
            TokenKind::OpenParen
            | TokenKind::CloseParen
            | TokenKind::OpenBracket
            | TokenKind::CloseBracket
            | TokenKind::Comment(_) => return Err(Error::new(ErrorKind::Unreachable, token.span)),
        };

        Ok(Self::new(kind, token.span))
    }
}
