// faye, a pretty lil lisp
// Copyright (c) 2023 fawn
//
// SPDX-License-Identifier: Apache-2.0

use faye::prelude::{Context, Highlighter, Parser, Span};

use editor::FayeEditor;

mod editor;

fn display_error(hl: Highlighter, span: &Span, err: &impl std::error::Error) {
    let loc = span.location();
    let end_loc = span.end_location();

    let fmt = hl.highlight(span.source.get_line(loc.line));
    let line = loc.line + 1;

    eprintln!(
        "\x1b[1;31merror\x1b[0;1m: {err}\n\x1b[1;36m{line:^4}|\x1b[0m {fmt}\n\x1b[1;36m    | {}\x1b[31m{}\x1b[0m",
        " ".repeat(loc.column),
        "^".repeat(end_loc.column - loc.column),
    );
}

fn display_short_error(span: &Span, err: &impl std::error::Error, pad: usize) {
    let loc = span.location();
    let end_loc = span.end_location();

    eprintln!(
        "{}\x1b[1;31m{}\nerror\x1b[0;1m: {err}",
        " ".repeat(loc.column + pad),
        "^".repeat(end_loc.column - loc.column),
    );
}

/// A Read-Eval-Print-Loop for faye
#[derive(Default)]
pub struct Repl {
    match_brackets: bool,
}

impl Repl {
    /// Create a new repl instance and specify whether to highlight matching brackets or not
    #[must_use]
    pub const fn new(match_brackets: bool) -> Self {
        Self { match_brackets }
    }

    /// Start the repl
    pub fn start(&self) {
        println!("\x1b[1;35mfaye \x1b[0m{}", env!("CARGO_PKG_VERSION"));
        println!("press \x1b[31mctrl+c\x1b[0m or \x1b[31mctrl+d\x1b[0m to exit\n");

        let ctx = Context::default();
        let hl = Highlighter::new(self.match_brackets);

        let prompt = "~> ";
        let mut pom = pomprt::with_multiline(FayeEditor::new(hl, ctx), prompt, "\\  ");

        loop {
            match pom.read() {
                Ok(line) => Self::eval(&mut pom.editor.ctx, &line, hl, prompt.len()),
                Err(pomprt::Interrupt) => return println!("\x1b[31mctrl-c\x1b[0m"),
                Err(pomprt::Eof) => return println!("\x1b[31mctrl-d\x1b[0m"),
                Err(err) => eprintln!("\x1b[1;31mrepl error\x1b[0m: {err}"),
            }
        }
    }

    fn eval(ctx: &mut Context, code: &str, hl: Highlighter, prompt_len: usize) {
        let mut parser = Parser::new(code);

        let ast = match parser.parse() {
            Ok(ast) => ast,
            Err(err) => return display_error(hl, &err.span, &err),
        };

        ast.iter().enumerate().for_each(|(i, n)| match ctx.eval(n) {
            Ok(res) => println!("{res}"),
            Err(err) if i == 0 && n.span.same_source(&err.span) => {
                display_short_error(&err.span, &err, prompt_len);
            }
            Err(err) => display_error(hl, &err.span, &err),
        });
    }
}
