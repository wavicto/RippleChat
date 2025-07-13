use iroh::{protocol::Router, Endpoint};
use iroh_gossip::{net::Gossip, proto::TopicId, api::GossipReceiver, api::GossipSender, ALPN};
use futures_lite::StreamExt;
use crate::message::Message;
use crate::ticket::ChatTicket;

pub struct User {
    endpoint : Endpoint,
    gossip : Gossip,
    router : Router,
    topic_id : TopicId,
    name : String
}

impl User {
    //Constructor for a User with a given name
    //Sets up iroh node, endpoint, gossip protocol, router, and generates a topic ID
    pub async fn new(name: String) -> anyhow::Result<Self> {
        let endpoint = Endpoint::builder().discovery_n0().bind().await?;

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
            name
        })
    }

    //Reads and displays incoming messages from the receiver stream
    pub async fn read(mut receiver: GossipReceiver) -> anyhow::Result<()> {
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
                            println!("{} : {}", msg.get_name(), msg.get_text());
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

    pub async fn join(&self, empty : bool) -> anyhow::Result<(GossipSender, GossipReceiver)> {
        if (!empty){
            
        }

        let (sender, receiver) = self.gossip
            .subscribe_and_join(self.topic_id, vec![])
            .await?
            .split();

        Ok((sender, receiver))
    }

    //Starts an input loop (sync)
    //Should be ran in a separate thread to avoid blocking the async runtime
    //Employs a mpsc channel that sends user messages between the async and sync threads
    pub fn input_loop(transmitter: tokio::sync::mpsc::Sender<String>) -> anyhow::Result<()> {
        let mut buffer = String::new();
        let stdin = std::io::stdin();
    
        loop {
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

}

