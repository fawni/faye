// faye, a pretty lil lisp
// Copyright (c) 2023 fawn
//
// SPDX-License-Identifier: Apache-2.0

use rustyline::{error::ReadlineError, validate};

use crate::eval::Context;
use crate::lexer::Lexer;
use crate::parser;
use crate::Highlighter;

pub fn start(match_brackets: bool) -> Result<(), Box<dyn std::error::Error>> {
    println!("\x1b[1;35mfaye \x1b[0m{}", env!("CARGO_PKG_VERSION"));
    println!("press \x1b[31mctrl+c\x1b[0m or \x1b[31mctrl+d\x1b[0m to exit\n");

    let mut ctx = Context::default();
    let hl = Highlighter::new(match_brackets);

    let prompt = "~> ";
    let config = rustyline::Config::builder()
        .auto_add_history(true)
        .max_history_size(100)?
        .build();
    let mut rl = rustyline::Editor::with_config(config)?;
    rl.set_helper(Some(FayeHelper { highlighter: hl }));

    loop {
        match rl.readline(prompt) {
            Ok(line) => run(&mut ctx, &line, prompt.len()),
            Err(ReadlineError::Interrupted) => return Ok(println!("\x1b[31mctrl-c\x1b[0m")),
            Err(ReadlineError::Eof) => return Ok(println!("\x1b[31mctrl-d\x1b[0m")),
            Err(err) => eprintln!("\x1b[1;31mrepl error\x1b[0m: {err}"),
        }
    }
}

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

fn run(ctx: &mut Context, line: &str, prompt_len: usize) {
    let mut lex = Lexer::new(line);

    let ast = match parser::parse(&mut lex) {
        Ok(ast) => ast,
        Err(err) => err!(err, prompt_len),
    };

    ast.iter().for_each(|n| match ctx.eval(n) {
        Ok(res) => println!("{res}"),
        Err(err) => err!(err, prompt_len),
    });
}

struct FayeHelper {
    highlighter: Highlighter,
}

impl rustyline::Helper for FayeHelper {}

impl rustyline::completion::Completer for FayeHelper {
    type Candidate = String;
}

impl rustyline::hint::Hinter for FayeHelper {
    type Hint = String;
    fn hint(&self, line: &str, a: usize, b: &rustyline::Context) -> Option<Self::Hint> {
        let hinter = rustyline::hint::HistoryHinter {};
        hinter.hint(line, a, b)
    }
}

impl rustyline::highlight::Highlighter for FayeHelper {
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        _default: bool,
    ) -> std::borrow::Cow<'b, str> {
        format!("\x1b[36m{prompt}\x1b[0m").into()
    }

    fn highlight_char(&self, _line: &str, _cursor_pos: usize) -> bool {
        true
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> std::borrow::Cow<'h, str> {
        format!("\x1b[3;90m{hint}\x1b[0m").into()
    }

    fn highlight<'l>(&self, line: &'l str, _cursor_pos: usize) -> std::borrow::Cow<'l, str> {
        self.highlighter.highlight(line).into()
    }
}

impl validate::Validator for FayeHelper {
    fn validate(
        &self,
        ctx: &mut validate::ValidationContext,
    ) -> rustyline::Result<validate::ValidationResult> {
        let mut lex = Lexer::new(ctx.input());
        match parser::parse(&mut lex) {
            Err(e) if e.kind == parser::ErrorKind::UnclosedParen => {
                Ok(validate::ValidationResult::Incomplete)
            }
            _ => Ok(validate::ValidationResult::Valid(None)),
        }
    }
}
