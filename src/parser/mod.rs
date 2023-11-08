// faye, a pretty lil lisp
// Copyright (c) 2023 fawn
//
// SPDX-License-Identifier: Apache-2.0

use crate::lexer::{Lexer, TokenKind};

pub use error::{Error, ErrorKind};
pub use node::{Node, NodeKind};

mod error;
mod node;

/// A parser for the AST
pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    /// Create a new parser instace from a string
    #[must_use]
    pub fn new(input: &'a str) -> Self {
        Self {
            lexer: Lexer::new(input),
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.lexer.set_name(name);
    }

    /// Parse the input string into an AST
    pub fn parse(&mut self) -> Result<Vec<Node>, Error> {
        let mut parents = Vec::new();
        let mut cur_node = Node::new(NodeKind::List(Vec::new()), self.lexer.span());

        while let Some(token) = self.lexer.read()? {
            match token.kind {
                TokenKind::Comment(_) => { /* TODO: maybe add metadata to fns? */ }
                TokenKind::OpenParen => {
                    let child = Node::new(NodeKind::List(Vec::new()), token.span);
                    parents.push(cur_node);
                    cur_node = child;
                }
                TokenKind::OpenBracket => {
                    let child = Node::new(NodeKind::Vector(Vec::new()), token.span);
                    parents.push(cur_node);
                    cur_node = child;
                }
                TokenKind::CloseParen => {
                    let mut parent = parents.pop().ok_or_else(|| {
                        Error::new(ErrorKind::UnexpectedCloseBracket, token.span.clone())
                    })?;
                    cur_node.span.extend(&token.span);
                    if !matches!(cur_node.kind, NodeKind::List(_)) {
                        return Err(Error::new(ErrorKind::UnmatchedBracket, token.span));
                    }
                    parent.push_node(cur_node)?;
                    cur_node = parent;
                }
                TokenKind::CloseBracket => {
                    let mut parent = parents.pop().ok_or_else(|| {
                        Error::new(ErrorKind::UnexpectedCloseBracket, token.span.clone())
                    })?;
                    cur_node.span.extend(&token.span);
                    if !matches!(cur_node.kind, NodeKind::Vector(_)) {
                        return Err(Error::new(ErrorKind::UnmatchedBracket, token.span));
                    }
                    parent.push_node(cur_node)?;
                    cur_node = parent;
                }
                _ => cur_node.push_node(Node::try_from(token)?)?,
            }
        }

        if !parents.is_empty() {
            return Err(Error::new(ErrorKind::UnclosedBracket, cur_node.span));
        }

        match cur_node.kind {
            NodeKind::List(body) => Ok(body),
            _ => Err(Error::new(ErrorKind::Unreachable, cur_node.span)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::{LexerError, Span, Symbol};

    use super::*;

    macro_rules! test {
        ($name:ident: $input:literal, $src:ident => $ast:expr) => {
            #[test]
            fn $name() {
                let mut parser = Parser::new($input);
                let $src = parser.lexer.source.clone();
                assert_eq!(parser.parse(), $ast);
            }
        };
    }

    test!(parse_list: "(+ 1 2)", src => Ok(vec![Node::new(
        NodeKind::List(vec![
            Node::new(NodeKind::Symbol(Symbol::from("+")), Span::new(1..2, src.clone())),
            Node::new(NodeKind::Number(1.), Span::new(3..4, src.clone())),
            Node::new(NodeKind::Number(2.), Span::new(5..6, src.clone())),
        ]),
        Span::new(0..7, src)
    )]));

    test!(parse_nested_list: "(+ 2.5 64 (* 2 3))", src => Ok(vec![Node::new(
        NodeKind::List(vec![
            Node::new(NodeKind::Symbol(Symbol::from("+")), Span::new(1..2, src.clone())),
            Node::new(NodeKind::Number(2.5), Span::new(3..6, src.clone())),
            Node::new(NodeKind::Number(64.), Span::new(7..9, src.clone())),
            Node::new(
                NodeKind::List(vec![
                    Node::new(NodeKind::Symbol(Symbol::from("*")), Span::new(11..12, src.clone())),
                    Node::new(NodeKind::Number(2.), Span::new(13..14, src.clone())),
                    Node::new(NodeKind::Number(3.), Span::new(15..16, src.clone())),
                ]),
                Span::new(10..17, src.clone())
            ),
        ]),
        Span::new(0..18, src)
    )]));

    test!(parse_multiple_expressions: "(/ 6 3 (+ 1 2)) (* 2 5)\n(- 10 5)", src => Ok(vec![
        Node::new(
            NodeKind::List(vec![
                Node::new(NodeKind::Symbol(Symbol::from("/")), Span::new(1..2, src.clone())),
                Node::new(NodeKind::Number(6.), Span::new(3..4, src.clone())),
                Node::new(NodeKind::Number(3.), Span::new(5..6, src.clone())),
                Node::new(
                    NodeKind::List(vec![
                        Node::new(NodeKind::Symbol(Symbol::from("+")), Span::new(8..9, src.clone())),
                        Node::new(NodeKind::Number(1.), Span::new(10..11, src.clone())),
                        Node::new(NodeKind::Number(2.), Span::new(12..13, src.clone())),
                    ]),
                    Span::new(7..14, src.clone())
                ),
            ]),
            Span::new(0..15, src.clone())
        ),
        Node::new(
            NodeKind::List(vec![
                Node::new(NodeKind::Symbol(Symbol::from("*")), Span::new(17..18, src.clone())),
                Node::new(NodeKind::Number(2.), Span::new(19..20, src.clone())),
                Node::new(NodeKind::Number(5.), Span::new(21..22, src.clone())),
            ]),
            Span::new(16..23, src.clone())
        ),
        Node::new(
            NodeKind::List(vec![
                Node::new(NodeKind::Symbol(Symbol::from("-")), Span::new(25..26, src.clone())),
                Node::new(NodeKind::Number(10.), Span::new(27..29, src.clone())),
                Node::new(NodeKind::Number(5.), Span::new(30..31, src.clone())),
            ]),
            Span::new(24..32, src)
        ),
    ]));

    test!(parse_float: "(2.500000)", src => Ok(vec![Node::new(
        NodeKind::List(vec![Node::new(NodeKind::Number(2.5), Span::new(1..9, src.clone()))]),
        Span::new(0..10, src)
    )]));

    test!(parse_empty: "", _src => Ok(vec![]));

    test!(error_invalid_number: "(+ 1.2.3)", src => Err(Error::new(
        ErrorKind::Lexer(LexerError::new(
            crate::lexer::ErrorKind::InvalidNumber("1.2.3".into()),
            Span::new(3..8, src.clone())
        )),
        Span::new(3..8, src)
    )));

    test!(error_unexpected_close_paren: ")", src => Err(Error::new(
        ErrorKind::UnexpectedCloseBracket,
        Span::new(0..1, src)
    )));
}
