mod lexer;
mod parser;
mod repl;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    repl::start()?;

    Ok(())
}
