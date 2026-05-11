use bevy::prelude::*;
use bevy_renet::renet::RenetServer;

use game_protocol::{ClientMessage, LobbyInfo, ServerMessage};

use crate::network::RELIABLE_CHANNEL_ID;
use crate::server_state::ServerState;

pub fn handle_server_events(mut server: ResMut<RenetServer>) {
    while let Some(event) = server.get_event() {
        println!("Server event: {:?}", event);
    }
}

pub fn receive_client_messages(mut server: ResMut<RenetServer>, mut state: ResMut<ServerState>) {
    for client_id in server.clients_id() {
        while let Some(message) = server.receive_message(client_id, RELIABLE_CHANNEL_ID) {
            let message: ClientMessage =
                bincode::deserialize(&message).expect("Invalid client packet");

            match message {
                ClientMessage::JoinLobby { username } => {
                    println!("{} joined", username);

                    state.connected_players.push(client_id.raw());

                    let response = ServerMessage::LobbyJoined {
                        lobby: LobbyInfo {
                            id: 1,
                            players: vec![username],
                            max_players: 4,
                        },
                    };

                    let serialized = bincode::serialize(&response).unwrap();

                    server.send_message(client_id, RELIABLE_CHANNEL_ID, serialized);
                }

                ClientMessage::PlayCard(action) => {
                    println!("Player action: {:?}", action);
                }

                _ => {}
            }
        }
    }
}

pub fn send_game_updates(mut server: ResMut<RenetServer>) {
    let packet = ServerMessage::Ping;
}
