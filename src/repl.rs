pub fn start() -> Result<(), Box<dyn std::error::Error>> {
    println!("\x1b[1;35mfaye\x1b[0m v0.0.1\npress \x1b[31mctrl+c\x1b[0m to exit\n");
    loop {
        let line = readline("-> ")?;
        println!("{line}");
    }
}

fn readline(prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
    use std::io::Write;

    print!("{prompt}");
    std::io::stdout().flush()?;

    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer)?;

    Ok(buffer.trim().to_string())
}
