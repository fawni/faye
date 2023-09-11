// faye, a pretty lil lisp
// Copyright (c) 2023 fawn
//
// SPDX-License-Identifier: Apache-2.0

use std::{collections::HashMap, io::IsTerminal};

use crate::{
    lexer::{Location, Symbol},
    parser::{Node, NodeKind},
};

type BuiltinCallback = dyn Fn(&mut Context, &[Node]) -> Result<Expr, Error>;

#[derive(Clone)]
pub struct BuiltinFn {
    name: Symbol,
    callback: &'static BuiltinCallback,
}

impl BuiltinFn {
    fn new<S: Into<String>>(name: S, callback: &'static BuiltinCallback) -> Self {
        Self {
            name: Symbol::from(name),
            callback,
        }
    }

    #[inline]
    fn eval(&self, ctx: &mut Context, args: &[Node]) -> Result<Expr, Error> {
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

#[derive(Clone, Debug)]
pub struct UserFn {
    name: Symbol,
    params: Vec<Symbol>,
    body: Node,
}

impl UserFn {
    fn eval(&self, ctx: &mut Context, args: &[Node]) -> Result<Expr, Error> {
        let mut locals = Scope::default();
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

        ctx.locals = locals;
        ctx.eval(&self.body)
    }
}

impl PartialEq for UserFn {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[derive(Default, Clone)]
struct Scope(HashMap<Symbol, Expr>);

impl Scope {
    fn new() -> Self {
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
            let [expr] = ctx.get_n(args)?;
            Ok(quote(expr))
        });
        scope.register("fn", &|ctx, args| {
            let [name, params, body] = ctx.get_n(args)?;
            let name = ctx.downcast::<Symbol>(&quote(name))?;
            let params = ctx.downcast::<Vec<Symbol>>(&quote(params))?;
            let body = body.clone();

            ctx.globals
                .insert(name.clone(), Expr::UserFn(UserFn { name, params, body }));

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

    #[inline]
    fn get(&self, k: &Symbol) -> Option<&Expr> {
        self.0.get(k)
    }

    #[inline]
    fn insert(&mut self, k: Symbol, v: Expr) {
        self.0.insert(k, v);
    }
}

#[derive(Clone)]
pub struct Context {
    globals: Scope,
    locals: Scope,
    start: Location,
    end: Location,
}

impl Context {
    #[must_use]
    pub fn new(start: Location, end: Location) -> Self {
        Self {
            globals: Scope::new(),
            locals: Scope::default(),
            start,
            end,
        }
    }

    #[must_use]
    pub fn get(&self, sym: &Symbol) -> Option<&Expr> {
        self.locals.get(sym).or_else(|| self.globals.get(sym))
    }

    const fn error(&self, kind: ErrorKind) -> Error {
        Error::new(kind, self.start, self.end)
    }

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
                        v => Err(self.error(ErrorKind::InvalidFunction(v))),
                    }
                }
                None => Ok(Expr::Nil),
            },
            n => Ok(quote(n)),
        }
    }

    fn downcast<'a, T>(&self, value: &'a Expr) -> Result<T, Error>
    where
        T: TryFrom<&'a Expr>,
    {
        value
            .try_into()
            .map_err(|_| self.error(ErrorKind::InvalidArgument(value.clone())))
    }

    fn downcast_all<'a, T>(&self, args: &'a [Expr]) -> Result<Vec<T>, Error>
    where
        T: TryFrom<&'a Expr>,
    {
        args.iter().map(|v| self.downcast(v)).collect()
    }

    fn eval_args(&mut self, args: &[Node]) -> Result<Vec<Expr>, Error> {
        args.iter().map(|n| self.eval(n)).collect()
    }

    fn get_n<'a, const N: usize>(&self, args: &'a [Node]) -> Result<&'a [Node; N], Error> {
        args.try_into().map_err(|_| {
            if args.len() < N {
                self.error(ErrorKind::MissingArguments)
            } else {
                self.error(ErrorKind::TooManyArguments)
            }
        })
    }
}

pub fn quote(ast: &Node) -> Expr {
    match &ast.0 {
        NodeKind::Number(n) => Expr::Number(*n),
        NodeKind::Bool(b) => Expr::Bool(*b),
        NodeKind::String(s) => Expr::String(s.clone()),
        NodeKind::Symbol(s) => Expr::Symbol(s.clone()),
        NodeKind::Keyword(Symbol(s)) => Expr::Keyword(s.clone()),
        NodeKind::List(l) if l.is_empty() => Expr::Nil,
        NodeKind::List(l) => Expr::List(l.iter().map(quote).collect()),
        NodeKind::Nil => Expr::Nil,
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new((0, 0), (0, 1))
    }
}

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

#[derive(Debug, PartialEq, Clone)]
pub struct Error {
    pub kind: ErrorKind,
    pub start: Location,
    pub end: Location,
}

impl Error {
    #[must_use]
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
    UnknownSymbol(Symbol),
    MissingArguments,
    TooManyArguments,
    InvalidFunction(Expr),
    InvalidArgument(Expr),
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownSymbol(sym) => write!(f, "Could not resolve symbol '{sym}' in scope"),
            Self::MissingArguments => write!(f, "Function is missing arguments"),
            Self::TooManyArguments => write!(f, "Function has extra arguments"),
            Self::InvalidFunction(v) => write!(f, "`{v}` is not a function"),
            Self::InvalidArgument(v) => {
                write!(f, "`{v}` is not a valid argument for this function")
            }
        }
    }
}

#[cfg(test)]
mod tests {
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
