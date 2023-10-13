// faye, a pretty lil lisp
// Copyright (c) 2023 fawn
//
// SPDX-License-Identifier: Apache-2.0

use faye::prelude::{Context, Highlighter, Parser};

use editor::FayeEditor;

mod editor;

macro_rules! err {
    ($src:tt => $err:ident, $hl:ident) => {
        eprintln!(
            "\x1b[1;31merror\x1b[0;1m: {}\n\x1b[1;36m{:^4}|\x1b[0m {}\n\x1b[1;36m    | {}\x1b[31m{}\x1b[0m",
            $err,
            $err.start.0 + 1,
            $hl.highlight($src.split('\n').nth($err.start.0).unwrap()),
            " ".repeat($err.start.1),
            "^".repeat($err.end.1 - $err.start.1),
        )
    };

    ($err:ident, $pl:ident) => {
        eprintln!(
            "{}\x1b[1;31m{}\nerror\x1b[0;1m: {}",
            " ".repeat($err.start.1 + $pl),
            "^".repeat($err.end.1 - $err.start.1),
            $err
        )
    };
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

        let mut pom = pomprt::with_multiline(FayeEditor::new(hl, ctx), "~> ", "\\  ");

        loop {
            match pom.read() {
                Ok(line) => Self::eval(&mut pom.editor.ctx, &line, hl, 3),
                Err(pomprt::Interrupt) => return println!("\x1b[31mctrl-c\x1b[0m"),
                Err(pomprt::Eof) => return println!("\x1b[31mctrl-d\x1b[0m"),
                Err(err) => eprintln!("\x1b[1;31mrepl error\x1b[0m: {err}"),
            }
        }
    }

    fn eval(ctx: &mut Context, code: &str, hl: Highlighter, prompt_len: usize) {
        let parser = Parser::new(code);

        let ast = match parser.parse() {
            Ok(ast) => ast,
            Err(err) => return err!(err, prompt_len),
        };

        ast.iter().enumerate().for_each(|(i, n)| match ctx.eval(n) {
            Ok(res) => println!("{res}"),
            Err(err) if i == 0 => err!(err, prompt_len),
            Err(err) => err!(code => err, hl),
        });
    }
}
