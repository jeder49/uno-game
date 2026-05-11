use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerInfo {
    pub id: Uuid,
    pub name: String,
    pub is_host: bool,
    pub is_ready: bool,
    pub is_ai: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LobbyState {
    /// Short alphanumeric code others use to join (e.g. "X7KQ2")
    pub room_code: String,
    pub players: Vec<PlayerInfo>,
    pub max_players: u8,
    /// Whether the server is accepting AI WebSocket connections for this room
    pub ai_api_enabled: bool,
}
