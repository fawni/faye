// faye, a pretty lil lisp
// Copyright (c) 2023 fawn
//
// SPDX-License-Identifier: Apache-2.0

use crate::lexer::{Lexer, Token, TokenKind};

pub use error::{Error, ErrorKind};
pub use node::{Node, NodeKind};

pub mod error;
pub mod node;

/// A parser for the AST
pub struct Parser {
    input: String,
}

impl Parser {
    /// Create a new parser instace from a string
    #[must_use]
    pub fn new(input: &str) -> Self {
        Self {
            input: input.to_owned(),
        }
    }

    /// Parse the input string into an AST
    pub fn parse(&self) -> Result<Vec<Node>, Error> {
        let mut lexer = Lexer::new(&self.input);
        let mut parents = Vec::new();
        let mut cur_node = Node(
            NodeKind::List(Vec::new()),
            lexer.location(),
            lexer.location(),
        );

        while let Some(token) = lexer.read()? {
            let child = match token {
                Token(TokenKind::Comment(_), ..) => continue,
                Token(TokenKind::OpenParen, start, end) => {
                    let child = Node(NodeKind::List(Vec::new()), start, end);
                    parents.push(cur_node);
                    cur_node = child;
                    continue;
                }
                Token(TokenKind::CloseParen, start, end) => {
                    let mut parent = parents
                        .pop()
                        .ok_or_else(|| Error::new(ErrorKind::UnexpectedCloseParen, start, end))?;
                    cur_node.2 = end;
                    parent.push(cur_node)?;
                    cur_node = parent;
                    continue;
                }
                _ => Node::try_from(token)?,
            };

            cur_node.push(child)?;
        }

        if !parents.is_empty() {
            return Err(Error::new(ErrorKind::UnclosedParen, cur_node.1, cur_node.2));
        }

        match cur_node {
            Node(NodeKind::List(body), ..) => Ok(body),
            _ => Err(Error::new(ErrorKind::Unreachable, cur_node.1, cur_node.2)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{LexerError, Symbol};

    use super::*;

    #[test]
    fn parse_list() {
        let parser = Parser::new("(+ 1 2)");
        let res = parser.parse();
        assert_eq!(
            res,
            Ok(vec![Node(
                NodeKind::List(vec![
                    Node(NodeKind::Symbol(Symbol::from("+")), (0, 1), (0, 2)),
                    Node(NodeKind::Number(1.), (0, 3), (0, 4)),
                    Node(NodeKind::Number(2.), (0, 5), (0, 6)),
                ]),
                (0, 0),
                (0, 7),
            )])
        );
    }

    #[test]
    fn parse_nested_list() {
        let parser = Parser::new("(+ 2.5 64 (* 2 3))");
        let res = parser.parse();
        assert_eq!(
            res,
            Ok(vec![Node(
                NodeKind::List(vec![
                    Node(NodeKind::Symbol(Symbol::from("+")), (0, 1), (0, 2)),
                    Node(NodeKind::Number(2.5), (0, 3), (0, 6)),
                    Node(NodeKind::Number(64.), (0, 7), (0, 9)),
                    Node(
                        NodeKind::List(vec![
                            Node(NodeKind::Symbol(Symbol::from("*")), (0, 11), (0, 12)),
                            Node(NodeKind::Number(2.), (0, 13), (0, 14)),
                            Node(NodeKind::Number(3.), (0, 15), (0, 16)),
                        ]),
                        (0, 10),
                        (0, 17)
                    ),
                ]),
                (0, 0),
                (0, 18)
            )])
        );
    }

    #[test]
    fn parse_multiple_expressions() {
        let parser = Parser::new("(/ 6 3 (+ 1 2)) (* 2 5)\n(- 10 5)");
        let res = parser.parse();
        assert_eq!(
            res,
            Ok(vec![
                Node(
                    NodeKind::List(vec![
                        Node(NodeKind::Symbol(Symbol::from("/")), (0, 1), (0, 2)),
                        Node(NodeKind::Number(6.), (0, 3), (0, 4)),
                        Node(NodeKind::Number(3.), (0, 5), (0, 6)),
                        Node(
                            NodeKind::List(vec![
                                Node(NodeKind::Symbol(Symbol::from("+")), (0, 8), (0, 9)),
                                Node(NodeKind::Number(1.), (0, 10), (0, 11)),
                                Node(NodeKind::Number(2.), (0, 12), (0, 13)),
                            ]),
                            (0, 7),
                            (0, 14),
                        ),
                    ]),
                    (0, 0),
                    (0, 15),
                ),
                Node(
                    NodeKind::List(vec![
                        Node(NodeKind::Symbol(Symbol::from("*")), (0, 17), (0, 18)),
                        Node(NodeKind::Number(2.), (0, 19), (0, 20)),
                        Node(NodeKind::Number(5.), (0, 21), (0, 22)),
                    ]),
                    (0, 16),
                    (0, 16 + 7),
                ),
                Node(
                    NodeKind::List(vec![
                        Node(NodeKind::Symbol(Symbol::from("-")), (1, 1), (1, 2)),
                        Node(NodeKind::Number(10.), (1, 3), (1, 5)),
                        Node(NodeKind::Number(5.), (1, 6), (1, 7)),
                    ]),
                    (1, 0),
                    (1, 8),
                ),
            ])
        );
    }

    #[test]
    fn parse_float() {
        let parser = Parser::new("(2.500000)");
        let res = parser.parse();
        assert_eq!(
            res,
            Ok(vec![Node(
                NodeKind::List(vec![Node(NodeKind::Number(2.5), (0, 1), (0, 9))]),
                (0, 0),
                (0, 10),
            )])
        );
    }

    #[test]
    fn parse_empty() {
        let parser = Parser::new("");
        let res = parser.parse();
        assert_eq!(res, Ok(Vec::new()));
    }

    #[test]
    fn error_invalid_number() {
        let parser = Parser::new("(+ 1.2.3)");
        let res = parser.parse();
        assert_eq!(
            res,
            Err(Error::new(
                ErrorKind::Lexer(LexerError::new(
                    crate::lexer::ErrorKind::InvalidNumber("1.2.3".into()),
                    (0, 3),
                    (0, 8),
                )),
                (0, 3),
                (0, 8),
            ))
        );
    }

    #[test]
    fn error_unexpected_close_paren() {
        let parser = Parser::new(")");
        let res = parser.parse();
        assert_eq!(
            res,
            Err(Error::new(ErrorKind::UnexpectedCloseParen, (0, 0), (0, 1)))
        );
    }
}
