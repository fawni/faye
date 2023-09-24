// faye, a pretty lil lisp
// Copyright (c) 2023 fawn
//
// SPDX-License-Identifier: Apache-2.0

use super::{Error, ErrorKind};
use crate::{Location, Symbol, Token, TokenKind};

/// A node in the AST with a start and end location
#[derive(Debug, PartialEq, Clone)]
pub struct Node(pub NodeKind, pub Location, pub Location);

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
    /// Push a child node onto a list node
    pub fn push(&mut self, child: Self) -> Result<(), Error> {
        match self {
            Self(NodeKind::List(c) | NodeKind::Vector(c), ..) => {
                c.push(child);
                Ok(())
            }
            _ => Err(Error::new(ErrorKind::Unreachable, child.1, child.2)),
        }
    }
}

impl TryFrom<Token> for Node {
    type Error = Error;

    /// Perform a conversion from a token to a node
    fn try_from(token: Token) -> Result<Self, Self::Error> {
        match token {
            Token(TokenKind::Number(n), start, end) => Ok(Self(NodeKind::Number(n), start, end)),
            Token(TokenKind::Bool(b), start, end) => Ok(Self(NodeKind::Bool(b), start, end)),
            Token(TokenKind::String(s), start, end) => Ok(Self(NodeKind::String(s), start, end)),
            Token(TokenKind::Char(c), start, end) => Ok(Self(NodeKind::Char(c), start, end)),
            Token(TokenKind::Symbol(s), start, end) => Ok(Self(NodeKind::Symbol(s), start, end)),
            Token(TokenKind::Keyword(s), start, end) => Ok(Self(NodeKind::Keyword(s), start, end)),
            Token(TokenKind::Nil, start, end) => Ok(Self(NodeKind::Nil, start, end)),
            Token(
                // not using `_` to get errors for unhandled tokens
                TokenKind::Comment(_)
                | TokenKind::OpenParen
                | TokenKind::CloseParen
                | TokenKind::OpenBracket
                | TokenKind::CloseBracket,
                start,
                end,
            ) => Err(Error::new(ErrorKind::Unreachable, start, end)),
        }
    }
}
