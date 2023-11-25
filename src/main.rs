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
use faye::prelude::{Context, Expr, Highlighter, Lexer, Parser as FayeParser, Span};

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
        let mut parser = FayeParser::new(&code);
        let ast = parser.parse()?;
        println!("{ast:#?}");

        return Ok(());
    }

    Repl::new(match_brackets).start();

    Ok(())
}

fn display_error(hl: Highlighter, span: &Span, err: &impl std::error::Error) {
    let loc = span.location();
    let end_loc = span.end_location();

    let fmt = hl.highlight(span.source.get_line(loc.line));
    let name = span.source.name().unwrap_or("<input>");
    let line = loc.line + 1;
    let col = loc.column + 1;

    eprintln!(
        "\x1b[1;36m   --> \x1b[0m{name}:{line}:{col}\n\
             \x1b[1;36m    |\n\
                  {line:^4}|\x1b[0m {fmt}\n\
             \x1b[1;36m    |\x1b[0m {}\x1b[1;31m{} {err}\x1b[0m",
        " ".repeat(loc.column),
        "^".repeat(end_loc.column - loc.column),
    );
}

fn eval(code: &str, path: Option<&str>, match_brackets: bool) {
    let mut ctx = Context::new();
    let hl = Highlighter::new(match_brackets);

    let mut parser = FayeParser::new(code);
    if let Some(p) = path {
        parser.set_name(p.to_owned());
    }

    let ast = match parser.parse() {
        Ok(ast) => ast,
        Err(err) => return display_error(hl, &err.span, &err),
    };

    ast.iter().for_each(|n| match ctx.eval(n) {
        Ok(Expr::Nil) => {}
        Ok(res) => println!("{res}"),
        Err(err) => display_error(hl, &err.span, &err),
    });
}
