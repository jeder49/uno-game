use crossbeam_channel::{Receiver, Sender};
use futures_util::{SinkExt, StreamExt};
use game_protocol::{ClientMessage, ServerMessage};
use tokio_tungstenite::{connect_async, tungstenite::Message};

/// Spawns a background OS thread containing a dedicated tokio runtime.
/// The thread owns the WebSocket connection and bridges it via channels.
pub fn spawn(url: String, outgoing: Receiver<ClientMessage>, incoming: Sender<ServerMessage>) {
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().expect("tokio runtime");
        rt.block_on(run(url, outgoing, incoming));
    });
}

async fn run(url: String, outgoing: Receiver<ClientMessage>, incoming: Sender<ServerMessage>) {
    let (ws, _) = match connect_async(&url).await {
        Ok(pair) => pair,
        Err(e) => {
            eprintln!("[net] connect failed: {e}");
            return;
        }
    };

    let (mut sink, mut stream) = ws.split();

    // Bevy → WebSocket: forward outgoing messages on a separate task.
    let send_task = tokio::spawn(async move {
        loop {
            match outgoing.try_recv() {
                Ok(msg) => {
                    let json = serde_json::to_string(&msg).unwrap();
                    if sink.send(Message::Text(json.into())).await.is_err() {
                        break;
                    }
                }
                Err(crossbeam_channel::TryRecvError::Disconnected) => break,
                Err(crossbeam_channel::TryRecvError::Empty) => {
                    tokio::time::sleep(std::time::Duration::from_millis(4)).await;
                }
            }
        }
    });

    // WebSocket → Bevy: forward inbound messages.
    while let Some(Ok(msg)) = stream.next().await {
        if let Message::Text(text) = msg {
            if let Ok(srv) = serde_json::from_str::<ServerMessage>(&text) {
                if incoming.send(srv).is_err() {
                    break; // Bevy dropped the receiver
                }
            }
        }
    }

    send_task.abort();
}
