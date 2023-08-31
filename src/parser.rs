// faye, a pretty lil lisp
// Copyright (c) 2023 fawn
//
// SPDX-License-Identifier: Apache-2.0

use crate::lexer::{Error as LexerError, Lexer, Symbol, Token, TokenKind};

#[derive(Debug, PartialEq, Clone)]
pub struct Node(pub Expr, pub (usize, usize), pub (usize, usize));

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Number(f64),
    Symbol(Symbol),
    List(Vec<Node>),
    CloseParen,
}

pub fn parse(lexer: &mut Lexer) -> Result<Vec<Node>, Error> {
    let mut ast: Vec<Node> = Vec::new();

    if lexer.current().is_none() {
        return Err(Error::new(
            ErrorKind::Empty,
            lexer.location(),
            lexer.location(),
        ));
    }

    while lexer.current().is_some() {
        match parse_next(lexer)? {
            Node(Expr::CloseParen, start, end) => {
                return Err(Error::new(ErrorKind::UnexpectedCloseParen, start, end))
            }
            node => ast.push(node),
        }
    }

    Ok(ast)
}

fn parse_next(lexer: &mut Lexer) -> Result<Node, Error> {
    match lexer.read()? {
        Some(Token(TokenKind::Number(n), start, end)) => Ok(Node(Expr::Number(n), start, end)),
        Some(Token(TokenKind::Symbol(sym), start, end)) => Ok(Node(Expr::Symbol(sym), start, end)),
        Some(Token(TokenKind::OpenParen, start, _)) => parse_list(lexer, start),
        Some(Token(TokenKind::CloseParen, start, end)) => Ok(Node(Expr::CloseParen, start, end)),
        None => Err(Error::new(
            ErrorKind::Empty,
            lexer.location(),
            lexer.location(),
        )),
    }
}

fn parse_list(lexer: &mut Lexer, start: (usize, usize)) -> Result<Node, Error> {
    let mut res: Vec<Node> = Vec::new();
    loop {
        match parse_next(lexer)? {
            Node(Expr::CloseParen, ..) => break,
            node => res.push(node),
        }
    }

    Ok(Node(Expr::List(res), start, lexer.location()))
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Error {
    pub kind: ErrorKind,
    pub start: (usize, usize),
    pub end: (usize, usize),
}

impl Error {
    pub fn new(kind: ErrorKind, start: (usize, usize), end: (usize, usize)) -> Self {
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ErrorKind {
    Lexer(LexerError),
    UnexpectedCloseParen,
    Empty,
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "Nothing to parse"),
            Self::Lexer(e) => write!(f, "{e}"),
            Self::UnexpectedCloseParen => write!(f, "Unexpected Close Paren"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_list() {
        let mut lexer = Lexer::new("(+ 1 2)");
        let res = parse(&mut lexer);
        assert_eq!(
            res,
            Ok(vec![Node(
                Expr::List(vec![
                    Node(Expr::Symbol(Symbol::Plus), (0, 1), (0, 2)),
                    Node(Expr::Number(1.0), (0, 3), (0, 4)),
                    Node(Expr::Number(2.0), (0, 5), (0, 6)),
                ]),
                (0, 0),
                (0, 7),
            )])
        );
    }

    #[test]
    fn parse_nested_list() {
        let mut lexer = Lexer::new("(+ 2.5 64 (* 2 3))");
        let res = parse(&mut lexer);
        assert_eq!(
            res,
            Ok(vec![Node(
                Expr::List(vec![
                    Node(Expr::Symbol(Symbol::Plus), (0, 1), (0, 2)),
                    Node(Expr::Number(2.5), (0, 3), (0, 6)),
                    Node(Expr::Number(64.0), (0, 7), (0, 9)),
                    Node(
                        Expr::List(vec![
                            Node(Expr::Symbol(Symbol::Multiply), (0, 11), (0, 12)),
                            Node(Expr::Number(2.0), (0, 13), (0, 14)),
                            Node(Expr::Number(3.0), (0, 15), (0, 16)),
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
        let mut lexer = Lexer::new("(/ 6 3 (+ 1 2)) (* 2 5)\n(- 10 5)");
        let res = parse(&mut lexer);
        assert_eq!(
            res,
            Ok(vec![
                Node(
                    Expr::List(vec![
                        Node(Expr::Symbol(Symbol::Divide), (0, 1), (0, 2)),
                        Node(Expr::Number(6.0), (0, 3), (0, 4)),
                        Node(Expr::Number(3.0), (0, 5), (0, 6)),
                        Node(
                            Expr::List(vec![
                                Node(Expr::Symbol(Symbol::Plus), (0, 8), (0, 9)),
                                Node(Expr::Number(1.0), (0, 10), (0, 11)),
                                Node(Expr::Number(2.0), (0, 12), (0, 13)),
                            ]),
                            (0, 7),
                            (0, 14),
                        ),
                    ]),
                    (0, 0),
                    (0, 15),
                ),
                Node(
                    Expr::List(vec![
                        Node(Expr::Symbol(Symbol::Multiply), (0, 17), (0, 18)),
                        Node(Expr::Number(2.0), (0, 19), (0, 20)),
                        Node(Expr::Number(5.0), (0, 21), (0, 22)),
                    ]),
                    (0, 16),
                    (0, 16 + 7),
                ),
                Node(
                    Expr::List(vec![
                        Node(Expr::Symbol(Symbol::Minus), (1, 1), (1, 2)),
                        Node(Expr::Number(10.0), (1, 3), (1, 5)),
                        Node(Expr::Number(5.0), (1, 6), (1, 7)),
                    ]),
                    (1, 0),
                    (1, 8),
                ),
            ])
        );
    }

    #[test]
    fn parse_float() {
        let mut lexer = Lexer::new("(2.500000)");
        let res = parse(&mut lexer);
        assert_eq!(
            res,
            Ok(vec![Node(
                Expr::List(vec![Node(Expr::Number(2.5), (0, 1), (0, 9))]),
                (0, 0),
                (0, 10),
            )])
        );
    }

    #[test]
    fn error_empty() {
        let mut lexer = Lexer::new("");
        let res = parse(&mut lexer);
        assert_eq!(res, Err(Error::new(ErrorKind::Empty, (0, 0), (0, 0))));
    }

    #[test]
    fn error_invalid_number() {
        let mut lexer = Lexer::new("(+ 1.2.3)");
        let res = parse(&mut lexer);
        assert_eq!(
            res,
            Err(Error::new(
                ErrorKind::Lexer(LexerError::new(
                    crate::lexer::ErrorKind::InvalidNumber,
                    "1.2.3".into(),
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
        let mut lexer = Lexer::new(")");
        let res = parse(&mut lexer);
        assert_eq!(
            res,
            Err(Error::new(ErrorKind::UnexpectedCloseParen, (0, 0), (0, 1)))
        );
    }
}
