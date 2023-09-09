// faye, a pretty lil lisp
// Copyright (c) 2023 fawn
//
// SPDX-License-Identifier: Apache-2.0

use clap::Parser;

mod eval;
mod lexer;
mod parser;
mod repl;

use eval::{Context, Expr};
use lexer::Lexer;

/// faye is a pretty lil lisp
#[derive(Parser)]
#[clap(version, author)]
pub struct Args {
    /// Evaluate a file
    #[arg()]
    pub file: Option<String>,

    /// Evaluate a string
    #[arg(value_name = "EXPRESSION", short, long)]
    pub eval: Option<String>,

    /// Print the lexer output
    #[arg(value_name = "EXPRESSION", short, long)]
    pub lex: Option<String>,

    /// Print the parser output
    #[arg(value_name = "EXPRESSION", short, long)]
    pub ast: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if let Some(path) = args.file {
        let file = path.trim_start_matches("./").trim_start_matches(".\\");
        eval(&std::fs::read_to_string(file)?, Some(file));
        return Ok(());
    }

    if let Some(code) = args.eval {
        eval(&code, None);
        return Ok(());
    }

    if let Some(code) = args.lex {
        let lex = lexer::Lexer::new(&code);
        for token in lex {
            println!("{:?}", token?);
        }

        return Ok(());
    }

    if let Some(code) = args.ast {
        let mut lex = lexer::Lexer::new(&code);
        let ast = parser::parse(&mut lex)?;
        println!("{ast:?}");

        return Ok(());
    }

    repl::start()
}

macro_rules! err {
    ($s:tt@$p:expr => $e:ident) => {
        eprintln!(
            "\x1b[1;36m   --> \x1b[0m{}:{}:{}\n\x1b[1;36m    |\n{:^4}|\x1b[0m {}\n\x1b[1;36m    |\x1b[0m{}\x1b[1;31m{} {}",
            $p,
            $e.start.0 + 1,
            $e.start.1 + 1,
            $e.start.0 + 1,
            $s.split('\n').nth($e.start.0).unwrap(),
            " ".repeat($e.start.1 + 1),
            "^".repeat($e.end.1 - $e.start.1),
            $e
        )
    };
}

fn eval(code: &str, path: Option<&str>) {
    let mut ctx = Context::default();

    let mut lex = Lexer::new(code);
    let path = path.unwrap_or("main.fy");

    let ast = match parser::parse(&mut lex) {
        Ok(ast) => ast,
        Err(err) => return err!(code@path => err),
    };

    ast.iter().for_each(|n| match ctx.eval(n) {
        Ok(Expr::Nil) => {}
        Ok(res) => println!("{res}"),
        Err(err) => err!(code@path => err),
    });
}
