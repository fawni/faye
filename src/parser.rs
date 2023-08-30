// faye, a pretty lil lisp
// Copyright (c) 2023 fawn
//
// SPDX-License-Identifier: Apache-2.0

use crate::lexer::{Error as LexerError, ErrorKind as LexerErrorKind, Lexer, Symbol, Token};

#[derive(Debug, PartialEq, Clone)]
pub struct Node(pub Expr, pub (usize, usize, usize));

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
        return Err(Error(ErrorKind::Empty, lexer.location(0)));
    }

    while lexer.current().is_some() {
        match parse_next(lexer)? {
            Node(Expr::CloseParen, _) => {
                return Err(Error(ErrorKind::UnexpectedCloseParen, lexer.location(1)))
            }
            node => ast.push(node),
        }
    }

    Ok(ast)
}

fn parse_next(lexer: &mut Lexer) -> Result<Node, Error> {
    match lexer.next() {
        Some(Ok(Token::Number(n))) => {
            Ok(Node(Expr::Number(n), lexer.location(n.to_string().len())))
        }
        Some(Ok(Token::Symbol(sym))) => Ok(Node(Expr::Symbol(sym), lexer.location(sym.len()))),
        Some(Ok(Token::OpenParen)) => parse_list(lexer),
        Some(Ok(Token::CloseParen)) => Ok(Node(Expr::CloseParen, lexer.location(1))),
        Some(Err(e)) => Err(Error::from(e)),
        None => Err(Error(ErrorKind::Empty, lexer.location(0))),
    }
}

fn parse_list(lexer: &mut Lexer) -> Result<Node, Error> {
    let (line, col_start, _) = lexer.location(1);
    let mut res: Vec<Node> = Vec::new();
    loop {
        match parse_next(lexer)? {
            Node(Expr::CloseParen, _) => break,
            node => res.push(node),
        }
    }
    let (_, col_end, _) = lexer.location(0);

    Ok(Node(
        Expr::List(res),
        (line, col_start, col_end - col_start),
    ))
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Error(pub ErrorKind, pub (usize, usize, usize));

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for Error {}

impl From<LexerError> for Error {
    fn from(e: LexerError) -> Self {
        Self(ErrorKind::Lexer(e.0), e.1)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ErrorKind {
    Lexer(LexerErrorKind),
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
                    Node(Expr::Symbol(Symbol::Plus), (0, 1, 1)),
                    Node(Expr::Number(1.0), (0, 3, 1)),
                    Node(Expr::Number(2.0), (0, 5, 1)),
                ]),
                (0, 0, 7)
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
                    Node(Expr::Symbol(Symbol::Plus), (0, 1, 1)),
                    Node(Expr::Number(2.5), (0, 3, 3)),
                    Node(Expr::Number(64.0), (0, 7, 2)),
                    Node(
                        Expr::List(vec![
                            Node(Expr::Symbol(Symbol::Multiply), (0, 11, 1)),
                            Node(Expr::Number(2.0), (0, 13, 1)),
                            Node(Expr::Number(3.0), (0, 15, 1)),
                        ]),
                        (0, 10, 7)
                    ),
                ]),
                (0, 0, 18)
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
                        Node(Expr::Symbol(Symbol::Divide), (0, 1, 1)),
                        Node(Expr::Number(6.0), (0, 3, 1)),
                        Node(Expr::Number(3.0), (0, 5, 1)),
                        Node(
                            Expr::List(vec![
                                Node(Expr::Symbol(Symbol::Plus), (0, 8, 1)),
                                Node(Expr::Number(1.0), (0, 10, 1)),
                                Node(Expr::Number(2.0), (0, 12, 1)),
                            ]),
                            (0, 7, 7)
                        ),
                    ]),
                    (0, 0, 15)
                ),
                Node(
                    Expr::List(vec![
                        Node(Expr::Symbol(Symbol::Multiply), (0, 17, 1)),
                        Node(Expr::Number(2.0), (0, 19, 1)),
                        Node(Expr::Number(5.0), (0, 21, 1)),
                    ]),
                    (0, 16, 7)
                ),
                Node(
                    Expr::List(vec![
                        Node(Expr::Symbol(Symbol::Minus), (1, 1, 1)),
                        Node(Expr::Number(10.0), (1, 3, 2)),
                        Node(Expr::Number(5.0), (1, 6, 1)),
                    ]),
                    (1, 0, 8)
                ),
            ])
        );
    }

    #[test]
    fn error_empty() {
        let mut lexer = Lexer::new("");
        let res = parse(&mut lexer);
        assert_eq!(res, Err(Error(ErrorKind::Empty, (0, 0, 0))));
    }

    #[test]
    fn error_invalid_number() {
        let mut lexer = Lexer::new("(+ 1.2.3)");
        let res = parse(&mut lexer);
        assert_eq!(
            res,
            Err(Error(
                ErrorKind::Lexer(LexerErrorKind::InvalidNumber("1.2.3".to_owned())),
                (0, 3, 5)
            ))
        );
    }

    #[test]
    fn error_unexpected_close_paren() {
        let mut lexer = Lexer::new(")");
        let res = parse(&mut lexer);
        assert_eq!(res, Err(Error(ErrorKind::UnexpectedCloseParen, (0, 0, 1))));
    }
}
