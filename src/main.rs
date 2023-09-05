// faye, a pretty lil lisp
// Copyright (c) 2023 fawn
//
// SPDX-License-Identifier: Apache-2.0

use clap::Parser;

mod eval;
mod lexer;
mod parser;
mod repl;

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
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if let Some(path) = args.file {
        let file = path.trim_start_matches("./").trim_start_matches(".\\");
        return eval(std::fs::read_to_string(file)?, Some(file));
    }

    match args.eval {
        Some(code) => eval(code, None),
        None => repl::start(),
    }
}

macro_rules! err {
    ($e:ident, $s:expr, $p:expr) => {
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

fn eval(code: String, path: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let mut lex = lexer::Lexer::new(&code);
    let path = path.unwrap_or("main.fy");

    let ast = match parser::parse(&mut lex) {
        Ok(ast) => ast,
        Err(err) => return Ok(err!(err, code, path)),
    };

    ast.iter().map(eval::eval).for_each(|res| match res {
        Ok(res) => println!("{res}"),
        Err(err) => err!(err, code, path),
    });

    Ok(())
}
