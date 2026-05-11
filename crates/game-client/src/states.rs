use bevy::prelude::*;

#[derive(States, Default, Clone, PartialEq, Eq, Hash, Debug)]
pub enum AppState {
    #[default]
    MainMenu,
    Lobby,
    InGame,
}

/// Only active while `AppState::InGame` is the current state.
#[derive(SubStates, Default, Clone, PartialEq, Eq, Hash, Debug)]
#[source(AppState = AppState::InGame)]
pub enum PauseState {
    #[default]
    Running,
    Paused,
}
