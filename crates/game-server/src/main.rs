mod lobby;
mod network;
mod server_state;
mod systems;

use bevy::prelude::*;
use bevy_renet::RenetServerPlugin;

use network::create_renet_server;
use server_state::ServerState;
use systems::*;

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(RenetServerPlugin)
        .insert_resource(create_renet_server())
        .insert_resource(ServerState::default())
        .add_systems(Update, handle_server_events)
        .add_systems(Update, receive_client_messages)
        .add_systems(Update, send_game_updates)
        .run();
}
