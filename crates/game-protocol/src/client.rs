use game_core::PlayerAction;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JoinRequest {
    pub name: String,
    /// `None` → create a new room.  `Some(code)` → join existing room.
    pub room_code: Option<String>,
    /// True when the connecting client is an AI program.
    pub is_ai: bool,
    /// Simple shared-secret token checked when `ai_api_enabled` is true.
    pub auth_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientMessage {
    /// Must be the first message on any new connection.
    Join(JoinRequest),
    /// Toggle ready state; host sending this also works as a ready toggle.
    SetReady(bool),
    /// Host-only: begins the game once all players are ready.
    StartGame,
    /// In-game: submit a player action.
    Action(PlayerAction),
    /// Keepalive — server responds with `ServerMessage::Pong`.
    Ping,
}
