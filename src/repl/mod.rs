// faye, a pretty lil lisp
// Copyright (c) 2023 fawn
//
// SPDX-License-Identifier: Apache-2.0

use rustyline::error::ReadlineError;

use crate::{Context, Highlighter, Parser};

use helper::FayeHelper;

mod helper;

macro_rules! err {
    ($e:ident, $l:ident) => {
        return eprintln!(
            "{}\x1b[1;31m{}\nerror\x1b[0m: {}",
            " ".repeat($e.start.1 + $l),
            "^".repeat($e.end.1 - $e.start.1),
            $e
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
    pub fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("\x1b[1;35mfaye \x1b[0m{}", env!("CARGO_PKG_VERSION"));
        println!("press \x1b[31mctrl+c\x1b[0m or \x1b[31mctrl+d\x1b[0m to exit\n");

        let mut ctx = Context::default();
        let hl = Highlighter::new(self.match_brackets);

        let prompt = "~> ";
        let config = rustyline::Config::builder()
            .auto_add_history(true)
            .max_history_size(100)?
            .build();
        let mut rl = rustyline::Editor::with_config(config)?;
        rl.set_helper(Some(FayeHelper::new(hl)));

        loop {
            match rl.readline(prompt) {
                Ok(line) => Self::run(&mut ctx, &line, prompt.len()),
                Err(ReadlineError::Interrupted) => return Ok(println!("\x1b[31mctrl-c\x1b[0m")),
                Err(ReadlineError::Eof) => return Ok(println!("\x1b[31mctrl-d\x1b[0m")),
                Err(err) => eprintln!("\x1b[1;31mrepl error\x1b[0m: {err}"),
            }
        }
    }

    fn run(ctx: &mut Context, line: &str, prompt_len: usize) {
        let parser = Parser::new(line);

        let ast = match parser.parse() {
            Ok(ast) => ast,
            Err(err) => err!(err, prompt_len),
        };

        ast.iter().for_each(|n| match ctx.eval(n) {
            Ok(res) => println!("{res}"),
            Err(err) => err!(err, prompt_len),
        });
    }
}
