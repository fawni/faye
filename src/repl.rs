// faye, a pretty lil lisp
// Copyright (c) 2023 fawn
//
// SPDX-License-Identifier: Apache-2.0

use rustyline::{error::ReadlineError, Config, Editor};

use crate::{
    eval::{self, Context},
    lexer::Lexer,
    parser,
};

pub fn start() -> Result<(), Box<dyn std::error::Error>> {
    println!("\x1b[1;35mfaye \x1b[0m{}", env!("CARGO_PKG_VERSION"));
    println!("press \x1b[31mctrl+c\x1b[0m or \x1b[31mctrl+d\x1b[0m to exit\n");

    let mut ctx = Context::default();

    let config = Config::builder()
        .auto_add_history(true)
        .max_history_size(100)?
        .build();
    let mut rl: Editor<(), _> = Editor::with_config(config)?;
    let prompt = "~> ";

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

fn run(ctx: &mut eval::Context, line: &str, prompt_len: usize) {
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
