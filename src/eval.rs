// faye, a pretty lil lisp
// Copyright (c) 2023 fawn
//
// SPDX-License-Identifier: Apache-2.0

use crate::{
    lexer::Symbol,
    parser::{Expr, Node},
};

pub fn eval(ast: &Node) -> Result<f64, Error> {
    match ast {
        Node(Expr::Number(n), ..) => Ok(*n),
        Node(Expr::List(list), start, end) => {
            if list.len() < 2 {
                return Err(Error::new(ErrorKind::MissingArguments, *start, *end));
            }

            let (func, sym_start, sym_end) = match list.get(0) {
                Some(Node(Expr::Symbol(sym), start, end)) => (sym, start, end),
                Some(Node(_, start, end)) => {
                    return Err(Error::new(ErrorKind::InvalidFunction, *start, *end))
                }
                None => return Err(Error::new(ErrorKind::MissingArguments, *start, *end)),
            };

            let args = list[1..].iter().map(eval).collect::<Result<Vec<_>, _>>()?;

            match func {
                Symbol::Plus => Ok(args.into_iter().sum()),
                Symbol::Minus => Ok(args.into_iter().reduce(|acc, x| acc - x).ok_or(
                    Error::new(ErrorKind::CalculationError, *sym_start, *sym_end),
                )?),
                Symbol::Multiply => Ok(args.into_iter().reduce(|acc, x| acc * x).ok_or(
                    Error::new(ErrorKind::CalculationError, *sym_start, *sym_end),
                )?),
                Symbol::Divide => Ok(args.into_iter().reduce(|acc, x| acc / x).ok_or(
                    Error::new(ErrorKind::CalculationError, *sym_start, *sym_end),
                )?),
            }
        }
        Node(Expr::Symbol(sym), start, end) => {
            Err(Error::new(ErrorKind::SymbolMisplaced(*sym), *start, *end))
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ErrorKind {
    SymbolMisplaced(Symbol),
    MissingArguments,
    InvalidFunction,
    CalculationError,
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SymbolMisplaced(sym) => write!(f, "Symbol '{sym}' should not be here"),
            Self::MissingArguments => write!(f, "Function is missing arguments"),
            Self::InvalidFunction => write!(f, "Function undefined"),
            Self::CalculationError => write!(f, "Calculation error"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_symbol_misplaced() {
        // (+ 1 2 *)
        let ast = Node(
            Expr::List(vec![
                Node(Expr::Symbol(Symbol::Plus), (0, 1), (0, 2)),
                Node(Expr::Number(1.0), (0, 3), (0, 4)),
                Node(Expr::Number(2.0), (0, 5), (0, 6)),
                Node(Expr::Symbol(Symbol::Multiply), (0, 7), (0, 8)),
            ]),
            (0, 0),
            (0, 9),
        );

        let res = eval(&ast);
        assert_eq!(
            res,
            Err(Error::new(
                ErrorKind::SymbolMisplaced(Symbol::Multiply),
                (0, 7),
                (0, 8),
            ))
        );
    }

    #[test]
    fn test_invalid_function() {
        // (1 + 2)
        let ast = Node(
            Expr::List(vec![
                Node(Expr::Number(1.0), (0, 1), (0, 2)),
                Node(Expr::Symbol(Symbol::Plus), (0, 3), (0, 4)),
                Node(Expr::Number(2.0), (0, 5), (0, 6)),
            ]),
            (0, 0),
            (0, 7),
        );

        let res = eval(&ast);
        assert_eq!(
            res,
            Err(Error::new(ErrorKind::InvalidFunction, (0, 1), (0, 2),))
        );
    }
}
