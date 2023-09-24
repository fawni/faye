// faye, a pretty lil lisp
// Copyright (c) 2023 fawn
//
// SPDX-License-Identifier: Apache-2.0

use super::{Context, Error, ErrorKind, Expr, Scope};
use crate::prelude::{Node, Symbol};

/// A user-defined function anonymous function
#[derive(Clone, Debug)]
pub struct Closure {
    scope: Box<Scope>,
    params: Vec<Symbol>,
    body: Node,
}

impl Closure {
    /// Create a new user-defined function
    pub fn new(scope: Scope, params: Vec<Symbol>, body: Node) -> Self {
        Self {
            scope: Box::new(scope),
            params,
            body,
        }
    }

    /// Evaluate the function with the given arguments
    pub(crate) fn eval(&self, ctx: &mut Context, args: &[Node]) -> Result<Expr, Error> {
        let mut locals = self.scope.clone();
        let mut args = args.iter();

        for param in &self.params {
            let value = match args.next() {
                Some(v) => ctx.eval(v)?,
                None => return Err(ctx.error(ErrorKind::MissingArguments)),
            };

            locals.insert(param.clone(), value);
        }

        if args.next().is_some() {
            return Err(ctx.error(ErrorKind::TooManyArguments));
        }

        ctx.eval_scoped(&self.body, *locals)
    }
}

impl PartialEq for Closure {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}
