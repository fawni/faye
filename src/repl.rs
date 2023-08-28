use crate::lexer::Lexer;
use crate::{eval, parser};

pub fn start() -> Result<(), Box<dyn std::error::Error>> {
    println!("\x1b[1;35mfaye\x1b[0m v0.0.1\npress \x1b[31mctrl+c\x1b[0m to exit\n");

    loop {
        let line = readline("~> ")?;
        run(&line);
    }
}

fn readline(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    use std::io::Write;

    print!("{prompt}");
    std::io::stdout().flush().ok();

    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer)?;

    Ok(buffer.trim().to_owned())
}

fn run(line: &str) {
    let mut lex = Lexer::new(line);

    let ast = match parser::parse(&mut lex) {
        Ok(ast) => ast,
        Err(err) => return println!("\x1b[1;31merror\x1b[0m: {err}"),
    };

    match eval::eval(ast) {
        Ok(res) => println!("\x1b[32m{res}\x1b[0m"),
        Err(err) => println!("\x1b[1;31merror\x1b[0m: {err}"),
    }
}
