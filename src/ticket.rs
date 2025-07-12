use iroh::NodeAddr;
use iroh_gossip::proto::TopicId;
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
struct ChatTicket {
    topic: TopicId,
    nodes: Vec<NodeAddr>,
}

impl ChatTicket {
    fn from_bytes(bytes: &[u8]) -> anyhow::Result<Self> {
        serde_json::from_slice(bytes).map_err(Into::into)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("serde_json::to_vec is infallible")
    }
}