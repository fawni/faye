// faye, a pretty lil lisp
// Copyright (c) 2023 fawn
//
// SPDX-License-Identifier: Apache-2.0

use super::{Context, Error, Expr};
use crate::prelude::{Node, Symbol};

/// type alias for builtin function callbacks
pub type Callback = fn(&mut Context, &[Node]) -> Result<Expr, Error>;

/// A builtin function
#[allow(unpredictable_function_pointer_comparisons)]
#[derive(Debug, Clone, PartialEq)]
pub struct BuiltinFn {
    pub(crate) name: Symbol,
    callback: Callback,
}

impl BuiltinFn {
    /// Create a new builtin function
    pub fn new<S: Into<String>>(name: S, callback: Callback) -> Self {
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
