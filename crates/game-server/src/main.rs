mod lobby;
mod session;

use axum::{
    Router,
    extract::ws::WebSocket,
    extract::{State, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
};
use std::sync::Arc;
use tokio::net::TcpListener;

use lobby::SharedLobby;

#[tokio::main]
async fn main() {
    let lobby = Arc::new(SharedLobby::new());

    let app = Router::new().route("/", get(ws_handler)).with_state(lobby);

    let addr = "127.0.0.1:9001";
    println!("game-server listening on ws://{addr}");
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(lobby): State<Arc<SharedLobby>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| session::handle(socket, lobby))
}
