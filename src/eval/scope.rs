// faye, a pretty lil lisp
// Copyright (c) 2023 fawn
//
// SPDX-License-Identifier: Apache-2.0

use std::{collections::HashMap, io::IsTerminal};

use crate::{Node, Symbol};

use super::{builtin::BuiltinFn, userfn::UserFn, Context, Error, ErrorKind, Expr};

/// A scope that stores functions
#[derive(Default, Clone)]
pub struct Scope(pub(crate) HashMap<Symbol, Expr>);

impl Scope {
    /// Create a new scope with builtin functions
    pub(crate) fn new() -> Self {
        let mut scope = Self::default();

        scope.register("+", &|ctx, args| {
            Ok(Expr::Number(
                ctx.eval_args(args)
                    .and_then(|v| ctx.downcast_all::<f64>(&v))?
                    .into_iter()
                    .sum(),
            ))
        });
        scope.register("*", &|ctx, args| {
            Ok(Expr::Number(
                ctx.eval_args(args)
                    .and_then(|v| ctx.downcast_all::<f64>(&v))?
                    .into_iter()
                    .product(),
            ))
        });
        scope.register("-", &|ctx, args| {
            Ok(Expr::Number(
                ctx.eval_args(args)
                    .and_then(|v| ctx.downcast_all::<f64>(&v))?
                    .into_iter()
                    .reduce(|acc, x| acc - x)
                    .ok_or_else(|| ctx.error(ErrorKind::MissingArguments))?,
            ))
        });
        scope.register("/", &|ctx, args| {
            Ok(Expr::Number(
                ctx.eval_args(args)
                    .and_then(|v| ctx.downcast_all::<f64>(&v))?
                    .into_iter()
                    .reduce(|acc, x| acc / x)
                    .ok_or_else(|| ctx.error(ErrorKind::MissingArguments))?,
            ))
        });
        scope.register("=", &|ctx, args| {
            let args = ctx.eval_args(args)?;
            Ok(Expr::Bool(args.iter().all(|n| n.eq(&args[0]))))
        });
        scope.register("<", &|ctx, args| ctx.compare::<f64>(args, |a, b| a < b));
        scope.register(">", &|ctx, args| ctx.compare::<f64>(args, |a, b| a > b));
        scope.register("<=", &|ctx, args| ctx.compare::<f64>(args, |a, b| a <= b));
        scope.register(">=", &|ctx, args| ctx.compare::<f64>(args, |a, b| a >= b));
        scope.register("str", &|ctx, args| {
            Ok(Expr::String(
                ctx.eval_args(args)
                    .and_then(|v| ctx.downcast_all::<String>(&v))?
                    .join(""),
            ))
        });
        scope.register("println", &|ctx, args| {
            let string = ctx
                .eval_args(args)
                .and_then(|v| ctx.downcast_all::<String>(&v))?
                .join(" ");
            if std::io::stdout().is_terminal() {
                println!("{string}\x1b[m");
            } else {
                println!("{string}");
            }
            Ok(Expr::Nil)
        });
        scope.register("quote", &|ctx, args| {
            let [node] = ctx.get_n(args)?;
            Ok(Expr::from(node))
        });
        scope.register("fn", &|ctx, args| {
            let [name, params, body] = ctx.get_n(args)?;
            let name = ctx.downcast::<Symbol>(&Expr::from(name))?;
            let params = ctx.downcast::<Vec<Symbol>>(&Expr::from(params))?;
            let body = body.clone();

            ctx.globals
                .insert(name.clone(), Expr::UserFn(UserFn::new(name, params, body)));

            Ok(Expr::Nil)
        });
        scope.register("if", &|ctx, args| match ctx.get_n(args) {
            Ok([cond, then, or_else]) => {
                if ctx.eval(cond).and_then(|v| ctx.downcast(&v))? {
                    ctx.eval(then)
                } else {
                    ctx.eval(or_else)
                }
            }
            Err(_) => {
                let [cond, then] = ctx.get_n(args)?;
                if ctx.eval(cond).and_then(|v| ctx.downcast(&v))? {
                    ctx.eval(then)
                } else {
                    Ok(Expr::Nil)
                }
            }
        });
        scope.register("and", &|ctx, args| {
            for n in args {
                if !ctx.eval(n).and_then(|v| ctx.downcast(&v))? {
                    return Ok(Expr::Bool(false));
                }
            }
            Ok(Expr::Bool(true))
        });
        scope.register("or", &|ctx, args| {
            for n in args {
                if ctx.eval(n).and_then(|v| ctx.downcast(&v))? {
                    return Ok(Expr::Bool(true));
                }
            }
            Ok(Expr::Bool(false))
        });

        scope
    }

    /// Register a builtin function
    fn register<S: Into<String> + Clone>(
        &mut self,
        name: S,
        callback: &'static impl Fn(&mut Context, &[Node]) -> Result<Expr, Error>,
    ) {
        self.insert(
            Symbol::from(name.clone()),
            Expr::BuiltinFn(BuiltinFn::new(name, callback)),
        );
    }

    /// Get a function callback from the scope by name
    #[inline]
    pub(crate) fn get(&self, k: &Symbol) -> Option<&Expr> {
        self.0.get(k)
    }

    /// Insert a function into the scope
    #[inline]
    pub(crate) fn insert(&mut self, k: Symbol, v: Expr) {
        self.0.insert(k, v);
    }
}
