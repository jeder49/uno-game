use bevy_renet::renet::{ConnectionConfig, RenetServer};

pub const PROTOCOL_ID: u64 = 7;
pub const PORT: u16 = 5000;

pub const RELIABLE_CHANNEL_ID: u8 = 0;

pub fn create_renet_server() -> RenetServer {
    RenetServer::new(ConnectionConfig::default())
}
