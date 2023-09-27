// faye, a pretty lil lisp
// Copyright (c) 2023 fawn
//
// SPDX-License-Identifier: Apache-2.0

use super::{BuiltinFn, Closure, UserFn};
use crate::prelude::{Node, NodeKind, Symbol};

/// The result of an evaluated expression
#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Number(f64),
    Bool(bool),
    String(String),
    Display(String),
    Char(char),
    Symbol(Symbol),
    Keyword(String),
    List(Vec<Expr>),
    Vector(Vec<Expr>),
    BuiltinFn(BuiltinFn),
    UserFn(UserFn),
    Closure(Closure),
    Nil,
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{n}"),
            Self::Bool(b) => write!(f, "{b}"),
            Self::String(s) => write!(f, "\"{}\"", s.replace('"', "\\\"")),
            Self::Display(s) => write!(f, "{s}"),
            Self::Char(c) => write!(f, "'{c}'"),
            Self::Symbol(s) => write!(f, "{s}"),
            Self::Keyword(s) => write!(f, ":{s}"),
            Self::List(v) => write!(
                f,
                "({})",
                v.iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(" ")
            ),
            Self::Vector(v) => write!(
                f,
                "[{}]",
                v.iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(" ")
            ),
            Self::Nil => write!(f, "nil"),
            Self::BuiltinFn(v) => write!(f, "{}", v.name),
            Self::UserFn(v) => write!(f, "{}", v.name),
            Self::Closure(_) => write!(f, "#<lambda>"),
        }
    }
}

impl From<&Node> for Expr {
    fn from(node: &Node) -> Self {
        match &node.0 {
            NodeKind::Number(n) => Self::Number(*n),
            NodeKind::Bool(b) => Self::Bool(*b),
            NodeKind::String(s) => Self::String(s.clone()),
            NodeKind::Char(c) => Self::Char(*c),
            NodeKind::Symbol(s) => Self::Symbol(s.clone()),
            NodeKind::Keyword(Symbol(s)) => Self::Keyword(s.clone()),
            NodeKind::List(l) if l.is_empty() => Self::Nil,
            NodeKind::List(l) => Self::List(l.iter().map(From::from).collect()),
            NodeKind::Vector(v) => Self::Vector(v.iter().map(From::from).collect()),
            NodeKind::Nil => Self::Nil,
        }
    }
}

impl From<&Expr> for String {
    fn from(v: &Expr) -> Self {
        match v {
            Expr::String(s) => s.clone(),
            Expr::Char(c) => c.to_string(),
            _ => v.to_string(),
        }
    }
}

macro_rules! impl_try_from {
    (@$t:ty => $e:tt) => {
        impl TryFrom<&Expr> for $t {
            type Error = ();

            fn try_from(v: &Expr) -> Result<Self, Self::Error> {
                match v {
                    Expr::$e(x) => Ok(x.clone()),
                    _ => Err(()),
                }
            }
        }
    };
}

impl_try_from!(@f64 => Number);
impl_try_from!(@bool => Bool);
impl_try_from!(@Symbol => Symbol);

impl<'a, T: TryFrom<&'a Expr>> TryFrom<&'a Expr> for Vec<T> {
    type Error = ();

    fn try_from(value: &'a Expr) -> Result<Self, Self::Error> {
        match value {
            Expr::List(n) => n.iter().map(|n| n.try_into().map_err(|_| ())).collect(),
            Expr::Nil => Ok(Self::new()),
            _ => Err(()),
        }
    }
}
