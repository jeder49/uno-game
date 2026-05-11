use bevy::prelude::*;
use game_core::state::GameState;

#[derive(Resource, Default)]
pub struct ServerState {
    pub connected_players: Vec<u64>,
    pub game_state: Option<GameState>,
}
