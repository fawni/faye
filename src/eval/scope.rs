// faye, a pretty lil lisp
// Copyright (c) 2023 fawn
//
// SPDX-License-Identifier: Apache-2.0

use std::{collections::HashMap, io::IsTerminal};

use super::{
    builtin::{BuiltinFn, Callback},
    closure::Closure,
    userfn::UserFn,
    Context, Error, ErrorKind, Expr,
};
use crate::prelude::{Node, NodeKind, Symbol};

/// A scope that stores functions
#[derive(Debug, Default, Clone)]
pub struct Scope(pub(crate) HashMap<Symbol, Expr>);

impl Scope {
    /// Create a new scope with builtin functions
    pub(crate) fn builtins() -> Self {
        let mut scope = Self::default();

        scope.insert(
            Symbol::from("@cmd-args"),
            Expr::Vector(std::env::args().map(Expr::String).collect::<Vec<_>>()),
        );

        scope.register("+", |ctx, args| {
            Ok(Expr::Number(
                ctx.eval_args(args)
                    .and_then(|v| ctx.downcast_all::<f64>(&v))?
                    .into_iter()
                    .sum(),
            ))
        });
        scope.register("*", |ctx, args| {
            Ok(Expr::Number(
                ctx.eval_args(args)
                    .and_then(|v| ctx.downcast_all::<f64>(&v))?
                    .into_iter()
                    .product(),
            ))
        });
        scope.register("-", |ctx, args| {
            Ok(Expr::Number(
                ctx.eval_args(args)
                    .and_then(|v| ctx.downcast_all::<f64>(&v))?
                    .into_iter()
                    .reduce(|acc, x| acc - x)
                    .ok_or_else(|| ctx.error(ErrorKind::MissingArguments))?,
            ))
        });
        scope.register("/", |ctx, args| {
            Ok(Expr::Number(
                ctx.eval_args(args)
                    .and_then(|v| ctx.downcast_all::<f64>(&v))?
                    .into_iter()
                    .reduce(|acc, x| acc / x)
                    .ok_or_else(|| ctx.error(ErrorKind::MissingArguments))?,
            ))
        });
        scope.register("=", |ctx, args| {
            let args = ctx.eval_args(args)?;
            Ok(Expr::Bool(args.iter().all(|n| n.eq(&args[0]))))
        });
        scope.register("<", |ctx, args| ctx.compare::<f64>(args, |a, b| a < b));
        scope.register(">", |ctx, args| ctx.compare::<f64>(args, |a, b| a > b));
        scope.register("<=", |ctx, args| ctx.compare::<f64>(args, |a, b| a <= b));
        scope.register(">=", |ctx, args| ctx.compare::<f64>(args, |a, b| a >= b));
        scope.register("str", |ctx, args| {
            Ok(Expr::String(
                ctx.eval_args(args)
                    .and_then(|v| ctx.downcast_all::<String>(&v))?
                    .join(""),
            ))
        });
        scope.register("chars", |ctx, args| {
            let [node] = ctx.get_n(args)?;
            let string = ctx.downcast::<String>(&Expr::from(node))?;

            Ok(Expr::Vector(string.chars().map(Expr::Char).collect()))
        });
        scope.register("join", |ctx, args| {
            let [sep, coll] = ctx.get_n(args)?;
            let sep = ctx.eval(sep).and_then(|expr| match expr {
                Expr::Char(c) => Ok(c.to_string()),
                Expr::String(s) => Ok(s),
                _ => Err(ctx.error(ErrorKind::InvalidArgument(expr))),
            })?;
            let vec = ctx.eval(coll).and_then(|expr| match expr {
                Expr::List(v) | Expr::Vector(v) => ctx.downcast_all::<String>(&v),
                Expr::Nil => Ok(vec![]),
                _ => Err(ctx.error(ErrorKind::InvalidArgument(expr))),
            })?;

            Ok(Expr::String(vec.join(&sep)))
        });
        scope.register("println", |ctx, args| {
            let string = ctx
                .eval_args(args)
                .and_then(|v| ctx.downcast_all::<String>(&v))?
                .join(" ");
            if std::io::stdout().is_terminal() {
                println!("{string}\x1b[m");
                Ok(Expr::Nil)
            } else {
                Ok(Expr::Display(string))
            }
        });
        scope.register("quote", |ctx, args| {
            let [node] = ctx.get_n(args)?;
            Ok(Expr::from(node))
        });
        scope.register("list", |ctx, args| Ok(Expr::List(ctx.eval_args(args)?)));
        scope.register("vector", |ctx, args| Ok(Expr::Vector(ctx.eval_args(args)?)));
        scope.register("vec", |ctx, args| {
            let [node] = ctx.get_n(args)?;
            let vec = match ctx.eval(node)? {
                Expr::List(v) | Expr::Vector(v) => v,
                Expr::Nil => Vec::new(),
                Expr::String(s) => s.chars().map(Expr::Char).collect(),
                e => return Err(ctx.error(ErrorKind::InvalidArgument(e))),
            };

            Ok(Expr::Vector(vec))
        });
        scope.register("nth", |ctx, args| {
            let (coll, nth, default) = match ctx.get_n(args) {
                Ok([coll, nth, default]) => (ctx.eval(coll)?, ctx.eval(nth)?, ctx.eval(default)?),
                Err(_) => {
                    let [coll, nth] = ctx.get_n(args)?;
                    (ctx.eval(coll)?, ctx.eval(nth)?, Expr::Nil)
                }
            };

            let coll = match coll {
                Expr::List(v) | Expr::Vector(v) => v,
                Expr::Nil => Vec::new(),
                Expr::String(s) => s.chars().map(Expr::Char).collect(),
                _ => return Err(ctx.error(ErrorKind::InvalidArgument(coll))),
            };
            let nth = match ctx.downcast::<f64>(&nth)? {
                n if n < 0. => coll.len() - n as usize,
                n => n as usize,
            };

            Ok(coll.get(nth).unwrap_or(&default).clone())
        });
        scope.register("lambda", lambda);
        scope.register("λ", lambda);
        scope.register("fn", |ctx, args| {
            let [name, params, body] = ctx.get_n(args)?;
            let name = ctx.downcast::<Symbol>(&Expr::from(name))?;
            let params = match Expr::from(params) {
                p @ Expr::Vector(_) => ctx.downcast::<Vec<Symbol>>(&p)?,
                p => return Err(ctx.error(ErrorKind::InvalidArgument(p))),
            };
            let body = body.clone();

            ctx.globals
                .insert(name.clone(), Expr::UserFn(UserFn::new(name, params, body)));

            Ok(Expr::Nil)
        });
        scope.register("let", |ctx, args| {
            let (body, bindings) = args
                .split_last()
                .ok_or_else(|| ctx.error(ErrorKind::MissingArguments))?;

            let mut locals = ctx.locals.clone();

            for bind in bindings {
                match &bind.kind {
                    NodeKind::List(b) => {
                        let [var, value] = ctx.get_n(b)?;
                        let var = ctx.downcast::<Symbol>(&Expr::from(var))?;
                        let value = ctx.eval(value)?;
                        locals.insert(var, value);
                    }
                    _ => return Err(ctx.error(ErrorKind::InvalidArgument(Expr::from(bind)))),
                }
            }

            ctx.eval_scoped(body, locals)
        });
        scope.register("const", |ctx, args| {
            let [name, value] = ctx.get_n(args)?;
            let name = ctx.downcast::<Symbol>(&Expr::from(name))?;
            let value = ctx.eval(value)?;

            ctx.globals.insert(name, value);

            Ok(Expr::Nil)
        });
        scope.register("if", |ctx, args| match ctx.get_n(args) {
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
        scope.register("and", |ctx, args| {
            for n in args {
                if !ctx.eval(n).and_then(|v| ctx.downcast(&v))? {
                    return Ok(Expr::Bool(false));
                }
            }
            Ok(Expr::Bool(true))
        });
        scope.register("or", |ctx, args| {
            for n in args {
                if ctx.eval(n).and_then(|v| ctx.downcast(&v))? {
                    return Ok(Expr::Bool(true));
                }
            }
            Ok(Expr::Bool(false))
        });
        scope.register("not", |ctx, args| {
            let [node] = ctx.get_n(args)?;
            Ok(Expr::Bool(
                !ctx.eval(node)
                    .and_then(|v| ctx.downcast(&v))
                    .unwrap_or(false),
            ))
        });
        scope.register("parse-num", |ctx, args| {
            let [v] = ctx.get_n(args)?;
            let expr = ctx.eval(v)?;
            let num = match &expr {
                Expr::String(s) => s.parse::<f64>().ok(),
                Expr::Char(c) => c.to_digit(10).map(f64::from),
                _ => None,
            };

            Ok(Expr::Number(num.ok_or_else(|| {
                ctx.error(ErrorKind::InvalidArgument(expr))
            })?))
        });

        scope
    }

    /// Register a builtin function
    fn register<S: Into<String> + Clone>(&mut self, name: S, callback: Callback) {
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

fn lambda(ctx: &mut Context, args: &[Node]) -> Result<Expr, Error> {
    let [params, body] = ctx.get_n(args)?;
    let params = ctx.downcast::<Vec<Symbol>>(&Expr::from(params))?;
    let body = body.clone();

    Ok(Expr::Closure(Closure::new(
        ctx.locals.clone(),
        params,
        body,
    )))
}
