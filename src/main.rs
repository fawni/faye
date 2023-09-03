// faye, a pretty lil lisp
// Copyright (c) 2023 fawn
//
// SPDX-License-Identifier: Apache-2.0

use clap::Parser;

mod eval;
mod lexer;
mod parser;
mod repl;

#[derive(Parser)]
#[clap(
    version = "0.1.1",
    author = "fawn <fawn@envs.net>",
    about = "faye is a pretty lil lisp! run faye with no arguments to start the repl."
)]
pub struct Args {
    #[arg(value_name = "Expression", short, long, help = "Evaluate a string")]
    pub eval: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match Args::parse().eval {
        Some(s) => eval(s),
        None => repl::start(),
    }
}

macro_rules! err {
    ($e:ident, $s: expr) => {
        eprintln!(
            "\x1b[1;36m    |\n{:^4}|\x1b[0m {}\n\x1b[1;36m    |\x1b[0m{}\x1b[1;31m{} error: {}",
            $e.start.0 + 1,
            $s.split('\n').nth($e.start.0).unwrap(),
            " ".repeat($e.start.1 + 1),
            "^".repeat($e.end.1 - $e.start.1),
            $e
        )
    };
}

fn eval(s: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut lex = lexer::Lexer::new(&s);

    let ast = match parser::parse(&mut lex) {
        Ok(ast) => ast,
        Err(err) => return Ok(err!(err, s)),
    };

    ast.iter().map(eval::eval).for_each(|res| match res {
        Ok(res) => println!("{res}"),
        Err(err) => err!(err, s),
    });

    Ok(())
}
