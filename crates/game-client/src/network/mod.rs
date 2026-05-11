mod socket;

use bevy::prelude::*;
use crossbeam_channel::{Receiver, Sender, bounded};
use game_protocol::{ClientMessage, ServerMessage};

// ── Events ────────────────────────────────────────────────────────────────────

/// Fire to open a WebSocket connection to the given URL.
#[derive(Message)]
pub struct ConnectCmd {
    pub url: String,
}

/// Received server message — fired once per frame per inbound message.
#[derive(Message)]
pub struct ServerMsg(pub ServerMessage);

/// Write this to send a message to the server.
#[derive(Message)]
pub struct SendMsg(pub ClientMessage);

// ── Resources ─────────────────────────────────────────────────────────────────

#[derive(Resource)]
pub struct NetChannels {
    pub tx: Sender<ClientMessage>,
    pub rx: Receiver<ServerMessage>,
}

#[derive(Resource, Default, PartialEq, Eq, Clone, Debug)]
pub enum NetStatus {
    #[default]
    Disconnected,
    Connecting,
    Connected,
    Error(String),
}

// ── Plugin ────────────────────────────────────────────────────────────────────

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NetStatus>()
            .add_message::<ConnectCmd>()
            .add_message::<ServerMsg>()
            .add_message::<SendMsg>()
            .add_systems(Update, (on_connect_cmd, poll_rx, flush_tx));
    }
}

// ── Systems ───────────────────────────────────────────────────────────────────

fn on_connect_cmd(
    mut evs: MessageReader<ConnectCmd>,
    mut commands: Commands,
    mut status: ResMut<NetStatus>,
) {
    for ConnectCmd { url } in evs.read() {
        let (out_tx, out_rx) = bounded::<ClientMessage>(64);
        let (in_tx, in_rx) = bounded::<ServerMessage>(64);

        socket::spawn(url.clone(), out_rx, in_tx);

        commands.insert_resource(NetChannels {
            tx: out_tx,
            rx: in_rx,
        });
        *status = NetStatus::Connecting;
    }
}

fn poll_rx(
    channels: Option<Res<NetChannels>>,
    mut evs: MessageWriter<ServerMsg>,
    mut status: ResMut<NetStatus>,
) {
    let Some(ch) = channels else { return };
    loop {
        match ch.rx.try_recv() {
            Ok(msg) => {
                if *status == NetStatus::Connecting {
                    *status = NetStatus::Connected;
                }
                evs.write(ServerMsg(msg));
            }
            Err(crossbeam_channel::TryRecvError::Empty) => break,
            Err(crossbeam_channel::TryRecvError::Disconnected) => {
                *status = NetStatus::Disconnected;
                break;
            }
        }
    }
}

fn flush_tx(channels: Option<Res<NetChannels>>, mut evs: MessageReader<SendMsg>) {
    let Some(ch) = channels else { return };
    for SendMsg(msg) in evs.read() {
        let _ = ch.tx.try_send(msg.clone());
    }
}
