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
use lexer::{Lexer, Token, TokenKind};

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
    ($src:tt@$path:expr => $err:ident) => {
        eprintln!(
            "\x1b[1;36m   --> \x1b[0m{}:{}:{}\n\x1b[1;36m    |\n{:^4}|\x1b[0m {}\n\x1b[1;36m    |\x1b[0m{}\x1b[1;31m{} {}",
            $path,
            $err.start.0 + 1,
            $err.start.1 + 1,
            $err.start.0 + 1,
            highlight($src.split('\n').nth($err.start.0).unwrap()),
            " ".repeat($err.start.1 + 1),
            "^".repeat($err.end.1 - $err.start.1),
            $err
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

#[must_use]
pub fn highlight(line: &str) -> String {
    let mut lex = Lexer::new(line);
    let mut colors = Vec::new();

    let mb = Args::parse().matching_brackets;
    let mut paren_idx = 0;
    let paren_colors = [
        "\x1b[91m", "\x1b[93m", "\x1b[92m", "\x1b[94m", "\x1b[96m", "\x1b[95m", "\x1b[90m",
    ];

    let mut loc = (0, 0);
    let mut is_fn = false;
    while let Some(res) = lex.next() {
        let color = match &res {
            Ok(Token(kind, ..)) => match kind {
                TokenKind::Comment(_) => "\x1b[3;90m",
                TokenKind::OpenParen | TokenKind::CloseParen if !mb => "\x1b[90m",
                TokenKind::OpenParen => {
                    if paren_idx > paren_colors.len() - 1 {
                        paren_idx = 0;
                    }
                    let c = paren_colors[paren_idx];
                    paren_idx += 1;
                    c
                }
                TokenKind::CloseParen => {
                    if paren_idx < 1 {
                        paren_idx = 7;
                    }
                    paren_idx -= 1;
                    paren_colors[paren_idx]
                }
                TokenKind::Number(_) | TokenKind::Bool(_) => "\x1b[36m",
                TokenKind::String(_) => "\x1b[33m",
                TokenKind::Symbol(_) if is_fn => "\x1b[35m",
                TokenKind::Symbol(_) => "\x1b[32m",
                TokenKind::Keyword(_) => "\x1b[34m",
                TokenKind::Nil => "\x1b[37m",
            },
            Err(_) => "\x1b[31m",
        };
        colors.push((loc, color));
        loc = lex.location();
        is_fn = matches!(res, Ok(Token(TokenKind::OpenParen, ..)));
    }

    let mut lines = line.split('\n').map(String::from).collect::<Vec<_>>();
    for (loc, c) in colors.iter().rev() {
        lines[loc.0].insert_str(loc.1, c);
    }

    lines.join("\n") + "\x1b[0m"
}
