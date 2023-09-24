// faye, a pretty lil lisp
// Copyright (c) 2023 fawn
//
// SPDX-License-Identifier: Apache-2.0

pub use builtin::BuiltinFn;
pub use closure::Closure;
pub use context::Context;
pub use error::{Error, ErrorKind};
pub use expr::Expr;
pub use scope::Scope;
pub use userfn::UserFn;

mod builtin;
mod closure;
mod context;
mod error;
mod expr;
mod scope;
mod userfn;

#[cfg(test)]
mod tests {
    use crate::prelude::{Node, NodeKind, Symbol};

    use super::*;

    #[test]
    fn add() {
        // (* 2 3)
        let ast = Node(
            NodeKind::List(vec![
                Node(NodeKind::Symbol(Symbol::from("*")), (0, 1), (0, 2)),
                Node(NodeKind::Number(2.), (0, 3), (0, 4)),
                Node(NodeKind::Number(3.), (0, 5), (0, 6)),
            ]),
            (0, 0),
            (0, 7),
        );
        let res = Context::default().eval(&ast);

        assert_eq!(res, Ok(Expr::Number(6.)));
    }

    #[test]
    fn error_invalid_function() {
        // (1 + 2)
        let ast = Node(
            NodeKind::List(vec![
                Node(NodeKind::Number(1.), (0, 1), (0, 2)),
                Node(NodeKind::Symbol(Symbol::from("+")), (0, 3), (0, 4)),
                Node(NodeKind::Number(2.), (0, 5), (0, 6)),
            ]),
            (0, 0),
            (0, 7),
        );

        let res = Context::default().eval(&ast);
        assert_eq!(
            res,
            Err(Error::new(
                ErrorKind::InvalidFunction(Expr::Number(1.)),
                (0, 1),
                (0, 2)
            ))
        );
    }

    #[test]
    fn error_add_string() {
        // (+ "hi" 5)
        let ast = Node(
            NodeKind::List(vec![
                Node(NodeKind::Symbol(Symbol::from("+")), (0, 1), (0, 2)),
                Node(NodeKind::String("hi".into()), (0, 3), (0, 7)),
                Node(NodeKind::Number(5.), (0, 8), (0, 9)),
            ]),
            (0, 0),
            (0, 10),
        );

        let res = Context::default().eval(&ast);
        assert_eq!(
            res,
            Err(Error::new(
                ErrorKind::InvalidArgument(Expr::String("hi".into())),
                (0, 1),
                (0, 2),
            ))
        );
    }
}
