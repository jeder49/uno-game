use std::collections::HashMap;
use std::sync::Mutex;

use game_core::state::GameState;
use game_protocol::{LobbyState, PlayerInfo};
use uuid::Uuid;

/// All mutable state for one server instance (single room for now).
pub struct SharedLobby {
    pub inner: Mutex<LobbyInner>,
}

pub struct LobbyInner {
    pub lobby: LobbyState,
    /// channel senders keyed by player id — used to push messages back to each client
    pub senders: HashMap<Uuid, tokio::sync::mpsc::UnboundedSender<String>>,
    pub game: Option<GameState>,
}

impl SharedLobby {
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(LobbyInner {
                lobby: LobbyState {
                    room_code: "MAIN".into(),
                    players: vec![],
                    max_players: 10,
                    ai_api_enabled: false,
                },
                senders: HashMap::new(),
                game: None,
            }),
        }
    }

    /// Broadcast a serialized message to every connected player.
    pub fn broadcast(&self, msg: &str) {
        let inner = self.inner.lock().unwrap();
        for tx in inner.senders.values() {
            let _ = tx.send(msg.to_string());
        }
    }

    /// Send a message to one specific player.
    pub fn send_to(&self, id: Uuid, msg: &str) {
        let inner = self.inner.lock().unwrap();
        if let Some(tx) = inner.senders.get(&id) {
            let _ = tx.send(msg.to_string());
        }
    }

    /// Add a player to the lobby; returns their assigned PlayerInfo.
    pub fn join(
        &self,
        id: Uuid,
        name: String,
        tx: tokio::sync::mpsc::UnboundedSender<String>,
    ) -> PlayerInfo {
        let mut inner = self.inner.lock().unwrap();
        let is_host = inner.lobby.players.is_empty();
        let info = PlayerInfo {
            id,
            name,
            is_host,
            is_ready: false,
            is_ai: false,
        };
        inner.lobby.players.push(info.clone());
        inner.senders.insert(id, tx);
        info
    }

    /// Remove a player; returns updated lobby.
    pub fn leave(&self, id: Uuid) -> LobbyState {
        let mut inner = self.inner.lock().unwrap();
        inner.lobby.players.retain(|p| p.id != id);
        inner.senders.remove(&id);
        // Promote next player to host if needed
        if !inner.lobby.players.is_empty() && !inner.lobby.players.iter().any(|p| p.is_host) {
            inner.lobby.players[0].is_host = true;
        }
        inner.lobby.clone()
    }

    /// Set ready state for a player; returns updated lobby.
    pub fn set_ready(&self, id: Uuid, ready: bool) -> LobbyState {
        let mut inner = self.inner.lock().unwrap();
        if let Some(p) = inner.lobby.players.iter_mut().find(|p| p.id == id) {
            p.is_ready = ready;
        }
        inner.lobby.clone()
    }
}
