// faye, a pretty lil lisp
// Copyright (c) 2023 fawn
//
// SPDX-License-Identifier: Apache-2.0

pub fn start() -> Result<(), Box<dyn std::error::Error>> {
    use rustyline::error::ReadlineError;
    use rustyline::{Config, Editor};

    println!("\x1b[1;35mfaye\x1b[0m v0.1.0\npress \x1b[31mctrl+c\x1b[0m or \x1b[31mctrl+d\x1b[0m to exit\n");

    let config = Config::builder()
        .auto_add_history(true)
        .max_history_size(100)?
        .build();
    let mut rl: Editor<(), _> = Editor::with_config(config)?;
    let prompt = "~> ";

    loop {
        match rl.readline(prompt) {
            Ok(line) => run(&line, prompt.len()),
            Err(ReadlineError::Interrupted) => return Ok(println!("\x1b[31mctrl-c\x1b[0m")),
            Err(ReadlineError::Eof) => return Ok(println!("\x1b[31mctrl-d\x1b[0m")),
            Err(err) => println!("\x1b[1;31mrepl error\x1b[0m: {err}"),
        }
    }
}

fn run(line: &str, prompt_len: usize) {
    use crate::lexer::Lexer;
    use crate::{eval, parser};

    let mut lex = Lexer::new(line);

    let ast = match parser::parse(&mut lex) {
        Ok(ast) => ast,
        Err(err) => {
            let (_, col, len) = err.1;
            return println!(
                "{}\x1b[1;31m{}\nerror\x1b[0m: {err}",
                " ".repeat(col + prompt_len),
                "^".repeat(len),
            );
        }
    };

    match eval::eval(&ast) {
        Ok(res) => println!("\x1b[32m{res}\x1b[0m"),
        Err(err) => println!("\x1b[1;31merror\x1b[0m: {err}"),
    }
}
