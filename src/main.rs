mod user;
mod message;
mod ticket;
use std::io::{self, Write};
use user::User;
use message::Message;
use ticket::ChatTicket;
use n0_watcher::Watcher; 

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let stdin = std::io::stdin();
    let mut name = String::new();

    const COMMAND_LIST: &str = r#"
    Commands:
        /help - Displays Commands
        /open - Opens a Chatroom
        /join [ticket] - Joins a Chatroom
        /leave - Leaves the Chatroom
        /exit - Closes the application
    "#;

    println!("Welcome to Speaky!");
    print!("Please enter a username: ");
    io::stdout().flush().expect("Failed to flush stdout");

    stdin
        .read_line(& mut name)
        .expect("Failed to read user input");

    let mut client = User::new(name.clone()).await?;
    let node_addr = client.get_endpoint().node_addr().initialized().await?;

    println!("{}", COMMAND_LIST);

    loop {
        print!("<{}>: ", name.trim());
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        stdin
            .read_line(& mut input)
            .expect("Failed to read user input");

        match input.as_str().trim() {
            "/help" => {
                println!("{}", COMMAND_LIST);
            }
            "/open" => {
                let ticket = ChatTicket::new(client.create_topic(), vec![node_addr.clone()]);
                println!("\n\tChatroom Ticket: \n\n{}\n", ticket);

                let (sender, receiver) = gossip.subscribe_and_join(topic, vec![]).await?.split();
                //add

            }
            "/leave" => {
                println!("Leaving the chatroom...");
            }
            "/exit" => {
                println!("Exiting the application...");
            }
            _ => {
                match input.split_once(' ') {
                    Some(("/join", ticket)) => {
                        println!("Joining chatroom with ticket: {}", ticket.trim());
                    }
                    _ => {
                        println!("Unknown command. Type /help for a list of commands.");
                    }
                }
            }
        }
    }

    Ok(())
}
