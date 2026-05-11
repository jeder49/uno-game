use crate::{lobby::LobbyState, view::GameStateView};
use game_core::GameEvent;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMessage {
    /// Sent once after a successful `ClientMessage::Join`.
    Welcome { your_id: Uuid, lobby: LobbyState },

    /// Lobby membership or readiness changed.
    LobbyUpdated(LobbyState),

    /// Game has started; contains the player's private opening hand.
    GameSnapshot(GameStateView),

    /// Something happened in the game — sent to every player in the room.
    Event(GameEvent),

    /// Follows every `Event` with the recipient's updated personal view.
    /// Clients render from this; they do not need to replay events.
    StateUpdate(GameStateView),

    /// The server refused an action the client sent.
    ActionRejected { reason: String },

    /// Unrecoverable error (room closed, kicked, server shutting down).
    Error { message: String },

    /// Response to `ClientMessage::Ping`.
    Pong,
}
