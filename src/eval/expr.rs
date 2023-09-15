// faye, a pretty lil lisp
// Copyright (c) 2023 fawn
//
// SPDX-License-Identifier: Apache-2.0

use crate::{Node, NodeKind, Symbol};

use super::{builtin::BuiltinFn, userfn::UserFn};

/// The result of an evaluated expression
#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Number(f64),
    Bool(bool),
    String(String),
    Symbol(Symbol),
    Keyword(String),
    List(Vec<Expr>),
    BuiltinFn(BuiltinFn),
    UserFn(UserFn),
    Nil,
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{n}"),
            Self::Bool(b) => write!(f, "{b}"),
            Self::String(s) => write!(f, "\"{}\"", s.replace('"', "\\\"")),
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
            Self::Nil => write!(f, "nil"),
            Self::BuiltinFn(v) => write!(f, "{}", v.name),
            Self::UserFn(v) => write!(f, "{}", v.name),
        }
    }
}

impl From<&Node> for Expr {
    fn from(node: &Node) -> Self {
        match &node.0 {
            NodeKind::Number(n) => Expr::Number(*n),
            NodeKind::Bool(b) => Expr::Bool(*b),
            NodeKind::String(s) => Expr::String(s.clone()),
            NodeKind::Symbol(s) => Expr::Symbol(s.clone()),
            NodeKind::Keyword(Symbol(s)) => Expr::Keyword(s.clone()),
            NodeKind::List(l) if l.is_empty() => Expr::Nil,
            NodeKind::List(l) => Expr::List(l.iter().map(From::from).collect()),
            NodeKind::Nil => Expr::Nil,
        }
    }
}

impl From<&Expr> for String {
    fn from(v: &Expr) -> Self {
        match v {
            Expr::String(s) => s.clone(),
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
