// faye, a pretty lil lisp
// Copyright (c) 2023 fawn
//
// SPDX-License-Identifier: Apache-2.0

use clap::Parser;

use faye::eval::{Context, Expr};
use faye::lexer::Lexer;
use faye::repl;
use faye::{parser, Highlighter};

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

    /// Highlight matching brackets
    #[arg(short, long)]
    pub matching_brackets: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let match_brackets = args.matching_brackets;

    if let Some(path) = args.file {
        let file = path.trim_start_matches("./").trim_start_matches(".\\");
        eval(&std::fs::read_to_string(file)?, Some(file), match_brackets);
        return Ok(());
    }

    if let Some(code) = args.eval {
        eval(&code, None, match_brackets);
        return Ok(());
    }

    if let Some(code) = args.lex {
        let lex = Lexer::new(&code);
        for token in lex {
            println!("{:?}", token?);
        }

        return Ok(());
    }

    if let Some(code) = args.ast {
        let mut lex = Lexer::new(&code);
        let ast = parser::parse(&mut lex)?;
        println!("{ast:?}");

        return Ok(());
    }

    repl::start(match_brackets)
}

macro_rules! err {
    ($src:tt@$path:expr => $err:ident, $hl:ident) => {
        eprintln!(
            "\x1b[1;36m   --> \x1b[0m{}:{}:{}\n\x1b[1;36m    |\n{:^4}|\x1b[0m {}\n\x1b[1;36m    |\x1b[0m{}\x1b[1;31m{} {}",
            $path,
            $err.start.0 + 1,
            $err.start.1 + 1,
            $err.start.0 + 1,
            $hl.highlight($src.split('\n').nth($err.start.0).unwrap()),
            " ".repeat($err.start.1 + 1),
            "^".repeat($err.end.1 - $err.start.1),
            $err
        )
    };
}

fn eval(code: &str, path: Option<&str>, match_brackets: bool) {
    let mut ctx = Context::default();
    let hl = Highlighter::new(match_brackets);

    let mut lex = Lexer::new(code);
    let path = path.unwrap_or("main.fy");

    let ast = match parser::parse(&mut lex) {
        Ok(ast) => ast,
        Err(err) => return err!(code@path => err, hl),
    };

    ast.iter().for_each(|n| match ctx.eval(n) {
        Ok(Expr::Nil) => {}
        Ok(res) => println!("{res}"),
        Err(err) => err!(code@path => err, hl),
    });
}
