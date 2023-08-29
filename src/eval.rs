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
        Node(Expr::Number(n), _) => Ok(*n),
        Node(Expr::List(list), location) => {
            if list.len() < 2 {
                return Err(Error(ErrorKind::MissingArguments, *location));
            }

            let (func, sym_location) = match list.get(0) {
                Some(Node(Expr::Symbol(sym), loc)) => (sym, loc),
                Some(Node(_, loc)) => return Err(Error(ErrorKind::InvalidFunction, *loc)),
                None => return Err(Error(ErrorKind::MissingArguments, *location)),
            };

            let args = list[1..].iter().map(eval).collect::<Result<Vec<_>, _>>()?;

            match func {
                Symbol::Plus => Ok(args.into_iter().sum()),
                Symbol::Minus => Ok(args
                    .into_iter()
                    .reduce(|acc, x| acc - x)
                    .ok_or(Error(ErrorKind::CalculationError, *sym_location))?),
                Symbol::Multiply => Ok(args
                    .into_iter()
                    .reduce(|acc, x| acc * x)
                    .ok_or(Error(ErrorKind::CalculationError, *sym_location))?),
                Symbol::Divide => Ok(args
                    .into_iter()
                    .reduce(|acc, x| acc / x)
                    .ok_or(Error(ErrorKind::CalculationError, *sym_location))?),
            }
        }
        Node(Expr::Symbol(sym), location) => {
            Err(Error(ErrorKind::SymbolMisplaced(*sym), *location))
        }
        Node(Expr::CloseParen, location) => Err(Error(ErrorKind::Unreachable, *location)),
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Error(pub ErrorKind, pub (usize, usize, usize));

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for Error {}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ErrorKind {
    Unreachable,
    SymbolMisplaced(Symbol),
    MissingArguments,
    InvalidFunction,
    CalculationError,
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unreachable => write!(f, "Supposedly unreachable code reached"),
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
                Node(Expr::Symbol(Symbol::Plus), (0, 1, 1)),
                Node(Expr::Number(1.0), (0, 3, 1)),
                Node(Expr::Number(2.0), (0, 5, 1)),
                Node(Expr::Symbol(Symbol::Multiply), (0, 7, 1)),
            ]),
            (0, 0, 9),
        );

        let res = eval(&ast);
        assert_eq!(
            res,
            Err(Error(
                ErrorKind::SymbolMisplaced(Symbol::Multiply),
                (0, 7, 1)
            ))
        );
    }

    #[test]
    fn test_invalid_function() {
        // (1 + 2)
        let ast = Node(
            Expr::List(vec![
                Node(Expr::Number(1.0), (0, 1, 1)),
                Node(Expr::Symbol(Symbol::Plus), (0, 3, 1)),
                Node(Expr::Number(2.0), (0, 5, 1)),
            ]),
            (0, 0, 7),
        );

        let res = eval(&ast);
        assert_eq!(res, Err(Error(ErrorKind::InvalidFunction, (0, 1, 1))));
    }
}
