use iroh::NodeAddr;
use iroh_gossip::proto::TopicId;
use std::fmt;
use std::str::FromStr;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
struct ChatTicket {
    topic: TopicId,
    nodes: Vec<NodeAddr>,
}

impl ChatTicket {
    //Constructor for a ChatTicket 
    pub fn new(topic: TopicId, nodes: Vec<NodeAddr>) -> Self {
        Self {
            topic,
            nodes
        }
    }

    //Deserialize json bytes to ChatTicket
    fn from_bytes(bytes: &[u8]) -> anyhow::Result<Self> {
        serde_json::from_slice(bytes).map_err(Into::into)
    }

    //Serialize ChatTicket to json in bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("serde_json::to_vec is infallible")
    }
}

//Turns ticket from bytes into base32 string
impl fmt::Display for ChatTicket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut text = data_encoding::BASE32_NOPAD.encode(&self.to_bytes()[..]);
        text.make_ascii_lowercase();
        write!(f, "{}", text)
    }
}

//Turns base32 string to bytes then to ticket
impl FromStr for ChatTicket {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = data_encoding::BASE32_NOPAD.decode(s.to_ascii_uppercase().as_bytes())?;
        Self::from_bytes(&bytes)
    }
}