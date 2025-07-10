use std::io::{self, Write};
mod user;
mod message;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let stdin = std::io::stdin();
    loop {
        print!("User: ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        stdin
            .read_line(& mut input)
            .expect("Failed to read user input");

        match input.as_str() {
            "open\n" => println!("Creating a topic"),
            "connect\n" => println!("Connecting to a topic"),
            "bye\n" => break,
            _ => println!("Unknown command"),
        }
    }

    Ok(())
}
