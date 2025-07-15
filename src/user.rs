use iroh::NodeId;
use iroh::{protocol::Router, Endpoint, discovery::static_provider::StaticProvider};
use iroh_gossip::{net::Gossip, proto::TopicId, api::GossipReceiver, api::GossipSender, ALPN};
use futures_lite::StreamExt;
use std::io::{self, Write};
use crate::message::Message;
use crate::ticket::ChatTicket;

pub struct User {
    endpoint : Endpoint,
    gossip : Gossip,
    router : Router,
    topic_id : TopicId,
    discovery : StaticProvider,
}

impl User {
    //Constructor for a User with a given name
    //Sets up iroh node, endpoint, gossip protocol, router, and generates a topic ID
    pub async fn new() -> anyhow::Result<Self> {
        let discovery = StaticProvider::new();

        let endpoint = Endpoint::builder().discovery_n0().add_discovery(discovery.clone()).bind().await?;

        let gossip = Gossip::builder().spawn(endpoint.clone());

        let router = Router::builder(endpoint.clone())
            .accept(ALPN, gossip.clone())
            .spawn();

        let topic_id = TopicId::from_bytes(rand::random());

        Ok(Self {
            endpoint,
            gossip,
            router,
            topic_id,
            discovery
        })
    }

    pub async fn open_room(&self) -> anyhow::Result<(GossipSender, GossipReceiver)> {
        let (sender, receiver) = self.gossip
            .subscribe_and_join(self.topic_id, vec![])
            .await?
            .split();

        Ok((sender, receiver))
    }

    pub async fn join_room(&self, ticket : &ChatTicket) -> anyhow::Result<(GossipSender, GossipReceiver)> {

        for addr in ticket.get_node_addrs() {
            self.discovery.add_node_info(addr.clone());
        }

        let ids: Vec<NodeId> = ticket.get_node_addrs().iter().map(|addr| addr.node_id).collect();

        let (sender, receiver) = self.gossip
            .subscribe_and_join(*ticket.get_topic_id(), ids)
            .await?
            .split();

        Ok((sender, receiver))
    }

    //Reads and displays incoming messages from the receiver stream
    pub async fn read(mut receiver: GossipReceiver, name : String) -> anyhow::Result<()> {
        while let Some(event) = receiver.try_next().await? {
            if let iroh_gossip::api::Event::Received(message) = event {
                let msg = Message::from_bytes(&message.content);
                match msg {
                    Ok(msg) => {
                        if !msg.verify() {
                            println!("Invalid msg signature. 
                                    Claiming to be from {} / {}", msg.get_name(), msg.get_id());
                        }
                        else {
                            print!("\r\x1b[2K");
                            io::stdout().flush().unwrap();
                            println!("<{}> : {}", msg.get_name().trim(), msg.get_text().trim());

                            print!("<{}>: ", name);
                            io::stdout().flush().expect("Failed to flush stdout");
                        }
                    }
                    Err(e) => {
                        eprintln!("Error deserializing message: {}", e);
                    }
                }
            }
        }
        Ok(())
    }

    //Starts an input loop (sync)
    //Should be ran in a separate thread to avoid blocking the async runtime
    //Employs a mpsc channel that sends user messages between the async and sync threads
    pub fn input_loop(name : String, transmitter: tokio::sync::mpsc::Sender<String>) -> anyhow::Result<()> {
        let mut buffer = String::new();
        let stdin = io::stdin();
    
        loop {
            print!("<{}>: ", name);
            io::stdout().flush().expect("Failed to flush stdout");
            stdin.read_line(&mut buffer)?;
            transmitter.blocking_send(buffer.clone())?;
            buffer.clear();
        }
    }

    pub fn create_topic(&mut self) -> TopicId {
            self.topic_id = TopicId::from_bytes(rand::random());
            self.topic_id.clone()
        }

    pub fn get_endpoint(&self) -> &Endpoint {
        &self.endpoint
    }

    //Errors are ignored since router is shutdown when the program is closed
    pub async fn shutdown(&self) -> anyhow::Result<()> {
        let _ = self.router.shutdown().await;
        Ok(())
    }

    pub async fn shutdown_chat(&self) {
        self.gossip.shutdown().await;
    }

    pub fn restart_chat(&mut self) {
            self.gossip = Gossip::builder().spawn(self.endpoint.clone());
            self.topic_id = TopicId::from_bytes(rand::random());
        }

}

