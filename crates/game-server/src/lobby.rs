use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct LobbyManager {
    pub players_waiting: Vec<u64>,
}
