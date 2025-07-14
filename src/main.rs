mod user;
mod message;
mod ticket;
use std::io::{self, Write};
use user::User;
use message::Message;
use ticket::ChatTicket;
use n0_watcher::Watcher; 
use std::str::FromStr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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

    let stdin = std::io::stdin();
    let mut name = String::new();

    stdin
        .read_line(&mut name)
        .expect("Failed to read user input");

    let mut client = User::new().await?;
    let node_addr = client.get_endpoint().node_addr().initialized().await?;

    println!("{}", COMMAND_LIST);

    loop {
        print!("<{}>: ", name.trim());
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        stdin
            .read_line(&mut input)
            .expect("Failed to read user input");

        match input.as_str().trim() {
            "/help" => {
                println!("{}", COMMAND_LIST);
            }
            "/leave" => {
                println!("You need to be in a chatroom to leave.");
            }
            "/exit" => {
                println!("Exiting the application...");
                client.shutdown().await?;
                break;
            }
            "/open" => {
                let ticket = ChatTicket::new(client.create_topic(), vec![node_addr.clone()]);
                println!("\n\tChatroom Ticket: \n\n{}\n", ticket);

                println!("Waiting for connections ...");
                let (mut sender, receiver) = match client.open_room().await {
                    Ok((s, r)) => (s, r),
                    Err(e) => {
                        eprintln!("Trouble opening room: try again? ({})", e);
                        continue;
                    }
                };

                println!("\nConnected");

                tokio::spawn(User::read(receiver));

                let (tx, mut rx) = tokio::sync::mpsc::channel(1);
                std::thread::spawn(move || User::input_loop(tx));

                while let Some(text) = rx.recv().await {
                    match text.trim() {
                        "/leave" => {
                            client.shutdown_chat().await;
                            client.restart_chat();
                            continue;
                        }
                        "/help" => {
                            println!("{}", COMMAND_LIST);
                        }
                        _ => {
                            let message = Message::new(
                                text.clone(),
                                name.clone(),
                                client.get_endpoint().node_id().clone(),
                                client.get_endpoint().secret_key().clone(),
                            );

                            sender.broadcast(message.to_vec().into()).await?;
                            println!("<{}>: {}", name, text);
                        }
                    }
                }
            }
            _ => {
                match input.split_once(' ') {
                    Some(("/join", ticket)) => {
                        match ChatTicket::from_str(ticket.trim()) {
                            Ok(t) => {
                                println!("\nJoining chatroom ...");

                                let (mut sender, receiver) = match client.join_room(&t).await {
                                    Ok((s, r)) => (s, r),
                                    Err(e) => {
                                        eprintln!("Trouble joining room: try again using commands ({})", e);
                                        continue;
                                    }
                                };

                                println!("\nRoom joined ...");

                                tokio::spawn(User::read(receiver));

                                let (tx, mut rx) = tokio::sync::mpsc::channel(1);
                                std::thread::spawn(move || User::input_loop(tx));

                                while let Some(text) = rx.recv().await {
                                    match text.trim() {
                                        "/leave" => {
                                            client.shutdown_chat();
                                            client.restart_chat();
                                            continue;
                                        }
                                        "/help" => {
                                            println!("{}", COMMAND_LIST);
                                        }
                                        _ => {
                                            let message = Message::new(
                                                text.clone(),
                                                name.clone(),
                                                client.get_endpoint().node_id().clone(),
                                                client.get_endpoint().secret_key().clone(),
                                            );

                                            sender.broadcast(message.to_vec().into()).await?;
                                            println!("<{}>: {}", name, text);
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                eprintln!("Invalid ticket: {}", e);
                                println!("Try again?");
                                continue;
                            }
                        }
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
