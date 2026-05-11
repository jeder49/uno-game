pub mod board;
pub mod hand;
pub mod hud;
pub mod pause;

use crate::{
    network::ServerMsg,
    states::{AppState, PauseState},
};
use bevy::prelude::*;
use game_protocol::{GameStateView, ServerMessage};

// ── Shared game resources ──────────────────────────────────────────────────────

/// Set after `ServerMessage::Welcome`; identifies this client's player slot.
#[derive(Resource)]
pub struct LocalPlayer {
    pub id: uuid::Uuid,
}

/// Updated after every `ServerMessage::StateUpdate`.
/// All rendering systems read from here — they never touch `GameState` directly.
#[derive(Resource, Default)]
pub struct LocalGameView(pub Option<GameStateView>);

/// Tracks whether the player has selected a wild card awaiting a colour choice.
#[derive(Resource, Default)]
pub struct HandState {
    pub pending_wild: Option<game_core::Card>,
}

// ── Plugin ────────────────────────────────────────────────────────────────────

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LocalGameView>()
            .init_resource::<HandState>()
            .add_plugins((
                board::BoardPlugin,
                hand::HandPlugin,
                hud::HudPlugin,
                pause::PausePlugin,
            ))
            // Route incoming server events while InGame
            .add_systems(Update, route_server_msgs.run_if(in_state(AppState::InGame)))
            // Toggle pause with Escape
            .add_systems(Update, toggle_pause.run_if(in_state(AppState::InGame)));
    }
}

fn route_server_msgs(mut evs: MessageReader<ServerMsg>, mut local_view: ResMut<LocalGameView>) {
    for ServerMsg(msg) in evs.read() {
        match msg {
            ServerMessage::StateUpdate(view) => {
                local_view.0 = Some(view.clone());
            }
            // ServerMessage::Event — animations / sounds would hook here
            _ => {}
        }
    }
}

fn toggle_pause(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<State<PauseState>>,
    mut next: ResMut<NextState<PauseState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        next.set(match state.get() {
            PauseState::Running => PauseState::Paused,
            PauseState::Paused => PauseState::Running,
        });
    }
}
