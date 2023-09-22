// faye, a pretty lil lisp
// Copyright (c) 2023 fawn
//
// SPDX-License-Identifier: Apache-2.0

use crate::{Location, Node, NodeKind, Symbol};

use super::{scope::Scope, Error, ErrorKind, Expr};

/// A context that stores global and local functions
#[derive(Clone)]
pub struct Context {
    pub(crate) globals: Scope,
    pub(crate) locals: Scope,
    start: Location,
    end: Location,
}

impl Context {
    /// Create a new context with the given start and end locations
    #[must_use]
    pub fn new(start: Location, end: Location) -> Self {
        Self {
            globals: Scope::new(),
            locals: Scope::default(),
            start,
            end,
        }
    }

    /// Get a function callback from local or global scope
    #[must_use]
    pub fn get(&self, sym: &Symbol) -> Option<&Expr> {
        self.locals.get(sym).or_else(|| self.globals.get(sym))
    }

    /// List all global functions
    #[must_use]
    pub fn list_globals(&self) -> Vec<Symbol> {
        self.globals.0.keys().cloned().collect()
    }

    /// Create a new evaluation error
    pub(crate) const fn error(&self, kind: ErrorKind) -> Error {
        Error::new(kind, self.start, self.end)
    }

    /// Evaluate an expression
    pub fn eval(&mut self, ast: &Node) -> Result<Expr, Error> {
        match ast {
            Node(NodeKind::Symbol(sym), start, end) => Ok(self
                .get(sym)
                .ok_or_else(|| Error::new(ErrorKind::UnknownSymbol(sym.clone()), *start, *end))?
                .clone()),
            Node(NodeKind::List(list), ..) => match list.split_first() {
                Some((fun, args)) => {
                    self.start = fun.1;
                    self.end = fun.2;
                    match self.eval(fun)? {
                        Expr::BuiltinFn(f) => f.eval(self, args),
                        Expr::UserFn(f) => f.eval(self, args),
                        Expr::Closure(f) => f.eval(self, args),
                        v => Err(self.error(ErrorKind::InvalidFunction(v))),
                    }
                }
                None => Ok(Expr::Nil),
            },
            n => Ok(Expr::from(n)),
        }
    }

    /// Evaluate an expression, temporarily replacing the current locals
    pub(crate) fn eval_scoped(&mut self, ast: &Node, locals: Scope) -> Result<Expr, Error> {
        let locals = std::mem::replace(&mut self.locals, locals);
        let res = self.eval(ast);
        self.locals = locals;
        res
    }

    /// Downcast an expression to a specific type
    pub(crate) fn downcast<'a, T>(&self, value: &'a Expr) -> Result<T, Error>
    where
        T: TryFrom<&'a Expr>,
    {
        value
            .try_into()
            .map_err(|_| self.error(ErrorKind::InvalidArgument(value.clone())))
    }

    /// Downcast a list of expressions to a specific type
    pub(crate) fn downcast_all<'a, T>(&self, args: &'a [Expr]) -> Result<Vec<T>, Error>
    where
        T: TryFrom<&'a Expr>,
    {
        args.iter().map(|v| self.downcast(v)).collect()
    }

    /// Evaluate a list of expressions
    pub(crate) fn eval_args(&mut self, args: &[Node]) -> Result<Vec<Expr>, Error> {
        args.iter().map(|n| self.eval(n)).collect()
    }

    /// Get a list of arguments with a specific length
    pub(crate) fn get_n<'a, const N: usize>(
        &self,
        args: &'a [Node],
    ) -> Result<&'a [Node; N], Error> {
        args.try_into().map_err(|_| {
            if args.len() < N {
                self.error(ErrorKind::MissingArguments)
            } else {
                self.error(ErrorKind::TooManyArguments)
            }
        })
    }

    /// Compare between list of arguments of the same type using a predicate
    pub(crate) fn compare<T>(
        &mut self,
        args: &[Node],
        op: impl Fn(&T, &T) -> bool,
    ) -> Result<Expr, Error>
    where
        T: for<'a> TryFrom<&'a Expr> + PartialEq,
    {
        let args = self
            .eval_args(args)
            .and_then(|v| self.downcast_all::<T>(&v))?;

        Ok(Expr::Bool(
            args.iter().zip(args.iter().skip(1)).all(|(a, b)| op(a, b)),
        ))
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new((0, 0), (0, 1))
    }
}
