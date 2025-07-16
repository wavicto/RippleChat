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
    io::stdout().flush().unwrap();

    let stdin = std::io::stdin();
    let mut name = String::new();

    stdin
        .read_line(&mut name)
        .expect("Failed to read user input");

    name = name.trim().to_string();

    let mut client = User::new().await?;
    let node_addr = client.get_endpoint().node_addr().initialized().await?;

    println!("{}", COMMAND_LIST);

    loop {
        print!("<{}>: ", name);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        stdin
            .read_line(&mut input)
            .expect("Failed to read user input");

        match input.as_str().trim() {
            "/help" => {
                println!("{}", COMMAND_LIST);
            }
            "/leave" => {
                println!("\nYou need to be in a chatroom to leave.\n");
            }
            "/exit" => {
                println!("\nExiting the application ...\n");
                client.shutdown_user().await?;
                break;
            }
            "/open" => {
                let read_clone: String = name.clone();
                let input_clone: String = name.clone();
                let ticket = ChatTicket::new(client.get_topic_id(), vec![node_addr.clone()]);
                println!("\n\tChatroom Ticket: \n\n{}\n", ticket);

                println!("Waiting for connections ...");
                let (mut sender, receiver) = match client.open_room().await {
                    Ok((s, r)) => (s, r),
                    Err(e) => {
                        eprintln!("Trouble opening room: try again? ({})", e);
                        continue;
                    }
                };

                println!("\nConnection found.\n");

                tokio::spawn(User::read(receiver, read_clone));
                let (tx, mut rx) = tokio::sync::mpsc::channel(1);
                std::thread::spawn(move || User::input_loop(input_clone, tx));

                while let Some(text) = rx.recv().await {
                    match text.trim() {
                        "/help" => {
                            print!("\r\x1b[2K");
                            io::stdout().flush().unwrap();
                            println!("{}", COMMAND_LIST);
                            print!("<{}>: ", name);
                            io::stdout().flush().unwrap();
                        }
                        "/exit" => {
                            print!("\r\x1b[2K");
                            io::stdout().flush().unwrap();
                            println!("\nPlease leave the chat first before exiting.\n");
                            print!("<{}>: ", name);
                            io::stdout().flush().unwrap();
                        }
                        "/leave" => {
                            let msg = String::from("System: User has disconnected.");
                            let disconnect_msg = Message::new(
                                msg,
                                name.clone(),
                                client.get_endpoint().node_id().clone(),
                                client.get_endpoint().secret_key().clone(),
                            );
                            sender.broadcast(disconnect_msg.to_vec().into()).await?;
                            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                            print!("\r\x1b[2K");
                            io::stdout().flush().unwrap();
                            if let Err(e) = client.shutdown_chat().await {
                                eprintln!("Error shutting down gossip: {}.\nMaybe restart program?", e);
                            }
                            client.restart_chat();
                            println!("\nYou have disconnected from the chat.\n");
                            break;
                        }
                        _ => {
                            let message = Message::new(
                                text.clone(),
                                name.clone(),
                                client.get_endpoint().node_id().clone(),
                                client.get_endpoint().secret_key().clone(),
                            );

                            sender.broadcast(message.to_vec().into()).await?;
                        }
                    }
                }
            }
            _ => {
                match input.split_once(' ') {
                    Some(("/join", ticket)) => {
                        match ChatTicket::from_str(ticket.trim()) {
                            Ok(t) => {
                                let read_clone: String = name.clone();
                                let input_clone: String = name.clone();
                                println!("\nJoining chatroom ...");

                                let (mut sender, receiver) = match client.join_room(&t).await {
                                    Ok((s, r)) => (s, r),
                                    Err(e) => {
                                        eprintln!("Trouble joining room: try again using commands ({})", e);
                                        continue;
                                    }
                                };

                                println!("\nRoom joined.\n");

                                tokio::spawn(User::read(receiver, read_clone));

                                let (tx, mut rx) = tokio::sync::mpsc::channel(1);
                                std::thread::spawn(move || User::input_loop(input_clone, tx));

                                while let Some(text) = rx.recv().await {
                                    match text.trim() {
                                        "/help" => {
                                            print!("\r\x1b[2K");
                                            io::stdout().flush().unwrap();
                                            println!("{}", COMMAND_LIST);
                                            print!("<{}>: ", name);
                                            io::stdout().flush().unwrap();
                                        }
                                        "/exit" => {
                                            print!("\r\x1b[2K");
                                            io::stdout().flush().unwrap();
                                            println!("\nPlease leave the chat first before exiting.\n");
                                            print!("<{}>: ", name);
                                            io::stdout().flush().unwrap();
                                        }
                                        "/leave" => {
                                            let msg = String::from("System: User has disconnected.");
                                            let disconnect_msg = Message::new(
                                                msg,
                                                name.clone(),
                                                client.get_endpoint().node_id().clone(),
                                                client.get_endpoint().secret_key().clone(),
                                            );
                                            sender.broadcast(disconnect_msg.to_vec().into()).await?;
                                            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                                            print!("\r\x1b[2K");
                                            io::stdout().flush().unwrap();
                                            if let Err(e) = client.shutdown_chat().await {
                                                eprintln!("Error shutting down gossip: {}.\nMaybe restart program?", e);
                                            }
                                            client.restart_chat();
                                            println!("\nYou have disconnected from the chat.\n");
                                            break;
                                        }
                                        _ => {
                                            let message = Message::new(
                                                text.clone(),
                                                name.clone(),
                                                client.get_endpoint().node_id().clone(),
                                                client.get_endpoint().secret_key().clone(),
                                            );

                                            sender.broadcast(message.to_vec().into()).await?;
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
                        println!("\nUnknown command. Type /help for a list of commands.\n");
                    }
                }
            }
        }
    }

    Ok(())
}
