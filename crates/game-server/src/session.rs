use std::sync::Arc;

use axum::extract::ws::{Message, WebSocket};
use futures_util::{SinkExt, StreamExt};
use game_core::{player::Player, state::GameState};
use game_protocol::{ClientMessage, ServerMessage};
use uuid::Uuid;

use crate::lobby::SharedLobby;

pub async fn handle(socket: WebSocket, lobby: Arc<SharedLobby>) {
    let (mut sink, mut stream) = socket.split();

    // Outbound channel: game logic → this connection
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<String>();

    // Spawn a task that forwards outbound messages to the WebSocket
    let mut send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if sink.send(Message::Text(msg.into())).await.is_err() {
                break;
            }
        }
    });

    // The player id for this session — set after the first Join message
    let mut my_id: Option<Uuid> = None;

    while let Some(Ok(Message::Text(text))) = stream.next().await {
        let Ok(msg) = serde_json::from_str::<ClientMessage>(&text) else {
            continue;
        };

        match msg {
            // ── Join ─────────────────────────────────────────────────────────
            ClientMessage::Join(req) => {
                let id = Uuid::new_v4();
                my_id = Some(id);

                lobby.join(id, req.name, tx.clone());

                // Tell this client who they are and what the lobby looks like
                let lobby_state = lobby.inner.lock().unwrap().lobby.clone();
                let welcome = ServerMessage::Welcome {
                    your_id: id,
                    lobby: lobby_state.clone(),
                };
                send_json(&tx, &welcome);

                // Tell everyone else the lobby changed
                let updated =
                    serde_json::to_string(&ServerMessage::LobbyUpdated(lobby_state)).unwrap();
                lobby.broadcast(&updated);
            }

            // ── Ready toggle ─────────────────────────────────────────────────
            ClientMessage::SetReady(ready) => {
                let Some(id) = my_id else { continue };
                let updated_lobby = lobby.set_ready(id, ready);
                let msg =
                    serde_json::to_string(&ServerMessage::LobbyUpdated(updated_lobby)).unwrap();
                lobby.broadcast(&msg);
            }

            // ── Start game (host only) ────────────────────────────────────────
            ClientMessage::StartGame => {
                let Some(id) = my_id else { continue };
                let mut inner = lobby.inner.lock().unwrap();

                // Only the host can start
                let is_host = inner.lobby.players.iter().any(|p| p.id == id && p.is_host);
                if !is_host {
                    send_json(
                        &tx,
                        &ServerMessage::ActionRejected {
                            reason: "Only the host can start the game".into(),
                        },
                    );
                    continue;
                }

                // Build GameState from lobby players
                let mut rng = rand::thread_rng();
                let mut game = GameState::new();
                for p in &inner.lobby.players {
                    let player = Player::new(p.id, p.name.clone());
                    let _ = game.add_player(player);
                }
                match game.start(&mut rng) {
                    Err(e) => {
                        drop(inner);
                        send_json(
                            &tx,
                            &ServerMessage::ActionRejected {
                                reason: e.to_string(),
                            },
                        );
                    }
                    Ok(_events) => {
                        // Send each player their personal snapshot
                        let player_ids: Vec<Uuid> =
                            inner.lobby.players.iter().map(|p| p.id).collect();
                        inner.game = Some(game);
                        let game_ref = inner.game.as_ref().unwrap();
                        for pid in player_ids {
                            if let Some(view) =
                                game_protocol::GameStateView::for_player(game_ref, pid)
                            {
                                let msg_str =
                                    serde_json::to_string(&ServerMessage::GameSnapshot(view))
                                        .unwrap();
                                if let Some(sender) = inner.senders.get(&pid) {
                                    let _ = sender.send(msg_str);
                                }
                            }
                        }
                    }
                }
            }

            // ── In-game action ────────────────────────────────────────────────
            ClientMessage::Action(action) => {
                let Some(id) = my_id else { continue };
                let mut inner = lobby.inner.lock().unwrap();
                let Some(game) = inner.game.as_mut() else {
                    continue;
                };

                let mut rng = rand::thread_rng();
                match game.apply(id, action, &mut rng) {
                    Err(e) => {
                        drop(inner);
                        send_json(
                            &tx,
                            &ServerMessage::ActionRejected {
                                reason: e.to_string(),
                            },
                        );
                    }
                    Ok(events) => {
                        // Broadcast each event
                        for event in events {
                            let ev_str =
                                serde_json::to_string(&ServerMessage::Event(event)).unwrap();
                            for sender in inner.senders.values() {
                                let _ = sender.send(ev_str.clone());
                            }
                        }
                        // Then send each player their updated view
                        let player_ids: Vec<Uuid> =
                            inner.lobby.players.iter().map(|p| p.id).collect();
                        let game_ref = inner.game.as_ref().unwrap();
                        for pid in player_ids {
                            if let Some(view) =
                                game_protocol::GameStateView::for_player(game_ref, pid)
                            {
                                let msg_str =
                                    serde_json::to_string(&ServerMessage::StateUpdate(view))
                                        .unwrap();
                                if let Some(sender) = inner.senders.get(&pid) {
                                    let _ = sender.send(msg_str);
                                }
                            }
                        }
                    }
                }
            }

            // ── Ping ──────────────────────────────────────────────────────────
            ClientMessage::Ping => {
                send_json(&tx, &ServerMessage::Pong);
            }
        }
    }

    // Client disconnected
    if let Some(id) = my_id {
        let updated = lobby.leave(id);
        let msg = serde_json::to_string(&ServerMessage::LobbyUpdated(updated)).unwrap();
        lobby.broadcast(&msg);
    }

    send_task.abort();
}

fn send_json(tx: &tokio::sync::mpsc::UnboundedSender<String>, msg: &ServerMessage) {
    if let Ok(s) = serde_json::to_string(msg) {
        let _ = tx.send(s);
    }
}
