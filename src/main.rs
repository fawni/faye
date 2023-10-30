// faye, a pretty lil lisp
// Copyright (c) 2023 fawn
//
// SPDX-License-Identifier: Apache-2.0

#[cfg(feature = "lsp")]
use clap::Subcommand;
use clap::{
    builder::{styling::AnsiColor, Styles},
    Parser,
};
use faye::prelude::{Context, Expr, Highlighter, Lexer, Parser as FayeParser};

use repl::Repl;

mod repl;

// legacy yellow and green clap style
fn clap_style() -> Styles {
    Styles::styled()
        .header(AnsiColor::Yellow.on_default())
        .usage(AnsiColor::Yellow.on_default())
        .literal(AnsiColor::Green.on_default())
        .placeholder(AnsiColor::Green.on_default())
}

/// faye is a pretty lil lisp
#[derive(Parser)]
#[clap(version, author, styles = clap_style())]
struct FayeArgs {
    #[cfg(feature = "lsp")]
    #[command(subcommand)]
    command: Option<Command>,

    /// Evaluate an expression from a file
    #[arg()]
    file: Option<String>,

    /// Evaluate an expression from input
    #[arg(value_name = "EXPRESSION", short, long)]
    eval: Option<String>,

    /// Lex an expression into tokens
    #[arg(value_name = "EXPRESSION", short, long)]
    lex: Option<String>,

    /// Parse an expression into an AST
    #[arg(value_name = "EXPRESSION", short, long)]
    ast: Option<String>,

    /// Highlight matching brackets in REPL
    #[arg(short, long)]
    matching_brackets: bool,
}

#[cfg(feature = "lsp")]
#[derive(Subcommand)]
enum Command {
    /// Run the language server
    Lsp,
}

#[cfg(feature = "lsp")]
#[tokio::main]
async fn lsp_main() {
    faye_lsp::run().await;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = FayeArgs::parse();
    let match_brackets = args.matching_brackets;

    #[cfg(feature = "lsp")]
    if let Some(Command::Lsp) = args.command {
        lsp_main();
        return Ok(());
    }

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
        let parser = FayeParser::new(&code);
        let ast = parser.parse()?;
        println!("{ast:?}");

        return Ok(());
    }

    Repl::new(match_brackets).start();

    Ok(())
}

macro_rules! err {
    ($src:tt@$path:expr => $err:ident, $hl:ident) => {
        eprintln!(
            "\x1b[1;36m   --> \x1b[0m{}:{}:{}\n\x1b[1;36m    |\n{:^4}|\x1b[0m {}\n\x1b[1;36m    |\x1b[0m{}\x1b[1;31m{} {}\x1b[0m",
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

    let parser = FayeParser::new(code);
    let path = path.unwrap_or("<input>");

    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(err) => return err!(code@path => err, hl),
    };

    ast.iter().for_each(|n| match ctx.eval(n) {
        Ok(Expr::Nil) => {}
        Ok(res) => println!("{res}"),
        Err(err) => err!(code@path => err, hl),
    });
}
