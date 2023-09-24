// faye, a pretty lil lisp
// Copyright (c) 2023 fawn
//
// SPDX-License-Identifier: Apache-2.0

use super::{Context, Error, Expr};
use crate::prelude::{Node, Symbol};

/// type alias for builtin function callbacks
type BuiltinCallback = dyn Fn(&mut Context, &[Node]) -> Result<Expr, Error>;

/// A builtin function
#[derive(Clone)]
pub struct BuiltinFn {
    pub(crate) name: Symbol,
    callback: &'static BuiltinCallback,
}

impl BuiltinFn {
    /// Create a new builtin function
    pub fn new<S: Into<String>>(name: S, callback: &'static BuiltinCallback) -> Self {
        Self {
            name: Symbol::from(name),
            callback,
        }
    }

    /// Evaluate a builtin function with the given arguments
    #[inline]
    pub(crate) fn eval(&self, ctx: &mut Context, args: &[Node]) -> Result<Expr, Error> {
        (self.callback)(ctx, args)
    }
}

impl std::fmt::Debug for BuiltinFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BuiltinFn")
            .field("name", &self.name)
            .field("callback", &(self.callback as *const _))
            .finish()
    }
}

impl PartialEq for BuiltinFn {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}
