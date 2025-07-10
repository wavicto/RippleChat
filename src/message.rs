use iroh::NodeId;
use iroh::SecretKey;
use serde::{Deserialize, Serialize};
use ed25519_dalek::Signature;
use sha2::{Sha256, Digest};

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    text : String,
    sender_name : String,
    sender_id : NodeId,
    signature : Signature,
    nonce: [u8; 16]
}

impl Message {
    pub fn new(text : String, sender_name: String, sender_id : NodeId, private_key : SecretKey) -> Self {
        let hash = Sha256::digest(&text);
        let signature: Signature = private_key.sign(&hash);
        Self {
            text,
            sender_name,
            sender_id,
            signature,
            nonce: rand::random(),
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> anyhow::Result<Self> {
        serde_json::from_slice(bytes).map_err(Into::into)
    }

    pub fn to_vec(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("serde_json::to_vec is infallible")
    }

    pub fn verify_msg (&self) -> bool {
        let hash = Sha256::digest(&self.text);
        self.sender_id.verify(&hash, &self.signature).is_ok()
    }
}