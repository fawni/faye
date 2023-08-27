mod lexer;
mod repl;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    repl::start()?;

    Ok(())
}
