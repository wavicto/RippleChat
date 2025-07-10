use iroh::{protocol::Router, Endpoint};
use iroh_gossip::{net::Gossip, proto::TopicId, ALPN};

pub struct User {
    endpoint : Endpoint,
    gossip : Gossip,
    router : Router,
    topic_id : TopicId,
    name : String
}

impl User {
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
}