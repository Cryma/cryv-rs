use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum NetworkMessage {
    EstablishConnection(String),
    ConnectionEstablished,
}

impl NetworkMessage {
    pub fn encode(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }

    pub fn decode(bytes: &[u8]) -> NetworkMessage {
        bincode::deserialize(bytes).unwrap()
    }
}
