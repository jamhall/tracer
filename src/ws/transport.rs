use crate::ws::Packet;

#[derive(Default)]
pub struct Transport;

impl Transport {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn decode(&self, json: &str) -> Option<Packet> {
        match serde_json::from_str::<Packet>(json) {
            Ok(packet) => Some(packet),
            Err(_) => None,
        }
    }

    pub fn encode(&self, packet: Packet) -> Option<String> {
        match serde_json::to_string(&packet) {
            Ok(json) => Some(json),
            Err(_) => None,
        }
    }
}
