// faye, a pretty lil lisp
// Copyright (c) 2023 fawn
//
// SPDX-License-Identifier: Apache-2.0

use crate::{
    lexer::{Location, Symbol},
    parser::{Expr, Node},
};

pub fn eval(ast: &Node) -> Result<Value, Error> {
    match ast {
        Node(Expr::Number(n), ..) => Ok(Value::Number(*n)),
        Node(Expr::Bool(b), ..) => Ok(Value::Bool(*b)),
        Node(Expr::String(s), ..) => Ok(Value::String(s.to_owned())),
        Node(Expr::List(list), start, end) => {
            if list.len() < 2 {
                return Err(Error::new(ErrorKind::MissingArguments, *start, *end));
            }

            let (func, sym_start, sym_end) = match list.get(0) {
                Some(Node(Expr::Function(sym), start, end)) => (sym, start, end),
                Some(Node(_, start, end)) => {
                    return Err(Error::new(ErrorKind::InvalidFunction, *start, *end))
                }
                None => return Err(Error::new(ErrorKind::MissingArguments, *start, *end)),
            };

            let args = list[1..].iter().map(eval).collect::<Result<Vec<_>, _>>()?;

            match func {
                Symbol::Plus => {
                    let args = collect_numbers(&args, *sym_start, *sym_end)?;
                    Ok(Value::Number(args.into_iter().sum()))
                }
                Symbol::Minus => {
                    let args = collect_numbers(&args, *sym_start, *sym_end)?;
                    Ok(Value::Number(
                        args.into_iter().reduce(|acc, x| acc - x).unwrap(),
                    ))
                }
                Symbol::Multiply => {
                    let args = collect_numbers(&args, *sym_start, *sym_end)?;
                    Ok(Value::Number(
                        args.into_iter().reduce(|acc, x| acc * x).unwrap(),
                    ))
                }
                Symbol::Divide => {
                    let args = collect_numbers(&args, *sym_start, *sym_end)?;
                    Ok(Value::Number(
                        args.into_iter().reduce(|acc, x| acc / x).unwrap(),
                    ))
                }
                Symbol::Equal => Ok(Value::Bool(args.iter().all(|n| n.eq(&args[0])))),
            }
        }
        Node(Expr::Function(sym), start, end) => {
            Err(Error::new(ErrorKind::SymbolMisplaced(*sym), *start, *end))
        }
    }
}

fn collect_numbers(args: &[Value], start: Location, end: Location) -> Result<Vec<f64>, Error> {
    args.iter()
        .map(|v| match v {
            Value::Number(n) => Ok(*n),
            _ => Err(Error::new(
                ErrorKind::InvalidArgument(v.clone()),
                start,
                end,
            )),
        })
        .collect()
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Number(f64),
    Bool(bool),
    String(String),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{n}"),
            Self::Bool(b) => write!(f, "{b}"),
            Self::String(s) => write!(f, "\"{s}\""),
        }
    }
}

impl TryFrom<Value> for f64 {
    type Error = ErrorKind;

    fn try_from(v: Value) -> Result<Self, ErrorKind> {
        match v {
            Value::Number(n) => Ok(n),
            _ => Err(ErrorKind::InvalidArgument(v)),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Error {
    pub kind: ErrorKind,
    pub start: Location,
    pub end: Location,
}

impl Error {
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

#[derive(Debug, PartialEq, Clone)]
pub enum ErrorKind {
    SymbolMisplaced(Symbol),
    MissingArguments,
    InvalidFunction,
    InvalidArgument(Value),
    CalculationError,
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SymbolMisplaced(sym) => write!(f, "Symbol '{sym}' should not be here"),
            Self::MissingArguments => write!(f, "Function is missing arguments"),
            Self::InvalidFunction => write!(f, "Function undefined"),
            Self::InvalidArgument(n) => {
                write!(f, "`{n}` is not a valid argument for this function")
            }
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
                Node(Expr::Function(Symbol::Plus), (0, 1), (0, 2)),
                Node(Expr::Number(1.0), (0, 3), (0, 4)),
                Node(Expr::Number(2.0), (0, 5), (0, 6)),
                Node(Expr::Function(Symbol::Multiply), (0, 7), (0, 8)),
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
                Node(Expr::Function(Symbol::Plus), (0, 3), (0, 4)),
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

    #[test]
    fn error_add_string() {
        // (+ "hello" 5)
        let ast = Node(
            Expr::List(vec![
                Node(Expr::Function(Symbol::Plus), (0, 1), (0, 2)),
                Node(Expr::String("hello".to_owned()), (0, 3), (0, 10)),
                Node(Expr::Number(5.0), (0, 11), (0, 12)),
            ]),
            (0, 0),
            (0, 13),
        );

        let res = eval(&ast);
        assert_eq!(
            res,
            Err(Error::new(
                ErrorKind::InvalidArgument(Value::String("hello".to_owned())),
                (0, 1),
                (0, 2),
            ))
        );
    }
}
